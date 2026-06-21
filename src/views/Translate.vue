<template>
  <div class="translate-container">
    <div class="translator-card">
      <div class="lang-bar-combined">
        <div class="lang-selector-group">
          <div class="lang-dropdown" ref="sourceDropdownRef">
            <div class="lang-dropdown-trigger" :class="{ active: sourceDropdownOpen }" @click="toggleSourceDropdown">
              <span>{{ langMap[sourceLang] || langMap['auto'] }}</span>
              <span class="dropdown-arrow">▼</span>
            </div>
            <div v-show="sourceDropdownOpen" class="lang-dropdown-panel" @click.stop>
              <div
                v-for="opt in langOptions"
                :key="opt.value"
                class="lang-dropdown-item"
                :class="{ selected: sourceLang === opt.value }"
                @click="selectSource(opt.value)"
              >{{ opt.label }}</div>
            </div>
          </div>
          <div class="lang-divider" @click="swapLanguages">⇄</div>
          <div class="target-lang-wrapper">
            <div class="lang-dropdown" ref="targetDropdownRef">
              <div class="lang-dropdown-trigger" :class="{ active: targetDropdownOpen }" @click="toggleTargetDropdown">
                <span>{{ langMap[targetLang] }}</span>
                <span class="dropdown-arrow">▼</span>
              </div>
              <div v-show="targetDropdownOpen" class="lang-dropdown-panel" @click.stop>
                <div
                  v-for="opt in langOptions.filter(o => o.value !== 'auto')"
                  :key="opt.value"
                  class="lang-dropdown-item"
                  :class="{ selected: targetLang === opt.value }"
                  @click="selectTarget(opt.value)"
                >{{ opt.label }}</div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <div class="dual-panes">
        <div class="pane">
          <div class="pane-header">
            <span class="pane-badge">{{ sourceLabel }}</span>
            <div class="tool-icons">
              <div class="tool-icon" v-if="sourceText" @click="clearText" data-tooltip="清空">
                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                  <line x1="18" y1="6" x2="6" y2="18" /><line x1="6" y1="6" x2="18" y2="18" />
                </svg>
              </div>
              <div class="tool-icon" @click="copyText(sourceText)" data-tooltip="复制">
                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                  <use href="/icons.svg#icon-copy"></use>
                </svg>
              </div>
              <div class="tool-icon" :class="{ speaking: speaking === 'source' }" @click="speak(sourceText, sourceLang, 'source')" data-tooltip="朗读">
                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                  <use href="/icons.svg#icon-speaker"></use>
                </svg>
              </div>
            </div>
          </div>
          <div
            class="input-content"
            contenteditable="true"
            ref="sourceInput"
            @input="onInput"
            @paste="onPaste"
            @keydown="onKeydown"
            data-placeholder="输入或者粘贴要翻译的文本..."
          ></div>
        </div>
        <div class="pane">
          <div class="pane-header">
            <span class="pane-badge">{{ targetLabel }}</span>
            <div class="tool-icons">
              <div class="tool-icon" @click="copyText(targetText)" data-tooltip="复制">
                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                  <use href="/icons.svg#icon-copy"></use>
                </svg>
              </div>
              <div class="tool-icon" :class="{ speaking: speaking === 'target' }" @click="speak(targetText, targetLang, 'target')" data-tooltip="朗读">
                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                  <use href="/icons.svg#icon-speaker"></use>
                </svg>
              </div>
            </div>
          </div>
          <div class="input-content output-content" v-html="targetDisplay"></div>
        </div>
      </div>
      <div class="translate-footer">
        <span>{{ charCount }} 字符</span>
        <button v-if="!autoTranslate.value" class="btn-translate" @click="manualTranslate">
          翻译 ({{ translateCombo }})
        </button>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, watch, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { useTts } from '../composables/useTts.js'
import { useSettings } from '../composables/useSettings.js'
import { useToast } from '../composables/useToast.js'
import { useHotkey } from '../composables/useHotkey.js'

const langOptions = [
  { value: 'auto', label: '自动检测' },
  { value: 'zh', label: '中文' },
  { value: 'en', label: '英语' },
  { value: 'ja', label: '日语' },
  { value: 'ko', label: '韩语' },
  { value: 'fr', label: '法语' },
  { value: 'de', label: '德语' },
  { value: 'es', label: '西班牙语' }
]

const langMap = {
  auto: '自动检测', zh: '中文', en: '英语', ja: '日语',
  ko: '韩语', fr: '法语', de: '德语', es: '西班牙语'
}

const sourceLang = ref('auto')
const targetLang = ref('en')
const sourceText = ref('')
const targetText = ref('')
const charCount = ref(0)
const translating = ref(false)
const translateError = ref('')
const sourceInput = ref(null)
const sourceDropdownRef = ref(null)
const targetDropdownRef = ref(null)
const sourceDropdownOpen = ref(false)
const targetDropdownOpen = ref(false)

const { activeTranslator, translatorKeys, autoTranslate, autoTranslateDelay, storeRecords, replaceNewlines } = useSettings()
const { showToast } = useToast()
const { useHotkeyListener, hotkeys } = useHotkey()

const translateCombo = computed(() => hotkeys.value.find(h => h.id === 'translate')?.combo || 'Ctrl+Enter')
const speaking = ref(null)

const isAuto = computed(() => sourceLang.value === 'auto')

const sourceLabel = computed(() => {
  return `原文 (${isAuto.value ? '自动检测' : (langMap[sourceLang.value] || sourceLang.value)})`
})

const targetLabel = computed(() => {
  return `译文 (${langMap[targetLang.value] || targetLang.value})`
})

const targetDisplay = computed(() => {
  const src = sourceText.value
  const srcName = isAuto.value ? '自动检测' : (langMap[sourceLang.value] || sourceLang.value)
  const tgtName = langMap[targetLang.value] || targetLang.value
  if (!src) {
    return `<div style="color:var(--text-tertiary);font-size:18px;">译文将显示在此处</div>`
  }
  if (translating.value) {
    return `<div style="color:var(--text-tertiary);font-size:18px;">翻译中...</div>`
  }
  if (translateError.value) {
    return `<div style="color:#e74c3c;font-size:18px;">翻译失败：${translateError.value}</div>`
  }
  if (targetText.value) {
    return targetText.value
  }
  return ''
})

let debounceTimer = null

async function doTranslate() {
  const text = sourceText.value
  if (!text || !activeTranslator.value) return
  const translateText = replaceNewlines.value ? text.replace(/[\r\n]+/g, ' ') : text
  translating.value = true
  translateError.value = ''
  try {
    const result = await invoke('translate_command', {
      text: translateText,
      sourceLang: sourceLang.value === 'auto' ? null : sourceLang.value,
      targetLang: targetLang.value,
      activeTranslator: activeTranslator.value,
      translatorKeys: translatorKeys.value,
    })
    targetText.value = result
    if (storeRecords.value) {
      const srcLabel = langMap[sourceLang.value === 'auto' ? 'auto' : sourceLang.value] || sourceLang.value
      const tgtLabel = langMap[targetLang.value] || targetLang.value
      try {
        await invoke('save_history', {
          sourceLang: srcLabel,
          targetLang: tgtLabel,
          sourceText: text,
          targetText: result,
          timestamp: Date.now(),
        })
      } catch (e) {
        console.warn('save history failed:', e)
      }
    }
  } catch (e) {
    translateError.value = typeof e === 'string' ? e : e?.message || '未知错误'
    targetText.value = ''
  } finally {
    translating.value = false
  }
}

function scheduleTranslate() {
  if (debounceTimer) clearTimeout(debounceTimer)
  if (!autoTranslate.value) return
  debounceTimer = setTimeout(doTranslate, autoTranslateDelay.value)
}

function manualTranslate() {
  if (!sourceText.value || !activeTranslator.value) return
  if (debounceTimer) clearTimeout(debounceTimer)
  doTranslate()
}

useHotkeyListener((hk) => {
  if (hk.id !== 'translate') return
  if (autoTranslate.value) {
    // showToast('自动翻译已开启，无需手动触发')
    return
  }
  manualTranslate()
})

watch([sourceText, sourceLang, targetLang], scheduleTranslate)
watch(activeTranslator, () => { if (sourceText.value) scheduleTranslate() })

function toggleSourceDropdown() {
  targetDropdownOpen.value = false
  sourceDropdownOpen.value = !sourceDropdownOpen.value
}

function toggleTargetDropdown() {
  sourceDropdownOpen.value = false
  targetDropdownOpen.value = !targetDropdownOpen.value
}

function selectSource(val) {
  sourceDropdownOpen.value = false
  sourceLang.value = val
  if (val === 'auto') {
    targetLang.value = 'en'
  }
}

function selectTarget(val) {
  targetDropdownOpen.value = false
  targetLang.value = val
}

function onClickOutside(e) {
  if (sourceDropdownRef.value && !sourceDropdownRef.value.contains(e.target)) {
    sourceDropdownOpen.value = false
  }
  if (targetDropdownRef.value && !targetDropdownRef.value.contains(e.target)) {
    targetDropdownOpen.value = false
  }
}

let unlistenOcr = null

onMounted(async () => {
  document.addEventListener('click', onClickOutside)
  unlistenOcr = await listen('ocr-result', (event) => {
    const text = event.payload
    if (sourceInput.value) {
      sourceInput.value.innerText = text
    }
    sourceText.value = text
    charCount.value = text.length
    if (sourceLang.value === 'auto') {
      targetLang.value = detectTargetLang(text)
    }
  })
})
onUnmounted(() => {
  document.removeEventListener('click', onClickOutside)
  if (unlistenOcr) unlistenOcr()
})

function onKeydown(e) {
  if ((e.ctrlKey || e.metaKey) && e.key === 'a') {
    e.preventDefault()
    const sel = window.getSelection()
    const range = document.createRange()
    range.selectNodeContents(sourceInput.value)
    sel.removeAllRanges()
    sel.addRange(range)
  }
}

function clearText() {
  sourceText.value = ''
  targetText.value = ''
  charCount.value = 0
  translateError.value = ''
  if (sourceInput.value) {
    sourceInput.value.innerText = ''
  }
}

function onInput(e) {
  sourceText.value = e.target.innerText
  charCount.value = sourceText.value.length
  if (sourceLang.value === 'auto') {
    targetLang.value = detectTargetLang(sourceText.value)
  }
}

function swapLanguages() {
  if (sourceLang.value === 'auto') return
  const tmp = sourceLang.value
  sourceLang.value = targetLang.value
  targetLang.value = tmp
}

function containsChinese(text) {
  return /[\u4e00-\u9fff\u3400-\u4dbf]/.test(text)
}

function detectTargetLang(text) {
  if (!text.trim()) return targetLang.value
  return containsChinese(text) ? 'en' : 'zh'
}

function onPaste(e) {
  e.preventDefault()
  const text = (e.clipboardData || window.clipboardData).getData('text/plain')
  if (!text) return
  document.execCommand('insertText', false, text)
  sourceText.value = sourceInput.value?.innerText || text
  charCount.value = sourceText.value.length
  if (sourceLang.value === 'auto') {
    targetLang.value = detectTargetLang(sourceText.value)
  }
  scheduleTranslate()
}

function copyText(text) {
  if (!text) return
  navigator.clipboard.writeText(text)
}

function speak(text, lang, pane) {
  const { speak: tts, stop } = useTts()
  if (speaking.value === pane) {
    stop()
    speaking.value = null
    return
  }
  stop()
  speaking.value = pane
  tts(text, lang, {
    onEnd: () => { speaking.value = null },
    onError: (msg) => { speaking.value = null; showToast(msg, 4000) },
  })
}
</script>

<style scoped>
.translate-container {
  padding: 12px;
  flex: 1;
  display: flex;
  flex-direction: column;
  min-height: 0;
  overflow: hidden;
}

.translator-card {
  background: var(--bg-card);
  border-radius: 12px;
  box-shadow: var(--shadow-card);
  overflow: hidden;
  display: flex;
  flex-direction: column;
  flex: 1;
  min-height: 0;
}

.lang-bar-combined {
  display: flex;
  align-items: center;
  justify-content: flex-start;
  padding: 16px 28px;
  background: var(--bg-card);
  border-bottom: 1px solid var(--border);
  flex-wrap: wrap;
  gap: 16px;
  flex-shrink: 0;
}

.lang-selector-group {
  display: flex;
  align-items: center;
  gap: 12px;
  background: var(--bg-hover);
  padding: 6px 16px 6px 20px;
  border-radius: 60px;
  box-shadow: 0 1px 2px rgba(0, 0, 0, 0.02);
  flex-wrap: wrap;
}

.lang-dropdown {
  position: relative;
}

.lang-dropdown-trigger {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 10px 14px;
  font-size: 15px;
  font-weight: 500;
  color: var(--text-primary);
  cursor: pointer;
  border-radius: 32px;
  background: var(--bg-card);
  border: 1px solid var(--border);
  transition: border-color 0.15s, box-shadow 0.15s;
  user-select: none;
  white-space: nowrap;
}

.lang-dropdown-trigger:hover {
  border-color: var(--accent);
  box-shadow: 0 0 0 3px var(--accent-transparent, rgba(75, 130, 240, 0.12));
}

.dropdown-arrow {
  font-size: 10px;
  color: var(--text-tertiary);
  transition: transform 0.2s;
}

.lang-dropdown-trigger.active .dropdown-arrow {
  transform: rotate(180deg);
}

.lang-dropdown-panel {
  position: absolute;
  top: calc(100% + 4px);
  left: 0;
  min-width: 140px;
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: 10px;
  box-shadow: 0 6px 24px rgba(0, 0, 0, 0.10), 0 1px 4px rgba(0, 0, 0, 0.06);
  padding: 6px;
  z-index: 100;
  animation: dropdown-fade-in 0.12s ease;
}

@keyframes dropdown-fade-in {
  from {
    opacity: 0;
    transform: translateY(-4px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.lang-dropdown-item {
  padding: 8px 14px;
  font-size: 14px;
  color: var(--text-primary);
  border-radius: 6px;
  cursor: pointer;
  transition: background 0.1s;
  white-space: nowrap;
}

.lang-dropdown-item:hover {
  background: var(--accent);
  color: #fff;
}

.lang-dropdown-item.selected {
  font-weight: 600;
  color: var(--accent);
}

.lang-dropdown-item.selected::before {
  content: '✓ ';
  font-weight: 700;
}

.lang-divider {
  font-size: 20px;
  color: var(--accent);
  margin: 0 4px;
  font-weight: 500;
  cursor: default;
  width: 36px;
  height: 36px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 50%;
  background: var(--bg-hover);
  transition: 0.1s;
}

.target-lang-wrapper {
  transition: all 0.2s ease;
  display: flex;
  align-items: center;
  gap: 8px;
}

.dual-panes {
  display: flex;
  flex-direction: row;
  flex: 1;
  min-height: 0;
}

.pane {
  flex: 1;
  padding: 20px 24px;
  border-right: 1px solid var(--border-strong);
  display: flex;
  flex-direction: column;
  min-height: 0;
}

.pane:last-child {
  border-right: none;
}

.pane-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
  flex-shrink: 0;
}

.pane-badge {
  font-size: 13px;
  font-weight: 600;
  color: var(--text-secondary);
  background: var(--bg-hover);
  padding: 4px 12px;
  border-radius: 30px;
}

.tool-icons {
  display: flex;
  gap: 12px;
}

.tool-icon {
  width: 30px;
  height: 30px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 8px;
  background: transparent;
  color: var(--text-tertiary);
  cursor: default;
  transition: color 0.15s, background 0.15s;
}

.tool-icon:hover {
  color: var(--text-primary);
  background: var(--bg-hover);
}

.tool-icon {
  position: relative;
}

.tool-icon::after {
  content: attr(data-tooltip);
  position: absolute;
  top: -36px;
  left: 50%;
  transform: translateX(-50%) scale(0.9);
  padding: 6px 12px;
  border-radius: 8px;
  font-size: 12px;
  font-weight: 500;
  white-space: nowrap;
  color: var(--text-inverse);
  background: var(--tooltip-bg);
  box-shadow: var(--shadow-tooltip);
  opacity: 0;
  pointer-events: none;
  transition: opacity 0.15s, transform 0.15s;
}

.tool-icon:hover::after {
  opacity: 1;
  transform: translateX(-50%) scale(1);
}

.tool-icon.speaking {
  color: var(--accent);
  animation: speak-pulse 0.8s ease-in-out infinite;
}

.tool-icon.speaking svg {
  animation: speak-wave 0.8s ease-in-out infinite;
}

@keyframes speak-pulse {
  0%, 100% { background: transparent; }
  50% { background: var(--accent-transparent); }
}

@keyframes speak-wave {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.5; transform: scale(1.1); }
}

.input-content {
  flex: 1;
  font-size: 18px;
  line-height: 1.5;
  font-family: inherit;
  background: transparent;
  outline: none;
  color: var(--text-primary);
  white-space: pre-wrap;
  word-break: break-word;
  overflow-y: auto;
  min-height: 0;
}

.input-content[contenteditable="true"] {
  cursor: text;
}

.input-content:empty:before {
  content: attr(data-placeholder);
  color: var(--text-tertiary);
  font-size: 18px;
  display: block;
  pointer-events: none;
}

.output-content {
  color: var(--text-secondary);
}

.translate-footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 28px;
  background: var(--bg-card);
  border-top: 1px solid var(--border);
  font-size: 12px;
  color: var(--text-tertiary);
  flex-shrink: 0;
}

.btn-translate {
  height: 38px;
  padding: 0 16px;
  background: var(--accent);
  color: var(--text-inverse);
  border: none;
  border-radius: 8px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: box-shadow 0.2s, transform 0.15s;
  letter-spacing: 0.3px;
  display: inline-flex;
  align-items: center;
  gap: 4px;
}

.btn-translate:hover {
  box-shadow: 0 2px 8px rgba(45, 122, 255, 0.35);
}

.btn-translate:active {
  box-shadow: none;
}

.input-content::-webkit-scrollbar {
  width: 4px;
}

.input-content::-webkit-scrollbar-track {
  background: var(--scrollbar-track);
  border-radius: 4px;
}

.input-content::-webkit-scrollbar-thumb {
  background: var(--scrollbar-thumb);
  border-radius: 4px;
}

@media (max-width: 1000px) {
  .translate-container {
    padding: 16px 20px 20px 20px;
  }

  .lang-bar-combined {
    padding: 12px 20px;
  }

  .pane {
    padding: 16px 16px;
  }

  .lang-select {
    padding: 6px 24px 6px 8px;
    min-width: 90px;
    font-size: 13px;
  }

  .pane-badge {
    font-size: 11px;
  }

  .input-content {
    font-size: 14px;
  }

  .input-content:empty:before {
    font-size: 14px;
  }
}
</style>
