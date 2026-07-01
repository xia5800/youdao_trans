use crate::util;
use serde::Serialize;

#[derive(Serialize)]
struct GenerateRequest {
    model: String,
    prompt: String,
    stream: bool,
}

pub async fn translate(
    text: &str,
    source_lang: Option<&str>,
    target_lang: &str,
    keys: &std::collections::HashMap<String, String>,
) -> Result<String, String> {
    let base_url = util::require_key(keys, "ollama_base_url", "Ollama 接口地址未配置")?.trim_end_matches('/');

    let model = util::require_key(keys, "ollama_model", "Ollama 模型名未配置")?;

    let source = source_lang.unwrap_or("auto");
    let prompt = format!(
        "Translate the following text from {} to {}. Return only the translation, no explanations.\n\n{}",
        source, target_lang, text
    );

    let body = GenerateRequest {
        model: model.to_string(),
        prompt,
        stream: false,
    };

    let client = util::http_client();
    let resp = client
        .post(format!("{}/api/generate", base_url))
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("Ollama 请求失败: {}", e))?;

    util::check_status(resp.status(), "Ollama")?;

    let resp_body: serde_json::Value = util::parse_json(resp, "Ollama").await?;

    if let Some(msg) = resp_body["error"].as_str() {
        return Err(format!("Ollama 错误: {}", msg));
    }

    util::or_empty(
        resp_body["response"].as_str().map(|s| s.trim().to_string()),
        "Ollama",
    )
}
