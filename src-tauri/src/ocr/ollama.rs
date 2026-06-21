use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
struct GenerateRequest {
    model: String,
    prompt: String,
    images: Vec<String>,
    stream: bool,
}

pub async fn ocr(base64_img: &str, keys: &HashMap<String, String>) -> Result<String, String> {
    let base_url = keys
        .get("ollama_ocr_base_url")
        .filter(|s| !s.is_empty())
        .map(|s| s.trim_end_matches('/'))
        .ok_or_else(|| "Ollama OCR 接口地址未配置".to_string())?;

    let model = keys
        .get("ollama_ocr_model")
        .filter(|s| !s.is_empty())
        .ok_or_else(|| "Ollama OCR 模型名未配置".to_string())?;

    let body = GenerateRequest {
        model: model.to_string(),
        prompt: "Extract all text from this image. Return only the recognized text, no explanations.".to_string(),
        images: vec![base64_img.to_string()],
        stream: false,
    };

    let client = reqwest::Client::new();
    let resp = client
        .post(format!("{}/api/generate", base_url))
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("Ollama OCR 请求失败: {}", e))?;

    let status = resp.status();
    let resp_body: serde_json::Value = resp
        .json()
        .await
        .map_err(|e| format!("解析 Ollama OCR 响应失败: {}", e))?;

    if !status.is_success() {
        let msg = resp_body["error"].as_str().unwrap_or("未知错误");
        return Err(format!("Ollama OCR 错误 ({}): {}", status, msg));
    }

    resp_body["response"]
        .as_str()
        .map(|s| s.trim().to_string())
        .ok_or_else(|| "Ollama OCR 返回结果为空".to_string())
}
