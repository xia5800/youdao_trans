use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct RequestItem {
    #[serde(rename = "Text")]
    pub text: String,
}

#[derive(Deserialize)]
pub struct ResponseItem {
    pub translations: Vec<TranslationItem>,
}

#[derive(Deserialize)]
pub struct TranslationItem {
    pub text: String,
}

pub const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36";
