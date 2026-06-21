import { createApp } from 'vue'
import ScreenshotOverlay from './views/ScreenshotOverlay.vue'

function applyTheme() {
  let theme = 'system'
  try { const saved = localStorage.getItem('youdao-theme'); if (saved) theme = saved } catch {}
  const isDark = theme === 'dark' || (theme === 'system' && window.matchMedia('(prefers-color-scheme: dark)').matches)
  document.documentElement.setAttribute('data-theme', isDark ? 'dark' : 'light')
}
applyTheme()

const app = createApp(ScreenshotOverlay)
app.mount('#app')
