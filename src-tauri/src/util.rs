use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn require_key<'a>(keys: &'a HashMap<String, String>, key: &str, msg: &str) -> Result<&'a str, String> {
    keys.get(key)
        .filter(|s| !s.is_empty())
        .map(|s| s.as_str())
        .ok_or_else(|| msg.to_string())
}

pub fn http_client() -> reqwest::Client {
    reqwest::Client::new()
}

pub fn unix_millis() -> Result<u128, String> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis())
        .map_err(|e| format!("time error: {}", e))
}

pub fn unix_secs() -> Result<u64, String> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .map_err(|e| format!("time error: {}", e))
}

pub async fn parse_json<T: serde::de::DeserializeOwned>(response: reqwest::Response, ctx: &str) -> Result<T, String> {
    response
        .json()
        .await
        .map_err(|e| format!("解析{}响应失败: {}", ctx, e))
}

pub fn check_status(status: reqwest::StatusCode, ctx: &str) -> Result<(), String> {
    if status.is_success() {
        Ok(())
    } else {
        Err(format!("{}请求失败 (HTTP {})", ctx, status))
    }
}

pub fn or_empty<T>(opt: Option<T>, name: &str) -> Result<T, String> {
    opt.ok_or_else(|| format!("{}返回结果为空", name))
}
