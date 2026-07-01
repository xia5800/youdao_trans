<template>
  <div>
    <div class="section-title">偏好设置</div>

    <div class="setting-item">
      <div>
        <div class="setting-label">主题</div>
        <div class="setting-desc">选择界面外观风格</div>
      </div>
      <div class="radio-group">
        <label class="radio-label"><input type="radio" name="theme" value="system" v-model="themeMode"> 跟随系统</label>
        <label class="radio-label"><input type="radio" name="theme" value="light" v-model="themeMode"> 明亮</label>
        <label class="radio-label"><input type="radio" name="theme" value="dark" v-model="themeMode"> 暗黑</label>
      </div>
    </div>

    <div class="setting-item">
      <div>
        <div class="setting-label">自动启动</div>
        <div class="setting-desc">是否开机自动启动</div>
      </div>
      <SwitchToggle v-model="autoStart" />
    </div>

    <div class="setting-item">
      <div>
        <div class="setting-label">划词默认前后延迟</div>
        <div class="setting-desc">选中文字后的延迟时间(ms)</div>
      </div>
      <NumberInput v-model="delayTime" step="50" />
    </div>

    <div class="setting-item">
      <div>
        <div class="setting-label">翻译记录</div>
        <div class="setting-desc">是否存储翻译记录</div>
      </div>
      <label class="checkbox-label">
        <input type="checkbox" v-model="storeRecords"> 存储翻译记录
      </label>
    </div>

    <div class="setting-item">
      <div>
        <div class="setting-label">文本处理</div>
        <div class="setting-desc">将翻译内容的【换行符】替换为【空格】</div>
      </div>
      <label class="checkbox-label">
        <input type="checkbox" v-model="replaceNewlines"> 启用替换
      </label>
    </div>

    <div class="subsection-title">功能设置</div>
    <div class="setting-item">
      <div>
        <div class="setting-label">自动翻译</div>
        <div class="setting-desc">开启后自动翻译输入内容，关闭后需按{{ translateCombo }}翻译</div>
      </div>
      <SwitchToggle v-model="autoTranslate" />
    </div>

    <div class="setting-item">
      <div>
        <div class="setting-label">自动翻译延迟</div>
        <div class="setting-desc">输入停止后的延迟时间(ms)</div>
      </div>
      <NumberInput v-model="autoTranslateDelay" step="50" :disabled="!autoTranslate" />
    </div>

    <div class="setting-item">
      <div>
        <div class="setting-label">截图十字线</div>
        <div class="setting-desc">截图时显示十字准星辅助线</div>
      </div>
      <SwitchToggle v-model="settings.showScreenshotCrosshair" />
    </div>

    <div class="subsection-title">关闭行为</div>
    <div class="setting-item">
      <div>
        <div class="setting-label">关闭主窗口时</div>
        <div class="setting-desc">点击关闭按钮后的默认操作</div>
      </div>
      <SelectInput
        :options="closeOptions"
        v-model="settings.closeBehavior"
      />
    </div>
  </div>
</template>

<script setup>
import { computed, watch } from 'vue'
import NumberInput from '../../components/NumberInput.vue'
import SelectInput from '../../components/SelectInput.vue'
import SwitchToggle from '../../components/SwitchToggle.vue'
import { useTheme } from '../../composables/useTheme.js'
import { useSettings } from '../../composables/useSettings.js'
import { useHotkey } from '../../composables/useHotkey.js'

const { themeMode } = useTheme()
const { hotkeys } = useHotkey()
const translateCombo = computed(() => hotkeys.value.find(h => h.id === 'translate')?.combo || 'Ctrl+Enter')

const {
  settings,
  autoStart,
  delayTime,
  storeRecords,
  replaceNewlines,
  autoTranslate,
  autoTranslateDelay,
} = useSettings()

const closeOptions = [
  { value: 'ask', label: '每次询问' },
  { value: 'tray', label: '最小化到系统托盘' },
  { value: 'exit', label: '退出程序' },
]

watch(themeMode, (val) => { settings.theme = val }, { immediate: true })
</script>

