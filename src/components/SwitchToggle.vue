<template>
  <div class="switch" :class="{ active: modelValue }" @click="$emit('update:modelValue', !modelValue)">
    <div class="switch-knob"></div>
  </div>
</template>

<script setup>
defineProps({ modelValue: Boolean })
defineEmits(['update:modelValue'])
</script>

<style scoped>
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
</style>
