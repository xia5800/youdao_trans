mod edge;

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::OnceLock;

use tauri::command;

type TtsFn = fn(
    String,
    String,
    f32,
) -> Pin<Box<dyn Future<Output = Result<Vec<u8>, String>> + Send>>;

fn registry() -> &'static HashMap<&'static str, TtsFn> {
    static REG: OnceLock<HashMap<&'static str, TtsFn>> = OnceLock::new();
    REG.get_or_init(|| {
        let mut m: HashMap<&'static str, TtsFn> = HashMap::new();
        m.insert("edge", |text, voice, speed| Box::pin(async move { edge::speak(&text, &voice, speed).await }));
        m
    })
}

#[derive(serde::Serialize, Clone)]
pub struct VoiceInfo {
    pub id: &'static str,
    pub label: &'static str,
}

const EDGE_VOICES: &[VoiceInfo] = &[
    VoiceInfo { id: "zh-CN-XiaoxiaoNeural", label: "晓晓 (中文女性)" },
    VoiceInfo { id: "zh-CN-YunxiNeural", label: "云希 (中文男性)" },
    VoiceInfo { id: "en-US-AriaNeural", label: "Aria (英文女性)" },
    VoiceInfo { id: "en-US-GuyNeural", label: "Guy (英文男性)" },
    VoiceInfo { id: "ja-JP-NanamiNeural", label: "Nanami (日语女性)" },
    VoiceInfo { id: "ja-JP-KeitaNeural", label: "Keita (日语男性)" },
    VoiceInfo { id: "ko-KR-SunHiNeural", label: "Sun-Hi (韩语女性)" },
    VoiceInfo { id: "ko-KR-InJoonNeural", label: "InJoon (韩语男性)" },
    VoiceInfo { id: "fr-FR-DeniseNeural", label: "Denise (法语女性)" },
    VoiceInfo { id: "de-DE-KatjaNeural", label: "Katja (德语女性)" },
    VoiceInfo { id: "es-ES-ElviraNeural", label: "Elvira (西语女性)" },
];

#[command]
pub async fn tts_speak(text: String, voice: String, speed: f32) -> Result<Vec<u8>, String> {
    let providers = registry();
    let speak = providers.get("edge").ok_or("TTS 引擎未注册")?;
    speak(text, voice, speed).await
}

#[command]
pub fn tts_list_voices() -> Vec<VoiceInfo> {
    EDGE_VOICES.to_vec()
}
