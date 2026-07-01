use crate::util;
use serde::{Deserialize, Serialize};

const DEFAULT_ENDPOINT: &str = "http://localhost:1188";

#[derive(Serialize)]
struct RequestBody {
    text: String,
    source_lang: String,
    target_lang: String,
}

#[derive(Deserialize)]
struct ResponseBody {
    #[allow(dead_code)]
    code: Option<i64>,
    data: Option<String>,
    #[allow(dead_code)]
    msg: Option<String>,
}

fn map_lang(code: &str) -> &str {
    match code {
        "zh" => "ZH",
        "en" => "EN",
        "ja" => "JA",
        "fr" => "FR",
        "de" => "DE",
        "es" => "ES",
        "ko" => "KO",
        _ => code,
    }
}

pub async fn translate(
    text: &str,
    source_lang: Option<&str>,
    target_lang: &str,
    keys: &std::collections::HashMap<String, String>,
) -> Result<String, String> {
    let base = keys
        .get("deeplx_endpoint")
        .filter(|s| !s.is_empty())
        .map(|s| s.trim_end_matches('/'))
        .unwrap_or(DEFAULT_ENDPOINT);
    let endpoint = format!("{}/translate", base);

    let source = source_lang.map(map_lang).unwrap_or("auto").to_uppercase();
    let target = map_lang(target_lang).to_uppercase();

    let body = RequestBody {
        text: text.to_string(),
        source_lang: source,
        target_lang: target,
    };

    let client = util::http_client();
    let resp = client
        .post(endpoint)
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("DeepLX 请求失败: {}", e))?;

    util::check_status(resp.status(), "DeepLX")?;

    let result: ResponseBody = util::parse_json(resp, "DeepLX").await?;

    util::or_empty(result.data, "DeepLX")
}
