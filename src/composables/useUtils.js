import { ref } from 'vue'
import { useToast } from './useToast.js'
import { invoke } from '@tauri-apps/api/core'

const lastToastTime = ref(0)
const lastToastMsg = ref('')

export const filled = v => v && v.trim()

export async function safeInvoke(command, args = {}) {
  try {
    return await invoke(command, args)
  } catch (e) {
    console.warn(`invoke ${command} failed:`, e)
    throw e
  }
}

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
