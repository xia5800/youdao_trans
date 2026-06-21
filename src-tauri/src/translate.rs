pub mod microsoft;
pub mod microsoft_free;
pub mod baidu;
pub mod aliyun;
pub mod youdao;
pub mod google;
pub mod openai;
pub mod ollama;
pub mod deeplx;

use serde::Deserialize;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::OnceLock;

#[derive(Deserialize)]
pub struct TranslateArgs {
    pub text: String,
    pub source_lang: Option<String>,
    pub target_lang: String,
    pub active_translator: Option<String>,
    pub translator_keys: HashMap<String, String>,
}

type TranslateFn = fn(
    String,
    Option<String>,
    String,
    HashMap<String, String>,
) -> Pin<Box<dyn Future<Output = Result<String, String>> + Send>>;

fn registry() -> &'static HashMap<&'static str, TranslateFn> {
    static REG: OnceLock<HashMap<&'static str, TranslateFn>> = OnceLock::new();
    REG.get_or_init(|| {
        let mut m: HashMap<&'static str, TranslateFn> = HashMap::new();
        m.insert("microsoft", |t, s, tl, k| Box::pin(async move { microsoft::translate(&t, s.as_deref(), &tl, &k).await }));
        m.insert("microsoft_free", |t, s, tl, k| Box::pin(async move { microsoft_free::translate(&t, s.as_deref(), &tl, &k).await }));
        m.insert("baidu", |t, s, tl, k| Box::pin(async move { baidu::translate(&t, s.as_deref(), &tl, &k).await }));
        m.insert("ali", |t, s, tl, k| Box::pin(async move { aliyun::translate(&t, s.as_deref(), &tl, &k).await }));
        m.insert("youdao", |t, s, tl, k| Box::pin(async move { youdao::translate(&t, s.as_deref(), &tl, &k).await }));
        m.insert("google", |t, s, tl, k| Box::pin(async move { google::translate(&t, s.as_deref(), &tl, &k).await }));
        m.insert("openai", |t, s, tl, k| Box::pin(async move { openai::translate(&t, s.as_deref(), &tl, &k).await }));
        m.insert("ollama", |t, s, tl, k| Box::pin(async move { ollama::translate(&t, s.as_deref(), &tl, &k).await }));
        m.insert("deeplx", |t, s, tl, k| Box::pin(async move { deeplx::translate(&t, s.as_deref(), &tl, &k).await }));
        m
    })
}

pub async fn run(args: TranslateArgs) -> Result<String, String> {
    let name = args.active_translator.unwrap_or_else(|| "microsoft".to_string());
    match registry().get(name.as_str()) {
        Some(f) => f(args.text, args.source_lang, args.target_lang, args.translator_keys).await,
        None => Err(format!("translator '{}' not supported", name)),
    }
}

#[tauri::command]
pub async fn translate_command(
    text: String,
    source_lang: Option<String>,
    target_lang: String,
    active_translator: Option<String>,
    translator_keys: std::collections::HashMap<String, String>,
) -> Result<String, String> {
    run(TranslateArgs { text, source_lang, target_lang, active_translator, translator_keys }).await
}
