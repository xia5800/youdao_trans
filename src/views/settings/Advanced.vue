<template>
  <div>
    <div class="section-title">语音播放设置</div>

    <div class="setting-item">
      <div>
        <div class="setting-label">语音播放音量</div>
        <div class="setting-desc">默认范围: 0 - 100%</div>
      </div>
      <div class="setting-control">
        <input type="range" class="slider" :min="VOL_RANGE.min" :max="VOL_RANGE.max" :step="VOL_RANGE.step" v-model.number="volume">
        <span class="volume-value">{{ volume }}%</span>
      </div>
    </div>

    <div class="setting-item">
      <div>
        <div class="setting-label">语音播放语速</div>
        <div class="setting-desc">默认范围: 0.5x - 2x</div>
      </div>
      <div class="setting-control">
        <input type="range" class="slider" :min="SPEED_RANGE.min" :max="SPEED_RANGE.max" :step="SPEED_RANGE.step" v-model.number="speed">
        <span class="volume-value">{{ speed.toFixed(1) }}x</span>
      </div>
    </div>

    <div class="setting-item">
      <div>
        <div class="setting-label">语音引擎</div>
        <div class="setting-desc">选择朗读译文时使用的语音引擎</div>
      </div>
      <select class="select-input" v-model="ttsEngine">
        <option value="browser">浏览器内置 (speechSynthesis)</option>
        <option value="local">Edge TTS (云服务)</option>
      </select>
    </div>

    <div v-if="ttsEngine === 'local'" class="setting-item">
      <div>
        <div class="setting-label">发声人</div>
        <div class="setting-desc">选择 Edge TTS 使用的语音</div>
      </div>
      <select class="select-input" v-model="ttsVoice">
        <option value="zh-CN-XiaoxiaoNeural">晓晓 (中文女性)</option>
        <option value="zh-CN-YunxiNeural">云希 (中文男性)</option>
        <option value="en-US-AriaNeural">Aria (英文女性)</option>
        <option value="en-US-GuyNeural">Guy (英文男性)</option>
        <option value="ja-JP-NanamiNeural">Nanami (日语女性)</option>
        <option value="ja-JP-KeitaNeural">Keita (日语男性)</option>
        <option value="ko-KR-SunHiNeural">Sun-Hi (韩语女性)</option>
        <option value="ko-KR-InJoonNeural">InJoon (韩语男性)</option>
        <option value="fr-FR-DeniseNeural">Denise (法语女性)</option>
        <option value="de-DE-KatjaNeural">Katja (德语女性)</option>
        <option value="es-ES-ElviraNeural">Elvira (西语女性)</option>
      </select>
    </div>
  </div>
</template>

<script setup>
import { useSettings } from '../../composables/useSettings.js'

const { volume, speed, ttsEngine, ttsVoice } = useSettings()

const VOL_RANGE = { min: 0, max: 100, unit: '%', step: 1 }
const SPEED_RANGE = { min: 0.5, max: 2, unit: 'x', step: 0.1 }
</script>
