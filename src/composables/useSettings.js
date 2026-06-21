import { reactive, watch, toRefs } from 'vue'

import { invoke } from '@tauri-apps/api/core'
import { enable as enableAutostart, disable as disableAutostart } from '@tauri-apps/plugin-autostart'
import { useTheme } from './useTheme.js'
import { useToast } from './useToast.js'

const FALLBACK = {
  theme: 'system',
  configPath: '',
  dbPath: '',
  autoStart: false,
  delayTime: 600,
  volume: 100,
  speed: 1.0,
  storeRecords: true,
  replaceNewlines: false,
  autoTranslate: false,
  autoTranslateDelay: 1200,
  hotkeys: { translate: 'Ctrl+Enter', screenshot: 'Alt+W', ocr: 'Alt+E', selectionTranslate: 'Alt+T' },
  activeTranslator: 'microsoft_free',
  translatorKeys: {},
  activeOcr: 'ollama_ocr',
  ocrKeys: {},
  closeBehavior: 'ask',
  showScreenshotCrosshair: true,
}

const state = reactive({ ...FALLBACK })
let loaded = false

async function doSave() {
  if (!loaded) return

  const mk = state.translatorKeys?.['microsoft']
  const mr = state.translatorKeys?.['microsoft_region']
  const ba = state.translatorKeys?.['baidu_appid']
  const bk = state.translatorKeys?.['baidu_appkey']
  const ya = state.translatorKeys?.['youdao_appid']
  const ys = state.translatorKeys?.['youdao_appsecret']
  const filled = v => v && v.trim()
  if ((filled(mk) && !filled(mr)) || (!filled(mk) && filled(mr))) {
    return
  }
  if ((filled(ba) && !filled(bk)) || (!filled(ba) && filled(bk))) {
    return
  }
  if ((filled(ya) && !filled(ys)) || (!filled(ya) && filled(ys))) {
    return
  }
  if (state.activeOcr === 'baidu_ocr') {
    const ok = state.ocrKeys?.['baidu_ocr-apikey']
    const os = state.ocrKeys?.['baidu_ocr-apisecret']
    if (!filled(ok) || !filled(os)) {
      return
    }
  }
  if (state.activeOcr === 'xunfei') {
    const oi = state.ocrKeys?.['xunfei-appid']
    const ok = state.ocrKeys?.['xunfei-apikey']
    const os = state.ocrKeys?.['xunfei-apisecret']
    if (!filled(oi) || !filled(ok) || !filled(os)) {
      return
    }
  }
  if (state.activeOcr === 'tencent') {
    const si = state.ocrKeys?.['tencent-secretid']
    const sk = state.ocrKeys?.['tencent-secretkey']
    if (!filled(si) || !filled(sk)) {
      return
    }
  }

  try {
    const savePath = state.configPath || null
    const json = JSON.stringify(state)
    await invoke('save_config', { json, path: savePath })
    if (savePath) {
      const pointer = JSON.stringify({ configPath: savePath })
      await invoke('save_config', { json: pointer, path: null })
    }
    await invoke('reload_hotkeys')
  } catch (e) {
    console.warn('save_config failed:', e)
    const { showToast } = useToast()
    showToast(`保存设置或注册快捷键失败: ${e}`)
  }
}

watch(() => ({ ...state }), doSave, { deep: true })

watch(() => state.autoStart, async (val) => {
  if (!loaded) return
  try {
    if (val) {
      await enableAutostart()
    } else {
      await disableAutostart()
    }
  } catch (e) {
    console.warn('autostart toggle failed:', e)
  }
})

function migrateHotkeys(hotkeys) {
  if ('selection_translate' in hotkeys && !('selectionTranslate' in hotkeys)) {
    hotkeys.selectionTranslate = hotkeys.selection_translate
  }
  delete hotkeys.selection_translate
  // "Alt+W" was the old default for selection translate, now used by screenshot
  if (hotkeys.selectionTranslate === 'Alt+W') {
    hotkeys.selectionTranslate = 'Alt+T'
  }
  return hotkeys
}

async function load() {
  try {
    const raw = await invoke('load_config', { path: null })
    const data = JSON.parse(raw)
    data.hotkeys = { ...FALLBACK.hotkeys, ...migrateHotkeys(data.hotkeys || {}) }
    Object.assign(state, data)
    if (state.configPath) {
      try {
        const raw2 = await invoke('load_config', { path: state.configPath })
        const data2 = JSON.parse(raw2)
        data2.hotkeys = { ...FALLBACK.hotkeys, ...migrateHotkeys(data2.hotkeys || {}) }
        Object.assign(state, data2)
      } catch (e2) {
        console.warn('custom config at', state.configPath, 'is broken, removing:', e2)
        await invoke('remove_config', { path: state.configPath })
        state.configPath = ''
      }
    }
  } catch (e) {
    console.warn('load_config error, using defaults:', e)
  }
  if (state.speed > 3) {
    state.speed = 1.0
  }
  if (!state.activeTranslator) {
    state.activeTranslator = 'microsoft_free'
  }
  if (!state.activeOcr) {
    state.activeOcr = 'ollama_ocr'
  }
  loaded = true
  try {
    if (state.autoStart) {
      await enableAutostart()
    } else {
      await disableAutostart()
    }
  } catch (e) {
    console.warn('autostart sync failed:', e)
  }
  if (state.theme) {
    const { themeMode } = useTheme()
    themeMode.value = state.theme
  }
}

export function useSettings() {
  return {
    settings: state,
    ...toRefs(state),
    load,
  }
}