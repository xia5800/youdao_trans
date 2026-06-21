use md5::{Md5, Digest};
use serde::Deserialize;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Deserialize)]
struct BaiduResponse {
    trans_result: Option<Vec<TranslationItem>>,
    error_code: Option<String>,
    error_msg: Option<String>,
}

#[derive(Deserialize)]
struct TranslationItem {
    dst: String,
}

const ENDPOINT: &str = "https://fanyi-api.baidu.com/api/trans/vip/translate";

fn map_lang(code: &str) -> &str {
    match code {
        "ja" => "jp",
        "ko" => "kor",
        "fr" => "fra",
        "es" => "spa",
        other => other,
    }
}

pub async fn translate(
    text: &str,
    source_lang: Option<&str>,
    target_lang: &str,
    keys: &std::collections::HashMap<String, String>,
) -> Result<String, String> {
    let appid = keys
        .get("baidu_appid")
        .filter(|s| !s.is_empty())
        .ok_or_else(|| "百度翻译 appid 未配置".to_string())?;
    let key = keys
        .get("baidu_appkey")
        .filter(|s| !s.is_empty())
        .ok_or_else(|| "百度翻译 appkey 未配置".to_string())?;

    let salt = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| format!("time error: {}", e))?
        .as_millis()
        .to_string();

    let sign_input = format!("{}{}{}{}", appid, text, salt, key);
    let sign = format!("{:x}", Md5::digest(sign_input.as_bytes()));

    let from = source_lang.map(map_lang).unwrap_or("auto");
    let to = map_lang(target_lang);

    let client = reqwest::Client::new();
    let params = [
        ("q", text),
        ("from", from),
        ("to", to),
        ("appid", appid),
        ("salt", &salt),
        ("sign", &sign),
    ];

    let response = client
        .post(ENDPOINT)
        .form(&params)
        .send()
        .await
        .map_err(|e| format!("请求百度翻译失败: {}", e))?;

    let body: BaiduResponse = response
        .json()
        .await
        .map_err(|e| format!("解析百度翻译响应失败: {}", e))?;

    if let Some(code) = body.error_code {
        let msg = body.error_msg.unwrap_or_default();
        return Err(format!("百度翻译错误 ({}): {}", code, msg));
    }

    body.trans_result
        .and_then(|mut r| r.pop())
        .map(|t| t.dst)
        .ok_or_else(|| "百度翻译返回结果为空".to_string())
}
