use crate::util;
use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
use hmac::{Hmac, Mac};
use sha1::Sha1;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

type HmacSha1 = Hmac<Sha1>;

const ENDPOINT: &str = "https://mt.aliyuncs.com/";

pub async fn translate(
    text: &str,
    source_lang: Option<&str>,
    target_lang: &str,
    keys: &HashMap<String, String>,
) -> Result<String, String> {
    let access_key = util::require_key(keys, "ali-accesskey", "阿里翻译 AccessKey 未配置")?;
    let secret_key = util::require_key(keys, "ali-secretkey", "阿里翻译 SecretKey 未配置")?;

    let now = SystemTime::now();
    let nonce = format!(
        "{}",
        now.duration_since(UNIX_EPOCH)
            .map_err(|e| format!("时间错误: {}", e))?
            .as_nanos()
    );
    let ts = utc_timestamp(&now)?;

    let mut params = vec![
        ("AccessKeyId", access_key),
        ("Action", "TranslateGeneral"),
        ("Format", "JSON"),
        ("FormatType", "text"),
        ("Scene", "general"),
        ("SignatureMethod", "HMAC-SHA1"),
        ("SignatureNonce", nonce.as_str()),
        ("SignatureVersion", "1.0"),
        ("SourceText", text),
        ("TargetLanguage", target_lang),
        ("Timestamp", ts.as_str()),
        ("Version", "2018-10-12"),
    ];

    params.push(("SourceLanguage", source_lang.unwrap_or("auto")));

    params.sort_by(|a, b| a.0.cmp(b.0));

    let canonicalized = params
        .iter()
        .map(|(k, v)| format!("{}={}", percent_encode(k), percent_encode(v)))
        .collect::<Vec<_>>()
        .join("&");

    let string_to_sign = format!("POST&%2F&{}", percent_encode(&canonicalized));

    let mut mac =
        HmacSha1::new_from_slice(format!("{}&", secret_key).as_bytes())
            .map_err(|e| format!("HMAC 初始化失败: {}", e))?;
    mac.update(string_to_sign.as_bytes());
    let signature = BASE64.encode(mac.finalize().into_bytes());

    params.push(("Signature", signature.as_str()));

    let client = reqwest::Client::new();

    let response = client
        .post(ENDPOINT)
        .form(&params)
        .send()
        .await
        .map_err(|e| format!("请求阿里翻译失败: {}", e))?;

    let body: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("解析阿里翻译响应失败: {}", e))?;

    let code = body["Code"]
        .as_str()
        .or_else(|| body["code"].as_str())
        .unwrap_or("unknown");

    if code != "200" {
        let msg = body["Message"]
            .as_str()
            .or_else(|| body["message"].as_str())
            .unwrap_or("未知错误");
        return Err(format!("阿里翻译错误 ({}): {}", code, msg));
    }

    body["Data"]["Translated"]
        .as_str()
        .or_else(|| body["data"]["translated"].as_str())
        .or_else(|| body["Data"]["translated"].as_str())
        .or_else(|| body["data"]["Translated"].as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| "阿里翻译返回结果为空".to_string())
}

fn percent_encode(s: &str) -> String {
    let mut result = String::with_capacity(s.len() * 3);
    for &byte in s.as_bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                result.push(byte as char);
            }
            b' ' => result.push_str("%20"),
            _ => result.push_str(&format!("%{:02X}", byte)),
        }
    }
    result
}

fn utc_timestamp(now: &SystemTime) -> Result<String, String> {
    let secs = now
        .duration_since(UNIX_EPOCH)
        .map_err(|e| format!("时间错误: {}", e))?
        .as_secs() as i64;
    let dt = time::OffsetDateTime::from_unix_timestamp(secs)
        .map_err(|e| format!("时间转换错误: {}", e))?;
    #[allow(deprecated)]
    let fmt = time::format_description::parse("[year]-[month]-[day]T[hour]:[minute]:[second]Z")
        .map_err(|e| format!("日期格式解析失败: {}", e))?;
    dt.format(&fmt).map_err(|e| format!("日期格式化失败: {}", e))
}
