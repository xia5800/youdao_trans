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
  </div>
</template>

<script setup>
import { useSettings } from '../../composables/useSettings.js'

const { volume, speed } = useSettings()

const VOL_RANGE = { min: 0, max: 100, unit: '%', step: 1 }
const SPEED_RANGE = { min: 0.5, max: 2, unit: 'x', step: 0.1 }
</script>
