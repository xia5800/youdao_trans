use serde::{Deserialize, Serialize};

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

const ENDPOINT: &str = "https://api.cognitive.microsofttranslator.com/translate";
const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36";

pub async fn translate(
    text: &str,
    source_lang: Option<&str>,
    target_lang: &str,
    keys: &std::collections::HashMap<String, String>,
) -> Result<String, String> {
    let api_key = keys
        .get("microsoft")
        .filter(|s| !s.is_empty())
        .ok_or_else(|| "Microsoft Translator API Key 未配置".to_string())?;
    let region = keys.get("microsoft_region").filter(|s| !s.is_empty());

    let items = vec![RequestItem { text: text.to_string() }];

    let mut params = vec![("api-version", "3.0"), ("to", target_lang)];
    if let Some(sl) = source_lang {
        params.push(("from", sl));
    }

    let client = reqwest::Client::builder()
        .http1_only()
        .build()
        .map_err(|e| format!("failed to build client: {}", e))?;

    let mut request = client
        .post(ENDPOINT)
        .query(&params)
        .header("Content-Type", "application/json")
        .header("User-Agent", USER_AGENT)
        .header("Ocp-Apim-Subscription-Key", api_key.as_str())
        .json(&items);

    if let Some(r) = region {
        request = request.header("Ocp-Apim-Subscription-Region", r.as_str());
    }

    let response = request.send().await.map_err(|e| format!("request failed: {}", e))?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
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
