use crate::util;
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;

type HmacSha256 = Hmac<Sha256>;

const SERVICE: &str = "ocr";
const HOST: &str = "ocr.tencentcloudapi.com";
const ACTION: &str = "RecognizeAgent";
const VERSION: &str = "2018-11-19";

#[derive(Serialize)]
struct TencentRequest {
    #[serde(rename = "QueryType")]
    query_type: u32,
    #[serde(rename = "ImageBase64")]
    image_base64: String,
}

#[derive(Deserialize)]
struct TencentResponse {
    #[serde(rename = "Response")]
    response: TencentResponseBody,
}

#[derive(Deserialize)]
struct TencentResponseBody {
    #[serde(rename = "Response")]
    response: Option<Vec<TencentResponseItem>>,
    #[serde(rename = "Error")]
    error: Option<TencentError>,
}

#[derive(Deserialize)]
struct TencentResponseItem {
    #[serde(rename = "TextDetections")]
    text_detections: Option<Vec<TextDetection>>,
}

#[derive(Deserialize)]
struct TextDetection {
    #[serde(rename = "DetectedText")]
    detected_text: String,
}

#[derive(Deserialize)]
struct TencentError {
    #[serde(rename = "Code")]
    code: String,
    #[serde(rename = "Message")]
    message: String,
}

fn sha256_hex(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hex_encode(&hasher.finalize())
}

fn hmac_sha256(key: &[u8], data: &[u8]) -> Vec<u8> {
    let mut mac = HmacSha256::new_from_slice(key).expect("HMAC key");
    mac.update(data);
    mac.finalize().into_bytes().to_vec()
}

fn hex_encode(data: &[u8]) -> String {
    let mut s = String::with_capacity(data.len() * 2);
    for byte in data {
        s.push_str(&format!("{:02x}", byte));
    }
    s
}

fn build_signature(secret_id: &str, secret_key: &str, timestamp: u64, body_json: &str) -> Result<String, String> {
    let dt = time::OffsetDateTime::from_unix_timestamp(timestamp as i64)
        .map_err(|e| format!("时间转换错误: {}", e))?;
    #[allow(deprecated)]
    let fmt = time::format_description::parse("[year]-[month]-[day]")
        .map_err(|e| format!("日期格式解析失败: {}", e))?;
    let date = dt.format(&fmt).map_err(|e| format!("日期格式化失败: {}", e))?;

    let payload_hash = sha256_hex(body_json.as_bytes());

    let canonical_request = format!(
        "POST\n/\n\ncontent-type:application/json; charset=utf-8\nhost:{}\nx-tc-action:{}\n\ncontent-type;host;x-tc-action\n{}",
        HOST, ACTION.to_lowercase(), payload_hash
    );
    let canonical_request_hash = sha256_hex(canonical_request.as_bytes());

    let credential_scope = format!("{}/{}/tc3_request", date, SERVICE);
    let string_to_sign = format!(
        "TC3-HMAC-SHA256\n{}\n{}\n{}",
        timestamp, credential_scope, canonical_request_hash
    );

    let secret = format!("TC3{}", secret_key);
    let secret_date = hmac_sha256(secret.as_bytes(), date.as_bytes());
    let secret_service = hmac_sha256(&secret_date, SERVICE.as_bytes());
    let secret_signing = hmac_sha256(&secret_service, b"tc3_request");
    let signature = hmac_sha256(&secret_signing, string_to_sign.as_bytes());
    let signature_hex = hex_encode(&signature);

    let authorization = format!(
        "TC3-HMAC-SHA256 Credential={}/{}, SignedHeaders=content-type;host;x-tc-action, Signature={}",
        secret_id, credential_scope, signature_hex
    );

    Ok(authorization)
}

pub async fn ocr(base64_img: &str, keys: &HashMap<String, String>) -> Result<String, String> {
    let secret_id = util::require_key(keys, "tencent-secretid", "腾讯云OCR SecretId 未配置")?;
    let secret_key = util::require_key(keys, "tencent-secretkey", "腾讯云OCR SecretKey 未配置")?;

    let body = TencentRequest {
        query_type: 0,
        image_base64: base64_img.to_string(),
    };
    let body_json = serde_json::to_string(&body).map_err(|e| format!("请求序列化失败: {}", e))?;

    let timestamp = util::unix_secs()?;

    let authorization = build_signature(secret_id, secret_key, timestamp, &body_json)?;

    let client = util::http_client();
    let resp = client
        .post(format!("https://{}/", HOST))
        .header("Host", HOST)
        .header("Content-Type", "application/json; charset=utf-8")
        .header("X-TC-Action", ACTION)
        .header("X-TC-Version", VERSION)
        .header("X-TC-Timestamp", timestamp.to_string())
        .header("Authorization", &authorization)
        .body(body_json)
        .send()
        .await
        .map_err(|e| format!("请求腾讯云OCR失败: {}", e))?;

    let status = resp.status();
    let resp_text = resp.text().await.map_err(|e| format!("读取响应失败: {}", e))?;

    if !status.is_success() {
        return Err(format!("腾讯云OCR HTTP错误 ({}): {}", status, resp_text));
    }

    let ocr_resp: TencentResponse = serde_json::from_str(&resp_text)
        .map_err(|e| format!("解析腾讯云OCR响应失败: {}", e))?;

    if let Some(err) = ocr_resp.response.error {
        return Err(format!("腾讯云OCR错误 ({}): {}", err.code, err.message));
    }

    let items = util::or_empty(ocr_resp.response.response, "腾讯云OCR")?;

    let texts: Vec<String> = items
        .iter()
        .flat_map(|item| {
            item.text_detections
                .as_ref()
                .map(|dets| dets.iter().map(|d| d.detected_text.clone()).collect::<Vec<_>>())
                .unwrap_or_default()
        })
        .collect();

    if texts.is_empty() {
        return Err("腾讯云OCR未识别到文字".to_string());
    }

    Ok(texts.join("\n"))
}
