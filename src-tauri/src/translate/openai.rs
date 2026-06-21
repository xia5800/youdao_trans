use serde::Serialize;

const DEFAULT_MODEL: &str = "gpt-4o-mini";
const DEFAULT_ENDPOINT: &str = "https://api.openai.com/v1/chat/completions";

#[derive(Serialize)]
struct Message {
    role: &'static str,
    content: String,
}

#[derive(Serialize)]
struct RequestBody {
    model: String,
    messages: Vec<Message>,
}

pub async fn translate(
    text: &str,
    source_lang: Option<&str>,
    target_lang: &str,
    keys: &std::collections::HashMap<String, String>,
) -> Result<String, String> {
    let api_key = keys
        .get("openai_key")
        .filter(|s| !s.is_empty())
        .ok_or_else(|| "OpenAI API Key 未配置".to_string())?;

    let model = keys
        .get("openai_model")
        .filter(|s| !s.is_empty())
        .map(|s| s.as_str())
        .unwrap_or(DEFAULT_MODEL);

    let endpoint = keys
        .get("openai_endpoint")
        .filter(|s| !s.is_empty())
        .map(|s| s.as_str())
        .unwrap_or(DEFAULT_ENDPOINT);

    let source = source_lang.unwrap_or("auto");
    let system_prompt = format!(
        "You are a professional translator. Translate the following text from {} to {}. Only return the translation, no explanations or notes.",
        source, target_lang
    );

    let body = RequestBody {
        model: model.to_string(),
        messages: vec![
            Message { role: "system", content: system_prompt },
            Message { role: "user", content: text.to_string() },
        ],
    };

    let client = reqwest::Client::new();
    let resp = client
        .post(endpoint)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("OpenAI 请求失败: {}", e))?;

    let status = resp.status();
    let resp_body: serde_json::Value = resp
        .json()
        .await
        .map_err(|e| format!("解析 OpenAI 响应失败: {}", e))?;

    if !status.is_success() {
        let msg = resp_body["error"]["message"].as_str().unwrap_or("未知错误");
        return Err(format!("OpenAI 错误 ({}): {}", status, msg));
    }

    resp_body["choices"][0]["message"]["content"]
        .as_str()
        .map(|s| s.trim().to_string())
        .ok_or_else(|| "OpenAI 返回结果为空".to_string())
}
