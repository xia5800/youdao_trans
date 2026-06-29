<template>
  <div>
    <div class="section-title">全局快捷键设置</div>
    <div class="setting-desc" style="margin-bottom:12px;color:var(--text-tertiary);font-size:12px;">点击快捷键组合即可录制修改，仅按修饰键无效</div>
    <div class="setting-item" v-for="hk in hotkeys" :key="hk.id">
      <div>
        <div class="setting-label">{{ hk.label }}</div>
        <div class="setting-desc">{{ hk.desc }}</div>
      </div>
      <div
        class="hotkey-recorder"
        :class="{ recording: isRecording(hk.id) }"
        @click="startRecording(hk.id)"
        tabindex="0"
      >
        <span v-if="isRecording(hk.id) && !tempCombo">按下按键...</span>
        <span v-else-if="isRecording(hk.id) && tempCombo">{{ tempCombo }}</span>
        <span v-else>{{ hk.combo }}</span>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted } from 'vue'
import { useHotkey } from '../../composables/useHotkey.js'
import { useSettings } from '../../composables/useSettings.js'
import { useToast } from '../../composables/useToast.js'

const { hotkeys, recordingId, startRecording, stopRecording, isRecording, updateCombo, eventToCombo } = useHotkey()
const { settings } = useSettings()
const { showToast } = useToast()

const tempCombo = ref('')
const ignoreKeydownUntil = ref(0)

onMounted(async () => {
  document.addEventListener('keydown', onRecordKeydown)
  for (const hk of hotkeys.value) {
    if (settings.hotkeys[hk.id]) {
      hk.combo = settings.hotkeys[hk.id]
    }
  }
})

onUnmounted(() => {
  document.removeEventListener('keydown', onRecordKeydown)
})

function onRecordKeydown(e) {
  if (Date.now() < ignoreKeydownUntil.value) return
  if (!recordingId.value) return
  if (e.key === 'Escape') {
    stopRecording()
    return
  }
  const combo = eventToCombo(e)
  if (!combo) return
  e.preventDefault()
  const conflict = hotkeys.value.find(h => h.id !== recordingId.value && h.combo === combo)
  if (conflict) {
    showToast(`快捷键冲突：${conflict.label} 已使用 ${combo}`)
    stopRecording()
    return
  }
  updateCombo(recordingId.value, combo)
  settings.hotkeys[recordingId.value] = combo
  ignoreKeydownUntil.value = Date.now() + 600
  stopRecording()
}
</script>

<style scoped>
.hotkey-recorder {
  padding: 6px 16px;
  border: 1px solid var(--border-strong);
  border-radius: 6px;
  font-size: 12px;
  font-family: monospace;
  background: var(--bg-input);
  min-width: 120px;
  text-align: center;
  color: var(--text-primary);
  cursor: pointer;
  transition: border-color 0.15s, box-shadow 0.15s;
  user-select: none;
}

.hotkey-recorder:hover {
  border-color: var(--accent);
}

.hotkey-recorder.recording {
  border-color: var(--accent);
  box-shadow: 0 0 0 3px var(--accent-transparent);
  animation: recorder-pulse 1s ease-in-out infinite;
}

@keyframes recorder-pulse {
  50% { box-shadow: 0 0 0 6px var(--accent-transparent); }
}
</style>
