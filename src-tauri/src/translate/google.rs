use crate::util;

const ENDPOINT: &str = "https://translate.googleapis.com/translate_a/single";

pub async fn translate(
    text: &str,
    source_lang: Option<&str>,
    target_lang: &str,
    _keys: &std::collections::HashMap<String, String>,
) -> Result<String, String> {
    let client = util::http_client();

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

    util::check_status(response.status(), "Google翻译")?;
    let body: serde_json::Value = util::parse_json(response, "Google翻译").await?;

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
