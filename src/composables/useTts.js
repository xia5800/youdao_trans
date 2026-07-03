import { useSettings } from './useSettings'
import { invoke } from '@tauri-apps/api/core'

const LANG_MAP = {
  zh: 'zh-CN', en: 'en-US', ja: 'ja-JP', ko: 'ko-KR',
  fr: 'fr-FR', de: 'de-DE', es: 'es-ES',
}

function mapLang(lang) {
  return LANG_MAP[lang] || lang || 'en-US'
}

export function useTts() {
  const { ttsEngine, ttsVoice } = useSettings()

  let audioCtx = null
  let sourceNode = null
  let isSpeaking = false

  function getAudioCtx() {
    if (!audioCtx) {
      audioCtx = new (window.AudioContext || window.webkitAudioContext)()
    }
    if (audioCtx.state === 'suspended') {
      audioCtx.resume()
    }
    return audioCtx
  }

  async function speakEdge(text, callbacks) {
    if (!text) return

    try {
      callbacks?.onStart?.()
      isSpeaking = true

      const mp3Bytes = await invoke('tts_speak', {
        text,
        voice: ttsVoice.value,
        speed: 1.0,
      })

      if (!mp3Bytes || mp3Bytes.length === 0) {
        isSpeaking = false
        callbacks?.onEnd?.()
        callbacks?.onError?.('语音合成返回空音频')
        return
      }

      if (!isSpeaking) {
        callbacks?.onEnd?.()
        return
      }

      const ctx = getAudioCtx()
      const arrayBuf = new Uint8Array(mp3Bytes).buffer
      let audioBuf
      try {
        audioBuf = await ctx.decodeAudioData(arrayBuf)
      } catch (decodeErr) {
        console.error('[useTts] decodeAudioData failed:', decodeErr)
        isSpeaking = false
        callbacks?.onEnd?.()
        callbacks?.onError?.(`音频解码失败: ${decodeErr}`)
        return
      }

      sourceNode = ctx.createBufferSource()
      sourceNode.buffer = audioBuf
      sourceNode.connect(ctx.destination)
      sourceNode.onended = () => {
        isSpeaking = false
        sourceNode = null
        callbacks?.onEnd?.()
      }
      sourceNode.start()
    } catch (e) {
      console.error('[useTts] speakEdge error:', e)
      isSpeaking = false
      callbacks?.onEnd?.()
      callbacks?.onError?.(`语音合成失败: ${e}`)
    }
  }

  function speak(text, lang, callbacks) {
    if (!text) return

    if (ttsEngine.value === 'local') {
      speakEdge(text, callbacks)
      return
    }

    if (!window.speechSynthesis) {
      callbacks?.onError?.('浏览器不支持语音合成')
      return
    }

    try {
      window.speechSynthesis.cancel()
      const utter = new SpeechSynthesisUtterance(text)
      utter.lang = mapLang(lang)
      utter.volume = 1
      utter.rate = 1.0
      utter.onstart = () => callbacks?.onStart?.()
      utter.onend = () => callbacks?.onEnd?.()
      utter.onerror = (e) => {
        if (e.error === 'interrupted' || e.error === 'canceled') {
          callbacks?.onEnd?.()
          return
        }
        let msg
        if (e.error === 'voice-unavailable') {
          msg = '当前语言没有可用的语音引擎'
        } else if (e.error === 'synthesis-failed') {
          msg = '语音合成失败。请在系统设置中安装日语语音包（设置 → 时间和语言 → 语音）'
        } else {
          msg = `语音朗读失败：${e.error || '未知错误'}`
        }
        console.error('useTts: speech error', e)
        callbacks?.onEnd?.()
        callbacks?.onError?.(msg)
      }
      window.speechSynthesis.speak(utter)
    } catch (e) {
      console.error('useTts: speak failed', e)
      callbacks?.onEnd?.()
      callbacks?.onError?.('语音朗读异常')
    }
  }

  function stop() {
    if (ttsEngine.value === 'local') {
      isSpeaking = false
      if (sourceNode) {
        try { sourceNode.stop() } catch (_) {}
        sourceNode = null
      }
      if (audioCtx) {
        audioCtx.close()
        audioCtx = null
      }
      return
    }
    if (window.speechSynthesis) {
      window.speechSynthesis.cancel()
    }
  }

  function close() {
    stop()
  }

  return { speak, stop, close }
}
