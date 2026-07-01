<template>
  <div class="num-wrap" :class="{ disabled }">
    <input
      ref="inputRef"
      type="number"
      class="num-input"
      :value="modelValue"
      @input="onInput"
      @keydown.up.prevent="stepUp"
      @keydown.down.prevent="stepDown"
      :step="step"
      :min="min"
      :max="max"
      :disabled="disabled"
    />
    <div class="num-btns">
      <button class="num-btn" @click="stepUp" :disabled="disabled" tabindex="-1">&#x25B2;</button>
      <button class="num-btn" @click="stepDown" :disabled="disabled" tabindex="-1">&#x25BC;</button>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'

const props = defineProps({
  modelValue: [Number, String],
  step: { type: [Number, String], default: 1 },
  min: { type: [Number, String], default: undefined },
  max: { type: [Number, String], default: undefined },
  disabled: Boolean,
})

const emit = defineEmits(['update:modelValue'])
const inputRef = ref(null)

function onInput(e) {
  const val = e.target.value
  emit('update:modelValue', val === '' ? '' : Number(val))
}

function stepUp() {
  const el = inputRef.value
  if (!el || el.disabled) return
  el.stepUp()
  emit('update:modelValue', Number(el.value))
}

function stepDown() {
  const el = inputRef.value
  if (!el || el.disabled) return
  el.stepDown()
  emit('update:modelValue', Number(el.value))
}
</script>

<style scoped>
.num-wrap {
  display: inline-flex;
  align-items: stretch;
  width: 90px;
  border: 1px solid var(--border-strong);
  border-radius: 6px;
  background: var(--bg-card);
  overflow: hidden;
  transition: border-color 0.15s;
}

.num-wrap:focus-within {
  border-color: var(--accent);
}

.num-wrap.disabled {
  opacity: 0.5;
}

.num-input {
  flex: 1;
  min-width: 0;
  border: none;
  outline: none;
  padding: 4px 6px;
  font-size: 13px;
  background: transparent;
  color: var(--text-primary);
  -moz-appearance: textfield;
}

.num-btns {
  display: flex;
  flex-direction: column;
  width: 20px;
  border-left: 1px solid var(--border-strong);
  flex-shrink: 0;
}

.num-btn {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  border: none;
  background: transparent;
  color: var(--text-tertiary);
  font-size: 8px;
  cursor: default;
  padding: 0;
  line-height: 1;
  transition: background 0.1s, color 0.1s;
}

.num-btn:hover:not(:disabled) {
  background: var(--bg-hover);
  color: var(--text-primary);
}

.num-btn:active:not(:disabled) {
  background: var(--accent);
  color: var(--text-inverse);
}

.num-btn:disabled {
  cursor: default;
  color: var(--text-tertiary);
}

.num-btn + .num-btn {
  border-top: 1px solid var(--border-strong);
}
</style>
