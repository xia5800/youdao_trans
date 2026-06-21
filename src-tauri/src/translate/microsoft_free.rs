use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::time::{Duration, Instant};

#[derive(Serialize)]
struct RequestItem {
    #[serde(rename = "Text")]
    text: String,
}

#[derive(Deserialize)]
struct ResponseItem {
    translations: Vec<TranslationItem>,
}

#[derive(Deserialize)]
struct TranslationItem {
    text: String,
}

const TOKEN_URL: &str = "https://edge.microsoft.com/translate/auth";
const ENDPOINT: &str = "https://api-edge.cognitive.microsofttranslator.com/translate";
const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36";

static TOKEN_CACHE: Mutex<Option<(String, Instant)>> = Mutex::new(None);

async fn get_free_token() -> Result<String, String> {
    {
        let cache = TOKEN_CACHE.lock().map_err(|e| format!("lock error: {}", e))?;
        if let Some((token, expiry)) = &*cache {
            if *expiry > Instant::now() {
                return Ok(token.clone());
            }
        }
    }

    let client = reqwest::Client::new();
    let response = client
        .get(TOKEN_URL)
        .header("User-Agent", USER_AGENT)
        .send()
        .await
        .map_err(|e| format!("failed to get auth token: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("auth token request failed (HTTP {})", response.status()));
    }

    let token = response.text().await.map_err(|e| format!("failed to read auth response: {}", e))?;

    let mut cache = TOKEN_CACHE.lock().map_err(|e| format!("lock error: {}", e))?;
    *cache = Some((token.clone(), Instant::now() + Duration::from_secs(crate::constants::TOKEN_CACHE_TTL_SECS)));
    Ok(token)
}

pub async fn translate(
    text: &str,
    source_lang: Option<&str>,
    target_lang: &str,
    _keys: &std::collections::HashMap<String, String>,
) -> Result<String, String> {
    let items = vec![RequestItem { text: text.to_string() }];

    let mut params = vec![("api-version", "3.0"), ("to", target_lang)];
    if let Some(sl) = source_lang {
        params.push(("from", sl));
    }

    let client = reqwest::Client::builder()
        .http1_only()
        .build()
        .map_err(|e| format!("failed to build client: {}", e))?;

    let token = get_free_token().await?;
    let request = client
        .post(ENDPOINT)
        .query(&params)
        .header("Content-Type", "application/json")
        .header("User-Agent", USER_AGENT)
        .header("Authorization", format!("Bearer {}", token))
        .json(&items);

    let response = request.send().await.map_err(|e| format!("request failed: {}", e))?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        if status == reqwest::StatusCode::UNAUTHORIZED {
            let mut cache = TOKEN_CACHE.lock().map_err(|_| "lock error".to_string())?;
            *cache = None;
        }
        return Err(format!("Microsoft translation failed (HTTP {}): {}", status, body));
    }

    let results: Vec<ResponseItem> = response
        .json()
        .await
        .map_err(|e| format!("failed to parse response: {}", e))?;

    results
        .first()
        .and_then(|r| r.translations.first())
        .map(|t| t.text.clone())
        .ok_or_else(|| "no translation result".to_string())
}
