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
      <div class="switch" :class="{ active: autoStart }" @click="autoStart = !autoStart">
        <div class="switch-knob"></div>
      </div>
    </div>

    <div class="setting-item">
      <div>
        <div class="setting-label">划词默认前后延迟</div>
        <div class="setting-desc">选中文字后的延迟时间(ms)</div>
      </div>
      <input type="number" class="number-input" v-model="delayTime" step="50">
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
      <div class="switch" :class="{ active: autoTranslate }" @click="autoTranslate = !autoTranslate">
        <div class="switch-knob"></div>
      </div>
    </div>

    <div class="setting-item">
      <div>
        <div class="setting-label">自动翻译延迟</div>
        <div class="setting-desc">输入停止后的延迟时间(ms)</div>
      </div>
      <input type="number" class="number-input" v-model="autoTranslateDelay" step="50" :disabled="!autoTranslate">
    </div>

    <div class="setting-item">
      <div>
        <div class="setting-label">截图十字线</div>
        <div class="setting-desc">截图时显示十字准星辅助线</div>
      </div>
      <div class="switch" :class="{ active: settings.showScreenshotCrosshair }" @click="settings.showScreenshotCrosshair = !settings.showScreenshotCrosshair">
        <div class="switch-knob"></div>
      </div>
    </div>

    <div class="setting-item">
      <div>
        <div class="setting-label">配置路径</div>
        <div class="setting-desc">应用程序配置存储位置，修改后自动保存到新路径</div>
      </div>
      <div class="path-group">
        <div class="path-display"><span>{{ configPath || defaultConfigDirPlaceholder }}</span></div>
        <button class="btn-select" @click="selectPath">选择</button>
      </div>
    </div>

    <div class="setting-item">
      <div>
        <div class="setting-label">翻译记录数据路径</div>
        <div class="setting-desc">翻译历史记录数据库存储位置</div>
      </div>
      <div class="path-group">
        <div class="path-display"><span>{{ dbPath || defaultDbDirPlaceholder }}</span></div>
        <button class="btn-select" @click="selectDbPath">选择</button>
      </div>
    </div>

    <div class="subsection-title">关闭行为</div>
    <div class="setting-item">
      <div>
        <div class="setting-label">关闭主窗口时</div>
        <div class="setting-desc">点击关闭按钮后的默认操作</div>
      </div>
      <select class="select-input" v-model="settings.closeBehavior">
        <option value="ask">每次询问</option>
        <option value="tray">最小化到系统托盘</option>
        <option value="exit">退出程序</option>
      </select>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, watch, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'
import { useTheme } from '../../composables/useTheme.js'
import { useSettings } from '../../composables/useSettings.js'
import { useHotkey } from '../../composables/useHotkey.js'

const { themeMode } = useTheme()
const { hotkeys } = useHotkey()
const translateCombo = computed(() => hotkeys.value.find(h => h.id === 'translate')?.combo || 'Ctrl+Enter')

const {
  settings,
  configPath,
  dbPath,
  autoStart,
  delayTime,
  storeRecords,
  replaceNewlines,
  autoTranslate,
  autoTranslateDelay,
} = useSettings()

watch(themeMode, (val) => { settings.theme = val }, { immediate: true })

const defaultConfigDirPlaceholder = ref('')
const defaultDbDirPlaceholder = ref('')

onMounted(async () => {
  try {
    defaultConfigDirPlaceholder.value = await invoke('default_config_dir')
    defaultDbDirPlaceholder.value = await invoke('default_db_dir')
  } catch (e) {
    console.warn('failed to fetch default paths:', e)
  }
})

async function selectPath() {
  const dir = await open({ directory: true, multiple: false, title: '选择配置存储目录' })
  if (!dir) return
  const oldPath = configPath.value || null
  const snap = JSON.parse(JSON.stringify(settings))
  snap.configPath = dir
  await invoke('save_config', { json: JSON.stringify(snap), path: oldPath })
  configPath.value = dir
}

async function selectDbPath() {
  const dir = await open({ directory: true, multiple: false, title: '选择翻译记录存储目录' })
  if (!dir) return
  dbPath.value = dir
}
</script>
