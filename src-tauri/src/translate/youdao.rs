use crate::util;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Deserialize)]
struct YoudaoResponse {
    translation: Option<Vec<String>>,
    #[serde(rename = "errorCode")]
    error_code: String,
}

const ENDPOINT: &str = "https://openapi.youdao.com/api";

fn map_lang(code: &str) -> &str {
    match code {
        "zh" => "zh-CHS",
        other => other,
    }
}

fn truncate(text: &str) -> String {
    let len = text.chars().count();
    if len <= 20 {
        text.to_string()
    } else {
        let first10: String = text.chars().take(10).collect();
        let last10: String = text.chars().skip(len - 10).take(10).collect();
        format!("{}{}{}", first10, len, last10)
    }
}

pub async fn translate(
    text: &str,
    source_lang: Option<&str>,
    target_lang: &str,
    keys: &std::collections::HashMap<String, String>,
) -> Result<String, String> {
    let app_key = util::require_key(keys, "youdao_appid", "有道翻译 appId 未配置")?;
    let app_secret = util::require_key(keys, "youdao_appsecret", "有道翻译 appSecret 未配置")?;

    let from = source_lang.map(map_lang).unwrap_or("auto");
    let to = map_lang(target_lang);

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| format!("time error: {}", e))?;

    let salt = now.as_millis().to_string();
    let curtime = now.as_secs().to_string();

    let sign_input = format!("{}{}{}{}{}", app_key, truncate(text), salt, curtime, app_secret);
    let sign = format!("{:x}", Sha256::digest(sign_input.as_bytes()));

    let client = reqwest::Client::new();
    let params = [
        ("q", text),
        ("from", from),
        ("to", to),
        ("appKey", app_key),
        ("salt", &salt),
        ("sign", &sign),
        ("signType", "v3"),
        ("curtime", &curtime),
    ];

    let response = client
        .post(ENDPOINT)
        .form(&params)
        .send()
        .await
        .map_err(|e| format!("请求有道翻译失败: {}", e))?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(format!("有道翻译请求失败 (HTTP {}): {}", status, body));
    }

    let body: YoudaoResponse = response
        .json()
        .await
        .map_err(|e| format!("解析有道翻译响应失败: {}", e))?;

    if body.error_code != "0" {
        return Err(format!("有道翻译错误 ({}): 请检查参数或配额", body.error_code));
    }

    body.translation
        .and_then(|mut t| t.pop())
        .ok_or_else(|| "有道翻译返回结果为空".to_string())
}
