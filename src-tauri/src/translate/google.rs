const ENDPOINT: &str = "https://translate.googleapis.com/translate_a/single";

pub async fn translate(
    text: &str,
    source_lang: Option<&str>,
    target_lang: &str,
    _keys: &std::collections::HashMap<String, String>,
) -> Result<String, String> {
    let client = reqwest::Client::new();

    let sl = source_lang.unwrap_or("auto");

    let response = client
        .get(ENDPOINT)
        .query(&[
            ("client", "gtx"),
            ("sl", sl),
            ("tl", target_lang),
            ("dt", "t"),
            ("q", text),
        ])
        .send()
        .await
        .map_err(|e| format!("Google translate request failed: {}", e))?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(format!("Google translation failed (HTTP {}): {}", status, body));
    }

    let body: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("failed to parse response: {}", e))?;

    let segments = body[0]
        .as_array()
        .ok_or_else(|| "unexpected response format".to_string())?;

    let mut result = String::new();
    for segment in segments {
        if let Some(translation) = segment[0].as_str() {
            result.push_str(translation);
        }
    }

    if result.is_empty() {
        return Err("empty translation result".to_string());
    }

    Ok(result)
}
