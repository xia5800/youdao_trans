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
    let base_url = keys
        .get("ollama_base_url")
        .filter(|s| !s.is_empty())
        .map(|s| s.trim_end_matches('/'))
        .ok_or_else(|| "Ollama 接口地址未配置".to_string())?;

    let model = keys
        .get("ollama_model")
        .filter(|s| !s.is_empty())
        .ok_or_else(|| "Ollama 模型名未配置".to_string())?;

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

    let client = reqwest::Client::new();
    let resp = client
        .post(format!("{}/api/generate", base_url))
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("Ollama 请求失败: {}", e))?;

    let status = resp.status();
    let resp_body: serde_json::Value = resp
        .json()
        .await
        .map_err(|e| format!("解析 Ollama 响应失败: {}", e))?;

    if !status.is_success() {
        let msg = resp_body["error"].as_str().unwrap_or("未知错误");
        return Err(format!("Ollama 错误 ({}): {}", status, msg));
    }

    resp_body["response"]
        .as_str()
        .map(|s| s.trim().to_string())
        .ok_or_else(|| "Ollama 返回结果为空".to_string())
}
