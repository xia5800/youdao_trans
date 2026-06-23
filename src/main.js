import { createApp, nextTick } from 'vue'
import App from './App.vue'
import router from './router'
import './assets/styles/common.css'
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow'
import { invoke } from '@tauri-apps/api/core'

const appWindow = getCurrentWebviewWindow()

const app = createApp(App)
app.use(router)
app.mount('#app')

if (appWindow.label === 'main') {
  function resolveThemeColor() {
    let saved = null
    try { saved = localStorage.getItem('youdao-theme') } catch {}
    let isDark
    if (saved === 'dark') isDark = true
    else if (saved === 'light') isDark = false
    else isDark = window.matchMedia('(prefers-color-scheme: dark)').matches
    return isDark ? { r: 26, g: 29, b: 35 } : { r: 245, g: 247, b: 251 }
  }

  nextTick(() => {
    requestAnimationFrame(() => {
      const { r, g, b } = resolveThemeColor()
      invoke('set_window_bg', { r, g, b }).then(() => {
        invoke('is_autostart_launched').then((isAutostart) => {
          if (!isAutostart) {
            appWindow.show()
          }
        })
      })
    })
  })
}
