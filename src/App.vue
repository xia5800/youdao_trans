<template>
  <div class="app-layout">
    <TitleBar />
    <div class="app-content">
      <AppSidebar />
      <div class="main-panel">
        <router-view />
      </div>
    </div>
    <div class="toast-container">
      <div
        v-for="t in toasts"
        :key="t.id"
        class="toast-item"
      >{{ t.message }}</div>
    </div>
  </div>
</template>

<script setup>
import { onMounted, onUnmounted } from 'vue'
import { useRouter } from 'vue-router'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import TitleBar from './components/TitleBar.vue'
import AppSidebar from './components/AppSidebar.vue'
import { useToast } from './composables/useToast.js'
import { useHotkey } from './composables/useHotkey.js'
import { useSettings } from './composables/useSettings.js'

const router = useRouter()
const { toasts, showToast } = useToast()
const { useHotkeyListener } = useHotkey()
const { load } = useSettings()

let unlistenError = null
let unlistenOcrError = null
let unlistenNavigate = null
let unlistenShortcutsToggled = null
let unlistenCheckOcr = null
onMounted(async () => {
  // 请求通知权限（Windows Toast）
  import('@tauri-apps/plugin-notification').then(({ isPermissionGranted, requestPermission }) => {
    isPermissionGranted().then((granted) => { if (!granted) requestPermission() })
  })

  // 禁用默认右键菜单
  document.addEventListener('contextmenu', (e) => e.preventDefault())

  load()
  unlistenError = await listen('selection-translate-error', (event) => {
    showToast(`划词翻译出错: ${event.payload}`)
    console.error('划词翻译出错:', event.payload)
  })
  unlistenOcrError = await listen('ocr-error', (event) => {
    showToast(`OCR 识别失败: ${event.payload}`)
  })
  unlistenNavigate = await listen('navigate', (event) => {
    router.push(event.payload)
  })
  unlistenShortcutsToggled = await listen('shortcuts-toggled', (event) => {
    showToast(event.payload ? '快捷键已启用' : '快捷键已停用')
  })
  unlistenCheckOcr = await listen('check-pending-ocr', () => {
    checkPendingOcr()
  })
})

onUnmounted(() => {
  if (unlistenError) unlistenError()
  if (unlistenOcrError) unlistenOcrError()
  if (unlistenNavigate) unlistenNavigate()
  if (unlistenShortcutsToggled) unlistenShortcutsToggled()
  if (unlistenCheckOcr) unlistenCheckOcr()
})

async function checkPendingOcr() {
  const pending = await invoke('take_pending_ocr')
  if (!pending) return
  try {
    const text = await invoke('ocr_command', { base64Img: pending })
    if (text) {
      console.log('OCR 识别完成:', text)
    }
  } catch (e) {
    console.error(`OCR 识别失败: ${e}`)
  }
}

useHotkeyListener((hk) => {
  if (hk.id === 'translate' || hk.id === 'selectionTranslate') return
  if (hk.id === 'screenshot') {
    invoke('start_screenshot').catch(e => showToast(`截图启动失败: ${e}`))
    return
  }
  if (hk.id === 'ocr') {
    return
  }
  showToast(`快捷键 ${hk.combo} 已触发：${hk.label} (功能待实现)`)
})
</script>

<style scoped>
.app-layout {
  flex-direction: column;
}

.app-content {
  display: flex;
  flex: 1;
  overflow: hidden;
  height: 100%;
}

.toast-container {
  position: fixed;
  top: 16px;
  left: 50%;
  transform: translateX(-50%);
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
  z-index: 9999;
  pointer-events: none;
}

.toast-item {
  background: var(--bg-card);
  color: var(--text-primary);
  padding: 10px 20px;
  border-radius: 8px;
  box-shadow: var(--shadow-tooltip);
  font-size: 13px;
  white-space: nowrap;
  animation: slideDown 0.3s ease;
  border: 1px solid var(--border);
}

@keyframes slideDown {
  from {
    opacity: 0;
    transform: translateY(-20px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}


</style>
