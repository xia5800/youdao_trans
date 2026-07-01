<template>
  <div class="tool-icon" :class="{ speaking }" :data-tooltip="tooltip" @click="$emit('click', $event)">
    <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
      <use :href="`/icons.svg#icon-${icon}`" />
    </svg>
  </div>
</template>

<script setup>
defineProps({
  icon: String,
  tooltip: String,
  speaking: Boolean,
})
defineEmits(['click'])
</script>

<style scoped>
.tool-icon {
  width: 30px;
  height: 30px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 8px;
  background: transparent;
  color: var(--text-tertiary);
  cursor: default;
  transition: color 0.15s, background 0.15s;
  position: relative;
}

.tool-icon:hover {
  color: var(--text-primary);
  background: var(--bg-hover);
}

.tool-icon::after {
  content: attr(data-tooltip);
  position: absolute;
  top: -36px;
  left: 50%;
  transform: translateX(-50%) scale(0.9);
  padding: 6px 12px;
  border-radius: 8px;
  font-size: 12px;
  font-weight: 500;
  white-space: nowrap;
  color: var(--text-inverse);
  background: var(--tooltip-bg);
  box-shadow: var(--shadow-tooltip);
  opacity: 0;
  pointer-events: none;
  transition: opacity 0.15s, transform 0.15s;
}

.tool-icon:hover::after {
  opacity: 1;
  transform: translateX(-50%) scale(1);
}

.tool-icon.speaking {
  color: var(--accent);
  animation: speak-pulse 0.8s ease-in-out infinite;
}

.tool-icon.speaking svg {
  animation: speak-wave 0.8s ease-in-out infinite;
}

@keyframes speak-pulse {
  0%, 100% { background: transparent; }
  50% { background: var(--accent-transparent); }
}

@keyframes speak-wave {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.5; transform: scale(1.1); }
}
</style>
