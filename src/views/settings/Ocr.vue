<template>
  <div class="translator-setting">
    <div class="section-title">文字识别 (OCR) 设置</div>
    <ServiceConfigLayout>
    <template #list>
        <div
          v-for="o in ocrList"
          :key="o.key"
          class="translator-row"
          :class="{ selected: selectedKey === o.key }"
          @click="selectedKey = o.key"
        >
          <span class="translator-row-name">{{ o.name }}</span>
          <div class="switch" :class="{ active: activeOcr === o.key }" @click.stop="toggleOcr(o.key)">
            <div class="switch-knob"></div>
          </div>
        </div>
    </template>
    <template #detail>
      <template v-if="selectedOcr">
        <div class="detail-desc">{{ selectedOcr.desc }}</div>

        <template v-if="selectedOcr.key === 'paddle_ocr'">
          <div v-if="!allModelsExist">
            <div class="detail-fields">
              <div class="detail-field">
                <label>HuggingFace 镜像</label>
                <div class="toggle-row">
                  <div class="switch" :class="{ active: useHuggingFaceMirror }" @click="useHuggingFaceMirror = !useHuggingFaceMirror">
                    <div class="switch-knob"></div>
                  </div>
                  <span class="toggle-label">使用镜像站点（hf-mirror.com）下载模型</span>
                </div>
              </div>
              <div class="detail-field">
                <label>GitHub 加速</label>
                <div class="toggle-row">
                  <div class="switch" :class="{ active: useGitHubMirror }" @click="useGitHubMirror = !useGitHubMirror">
                    <div class="switch-knob"></div>
                  </div>
                  <span class="toggle-label">使用 CDN 加速（jsDelivr）下载字典文件</span>
                </div>
              </div>
            </div>

            <div v-if="modelStatus" class="model-status-section">
              <div class="detail-desc" style="margin-top:16px;margin-bottom:12px;">模型文件状态</div>
              <div v-for="(exists, name) in modelStatus" :key="name" class="model-file-row">
                <span class="model-file-name">{{ name }}</span>
                <span v-if="exists" class="model-status-ok">✓ 已存在</span>
                <span v-else class="model-status-missing">✗ 未找到</span>
              </div>

              <template v-if="downloading">
                <div class="download-section">
                  <div class="download-title">正在下载模型文件...</div>
                  <div v-for="dl in downloadList" :key="dl.fileName" class="download-file-item">
                    <div class="download-file-header">
                      <span class="download-file-name">{{ dl.fileName }}</span>
                      <span v-if="dl.status === 'downloading' && dl.total > 0" class="download-percent">{{ Math.round(dl.downloaded / dl.total * 100) }}%</span>
                      <span v-else-if="dl.status === 'downloading'" class="download-percent">下载中...</span>
                      <span v-else-if="dl.status === 'completed'" class="download-status-completed">已完成</span>
                      <span v-else-if="dl.status === 'error'" class="download-status-error">失败</span>
                    </div>
                    <div class="progress-bar-bg">
                      <div class="progress-bar-fill" :class="{ indeterminate: dl.total === 0 }" :style="dl.total > 0 ? { width: Math.round(dl.downloaded / dl.total * 100) + '%' } : {}"></div>
                    </div>
                    <div v-if="dl.status === 'error'" class="download-retry-row">
                      <button class="btn-retry" @click="retryDownload(dl.fileName)">重新下载</button>
                      <span class="download-error-msg">{{ dl.errorMsg }}</span>
                    </div>
                  </div>
                </div>
              </template>

              <template v-else>
                <button class="btn-download" @click="startDownload">下载缺失模型</button>
                <div v-if="downloadError" class="download-error-msg" style="margin-top:8px;">{{ downloadError }}</div>
              </template>
            </div>
          </div>

          <div class="detail-no-config" v-else>
            <div>模型文件已就绪，可直接启用使用。</div>
            <div v-if="modelsDir" class="models-dir-path">当前模型位置：{{ modelsDir }}</div>
          </div>
        </template>

        <template v-else-if="selectedOcr.key === 'ollama_ocr'">
          <div class="detail-fields">
            <div class="detail-field">
              <label>接口地址 <span class="required">*</span></label>
              <input type="text" class="key-input detail-input" :class="{ 'input-error': ollamaOcrKeyError }" placeholder="例如: http://localhost:11434" v-model="ocrKeys['ollama_ocr_base_url']">
            </div>
            <div class="detail-field">
              <label>模型名 <span class="required">*</span></label>
              <input type="text" class="key-input detail-input" :class="{ 'input-error': ollamaOcrKeyError }" placeholder="例如: maternion/Qianfan-OCR" v-model="ocrKeys['ollama_ocr_model']">
            </div>
            <div class="error-hint" v-if="ollamaOcrKeyError">{{ ollamaOcrKeyError }}</div>
          </div>
          <div class="btn-save-wrap">
            <button class="btn-save" @click="saveConfig">保存</button>
          </div>
          <div class="detail-help">
            <div class="detail-help-title">如何获取：</div>
            <p>确保已部署 Ollama 服务并拉取了视觉模型。可在 Ollama 官网查看支持的模型列表。</p>
          </div>
        </template>

        <template v-else-if="selectedOcr.key === 'baidu_ocr'">
          <div class="detail-fields">
            <div class="detail-field">
              <label>API Key <span class="required">*</span></label>
              <PasswordField v-model="ocrKeys['baidu_ocr-apikey']" placeholder="请输入 API Key" />
            </div>
            <div class="detail-field">
              <label>Secret Key <span class="required">*</span></label>
              <PasswordField v-model="ocrKeys['baidu_ocr-apisecret']" placeholder="请输入 Secret Key" />
            </div>
            <div class="error-hint" v-if="baiduOcrKeyError">{{ baiduOcrKeyError }}</div>
          </div>
          <div class="btn-save-wrap">
            <button class="btn-save" @click="saveConfig">保存</button>
          </div>
          <div class="detail-help">
            <div class="detail-help-title">如何获取：</div>
            <p>前往 <a class="link" @click.prevent="openUrl('https://console.bce.baidu.com')">百度云控制台</a> 创建 OCR 应用，获取 API Key 和 Secret Key。</p>
          </div>
        </template>

        <template v-else-if="selectedOcr.key === 'xunfei'">
          <div class="detail-fields">
            <div class="detail-field">
              <label>App Id <span class="required">*</span></label>
              <PasswordField v-model="ocrKeys['xunfei-appid']" placeholder="请输入 AppId" />
            </div>
            <div class="detail-field">
              <label>API Key <span class="required">*</span></label>
              <PasswordField v-model="ocrKeys['xunfei-apikey']" placeholder="请输入 API Key" />
            </div>
            <div class="detail-field">
              <label>Secret Key <span class="required">*</span></label>
              <PasswordField v-model="ocrKeys['xunfei-apisecret']" placeholder="请输入 Secret Key" />
            </div>
            <div class="error-hint" v-if="xunfeiOcrKeyError">{{ xunfeiOcrKeyError }}</div>
          </div>
          <div class="btn-save-wrap">
            <button class="btn-save" @click="saveConfig">保存</button>
          </div>
          <div class="detail-help">
            <div class="detail-help-title">如何获取：</div>
            <p>前往 <a class="link" @click.prevent="openUrl('https://www.xfyun.cn')">讯飞开放平台</a> 创建 OCR 应用，获取 App Id、API Key 和 Secret Key。</p>
          </div>
        </template>

        <template v-else-if="selectedOcr.key === 'tencent'">
          <div class="detail-fields">
            <div class="detail-field">
              <label>Secret Id <span class="required">*</span></label>
              <PasswordField v-model="ocrKeys['tencent-secretid']" placeholder="请输入 SecretId" />
            </div>
            <div class="detail-field">
              <label>Secret Key <span class="required">*</span></label>
              <PasswordField v-model="ocrKeys['tencent-secretkey']" placeholder="请输入 SecretKey" />
            </div>
            <div class="error-hint" v-if="tencentOcrKeyError">{{ tencentOcrKeyError }}</div>
          </div>
          <div class="btn-save-wrap">
            <button class="btn-save" @click="saveConfig">保存</button>
          </div>
          <div class="detail-help">
            <div class="detail-help-title">如何获取：</div>
            <p>前往 <a class="link" @click.prevent="openUrl('https://console.cloud.tencent.com')">腾讯云控制台</a> 的访问管理创建 SecretId 和 SecretKey。</p>
          </div>
        </template>
      </template>
    </template>
  </ServiceConfigLayout>
</div>
</template>

<script setup>
import { ref, computed, watch, onMounted } from 'vue'
import PasswordField from '../../components/PasswordField.vue'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { useSettings } from '../../composables/useSettings.js'
import { filled, useUtils } from '../../composables/useUtils.js'
import ServiceConfigLayout from '../../components/ServiceConfigLayout.vue'

const { settings, activeOcr, ocrKeys } = useSettings()
const { showToastOnce, showToast, openUrl } = useUtils()

const useHuggingFaceMirror = ref(localStorage.getItem('useHuggingFaceMirror') === 'true')
const useGitHubMirror = ref(localStorage.getItem('useGitHubMirror') === 'true')
watch(useHuggingFaceMirror, v => localStorage.setItem('useHuggingFaceMirror', v))
watch(useGitHubMirror, v => localStorage.setItem('useGitHubMirror', v))

const ocrList = [
  { key: 'paddle_ocr', name: 'PaddleOCR', desc: '本地 PaddleOCR 模型识别，无需 API Key，激活即用' },
  { key: 'ollama_ocr', name: 'Ollama OCR', desc: '本地视觉模型 OCR，需部署 Ollama 和视觉模型' },
  { key: 'baidu_ocr', name: '百度云OCR', desc: '通用文字识别(标准版)' },
  { key: 'xunfei', name: '讯飞OCR', desc: '通用文档识别(OCR大模型)' },
  { key: 'tencent', name: '腾讯云OCR', desc: '通用文字识别Agent(RecognizeAgent)' }
]

const selectedKey = ref(activeOcr.value || ocrList[0].key)
const selectedOcr = computed(() => ocrList.find(o => o.key === selectedKey.value))

const baiduOcrKeyError = ref('')
const xunfeiOcrKeyError = ref('')
const tencentOcrKeyError = ref('')
const ollamaOcrKeyError = ref('')

watch([() => ocrKeys.value['baidu_ocr-apikey'], () => ocrKeys.value['baidu_ocr-apisecret'], activeOcr], ([apikey, apisecret, active]) => {
  const filled = k => k && k.trim()
  if (active === 'baidu_ocr' && (!filled(apikey) || !filled(apisecret))) {
    baiduOcrKeyError.value = 'API Key 和 Secret Key 必须同时填写'
  } else {
    baiduOcrKeyError.value = ''
  }
})

watch([() => ocrKeys.value['xunfei-appid'], () => ocrKeys.value['xunfei-apikey'], () => ocrKeys.value['xunfei-apisecret'], activeOcr], ([appid, apikey, apisecret, active]) => {
  const filled = k => k && k.trim()
  if (active === 'xunfei' && (!filled(appid) || !filled(apikey) || !filled(apisecret))) {
    xunfeiOcrKeyError.value = 'AppId、API Key 和 Secret Key 必须同时填写'
  } else {
    xunfeiOcrKeyError.value = ''
  }
})

watch([() => ocrKeys.value['tencent-secretid'], () => ocrKeys.value['tencent-secretkey'], activeOcr], ([secretid, secretkey, active]) => {
  const filled = k => k && k.trim()
  if (active === 'tencent' && (!filled(secretid) || !filled(secretkey))) {
    tencentOcrKeyError.value = 'SecretId 和 SecretKey 必须同时填写'
  } else {
    tencentOcrKeyError.value = ''
  }
})

watch([() => ocrKeys.value['ollama_ocr_base_url'], () => ocrKeys.value['ollama_ocr_model'], activeOcr], ([url, model, active]) => {
  const filled = k => k && k.trim()
  if (active === 'ollama_ocr' && (!filled(url) || !filled(model))) {
    ollamaOcrKeyError.value = '接口地址和模型名必须填写'
  } else {
    ollamaOcrKeyError.value = ''
  }
})

function toggleOcr(key) {
  selectedKey.value = key
  activeOcr.value = activeOcr.value === key ? null : key
}

// Model download state
const modelStatus = ref(null)
const allModelsExist = ref(true)
const modelsDir = ref('')
const downloading = ref(false)
const downloadError = ref('')
const downloadList = ref([])

async function refreshModelStatus() {
  try {
    modelStatus.value = await invoke('check_ocr_models_state')
    allModelsExist.value = Object.values(modelStatus.value).every(v => v)
    modelsDir.value = await invoke('ocr_models_data_dir')
  } catch (e) {
    console.warn('refreshModelStatus failed:', e)
  }
}

let unlistenList = []

async function setupDownloadListeners() {
  const unlisten1 = await listen('download-start', (event) => {
    const f = event.payload
    const existing = downloadList.value.find(d => d.fileName === f.fileName)
    if (existing) {
      existing.total = f.total
      existing.downloaded = 0
      existing.status = 'downloading'
      existing.errorMsg = ''
    } else {
      downloadList.value.push({
        fileName: f.fileName,
        total: f.total,
        downloaded: 0,
        status: 'downloading',
        errorMsg: '',
      })
    }
  })

  const unlisten2 = await listen('download-progress', (event) => {
    const f = event.payload
    const item = downloadList.value.find(d => d.fileName === f.fileName)
    if (item) {
      item.total = f.total
      item.downloaded = f.downloaded
    }
  })

  const unlisten3 = await listen('download-complete', (event) => {
    const f = event.payload
    const item = downloadList.value.find(d => d.fileName === f.fileName)
    if (item) {
      item.status = 'completed'
      item.downloaded = item.total
    }
    refreshModelStatus()
  })

  const unlisten4 = await listen('download-error', (event) => {
    const f = event.payload
    const item = downloadList.value.find(d => d.fileName === f.fileName)
    if (item) {
      item.status = 'error'
      item.errorMsg = f.status || '下载失败'
    }
    refreshModelStatus()
  })

  unlistenList = [unlisten1, unlisten2, unlisten3, unlisten4]
}

async function startDownload() {
  downloading.value = true
  downloadError.value = ''
  downloadList.value = []

  try {
    await invoke('download_ocr_models', { useMirror: useHuggingFaceMirror.value, useGithubMirror: useGitHubMirror.value })
    await refreshModelStatus()
    showToastOnce('模型下载完成')
  } catch (e) {
    downloadError.value = typeof e === 'string' ? e : '下载失败'
    await refreshModelStatus()
    showToastOnce('部分模型下载失败，可点击重试')
  } finally {
    downloading.value = false
  }
}

async function retryDownload(fileName) {
  const item = downloadList.value.find(d => d.fileName === fileName)
  if (item) {
    item.status = 'downloading'
    item.downloaded = 0
    item.errorMsg = ''
  }

  try {
    await invoke('retry_download_ocr_file', { fileName, useMirror: useHuggingFaceMirror.value, useGithubMirror: useGitHubMirror.value })
    await refreshModelStatus()
  } catch (e) {
    const item2 = downloadList.value.find(d => d.fileName === fileName)
    if (item2) {
      item2.status = 'error'
      item2.errorMsg = typeof e === 'string' ? e : '重试失败'
    }
  }
}

watch(selectedKey, async (newKey) => {
  if (newKey === 'paddle_ocr') {
    await refreshModelStatus()
  }
})

onMounted(async () => {
  await refreshModelStatus()
  await setupDownloadListeners()
})

async function saveConfig() {
  const key = selectedKey.value

  if (key === 'baidu_ocr') {
    const k = ocrKeys.value['baidu_ocr-apikey']
    const s = ocrKeys.value['baidu_ocr-apisecret']
    if (!filled(k) || !filled(s)) {
      showToastOnce('API Key 和 Secret Key 必须同时填写')
      return
    }
  } else if (key === 'xunfei') {
    const a = ocrKeys.value['xunfei-appid']
    const k = ocrKeys.value['xunfei-apikey']
    const s = ocrKeys.value['xunfei-apisecret']
    if (!filled(a) || !filled(k) || !filled(s)) {
      showToastOnce('AppId、API Key 和 Secret Key 必须同时填写')
      return
    }
  } else if (key === 'tencent') {
    const s = ocrKeys.value['tencent-secretid']
    const k = ocrKeys.value['tencent-secretkey']
    if (!filled(s) || !filled(k)) {
      showToastOnce('SecretId 和 SecretKey 必须同时填写')
      return
    }
  } else if (key === 'ollama_ocr') {
    const u = ocrKeys.value['ollama_ocr_base_url']
    const m = ocrKeys.value['ollama_ocr_model']
    if (!filled(u) || !filled(m)) {
      showToastOnce('接口地址和模型名必须填写')
      return
    }
  }

  try {
    await invoke('save_config', { json: JSON.stringify(settings) })
    showToastOnce('已保存')
  } catch (e) {
    showToast(`保存失败: ${e}`)
  }
}
</script>

<style scoped>
.translator-setting {
  height: 100%;
  display: flex;
  flex-direction: column;
}

.translator-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 10px 12px;
  cursor: default;
  transition: background 0.15s;
  gap: 8px;
}

.translator-row:hover {
  background: var(--bg-hover);
}

.translator-row.selected {
  background: var(--bg-active);
}

.translator-row-name {
  font-size: 13px;
  color: var(--text-primary);
  font-weight: 500;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.detail-desc {
  font-size: 13px;
  color: var(--text-tertiary);
  margin-bottom: 20px;
  line-height: 1.5;
}

.detail-fields {
  display: flex;
  flex-direction: column;
  gap: 14px;
}

.detail-field {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.detail-field label {
  font-size: 13px;
  font-weight: 500;
  color: var(--text-primary);
}

.required {
  color: #e74c3c;
}

.detail-input {
  width: 100%;
  max-width: 360px;
  box-sizing: border-box;
}

.translator-detail .password-field {
  display: flex;
  width: 100%;
  max-width: 360px;
}

.translator-detail .password-field .key-input {
  flex: 1;
  width: auto;
}

.detail-no-config {
  font-size: 12px;
  color: var(--text-tertiary);
  padding: 20px 0;
}

.btn-save-wrap {
  width: 100%;
  max-width: 360px;
  margin-top: 20px;
  display: flex;
  justify-content: center;
}

.btn-save {
  padding: 8px 28px;
  background: var(--accent);
  color: var(--text-inverse);
  border: none;
  border-radius: 6px;
  font-size: 13px;
  cursor: default;
  transition: opacity 0.2s;
}

.btn-save:hover {
  opacity: 0.85;
}

.detail-help {
  margin-top: 24px;
  padding: 14px 16px;
  background: var(--bg-sidebar);
  border: 1px solid var(--border);
  border-radius: 8px;
  font-size: 12px;
  color: var(--text-tertiary);
  line-height: 1.7;
}

.detail-help-title {
  font-weight: 600;
  color: var(--text-secondary);
  margin-bottom: 4px;
}

.detail-help .link {
  color: var(--accent);
  cursor: pointer;
  text-decoration: none;
}

.detail-help .link:hover {
  text-decoration: underline;
}

.detail-help p {
  margin: 0;
}

.toggle-row {
  display: flex;
  align-items: center;
  gap: 8px;
}

.toggle-label {
  font-size: 12px;
  color: var(--text-tertiary);
}

.model-status-section {
  margin-top: 8px;
}

.model-file-row {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 6px 0;
  font-size: 13px;
}

.model-file-name {
  font-family: monospace;
  font-size: 12px;
  color: var(--text-secondary);
}

.model-status-ok {
  color: #27ae60;
  font-size: 12px;
}

.model-status-missing {
  color: #e74c3c;
  font-size: 12px;
}

.btn-download {
  margin-top: 16px;
  padding: 8px 28px;
  background: var(--accent);
  color: var(--text-inverse);
  border: none;
  border-radius: 6px;
  font-size: 13px;
  cursor: default;
  transition: opacity 0.2s;
}

.btn-download:hover {
  opacity: 0.85;
}

.download-section {
  margin-top: 12px;
  padding: 12px 14px;
  background: var(--bg-sidebar);
  border: 1px solid var(--border);
  border-radius: 8px;
}

.download-title {
  font-size: 13px;
  font-weight: 600;
  color: var(--text-primary);
  margin-bottom: 12px;
}

.download-file-item {
  margin-bottom: 12px;
}

.download-file-item:last-child {
  margin-bottom: 0;
}

.download-file-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 4px;
}

.download-file-name {
  font-family: monospace;
  font-size: 11px;
  color: var(--text-secondary);
}

.download-percent {
  font-size: 11px;
  color: var(--accent);
  font-weight: 600;
}

.download-status-completed {
  font-size: 11px;
  color: #27ae60;
  font-weight: 600;
}

.download-status-error {
  font-size: 11px;
  color: #e74c3c;
  font-weight: 600;
}

.progress-bar-bg {
  height: 6px;
  background: var(--border-strong);
  border-radius: 3px;
  overflow: hidden;
}

.progress-bar-fill {
  height: 100%;
  background: var(--accent);
  border-radius: 3px;
  transition: width 0.2s;
}

.progress-bar-fill.indeterminate {
  width: 30% !important;
  animation: indeterminate-bar 1.5s ease-in-out infinite;
}

@keyframes indeterminate-bar {
  0%   { transform: translateX(-100%); }
  100% { transform: translateX(400%); }
}

.download-retry-row {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-top: 4px;
}

.btn-retry {
  padding: 2px 10px;
  background: #e74c3c;
  color: #fff;
  border: none;
  border-radius: 4px;
  font-size: 11px;
  cursor: default;
  transition: opacity 0.2s;
}

.btn-retry:hover {
  opacity: 0.8;
}

.download-error-msg {
  font-size: 11px;
  color: #e74c3c;
}
</style>
