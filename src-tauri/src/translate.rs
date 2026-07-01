pub mod microsoft;
pub mod microsoft_common;
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

#[macro_export]
macro_rules! make_registry {
    ($fn_type:ty, $reg_name:ident, [$(($key:expr, $provider:expr)),* $(,)?]) => {
        fn $reg_name() -> &'static std::collections::HashMap<&'static str, $fn_type> {
            static REG: std::sync::OnceLock<std::collections::HashMap<&'static str, $fn_type>> = std::sync::OnceLock::new();
            REG.get_or_init(|| {
                let mut m = std::collections::HashMap::<&'static str, $fn_type>::new();
                $( m.insert($key, $provider); )*
                m
            })
        }
    };
}

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

make_registry!(TranslateFn, registry, [
    ("microsoft", |t: String, s: Option<String>, tl: String, k: HashMap<String, String>| Box::pin(async move { microsoft::translate(&t, s.as_deref(), &tl, &k).await })),
    ("microsoft_free", |t: String, s: Option<String>, tl: String, k: HashMap<String, String>| Box::pin(async move { microsoft_free::translate(&t, s.as_deref(), &tl, &k).await })),
    ("baidu", |t: String, s: Option<String>, tl: String, k: HashMap<String, String>| Box::pin(async move { baidu::translate(&t, s.as_deref(), &tl, &k).await })),
    ("ali", |t: String, s: Option<String>, tl: String, k: HashMap<String, String>| Box::pin(async move { aliyun::translate(&t, s.as_deref(), &tl, &k).await })),
    ("youdao", |t: String, s: Option<String>, tl: String, k: HashMap<String, String>| Box::pin(async move { youdao::translate(&t, s.as_deref(), &tl, &k).await })),
    ("google", |t: String, s: Option<String>, tl: String, k: HashMap<String, String>| Box::pin(async move { google::translate(&t, s.as_deref(), &tl, &k).await })),
    ("openai", |t: String, s: Option<String>, tl: String, k: HashMap<String, String>| Box::pin(async move { openai::translate(&t, s.as_deref(), &tl, &k).await })),
    ("ollama", |t: String, s: Option<String>, tl: String, k: HashMap<String, String>| Box::pin(async move { ollama::translate(&t, s.as_deref(), &tl, &k).await })),
    ("deeplx", |t: String, s: Option<String>, tl: String, k: HashMap<String, String>| Box::pin(async move { deeplx::translate(&t, s.as_deref(), &tl, &k).await })),
]);

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
