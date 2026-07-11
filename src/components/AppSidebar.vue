<template>
  <div class="sidebar" :class="{ collapsed: isCollapsed }" id="sidebar">
    <div class="logo-area-side">
      <div class="logo-icon-side">Y</div>
      <div class="logo-text-side">优道<span>翻译</span></div>
    </div>
    <div class="nav-menu">
      <div
        v-for="item in menuItems"
        :key="item.name"
        ref="menuItemRefs"
        class="menu-item"
        :class="{ active: isActive(item.route) }"
        :data-tooltip="item.label"
        @click="navigate(item.route)"
        @mouseenter="updateTooltipPos"
      >
        <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
          <use :href="`/icons.svg#icon-${item.icon}`"></use>
        </svg>
        <span>{{ item.label }}</span>
      </div>
    </div>
    <div class="sidebar-footer">
      <div class="settings-divider"></div>
      <div
        class="collapse-menu-item"
        id="collapseBtn"
        :data-tooltip="isCollapsed ? '展开' : '收起'"
        @click="toggle"
        @mouseenter="updateTooltipPos"
      >
        <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"
             :class="{ 'icon-flip': isCollapsed }">
          <use href="/icons.svg#icon-collapse"></use>
        </svg>
        <span class="collapse-btn-text">{{ isCollapsed ? '展开' : '收起' }}</span>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'
import { useRouter, useRoute } from 'vue-router'

const router = useRouter()
const route = useRoute()
const isCollapsed = ref(true)

const menuItems = [
  { route: '/translate', label: '首页', icon: 'home' },
  { route: '/dictionary', label: '词典', icon: 'dictionary' },
  { route: '/history', label: '翻译记录', icon: 'history' },
  { route: '/settings', label: '设置', icon: 'settings' }
]

function isActive(path) {
  return route.path === path || route.path.startsWith(path + '/')
}

function navigate(path) {
  router.push(path)
}

function updateTooltipPos(e) {
  if (!isCollapsed.value) return
  const rect = e.currentTarget.getBoundingClientRect()
  e.currentTarget.style.setProperty('--tooltip-x', `${rect.right + 12}px`)
  e.currentTarget.style.setProperty('--tooltip-y', `${rect.top + rect.height / 2}px`)
}

function toggle() {
  isCollapsed.value = !isCollapsed.value
}
</script>

<style scoped>
.sidebar {
  width: 160px;
  background-color: var(--bg-sidebar);
  border-right: 1px solid var(--border-hover);
  display: flex;
  flex-direction: column;
  transition: width 0.2s cubic-bezier(0.2, 0.9, 0.4, 1.1);
  flex-shrink: 0;
  overflow-x: hidden;
  overflow-y: auto;
  white-space: nowrap;
  height: 100%;
  position: relative;
  z-index: 100;
}

.sidebar.collapsed {
  width: 64px;
}

.sidebar.collapsed .logo-text-side,
.sidebar.collapsed .menu-item span,
.sidebar.collapsed .collapse-btn-text {
  display: none;
}

.sidebar.collapsed .logo-area-side {
  justify-content: center;
  padding: 24px 0 16px 0;
}

.sidebar.collapsed .menu-item {
  justify-content: center;
  padding: 12px 0;
  position: relative;
}

.sidebar.collapsed .collapse-menu-item {
  justify-content: center;
  padding: 12px 0;
  position: relative;
}

.sidebar:not(.collapsed) .logo-text-side,
.sidebar:not(.collapsed) .menu-item span,
.sidebar:not(.collapsed) .collapse-btn-text {
  display: inline-block;
}

.sidebar:not(.collapsed) .logo-area-side {
  justify-content: flex-start;
  padding: 24px 10px 16px 14px;
}

.sidebar:not(.collapsed) .menu-item {
  justify-content: flex-start;
  padding: 12px 10px;
}

.sidebar:not(.collapsed) .collapse-menu-item {
  justify-content: flex-start;
  padding: 14px 10px;
}

.sidebar.collapsed .menu-item:hover::after,
.sidebar.collapsed .collapse-menu-item:hover::after {
  content: attr(data-tooltip);
  position: fixed;
  left: var(--tooltip-x, 92px);
  top: var(--tooltip-y, 50%);
  transform: translateY(-50%);
  padding: 8px 14px;
  background-color: var(--tooltip-bg);
  color: var(--text-inverse);
  font-size: 13px;
  font-weight: 500;
  white-space: nowrap;
  border-radius: 8px;
  box-shadow: var(--shadow-tooltip);
  z-index: 9999;
  pointer-events: none;
  font-family: inherit;
}

.sidebar.collapsed .menu-item:hover::before,
.sidebar.collapsed .collapse-menu-item:hover::before {
  content: '';
  position: fixed;
  left: calc(var(--tooltip-x, 92px) - 8px);
  top: var(--tooltip-y, 50%);
  transform: translateY(-50%);
  width: 0;
  height: 0;
  border-width: 6px;
  border-style: solid;
  border-color: transparent var(--tooltip-bg) transparent transparent;
  z-index: 9999;
  pointer-events: none;
}

.logo-area-side {
  padding: 24px 10px 16px 14px;
  display: flex;
  align-items: center;
  gap: 10px;
  border-bottom: 1px solid var(--border);
  margin-bottom: 24px;
  transition: all 0.2s;
  flex-shrink: 0;
}

.logo-icon-side {
  width: 34px;
  height: 34px;
  background: var(--accent);
  border-radius: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 20px;
  font-weight: 700;
  color: var(--text-inverse);
  box-shadow: 0 4px 8px rgba(45, 122, 255, 0.2);
  flex-shrink: 0;
}

.logo-text-side {
  font-size: 20px;
  font-weight: 650;
  color: var(--text-primary);
  letter-spacing: 1px;
}

.logo-text-side span {
  color: var(--accent);
}

.nav-menu {
  flex: 1;
  padding: 0 10px;
  display: flex;
  flex-direction: column;
  gap: 8px;
  overflow-y: auto;
  overflow-x: hidden;
}

.nav-menu::-webkit-scrollbar {
  width: 4px;
}

.nav-menu::-webkit-scrollbar-track {
  background: transparent;
}

.nav-menu::-webkit-scrollbar-thumb {
  background: var(--scrollbar-thumb);
  border-radius: 4px;
}

.menu-item {
  display: flex;
  align-items: center;
  gap: 14px;
  padding: 12px 16px;
  border-radius: 14px;
  font-size: 15px;
  font-weight: 500;
  color: var(--text-secondary);
  cursor: default;
  transition: all 0.2s ease;
  background-color: transparent;
}

.menu-item svg {
  width: 20px;
  height: 20px;
  stroke: var(--text-tertiary);
  stroke-width: 1.5;
  fill: none;
  flex-shrink: 0;
}

.menu-item.active {
  background-color: var(--bg-active);
  color: var(--accent);
}

.menu-item.active svg {
  stroke: var(--accent);
}

.menu-item:hover:not(.active) {
  background-color: var(--bg-hover);
  color: var(--text-primary);
}

.collapse-menu-item {
  display: flex;
  align-items: center;
  gap: 14px;
  padding: 14px 10px;
  border-radius: 14px;
  font-size: 17px;
  font-weight: 500;
  color: var(--text-secondary);
  cursor: default;
  transition: all 0.2s ease;
  background-color: transparent;
  margin: 0 10px 20px 10px;
}

.collapse-menu-item svg {
  width: 22px;
  height: 22px;
  stroke: var(--text-tertiary);
  stroke-width: 1.5;
  fill: none;
  flex-shrink: 0;
}

.collapse-menu-item:hover {
  background-color: var(--bg-hover);
  color: var(--text-primary);
}

.collapse-btn-text {
  font-size: 17px;
  font-weight: 500;
}

.icon-flip {
  transform: scaleX(-1);
}

.settings-divider {
  margin: 0 20px 10px 20px;
  border-top: 1px solid var(--border);
  flex-shrink: 0;
}

.sidebar-footer {
  display: flex;
  flex-direction: column;
  flex-shrink: 0;
}
</style>
