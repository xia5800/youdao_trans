use crate::util;
use base64::engine::general_purpose;
use base64::Engine;
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::collections::HashMap;
use std::fmt::Write;
#[allow(deprecated)]
use time::format_description::parse;
use time::OffsetDateTime;

type HmacSha256 = Hmac<Sha256>;

const HOST: &str = "cbm01.cn-huabei-1.xf-yun.com";
const PATH: &str = "/v1/private/se75ocrbm";

#[derive(Serialize)]
struct OcrRequest {
    header: OcrHeader,
    parameter: OcrParameter,
    payload: OcrPayload,
}

#[derive(Serialize)]
struct OcrHeader {
    app_id: String,
    status: u32,
}

#[derive(Serialize)]
struct OcrParameter {
    ocr: OcrConfig,
}

#[derive(Serialize)]
struct OcrConfig {
    #[serde(rename = "result_format")]
    result_format: String,
    #[serde(rename = "output_type")]
    output_type: String,
    result: OcrResultConfig,
}

#[derive(Serialize)]
struct OcrResultConfig {
    encoding: String,
    compress: String,
    format: String,
}

#[derive(Serialize)]
struct OcrPayload {
    image: OcrImage,
}

#[derive(Serialize)]
struct OcrImage {
    encoding: String,
    image: String,
    status: u32,
}

#[derive(Deserialize)]
struct OcrResponse {
    header: OcrResponseHeader,
    payload: Option<OcrResponsePayload>,
}

#[derive(Deserialize)]
struct OcrResponseHeader {
    code: i32,
    message: String,
}

#[derive(Deserialize)]
struct OcrResponsePayload {
    result: Option<OcrResponseResult>,
}

#[derive(Deserialize)]
struct OcrResponseResult {
    text: Option<String>,
}

fn rfc1123_date() -> Result<String, String> {
    let now = OffsetDateTime::now_utc();
    #[allow(deprecated)]
    let fmt = parse("[weekday repr:short], [day] [month repr:short] [year] [hour]:[minute]:[second] GMT")
        .map_err(|e| format!("日期格式解析失败: {}", e))?;
    now.format(&fmt).map_err(|e| format!("日期格式化失败: {}", e))
}

fn build_signature(api_secret: &str, date: &str) -> Result<String, String> {
    let sig_origin = format!("host: {}\ndate: {}\nPOST {} HTTP/1.1", HOST, date, PATH);
    let mut mac = HmacSha256::new_from_slice(api_secret.as_bytes())
        .map_err(|e| format!("HMAC初始化失败: {}", e))?;
    mac.update(sig_origin.as_bytes());
    let code = mac.finalize().into_bytes();
    Ok(general_purpose::STANDARD.encode(&code))
}

fn url_encode(s: &str) -> String {
    let mut out = String::with_capacity(s.len() * 3);
    for byte in s.as_bytes() {
        match *byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'.' | b'-' | b'~' | b'_' => out.push(*byte as char),
            b' ' => out.push_str("%20"),
            _ => { let _ = write!(out, "%{:02X}", byte); }
        }
    }
    out
}

pub async fn ocr(base64_img: &str, keys: &HashMap<String, String>) -> Result<String, String> {
    let app_id = util::require_key(keys, "xunfei-appid", "讯飞OCR AppId 未配置")?;
    let api_key = util::require_key(keys, "xunfei-apikey", "讯飞OCR API Key 未配置")?;
    let api_secret = util::require_key(keys, "xunfei-apisecret", "讯飞OCR Secret Key 未配置")?;

    let date = rfc1123_date()?;
    let signature = build_signature(api_secret, &date)?;

    let auth_origin = format!(
        r#"api_key="{}", algorithm="hmac-sha256", headers="host date request-line", signature="{}""#,
        api_key, signature
    );
    let authorization = general_purpose::STANDARD.encode(auth_origin);

    let url = format!(
        "https://{}{}?authorization={}&host={}&date={}",
        HOST, PATH, url_encode(&authorization), HOST, url_encode(&date)
    );

    let body = OcrRequest {
        header: OcrHeader {
            app_id: app_id.to_string(),
            status: 0,
        },
        parameter: OcrParameter {
            ocr: OcrConfig {
                result_format: "json".to_string(),
                output_type: "one_shot".to_string(),
                result: OcrResultConfig {
                    encoding: "utf8".to_string(),
                    compress: "raw".to_string(),
                    format: "json".to_string(),
                },
            },
        },
        payload: OcrPayload {
            image: OcrImage {
                encoding: "png".to_string(),
                image: base64_img.to_string(),
                status: 0,
            },
        },
    };

    let body_json = serde_json::to_string(&body).map_err(|e| format!("请求序列化失败: {}", e))?;

    let client = reqwest::Client::new();
    let resp = client.post(&url)
        .header("Content-Type", "application/json")
        .body(body_json)
        .send()
        .await
        .map_err(|e| format!("请求讯飞OCR失败: {}", e))?;

    let status = resp.status();
    let resp_text = resp.text().await.map_err(|e| format!("读取响应失败: {}", e))?;

    if !status.is_success() {
        return Err(format!("讯飞OCR HTTP错误 ({}): {}", status, resp_text));
    }

    let ocr_resp: OcrResponse = serde_json::from_str(&resp_text)
        .map_err(|e| format!("解析讯飞OCR响应失败: {}", e))?;

    if ocr_resp.header.code != 0 {
        return Err(format!("讯飞OCR错误 ({}): {}", ocr_resp.header.code, ocr_resp.header.message));
    }

    let text_b64 = ocr_resp.payload.and_then(|p| p.result).and_then(|r| r.text)
        .ok_or_else(|| "讯飞OCR返回结果为空".to_string())?;

    let text_json_str = String::from_utf8(
        general_purpose::STANDARD.decode(&text_b64)
            .map_err(|e| format!("base64解码结果失败: {}", e))?
    ).map_err(|e| format!("结果UTF-8解码失败: {}", e))?;

    let text_val: serde_json::Value = serde_json::from_str(&text_json_str)
        .map_err(|e| format!("解析OCR结果JSON失败: {}", e))?;

    let lines = extract_text_lines(&text_val);
    if lines.is_empty() {
        return Err("讯飞OCR未识别到文字".to_string());
    }
    Ok(lines.join("\n"))
}

fn extract_text_lines(val: &serde_json::Value) -> Vec<String> {
    let mut texts = Vec::new();
    if let Some(obj) = val.as_object() {
        if obj.get("type").and_then(|t| t.as_str()) == Some("text_block") {
            if let Some(arr) = obj.get("text").and_then(|t| t.as_array()) {
                for t in arr {
                    if let Some(s) = t.as_str() {
                        texts.push(s.to_string());
                    }
                }
            }
        } else {
            for v in obj.values() {
                texts.extend(extract_text_lines(v));
            }
        }
    } else if let Some(arr) = val.as_array() {
        for v in arr {
            texts.extend(extract_text_lines(v));
        }
    }
    texts
}
