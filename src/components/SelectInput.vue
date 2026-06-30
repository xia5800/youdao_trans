<template>
  <div class="sel-wrap" ref="wrapRef">
    <div class="sel-trigger" :class="{ open }" @click="toggle">
      <span>{{ displayText }}</span>
      <span class="sel-arrow">▼</span>
    </div>
    <div v-show="open" class="sel-panel" :style="panelStyle" @click.stop>
      <div
        v-for="opt in options"
        :key="opt.value"
        class="sel-item"
        :class="{ selected: modelValue === opt.value }"
        @click="select(opt.value)"
      >{{ opt.label }}</div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, nextTick, onMounted, onUnmounted } from 'vue'

const props = defineProps({
  options: { type: Array, required: true },
  modelValue: { type: String, default: '' },
})

const emit = defineEmits(['update:modelValue'])

const open = ref(false)
const wrapRef = ref(null)
const panelStyle = ref({})

const displayText = computed(() => {
  const found = props.options.find(o => o.value === props.modelValue)
  return found ? found.label : ''
})

async function toggle() {
  open.value = !open.value
  if (open.value) {
    await nextTick()
    positionPanel()
  }
}

function positionPanel() {
  const wrap = wrapRef.value
  if (!wrap) return
  const rect = wrap.getBoundingClientRect()
  const panel = wrap.querySelector('.sel-panel')
  if (!panel) return
  const panelRect = panel.getBoundingClientRect()

  const style = {}
  const spaceBelow = window.innerHeight - rect.bottom
  const spaceAbove = rect.top
  const spaceRight = window.innerWidth - rect.left
  const panelW = panelRect.width

  if (spaceBelow < panelRect.height && spaceAbove > spaceBelow) {
    style.bottom = '100%'
    style.marginBottom = '4px'
    style.top = 'auto'
  }
  if (panelW > spaceRight) {
    style.right = '0'
    style.left = 'auto'
  }
  panelStyle.value = style
}

function select(val) {
  open.value = false
  emit('update:modelValue', val)
}

function onClickOutside(e) {
  if (wrapRef.value && !wrapRef.value.contains(e.target)) {
    open.value = false
  }
}

onMounted(() => document.addEventListener('click', onClickOutside))
onUnmounted(() => document.removeEventListener('click', onClickOutside))
</script>

<style scoped>
.sel-wrap {
  position: relative;
}

.sel-trigger {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 4px 8px;
  border: 1px solid var(--border-strong);
  border-radius: 6px;
  font-size: 13px;
  background: var(--bg-card);
  color: var(--text-primary);
  cursor: default;
  user-select: none;
  white-space: nowrap;
  transition: border-color 0.15s;
}

.sel-trigger:hover {
  border-color: var(--accent);
}

.sel-trigger.open {
  border-color: var(--accent);
}

.sel-arrow {
  font-size: 10px;
  color: var(--text-tertiary);
  transition: transform 0.2s;
}

.sel-trigger.open .sel-arrow {
  transform: rotate(180deg);
}

.sel-panel {
  position: absolute;
  top: calc(100% + 4px);
  left: 0;
  min-width: 100%;
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: 8px;
  box-shadow: 0 6px 24px rgba(0,0,0,0.10), 0 1px 4px rgba(0,0,0,0.06);
  padding: 4px;
  z-index: 100;
  animation: sel-fade-in 0.1s ease;
  white-space: nowrap;
}

@keyframes sel-fade-in {
  from { opacity: 0; transform: translateY(-4px); }
  to { opacity: 1; transform: translateY(0); }
}

.sel-item {
  padding: 6px 12px;
  font-size: 13px;
  color: var(--text-primary);
  border-radius: 4px;
  cursor: default;
  transition: background 0.1s;
}

.sel-item.selected {
  font-weight: 600;
  color: var(--accent);
}

.sel-item:hover {
  background: var(--accent);
  color: #fff;
}

.sel-item.selected:hover {
  color: #fff;
}
</style>
