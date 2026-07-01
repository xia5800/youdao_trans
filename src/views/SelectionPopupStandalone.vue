<template>
  <div class="popup-root" @keydown.esc="closeWindow">
    <div class="popup-header" @dblclick="closeWindow">
      <span class="popup-label">划词翻译</span>
      <div class="header-actions">
        <select v-model="currentTargetLang" class="lang-select" @click.stop>
          <option value="zh">中文</option>
          <option value="en">英文</option>
          <option value="ja">日文</option>
          <option value="ko">韩文</option>
          <option value="fr">法文</option>
          <option value="de">德文</option>
          <option value="es">西班牙文</option>
        </select>
        <div class="icon-btn" data-tooltip="重新翻译" @click.stop="retranslate">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <polyline points="23 4 23 10 17 10"/>
            <path d="M20.49 15a9 9 0 1 1-2.12-9.36L23 10"/>
          </svg>
        </div>
        <div class="icon-btn" data-tooltip="复制译文" @click.stop="copyResult">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <rect x="9" y="9" width="13" height="13" rx="2" ry="2"/>
            <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"/>
          </svg>
        </div>
        <div class="icon-btn" data-tooltip="朗读译文" @click.stop="speakResult">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5"/>
            <path d="M19.07 4.93a10 10 0 0 1 0 14.14M15.54 8.46a5 5 0 0 1 0 7.07"/>
          </svg>
        </div>
        <div class="icon-btn" data-tooltip="关闭" @click.stop="closeWindow">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/>
          </svg>
        </div>
      </div>
    </div>
    <div class="popup-body">
      <div v-if="sourceText" class="popup-source">{{ sourceText }}</div>
      <div v-if="sourceText" class="popup-divider"></div>
      <div class="popup-result">
        <span v-if="isTranslating" class="translating-hint">
          {{ sourceText ? '正在翻译...' : '正在取词...' }}
        </span>
        <span v-else-if="!sourceText && translatedText" class="error-text">{{ translatedText }}</span>
        <span v-else>{{ translatedText }}</span>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, watch, nextTick, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow'
import { useTts } from '../composables/useTts.js'
import { copyText } from '../composables/useUtils.js'

const appWindow = getCurrentWebviewWindow()

const sourceText = ref('')
const translatedText = ref('')
const currentTargetLang = ref('zh')
const activeTranslator = ref(null)
const translatorKeys = ref({})
const isTranslating = ref(true)
const programmerMode = ref(false)

const { speak, stop } = useTts()
let unlistenUpdate = null

onMounted(async () => {
  document.documentElement.style.backgroundColor = 'transparent'
  document.body.style.backgroundColor = 'transparent'
  document.body.style.overflow = 'hidden'

  try {
    const data = await invoke('get_selection_payload')
    if (!data) {
      await appWindow.close()
      return
    }
    sourceText.value = data.source_text
    translatedText.value = data.translated_text
    currentTargetLang.value = data.target_lang || 'zh'
    activeTranslator.value = data.active_translator || null
    translatorKeys.value = data.translator_keys || {}
    isTranslating.value = data.is_translating ?? false
    programmerMode.value = data.programmer_mode ?? false
  } catch (e) {
    console.warn('popup: no payload', e)
    await appWindow.close()
  }

  unlistenUpdate = await listen('selection-update', (event) => {
    const data = event.payload
    sourceText.value = data.source_text ?? sourceText.value
    translatedText.value = data.translated_text
    currentTargetLang.value = data.target_lang ?? currentTargetLang.value
    activeTranslator.value = data.active_translator ?? activeTranslator.value
    translatorKeys.value = data.translator_keys ?? translatorKeys.value
    isTranslating.value = data.is_translating ?? false
    programmerMode.value = data.programmer_mode ?? programmerMode.value
    // Auto-save to history when translation finishes
    if (!isTranslating.value && data.store_records && data.source_text && data.translated_text) {
      saveToHistory(data)
    }
  })

  await nextTick()
  await new Promise(r => requestAnimationFrame(r))
  await appWindow.show()
})

onUnmounted(() => {
  if (unlistenUpdate) unlistenUpdate()
  stop()
})

async function closeWindow() {
  try {
    await appWindow.close()
  } catch (e) {
    console.warn('close failed:', e)
  }
}

async function saveToHistory(data) {
  try {
    await invoke('save_history', {
      sourceLang: data.source_lang || (data.target_lang === 'zh' ? '英文' : '中文'),
      targetLang: data.target_lang === 'zh' ? '中文' : '英文',
      sourceText: data.source_text,
      targetText: data.translated_text,
      timestamp: Date.now(),
    })
  } catch (e) {
    console.warn('save history failed:', e)
  }
}

function copyResult() {
  copyText(translatedText.value)
}

function speakResult() {
  if (!translatedText.value) return
  speak(translatedText.value, currentTargetLang.value)
}

function splitProgrammerText(text) {
  let result = text.replace(/[_-]+/g, ' ')
  result = result.replace(/([a-z])([A-Z])/g, '$1 $2')
  result = result.replace(/([A-Z]+)([A-Z][a-z])/g, '$1 $2')
  result = result.replace(/([\u4e00-\u9fff\u3400-\u4dbf])([a-zA-Z])/g, '$1 $2')
  result = result.replace(/([a-zA-Z])([\u4e00-\u9fff\u3400-\u4dbf])/g, '$1 $2')
  return result.replace(/\s+/g, ' ').trim()
}

async function retranslate() {
  if (!sourceText.value || isTranslating.value) return
  isTranslating.value = true
  try {
    let text = sourceText.value
    if (programmerMode.value) {
      text = splitProgrammerText(text)
    }
    const result = await invoke('translate_command', {
      text,
      sourceLang: null,
      targetLang: currentTargetLang.value,
      activeTranslator: activeTranslator.value,
      translatorKeys: translatorKeys.value,
    })
    translatedText.value = result
  } catch (e) {
    console.warn('retranslate failed:', e)
  } finally {
    isTranslating.value = false
  }
}

watch(currentTargetLang, async (lang) => {
  if (!sourceText.value || isTranslating.value) return
  await retranslate()
})
</script>

<style scoped>
.popup-root {
  height: 100vh;
  display: flex;
  flex-direction: column;
  background: var(--bg-card);
  border-radius: 8px;
  overflow: hidden;
}
.popup-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 6px 10px;
  -webkit-app-region: drag;
  user-select: none;
  cursor: default;
}
.popup-label {
  font-size: 11px;
  color: var(--text-tertiary);
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}
.header-actions {
  display: flex;
  align-items: center;
  gap: 2px;
  -webkit-app-region: no-drag;
}
.lang-select {
  font-size: 11px;
  padding: 2px 4px;
  border-radius: 4px;
  border: 1px solid var(--border);
  background: var(--bg-input);
  color: var(--text-primary);
}
.icon-btn {
  position: relative;
  display: flex;
  align-items: center;
  justify-content: center;
  width: 24px;
  height: 24px;
  border: none;
  border-radius: 4px;
  background: transparent;
  color: var(--text-tertiary);
  cursor: pointer;
}
.icon-btn:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}
.icon-btn::after {
  content: attr(data-tooltip);
  position: absolute;
  bottom: -28px;
  left: 50%;
  transform: translateX(-50%) scale(0.9);
  padding: 4px 8px;
  border-radius: 6px;
  font-size: 11px;
  font-weight: 500;
  white-space: nowrap;
  color: var(--text-inverse);
  background: var(--tooltip-bg);
  box-shadow: var(--shadow-tooltip);
  opacity: 0;
  pointer-events: none;
  transition: opacity 0.15s, transform 0.15s;
  z-index: 100;
}
.icon-btn:hover::after {
  opacity: 1;
  transform: translateX(-50%) scale(1);
}
.popup-body {
  flex: 1;
  padding: 0 10px 10px;
  overflow-y: auto;
}
.popup-source {
  color: var(--text-secondary);
  font-size: 12px;
  margin-bottom: 4px;
  word-break: break-word;
}
.popup-divider {
  height: 1px;
  background: var(--border);
  margin: 4px 0;
}
.popup-result {
  color: var(--text-primary);
  font-size: 13px;
  word-break: break-word;
}
.translating-hint {
  color: var(--text-tertiary);
  font-style: italic;
  font-size: 12px;
}
.error-text {
  color: var(--text-danger, #e74c3c);
  font-size: 12px;
}
</style>
