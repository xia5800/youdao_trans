import { ref, computed, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'

function filled(v) { return v && v.trim() }

export function useServiceConfig({ serviceList, activeService, keys, validators }) {
  const selectedKey = ref(activeService.value || serviceList[0].key)
  const selectedService = computed(() => serviceList.find(s => s.key === selectedKey.value))

  const errors = {}
  const errorRefs = {}

  if (validators) {
    for (const v of validators) {
      const err = ref('')
      const deps = v.fields.map(f => () => keys.value[f])
      if (v.activeKey) deps.push(activeService)
      errorRefs[v.key] = err
      errors[v.key] = err
      watch(deps, () => {
        if (v.activeKey && activeService.value !== v.activeKey) {
          err.value = ''
          return
        }
        const allFilled = v.fields.every(f => filled(keys.value[f]))
        if (!allFilled) {
          err.value = v.message
        } else {
          err.value = ''
        }
      })
    }
  }

  function toggleService(key) {
    selectedKey.value = key
    activeService.value = activeService.value === key ? null : key
  }

  async function saveConfig({ settings, showToastOnce, showToast }) {
    const key = selectedKey.value

    if (validators) {
      for (const v of validators) {
        if (v.activeKey && key !== v.activeKey) continue
        const allFilled = v.fields.every(f => filled(keys.value[f]))
        if (!allFilled) {
          showToastOnce(v.message)
          return
        }
      }
    }

    try {
      await invoke('save_config', { json: JSON.stringify(settings) })
      showToastOnce('已保存')
    } catch (e) {
      showToast(`保存失败: ${e}`)
    }
  }

  return { selectedKey, selectedService, errors, errorRefs, toggleService, saveConfig }
}

export { filled }
