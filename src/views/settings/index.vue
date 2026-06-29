<template>
  <div class="settings-container">
    <div class="settings-sidebar">
      <div class="settings-category">设置</div>
      <div
        v-for="item in navItems"
        :key="item.id"
        class="settings-nav-item"
        :class="{ active: isActive(item.id) }"
        @click="navigate(item.id)"
      >{{ item.label }}</div>
    </div>
    <div class="settings-content">
      <router-view />
    </div>
  </div>
</template>

<script setup>
import { useRouter, useRoute } from 'vue-router'

const router = useRouter()
const route = useRoute()

const navItems = [
  { id: 'preference', label: '偏好设置' },
  { id: 'hotkey', label: '快捷键设置' },
  { id: 'advanced', label: '语音播放设置' },
  { id: 'translator', label: '翻译源设置' },
  { id: 'ocr', label: 'OCR设置' },
  { id: 'about', label: '关于' }
]

function isActive(id) {
  return route.path === `/settings/${id}`
}

function navigate(id) {
  router.push(`/settings/${id}`)
}
</script>

<style scoped>
.settings-container {
  flex: 1;
  display: flex;
  margin: 12px;
  background: var(--bg-card);
  border-radius: 12px;
  box-shadow: var(--shadow-card);
  overflow: hidden;
  min-height: 0;
}

.settings-sidebar {
  width: 140px;
  background-color: var(--bg-sidebar);
  border-right: 1px solid var(--border);
  padding: 20px 0;
  flex-shrink: 0;
  overflow-y: auto;
}

.settings-category {
  padding: 6px 16px;
  font-size: 14px;
  font-weight: 600;
  color: var(--text-tertiary);
  letter-spacing: 0.5px;
}

.settings-nav-item {
  padding: 8px 16px;
  font-size: 13px;
  color: var(--text-secondary);
  cursor: default;
  transition: all 0.2s;
  border-left: 3px solid transparent;
}

.settings-nav-item.active {
  background-color: var(--bg-active);
  color: var(--accent);
  border-left-color: var(--accent);
}

.settings-nav-item:hover:not(.active) {
  background-color: var(--bg-hover);
  color: var(--text-primary);
}

.settings-content {
  flex: 1;
  padding: 20px 24px;
  overflow-y: auto;
  background: var(--bg-card);
}

@media (max-width: 1000px) {
  .settings-container {
    margin: 16px 20px 20px 20px;
  }
  .settings-sidebar {
    width: 140px;
  }
  .settings-nav-item {
    padding: 6px 12px;
    font-size: 12px;
  }
  .settings-category {
    padding: 4px 12px;
    font-size: 10px;
  }
  .settings-content {
    padding: 16px;
  }
}
</style>

<style>
.section-title {
  font-size: 18px;
  font-weight: 600;
  color: var(--text-primary);
  margin-bottom: 20px;
  padding-bottom: 10px;
  border-bottom: 2px solid var(--border);
}

.subsection-title {
  font-size: 15px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 20px 0 12px 0;
  padding-left: 8px;
  border-left: 3px solid var(--accent);
}

.setting-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px 0;
  border-bottom: 1px solid var(--border);
  flex-wrap: wrap;
  gap: 24px;
}

.setting-item:last-child {
  border-bottom: none;
}

.setting-label {
  font-size: 14px;
  color: var(--text-primary);
  font-weight: 500;
}

.setting-desc {
  font-size: 12px;
  color: var(--text-tertiary);
  margin-top: 2px;
}

.setting-control {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  gap: 12px;
  flex-wrap: wrap;
}

.radio-group {
  display: flex;
  gap: 12px;
  flex-wrap: wrap;
}

.radio-label {
  display: flex;
  align-items: center;
  gap: 4px;
  font-size: 13px;
  color: var(--text-secondary);
  cursor: default;
}

.switch {
  width: 40px;
  height: 22px;
  background-color: var(--bg-switch-off);
  border-radius: 22px;
  position: relative;
  cursor: default;
  transition: 0.2s;
  flex-shrink: 0;
}

.switch.active {
  background-color: var(--accent);
}

.switch-knob {
  width: 18px;
  height: 18px;
  background-color: var(--text-inverse);
  border-radius: 50%;
  position: absolute;
  top: 2px;
  left: 2px;
  transition: 0.2s;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.2);
}

.switch.active .switch-knob {
  left: 20px;
}

.number-input {
  width: 90px;
  padding: 4px 8px;
  border: 1px solid var(--border-strong);
  border-radius: 6px;
  font-size: 13px;
  outline: none;
  background: var(--bg-card);
  color: var(--text-primary);
}

.number-input:focus {
  border-color: var(--accent);
}

.path-group {
  display: flex;
  gap: 6px;
  align-items: center;
  max-width: 360px;
}

.path-display {
  flex: 1;
  min-width: 0;
  overflow: auto;
  white-space: nowrap;
  padding: 6px 10px;
  border: 1px solid var(--border-strong);
  border-radius: 6px;
  font-size: 12px;
  background: var(--bg-input);
  color: var(--text-tertiary);
  cursor: text;
  user-select: text;
}

.path-display::-webkit-scrollbar {
  height: 4px;
}

.path-display::-webkit-scrollbar-thumb {
  background: var(--scrollbar-thumb);
  border-radius: 2px;
}

.btn-select {
  padding: 4px 12px;
  background: var(--bg-hover);
  border: 1px solid var(--border-strong);
  border-radius: 6px;
  font-size: 12px;
  cursor: default;
  transition: 0.2s;
  color: var(--text-secondary);
}

.btn-select:hover {
  background: var(--border-strong);
}

.slider {
  width: 150px;
  height: 4px;
  -webkit-appearance: none;
  background: var(--border-strong);
  border-radius: 2px;
  outline: none;
}

.slider::-webkit-slider-thumb {
  -webkit-appearance: none;
  width: 14px;
  height: 14px;
  background: var(--accent);
  border-radius: 50%;
  cursor: default;
}

.volume-value {
  font-size: 12px;
  color: var(--text-secondary);
  min-width: 35px;
}

.checkbox-label {
  display: flex;
  align-items: center;
  gap: 6px;
  cursor: default;
  color: var(--text-primary);
}

.checkbox-label input,
.radio-label input {
  accent-color: var(--accent);
  width: 14px;
  height: 14px;
  cursor: default;
}

.select-input {
  padding: 4px 8px;
  border: 1px solid var(--border-strong);
  border-radius: 6px;
  font-size: 13px;
  background: var(--bg-card);
  color: var(--text-primary);
  outline: none;
  cursor: default;
}

.select-input:focus {
  border-color: var(--accent);
}

.password-field {
  position: relative;
  display: inline-flex;
  align-items: center;
}

.password-field input[type="password"]::-ms-reveal,
.password-field input[type="password"]::-ms-clear {
  display: none;
}

.password-field input[type="password"]::-webkit-reveal {
  display: none;
}

.password-field .key-input {
  padding-right: 26px;
}

.eye-btn {
  position: absolute;
  right: 4px;
  display: flex;
  align-items: center;
  justify-content: center;
  width: 20px;
  height: 20px;
  border: none;
  background: transparent;
  color: var(--text-tertiary);
  cursor: pointer;
  padding: 0;
  border-radius: 4px;
  transition: color 0.15s;
}

.eye-btn:hover {
  color: var(--text-primary);
}

.key-input {
  width: 160px;
  padding: 4px 8px;
  border: 1px solid var(--border-strong);
  border-radius: 6px;
  font-size: 11px;
  background: var(--bg-card);
  color: var(--text-primary);
}

.key-input.input-error {
  border-color: #e74c3c;
}

.error-hint {
  flex-basis: 100%;
  order: 99;
  font-size: 11px;
  color: #e74c3c;
  line-height: 1.4;
}

.microsoft-wrap {
  display: block;
}

@media (max-width: 1000px) {
  .section-title {
    font-size: 16px;
    margin-bottom: 16px;
  }
  .key-input {
    width: 120px;
  }
}
</style>
