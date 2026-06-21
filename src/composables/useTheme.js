import { ref, watch } from 'vue'

const STORAGE_KEY = 'youdao-theme'

function getSystemTheme() {
  return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light'
}

function loadSaved() {
  try {
    return localStorage.getItem(STORAGE_KEY)
  } catch {
    return null
  }
}

function save(val) {
  try {
    if (val === 'system') {
      localStorage.removeItem(STORAGE_KEY)
    } else {
      localStorage.setItem(STORAGE_KEY, val)
    }
  } catch { /* noop */ }
}

function applyTheme(mode) {
  document.documentElement.setAttribute('data-theme', mode)
}

function migrateIfNeeded() {
  const saved = loadSaved()
  if (saved === null) return 'system'
  if (saved === 'light' || saved === 'dark') {
    applyTheme(saved)
    return saved
  }
  return 'system'
}

const themeMode = ref(migrateIfNeeded())

let mql = null
function onSystemChange(e) {
  if (themeMode.value === 'system') {
    applyTheme(e.matches ? 'dark' : 'light')
  }
}

function startListening() {
  mql = window.matchMedia('(prefers-color-scheme: dark)')
  if (mql.addEventListener) {
    mql.addEventListener('change', onSystemChange)
  }
}

function stopListening() {
  if (mql && mql.removeEventListener) {
    mql.removeEventListener('change', onSystemChange)
  }
}

function resolveMode(mode) {
  if (mode === 'system') return getSystemTheme()
  return mode
}

watch(themeMode, (val) => {
  applyTheme(resolveMode(val))
  save(val)
  if (val === 'system') {
    startListening()
  } else {
    stopListening()
  }
}, { immediate: true })

export function useTheme() {
  return { themeMode }
}
