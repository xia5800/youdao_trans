<template>
  <div class="translator-page">
    <div class="section-title">文字识别 (OCR) 设置</div>
    <div class="translator-layout">
      <div class="translator-list">
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
      </div>
      <div class="translator-detail" v-if="selectedOcr">
        <div class="detail-desc">{{ selectedOcr.desc }}</div>

        <template v-if="selectedOcr.key === 'paddle_ocr'">
          <div class="detail-fields">
            <div class="detail-no-config">无需额外配置，直接启用即可使用。</div>
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
          <button class="btn-save" @click="saveConfig">保存</button>
          <div class="detail-help">
            <div class="detail-help-title">如何获取：</div>
            <p>确保已部署 Ollama 服务并拉取了视觉模型。可在 Ollama 官网查看支持的模型列表。</p>
          </div>
        </template>

        <template v-else-if="selectedOcr.key === 'baidu_ocr'">
          <div class="detail-fields">
            <div class="detail-field">
              <label>API Key <span class="required">*</span></label>
              <div class="password-field">
                <input :type="fieldVisibility['baidu_ocr-apikey'] ? 'text' : 'password'" class="key-input detail-input" :class="{ 'input-error': baiduOcrKeyError }" placeholder="请输入 API Key" v-model="ocrKeys['baidu_ocr-apikey']">
                <button class="eye-btn" @click="toggleFieldVisibility('baidu_ocr-apikey')" type="button">
                  <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                    <use :href="`/icons.svg#icon-eye-${fieldVisibility['baidu_ocr-apikey'] ? 'off' : 'open'}`"></use>
                  </svg>
                </button>
              </div>
            </div>
            <div class="detail-field">
              <label>Secret Key <span class="required">*</span></label>
              <div class="password-field">
                <input :type="fieldVisibility['baidu_ocr-apisecret'] ? 'text' : 'password'" class="key-input detail-input" :class="{ 'input-error': baiduOcrKeyError }" placeholder="请输入 Secret Key" v-model="ocrKeys['baidu_ocr-apisecret']">
                <button class="eye-btn" @click="toggleFieldVisibility('baidu_ocr-apisecret')" type="button">
                  <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                    <use :href="`/icons.svg#icon-eye-${fieldVisibility['baidu_ocr-apisecret'] ? 'off' : 'open'}`"></use>
                  </svg>
                </button>
              </div>
            </div>
            <div class="error-hint" v-if="baiduOcrKeyError">{{ baiduOcrKeyError }}</div>
          </div>
          <button class="btn-save" @click="saveConfig">保存</button>
          <div class="detail-help">
            <div class="detail-help-title">如何获取：</div>
            <p>前往 <a class="link" @click.prevent="openUrl('https://console.bce.baidu.com')">百度云控制台</a> 创建 OCR 应用，获取 API Key 和 Secret Key。</p>
          </div>
        </template>

        <template v-else-if="selectedOcr.key === 'xunfei'">
          <div class="detail-fields">
            <div class="detail-field">
              <label>App Id <span class="required">*</span></label>
              <div class="password-field">
                <input :type="fieldVisibility['xunfei-appid'] ? 'text' : 'password'" class="key-input detail-input" :class="{ 'input-error': xunfeiOcrKeyError }" placeholder="请输入 AppId" v-model="ocrKeys['xunfei-appid']">
                <button class="eye-btn" @click="toggleFieldVisibility('xunfei-appid')" type="button">
                  <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                    <use :href="`/icons.svg#icon-eye-${fieldVisibility['xunfei-appid'] ? 'off' : 'open'}`"></use>
                  </svg>
                </button>
              </div>
            </div>
            <div class="detail-field">
              <label>API Key <span class="required">*</span></label>
              <div class="password-field">
                <input :type="fieldVisibility['xunfei-apikey'] ? 'text' : 'password'" class="key-input detail-input" :class="{ 'input-error': xunfeiOcrKeyError }" placeholder="请输入 API Key" v-model="ocrKeys['xunfei-apikey']">
                <button class="eye-btn" @click="toggleFieldVisibility('xunfei-apikey')" type="button">
                  <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                    <use :href="`/icons.svg#icon-eye-${fieldVisibility['xunfei-apikey'] ? 'off' : 'open'}`"></use>
                  </svg>
                </button>
              </div>
            </div>
            <div class="detail-field">
              <label>Secret Key <span class="required">*</span></label>
              <div class="password-field">
                <input :type="fieldVisibility['xunfei-apisecret'] ? 'text' : 'password'" class="key-input detail-input" :class="{ 'input-error': xunfeiOcrKeyError }" placeholder="请输入 Secret Key" v-model="ocrKeys['xunfei-apisecret']">
                <button class="eye-btn" @click="toggleFieldVisibility('xunfei-apisecret')" type="button">
                  <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                    <use :href="`/icons.svg#icon-eye-${fieldVisibility['xunfei-apisecret'] ? 'off' : 'open'}`"></use>
                  </svg>
                </button>
              </div>
            </div>
            <div class="error-hint" v-if="xunfeiOcrKeyError">{{ xunfeiOcrKeyError }}</div>
          </div>
          <button class="btn-save" @click="saveConfig">保存</button>
          <div class="detail-help">
            <div class="detail-help-title">如何获取：</div>
            <p>前往 <a class="link" @click.prevent="openUrl('https://www.xfyun.cn')">讯飞开放平台</a> 创建 OCR 应用，获取 App Id、API Key 和 Secret Key。</p>
          </div>
        </template>

        <template v-else-if="selectedOcr.key === 'tencent'">
          <div class="detail-fields">
            <div class="detail-field">
              <label>Secret Id <span class="required">*</span></label>
              <div class="password-field">
                <input :type="fieldVisibility['tencent-secretid'] ? 'text' : 'password'" class="key-input detail-input" :class="{ 'input-error': tencentOcrKeyError }" placeholder="请输入 SecretId" v-model="ocrKeys['tencent-secretid']">
                <button class="eye-btn" @click="toggleFieldVisibility('tencent-secretid')" type="button">
                  <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                    <use :href="`/icons.svg#icon-eye-${fieldVisibility['tencent-secretid'] ? 'off' : 'open'}`"></use>
                  </svg>
                </button>
              </div>
            </div>
            <div class="detail-field">
              <label>Secret Key <span class="required">*</span></label>
              <div class="password-field">
                <input :type="fieldVisibility['tencent-secretkey'] ? 'text' : 'password'" class="key-input detail-input" :class="{ 'input-error': tencentOcrKeyError }" placeholder="请输入 SecretKey" v-model="ocrKeys['tencent-secretkey']">
                <button class="eye-btn" @click="toggleFieldVisibility('tencent-secretkey')" type="button">
                  <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                    <use :href="`/icons.svg#icon-eye-${fieldVisibility['tencent-secretkey'] ? 'off' : 'open'}`"></use>
                  </svg>
                </button>
              </div>
            </div>
            <div class="error-hint" v-if="tencentOcrKeyError">{{ tencentOcrKeyError }}</div>
          </div>
          <button class="btn-save" @click="saveConfig">保存</button>
          <div class="detail-help">
            <div class="detail-help-title">如何获取：</div>
            <p>前往 <a class="link" @click.prevent="openUrl('https://console.cloud.tencent.com')">腾讯云控制台</a> 的访问管理创建 SecretId 和 SecretKey。</p>
          </div>
        </template>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, reactive, computed, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useSettings } from '../../composables/useSettings.js'
import { useUtils } from '../../composables/useUtils.js'

const { settings, activeOcr, ocrKeys, configPath } = useSettings()
const { showToastOnce, openUrl } = useUtils()

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

const fieldVisibility = reactive({})

function toggleFieldVisibility(id) {
  fieldVisibility[id] = !fieldVisibility[id]
}

function clearOcrVisibility(key) {
  if (key === 'xunfei') {
    fieldVisibility['xunfei-appid'] = false
    fieldVisibility['xunfei-apikey'] = false
    fieldVisibility['xunfei-apisecret'] = false
  } else if (key === 'baidu_ocr') {
    fieldVisibility['baidu_ocr-apikey'] = false
    fieldVisibility['baidu_ocr-apisecret'] = false
  } else if (key === 'tencent') {
    fieldVisibility['tencent-secretid'] = false
    fieldVisibility['tencent-secretkey'] = false
  } else {
    fieldVisibility[key + '-appid'] = false
    fieldVisibility[key + '-appkey'] = false
  }
}

function toggleOcr(key) {
  selectedKey.value = key
  if (activeOcr.value && activeOcr.value !== key) {
    clearOcrVisibility(activeOcr.value)
  }
  activeOcr.value = activeOcr.value === key ? null : key
}

async function saveConfig() {
  const filled = v => v && v.trim()
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
    const path = configPath.value || null
    await invoke('save_config', { json: JSON.stringify(settings), path })
    showToastOnce('已保存')
  } catch (e) {
    showToast(`保存失败: ${e}`)
  }
}
</script>

<style scoped>
.translator-page {
  height: 100%;
  display: flex;
  flex-direction: column;
}

.translator-layout {
  display: flex;
  gap: 0;
  border: 1px solid var(--border-strong);
  border-radius: 8px;
  flex: 1;
  min-height: 0;
}

.translator-list {
  width: 200px;
  flex-shrink: 0;
  border-right: 1px solid var(--border-strong);
  background: var(--bg-sidebar);
  border-radius: 8px 0 0 8px;
  padding: 8px 0;
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

.translator-detail {
  flex: 1;
  padding: 20px 24px;
  background: var(--bg-card);
  border-radius: 0 8px 8px 0;
  overflow-y: auto;
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

.btn-save {
  margin-top: 20px;
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
</style>
