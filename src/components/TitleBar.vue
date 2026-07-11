<template>
  <div class="title-bar" data-tauri-drag-region @dblclick="toggleMaximize" @mousedown="onDragStart">
    <div class="title-bar-left">
      <img src="/logo.png" width="18" height="18" alt="优道翻译">
      <span class="title-text">优道翻译</span>
    </div>
    <div class="title-bar-center" />
    <div class="title-bar-right">
      <button class="title-btn pin-btn" :class="{ active: isPinned }" @click="togglePin" :title="isPinned ? '取消置顶' : '置顶窗口'">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8">
          <use :href="isPinned ? '/icons.svg#icon-pin' : '/icons.svg#icon-pin-off'"></use>
        </svg>
      </button>
      <button class="title-btn minimize-btn" @click="minimize">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <use href="/icons.svg#icon-minimize"></use>
        </svg>
      </button>
      <button class="title-btn maximize-btn" @click="toggleMaximize">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <use :href="isMaximized ? '/icons.svg#icon-restore' : '/icons.svg#icon-maximize'"></use>
        </svg>
      </button>
      <button class="title-btn close-btn" @click="close">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <use href="/icons.svg#icon-close"></use>
        </svg>
      </button>
    </div>
  </div>

  <Teleport to="body">
    <div v-if="showConfirm" class="close-dialog-overlay" @click.self="cancelClose">
      <div class="close-dialog">
        <div class="dialog-title">关闭确认</div>
        <div class="dialog-desc">关闭后程序将运行在系统托盘中，继续后台服务</div>
        <div class="dialog-options">
          <label class="dialog-option">
            <input type="radio" v-model="closeAction" value="tray">
            <span>最小化到系统托盘</span>
          </label>
          <label class="dialog-option">
            <input type="radio" v-model="closeAction" value="exit">
            <span>退出程序</span>
          </label>
        </div>
        <label class="dialog-remember">
          <input type="checkbox" v-model="rememberChoice">
          <span>记住选择，下次不再询问</span>
        </label>
        <div class="dialog-buttons">
          <button class="dialog-btn cancel" @click="cancelClose">取消</button>
          <button class="dialog-btn confirm" @click="confirmClose">确定</button>
        </div>
      </div>
    </div>
  </Teleport>
</template>

<script setup>
import { ref, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow'
import { listen } from '@tauri-apps/api/event'
import { useSettings } from '../composables/useSettings.js'

const appWindow = getCurrentWebviewWindow()
const { settings } = useSettings()
const isPinned = ref(false)
const isMaximized = ref(false)

const showConfirm = ref(false)
const closeAction = ref('tray')
const rememberChoice = ref(false)
let unlistenDismiss = null

onMounted(async () => {
  unlistenDismiss = await listen('dismiss-close-dialog', () => {
    showConfirm.value = false
  })
  try {
    isPinned.value = await invoke('is_window_pinned')
  } catch (e) {
    console.error('Failed to get pin state:', e)
  }
  try {
    isMaximized.value = await invoke('is_window_maximized')
  } catch (e) {
    console.error('Failed to get maximize state:', e)
  }
})

onUnmounted(() => {
  if (unlistenDismiss) unlistenDismiss()
})

async function onDragStart(e) {
  if (e.target.closest('.title-btn')) return
  try {
    await appWindow.startDragging()
  } catch (e) {
    console.error('Failed to start dragging:', e)
  }
}

async function togglePin() {
  try {
    isPinned.value = await invoke('pin_window')
  } catch (e) {
    console.error('Failed to toggle pin:', e)
  }
}

async function minimize() {
  try {
    await invoke('minimize_window')
  } catch (e) {
    console.error('Failed to minimize:', e)
  }
}

async function toggleMaximize() {
  try {
    isMaximized.value = await invoke('maximize_window')
  } catch (e) {
    console.error('Failed to toggle maximize:', e)
  }
}

async function close() {
  const behavior = settings.closeBehavior || 'ask'
  if (behavior === 'exit') {
    await invoke('app_exit')
  } else if (behavior === 'tray') {
    await appWindow.hide()
  } else {
    closeAction.value = 'tray'
    rememberChoice.value = false
    showConfirm.value = true
  }
}

function cancelClose() {
  showConfirm.value = false
}

async function confirmClose() {
  const action = closeAction.value
  showConfirm.value = false
  if (rememberChoice.value) {
    settings.closeBehavior = action
  }
  if (action === 'exit') {
    await invoke('app_exit')
  } else {
    await appWindow.hide()
  }
}
</script>

<style scoped>
.title-bar {
  display: flex;
  align-items: center;
  height: 32px;
  min-height: 32px;
  background: var(--bg-card);
  border-bottom: 1px solid var(--border);
  user-select: none;
  flex-shrink: 0;
  position: relative;
  z-index: 200;
}

.title-bar-left {
  display: flex;
  align-items: center;
  gap: 8px;
  padding-left: 12px;
  height: 100%;
  min-width: 160px;
}

.title-text {
  font-size: 13px;
  font-weight: 600;
  color: var(--text-secondary);
  letter-spacing: 0.3px;
}

.title-bar-center {
  flex: 1;
  height: 100%;
}

.title-bar-right {
  display: flex;
  align-items: center;
  height: 100%;
}

.title-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 46px;
  height: 100%;
  border: none;
  background: transparent;
  color: var(--text-secondary);
  cursor: pointer;
  transition: background 0.15s, color 0.15s;
  outline: none;
}

.title-btn:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

.title-btn:active {
  background: var(--border-strong);
}

.close-btn:hover {
  background: var(--danger);
  color: #fff;
}

.close-btn:active {
  background: #c0392b;
  color: #fff;
}

.pin-btn.active {
  color: var(--accent);
}

.pin-btn.active:hover {
  background: var(--bg-active);
}

.close-dialog-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.35);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 9999;
}

.close-dialog {
  background: var(--bg-card);
  border-radius: 12px;
  padding: 24px 28px;
  width: 360px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.2);
}

.dialog-title {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
  margin-bottom: 8px;
}

.dialog-desc {
  font-size: 12px;
  font-weight: 400;
  color: var(--text-tertiary);
  margin-bottom: 14px;
}

.dialog-options {
  display: flex;
  flex-direction: column;
  gap: 10px;
  margin-bottom: 14px;
}

.dialog-option {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 13px;
  color: var(--text-primary);
  cursor: default;
}

.dialog-option input {
  accent-color: var(--accent);
}

.dialog-remember {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 12px;
  color: var(--text-secondary);
  cursor: default;
  margin-bottom: 18px;
}

.dialog-remember input {
  accent-color: var(--accent);
}

.dialog-buttons {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
}

.dialog-btn {
  padding: 6px 20px;
  border-radius: 6px;
  font-size: 13px;
  border: 1px solid var(--border-strong);
  background: var(--bg-card);
  color: var(--text-primary);
  cursor: default;
  transition: background 0.15s;
}

.dialog-btn:hover {
  background: var(--bg-hover);
}

.dialog-btn.confirm {
  background: var(--accent);
  color: var(--text-inverse);
  border-color: var(--accent);
}

.dialog-btn.confirm:hover {
  background: var(--accent-hover);
}
</style>
