import { ref } from 'vue'

const toasts = ref([])
let nextId = 0

export function useToast() {
  function showToast(message, duration = 3000) {
    const id = ++nextId
    toasts.value.push({ id, message })
    setTimeout(() => {
      toasts.value = toasts.value.filter(t => t.id !== id)
    }, duration)
  }

  return { toasts, showToast }
}
