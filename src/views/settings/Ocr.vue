<template>
  <div>
    <div class="section-title">文字识别 (OCR) 设置</div>
    <div class="tab-hint">以下OCR源互斥，只能开启一个</div>
    <div class="translator-item" v-for="o in ocrList" :key="o.key">
      <div class="translator-info">
        <div class="translator-name">{{ o.name }}</div>
        <div class="translator-desc">{{ o.desc }}</div>
      </div>
      <div class="setting-control">
        <div style="display: flex; gap: 6px;" :style="{ display: activeOcr === o.key ? 'flex' : 'none' }">
          <template v-if="o.key === 'xunfei'">
            <div class="microsoft-wrap" style="display:block;">
              <div style="display: flex; gap: 6px; align-items: center;">
                <div class="password-field">
                  <input :type="fieldVisibility['xunfei-appid'] ? 'text' : 'password'" class="key-input" :class="{ 'input-error': xunfeiOcrKeyError }" placeholder="AppId" style="width: 100px;" v-model="ocrKeys['xunfei-appid']">
                  <button class="eye-btn" @click="toggleFieldVisibility('xunfei-appid')" type="button">
                    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                      <use :href="`/icons.svg#icon-eye-${fieldVisibility['xunfei-appid'] ? 'off' : 'open'}`"></use>
                    </svg>
                  </button>
                </div>
                <div class="password-field">
                  <input :type="fieldVisibility['xunfei-apikey'] ? 'text' : 'password'" class="key-input" :class="{ 'input-error': xunfeiOcrKeyError }" placeholder="API Key" style="width: 120px;" v-model="ocrKeys['xunfei-apikey']">
                  <button class="eye-btn" @click="toggleFieldVisibility('xunfei-apikey')" type="button">
                    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                      <use :href="`/icons.svg#icon-eye-${fieldVisibility['xunfei-apikey'] ? 'off' : 'open'}`"></use>
                    </svg>
                  </button>
                </div>
                <div class="password-field">
                  <input :type="fieldVisibility['xunfei-apisecret'] ? 'text' : 'password'" class="key-input" :class="{ 'input-error': xunfeiOcrKeyError }" placeholder="Secret Key" style="width: 120px;" v-model="ocrKeys['xunfei-apisecret']">
                  <button class="eye-btn" @click="toggleFieldVisibility('xunfei-apisecret')" type="button">
                    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                      <use :href="`/icons.svg#icon-eye-${fieldVisibility['xunfei-apisecret'] ? 'off' : 'open'}`"></use>
                    </svg>
                  </button>
                </div>
              </div>
              <div class="error-hint" v-if="xunfeiOcrKeyError">{{ xunfeiOcrKeyError }}</div>
            </div>
          </template>
          <template v-else-if="o.key === 'baidu_ocr'">
            <div class="microsoft-wrap" style="display:block;">
              <div style="display: flex; gap: 6px; align-items: center;">
                <div class="password-field">
                  <input :type="fieldVisibility['baidu_ocr-apikey'] ? 'text' : 'password'" class="key-input" :class="{ 'input-error': baiduOcrKeyError }" placeholder="API Key" style="width: 120px;" v-model="ocrKeys['baidu_ocr-apikey']">
                  <button class="eye-btn" @click="toggleFieldVisibility('baidu_ocr-apikey')" type="button">
                    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                      <use :href="`/icons.svg#icon-eye-${fieldVisibility['baidu_ocr-apikey'] ? 'off' : 'open'}`"></use>
                    </svg>
                  </button>
                </div>
                <div class="password-field">
                  <input :type="fieldVisibility['baidu_ocr-apisecret'] ? 'text' : 'password'" class="key-input" :class="{ 'input-error': baiduOcrKeyError }" placeholder="Secret Key" style="width: 120px;" v-model="ocrKeys['baidu_ocr-apisecret']">
                  <button class="eye-btn" @click="toggleFieldVisibility('baidu_ocr-apisecret')" type="button">
                    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                      <use :href="`/icons.svg#icon-eye-${fieldVisibility['baidu_ocr-apisecret'] ? 'off' : 'open'}`"></use>
                    </svg>
                  </button>
                </div>
              </div>
              <div class="error-hint" v-if="baiduOcrKeyError">{{ baiduOcrKeyError }}</div>
            </div>
          </template>
          <template v-else-if="o.key === 'tencent'">
            <div class="microsoft-wrap" style="display:block;">
              <div style="display: flex; gap: 6px; align-items: center;">
                <div class="password-field">
                  <input :type="fieldVisibility['tencent-secretid'] ? 'text' : 'password'" class="key-input" :class="{ 'input-error': tencentOcrKeyError }" placeholder="SecretId" style="width: 120px;" v-model="ocrKeys['tencent-secretid']">
                  <button class="eye-btn" @click="toggleFieldVisibility('tencent-secretid')" type="button">
                    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                      <use :href="`/icons.svg#icon-eye-${fieldVisibility['tencent-secretid'] ? 'off' : 'open'}`"></use>
                    </svg>
                  </button>
                </div>
                <div class="password-field">
                  <input :type="fieldVisibility['tencent-secretkey'] ? 'text' : 'password'" class="key-input" :class="{ 'input-error': tencentOcrKeyError }" placeholder="SecretKey" style="width: 120px;" v-model="ocrKeys['tencent-secretkey']">
                  <button class="eye-btn" @click="toggleFieldVisibility('tencent-secretkey')" type="button">
                    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                      <use :href="`/icons.svg#icon-eye-${fieldVisibility['tencent-secretkey'] ? 'off' : 'open'}`"></use>
                    </svg>
                  </button>
                </div>
              </div>
              <div class="error-hint" v-if="tencentOcrKeyError">{{ tencentOcrKeyError }}</div>
            </div>
          </template>
          <template v-else-if="o.key === 'ollama_ocr'">
            <div class="microsoft-wrap" style="display:block;">
              <div style="display: flex; gap: 6px; align-items: center;">
                <input type="text" class="key-input" :class="{ 'input-error': ollamaOcrKeyError }" placeholder="接口地址" style="width: 200px;" v-model="ocrKeys['ollama_ocr_base_url']">
                <input type="text" class="key-input" :class="{ 'input-error': ollamaOcrKeyError }" placeholder="模型名" style="width: 160px;" v-model="ocrKeys['ollama_ocr_model']">
              </div>
              <div class="error-hint" v-if="ollamaOcrKeyError">{{ ollamaOcrKeyError }}</div>
            </div>
          </template>
          <template v-else-if="o.key === 'paddle_ocr'">
          </template>
          <template v-else>
            <div class="password-field">
              <input :type="fieldVisibility[o.key + '-appid'] ? 'text' : 'password'" class="key-input" placeholder="AppId" style="width: 120px;" v-model="ocrKeys[o.key + '-appid']">
              <button class="eye-btn" @click="toggleFieldVisibility(o.key + '-appid')" type="button">
                <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                  <use :href="`/icons.svg#icon-eye-${fieldVisibility[o.key + '-appid'] ? 'off' : 'open'}`"></use>
                </svg>
              </button>
            </div>
            <div class="password-field">
              <input :type="fieldVisibility[o.key + '-appkey'] ? 'text' : 'password'" class="key-input" placeholder="AppKey" style="width: 120px;" v-model="ocrKeys[o.key + '-appkey']">
              <button class="eye-btn" @click="toggleFieldVisibility(o.key + '-appkey')" type="button">
                <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                  <use :href="`/icons.svg#icon-eye-${fieldVisibility[o.key + '-appkey'] ? 'off' : 'open'}`"></use>
                </svg>
              </button>
            </div>
          </template>
        </div>
        <div class="switch" :class="{ active: activeOcr === o.key }" @click="toggleOcr(o.key)">
          <div class="switch-knob"></div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, reactive, watch } from 'vue'
import { useSettings } from '../../composables/useSettings.js'

const { activeOcr, ocrKeys } = useSettings()

const ocrList = [
  { key: 'paddle_ocr', name: 'PaddleOCR', desc: '本地 PaddleOCR 模型识别，无需 API Key，激活即用' },
  { key: 'ollama_ocr', name: 'Ollama OCR', desc: '本地视觉模型 OCR，需部署 Ollama 和视觉模型' },
  { key: 'baidu_ocr', name: '百度云OCR', desc: '通用文字识别(标准版)' },
  { key: 'xunfei', name: '讯飞OCR', desc: '通用文档识别(OCR大模型)' },
  { key: 'tencent', name: '腾讯云OCR', desc: '通用文字识别Agent(RecognizeAgent)' }
]

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
  if (activeOcr.value && activeOcr.value !== key) {
    clearOcrVisibility(activeOcr.value)
  }
  activeOcr.value = activeOcr.value === key ? null : key
}
</script>

<style scoped>
.translator-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 8px 0;
  border-bottom: 1px solid var(--border);
  flex-wrap: wrap;
  gap: 10px;
}

.translator-info {
  flex: 1;
}

.translator-name {
  font-size: 13px;
  font-weight: 500;
  color: var(--text-primary);
}

.translator-desc {
  font-size: 11px;
  color: var(--text-tertiary);
  margin-top: 2px;
}

.tab-hint {
  margin-bottom: 10px;
  font-size: 11px;
  color: var(--text-tertiary);
}
</style>
