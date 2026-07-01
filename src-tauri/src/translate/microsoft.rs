use crate::util;
use super::microsoft_common::{RequestItem, ResponseItem, USER_AGENT};

const ENDPOINT: &str = "https://api.cognitive.microsofttranslator.com/translate";

pub async fn translate(
    text: &str,
    source_lang: Option<&str>,
    target_lang: &str,
    keys: &std::collections::HashMap<String, String>,
) -> Result<String, String> {
    let api_key = util::require_key(keys, "microsoft", "Microsoft Translator API Key 未配置")?;
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
        .header("Ocp-Apim-Subscription-Key", api_key)
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

    let results: Vec<ResponseItem> = util::parse_json(response, "Microsoft翻译").await?;

    results
        .first()
        .and_then(|r| r.translations.first())
        .map(|t| t.text.clone())
        .ok_or_else(|| "no translation result".to_string())
}
