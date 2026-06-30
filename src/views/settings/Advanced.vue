<template>
  <div>
    <div class="section-title">语音播放设置</div>

    <div class="setting-item">
      <div>
        <div class="setting-label">语音引擎</div>
        <div class="setting-desc">选择朗读译文时使用的语音引擎</div>
      </div>
      <SelectInput
        :options="engineOptions"
        v-model="ttsEngine"
      />
    </div>

    <div v-if="ttsEngine === 'local'" class="setting-item">
      <div>
        <div class="setting-label">发声人</div>
        <div class="setting-desc">选择 Edge TTS 使用的语音</div>
      </div>
      <SelectInput
        :options="voiceOptions"
        v-model="ttsVoice"
      />
    </div>
  </div>
</template>

<script setup>
import SelectInput from '../../components/SelectInput.vue'
import { useSettings } from '../../composables/useSettings.js'

const { ttsEngine, ttsVoice } = useSettings()

const engineOptions = [
  { value: 'browser', label: '浏览器内置 (speechSynthesis)' },
  { value: 'local', label: 'Edge TTS (云服务)' },
]

const voiceOptions = [
  { value: 'zh-CN-XiaoxiaoNeural', label: '晓晓 (中文女性)' },
  { value: 'zh-CN-YunxiNeural', label: '云希 (中文男性)' },
  { value: 'en-US-AriaNeural', label: 'Aria (英文女性)' },
  { value: 'en-US-GuyNeural', label: 'Guy (英文男性)' },
  { value: 'ja-JP-NanamiNeural', label: 'Nanami (日语女性)' },
  { value: 'ja-JP-KeitaNeural', label: 'Keita (日语男性)' },
  { value: 'ko-KR-SunHiNeural', label: 'Sun-Hi (韩语女性)' },
  { value: 'ko-KR-InJoonNeural', label: 'InJoon (韩语男性)' },
  { value: 'fr-FR-DeniseNeural', label: 'Denise (法语女性)' },
  { value: 'de-DE-KatjaNeural', label: 'Katja (德语女性)' },
  { value: 'es-ES-ElviraNeural', label: 'Elvira (西语女性)' },
]
</script>
