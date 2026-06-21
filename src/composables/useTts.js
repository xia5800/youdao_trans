import { useSettings } from './useSettings'

const LANG_MAP = {
  zh: 'zh-CN', en: 'en-US', ja: 'ja-JP', ko: 'ko-KR',
  fr: 'fr-FR', de: 'de-DE', es: 'es-ES',
}

function mapLang(lang) {
  return LANG_MAP[lang] || lang || 'en-US'
}

export function useTts() {
  const { volume, speed } = useSettings()

  function speak(text, lang, callbacks) {
    if (!text) return
    if (!window.speechSynthesis) {
      callbacks?.onError?.('浏览器不支持语音合成')
      return
    }

    try {
      window.speechSynthesis.cancel()
      const utter = new SpeechSynthesisUtterance(text)
      utter.lang = mapLang(lang)
      utter.volume = (volume.value ?? 100) / 100
      utter.rate = speed.value ?? 1.0
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
    if (window.speechSynthesis) {
      window.speechSynthesis.cancel()
    }
  }

  return { speak, stop }
}
