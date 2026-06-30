use msedge_tts::tts::client::tokio_runtime::connect_async;
use msedge_tts::tts::SpeechConfig;

fn speed_to_rate(speed: f32) -> i32 {
    if speed <= 0.0 {
        return 0;
    }
    ((speed - 1.0) * 100.0).round() as i32
}

pub async fn speak(text: &str, voice: &str, speed: f32) -> Result<Vec<u8>, String> {
    let config = SpeechConfig {
        voice_name: voice.to_string(),
        audio_format: "audio-24khz-96kbitrate-mono-mp3".to_string(),
        pitch: 0,
        rate: speed_to_rate(speed),
        volume: 100,
    };

    let mut tts = connect_async()
        .await
        .map_err(|e| format!("连接 Edge TTS 失败: {}", e))?;

    let audio = tts
        .synthesize(text, &config)
        .await
        .map_err(|e| format!("Edge TTS 合成失败: {}", e))?;

    if audio.audio_bytes.is_empty() {
        log::warn!("[edge-tts] no audio data received");
    }

    Ok(audio.audio_bytes)
}
