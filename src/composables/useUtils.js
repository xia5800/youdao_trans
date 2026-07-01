import { ref } from 'vue'
import { useToast } from './useToast.js'

const lastToastTime = ref(0)
const lastToastMsg = ref('')

export const filled = v => v && v.trim()

export function copyText(text) {
  if (!text) return
  navigator.clipboard.writeText(text).catch(() => {})
}

export function useUtils() {
  const { showToast } = useToast()

  function showToastOnce(msg) {
    const now = Date.now()
    if (msg === lastToastMsg.value && now - lastToastTime.value < 2000) return
    lastToastTime.value = now
    lastToastMsg.value = msg
    showToast(msg)
  }

  function openUrl(url) {
    import('@tauri-apps/plugin-shell').then(({ open }) => { open(url) })
  }

  return { showToastOnce, openUrl }
}
