<template>
  <div class="password-field">
    <input :type="visible ? 'text' : 'password'"
           class="key-input detail-input"
           :placeholder="placeholder"
           :value="modelValue"
           @input="$emit('update:modelValue', $event.target.value)">
    <button class="eye-btn" @click="visible = !visible" type="button">
      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
        <use :href="`/icons.svg#icon-eye-${visible ? 'off' : 'open'}`"></use>
      </svg>
    </button>
  </div>
</template>

<script setup>
import { ref } from 'vue'

defineProps({
  modelValue: String,
  placeholder: { type: String, default: '' }
})

defineEmits(['update:modelValue'])

const visible = ref(false)
</script>

<style scoped>
.password-field {
  display: flex;
  align-items: center;
  gap: 4px;
}

.password-field .key-input {
  flex: 1;
}
</style>
