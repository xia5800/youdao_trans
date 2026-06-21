import { ref, onMounted, onUnmounted } from 'vue'

const GROUP_LABEL = '有道翻译'

export const HOTKEYS = [
  { id: 'translate', label: '翻译', combo: 'Ctrl+Enter', desc: '按下此快捷键进行翻译' },
  { id: 'screenshot', label: '截图翻译', combo: 'Alt+W', desc: '按下此快捷键截图区域并识别翻译' },
  { id: 'selectionTranslate', label: '划词翻译', combo: 'Alt+T', desc: '按下此快捷键复制选中文字并翻译' },
]

const hotkeys = ref(HOTKEYS.map(h => ({ ...h })))
const recordingId = ref(null)
const translateTrigger = ref(0)

export function useHotkey() {
  function eventToCombo(e) {
    const parts = []
    if (e.ctrlKey || e.metaKey) parts.push('Ctrl')
    if (e.altKey) parts.push('Alt')
    if (e.shiftKey) parts.push('Shift')
    if (e.key === 'Control' || e.key === 'Alt' || e.key === 'Shift' || e.key === 'Meta') {
      return null
    }
    const key = e.key.length === 1 ? e.key.toUpperCase() : e.key
    if (key === ' ') return 'Space'
    parts.push(key)
    return parts.join('+')
  }

  function startRecording(id) {
    recordingId.value = id
  }

  function stopRecording() {
    recordingId.value = null
  }

  function isRecording(id) {
    return recordingId.value === id
  }

  function updateCombo(id, combo) {
    const hk = hotkeys.value.find(h => h.id === id)
    if (hk) hk.combo = combo
  }

  function comboMatches(e, combo) {
    const parts = combo.split('+')
    const hasCtrl = parts.includes('Ctrl')
    const hasAlt = parts.includes('Alt')
    const hasShift = parts.includes('Shift')
    const key = parts[parts.length - 1]
    if ((e.ctrlKey || e.metaKey) !== hasCtrl) return false
    if (e.altKey !== hasAlt) return false
    if (e.shiftKey !== hasShift) return false
    const pressedKey = e.key.length === 1 ? e.key.toUpperCase() : e.key
    return pressedKey === key || (key === 'Space' && e.key === ' ')
  }

  function useHotkeyListener(onTrigger) {
    function handler(e) {
      for (const hk of hotkeys.value) {
        if (comboMatches(e, hk.combo)) {
          e.preventDefault()
          onTrigger(hk)
          return
        }
      }
    }
    onMounted(() => document.addEventListener('keydown', handler))
    onUnmounted(() => document.removeEventListener('keydown', handler))
  }

  return { hotkeys, recordingId, translateTrigger, startRecording, stopRecording, isRecording, updateCombo, eventToCombo, comboMatches, useHotkeyListener, GROUP_LABEL }
}
