<template>
  <div>
    <div class="section-title">翻译源设置</div>

    <div class="tab-bar">
      <div class="tab-item" :class="{ active: translatorTab === 'translate' }" @click="translatorTab = 'translate'">翻译源</div>
      <div class="tab-item" :class="{ active: translatorTab === 'ocr' }" @click="translatorTab = 'ocr'">文字识别 (OCR)</div>
    </div>
    <div class="tab-panel">
    <template v-if="translatorTab === 'translate'">
      <div class="tab-hint">以下翻译源互斥，只能开启一个</div>
      <div class="translator-item" v-for="t in translatorList" :key="t.key">
        <div class="translator-info">
          <div class="translator-name">{{ t.name }}</div>
          <div class="translator-desc">{{ t.desc }}</div>
        </div>
        <div class="setting-control">
          <div v-if="t.key === 'ali'" style="display: flex; gap: 6px;" :style="{ display: activeTranslator === t.key ? 'flex' : 'none' }">
            <div class="password-field">
              <input :type="fieldVisibility['ali-accesskey'] ? 'text' : 'password'" class="key-input" placeholder="AccessKey" style="width: 120px;" v-model="translatorKeys['ali-accesskey']">
              <button class="eye-btn" @click="toggleFieldVisibility('ali-accesskey')" type="button">
                <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                  <use :href="`/icons.svg#icon-eye-${fieldVisibility['ali-accesskey'] ? 'off' : 'open'}`"></use>
                </svg>
              </button>
            </div>
            <div class="password-field">
              <input :type="fieldVisibility['ali-secretkey'] ? 'text' : 'password'" class="key-input" placeholder="SecretKey" style="width: 120px;" v-model="translatorKeys['ali-secretkey']">
              <button class="eye-btn" @click="toggleFieldVisibility('ali-secretkey')" type="button">
                <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                  <use :href="`/icons.svg#icon-eye-${fieldVisibility['ali-secretkey'] ? 'off' : 'open'}`"></use>
                </svg>
              </button>
            </div>
          </div>
          <template v-else-if="t.key === 'microsoft_free'">
            <div class="switch" :class="{ active: activeTranslator === t.key }" @click="toggleTranslator(t.key)">
              <div class="switch-knob"></div>
            </div>
          </template>
          <template v-else-if="t.key === 'microsoft'">
            <div class="microsoft-wrap" :style="{ display: activeTranslator === t.key ? 'block' : 'none' }">
              <div style="display: flex; gap: 6px; align-items: center;">
                <div class="password-field">
                  <input :type="fieldVisibility['microsoft'] ? 'text' : 'password'" class="key-input" :class="{ 'input-error': microsoftKeyError }" placeholder="密钥Key" v-model="translatorKeys['microsoft']">
                  <button class="eye-btn" @click="toggleFieldVisibility('microsoft')" type="button">
                    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                      <use :href="`/icons.svg#icon-eye-${fieldVisibility['microsoft'] ? 'off' : 'open'}`"></use>
                    </svg>
                  </button>
                </div>
                <input type="text" class="key-input" :class="{ 'input-error': microsoftKeyError }" placeholder="区域 Region" v-model="translatorKeys['microsoft_region']" style="width: 100px;">
              </div>
              <div class="error-hint" v-if="microsoftKeyError">{{ microsoftKeyError }}</div>
            </div>
            <div class="switch" :class="{ active: activeTranslator === t.key }" @click="toggleTranslator(t.key)">
              <div class="switch-knob"></div>
            </div>
          </template>
          <template v-else-if="t.key === 'baidu'">
            <div class="microsoft-wrap" :style="{ display: activeTranslator === t.key ? 'block' : 'none' }">
              <div style="display: flex; gap: 6px; align-items: center;">
                <div class="password-field">
                  <input :type="fieldVisibility['baidu_appid'] ? 'text' : 'password'" class="key-input" :class="{ 'input-error': baiduKeyError }" placeholder="APP ID" v-model="translatorKeys['baidu_appid']">
                  <button class="eye-btn" @click="toggleFieldVisibility('baidu_appid')" type="button">
                    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                      <use :href="`/icons.svg#icon-eye-${fieldVisibility['baidu_appid'] ? 'off' : 'open'}`"></use>
                    </svg>
                  </button>
                </div>
                <div class="password-field">
                  <input :type="fieldVisibility['baidu_appkey'] ? 'text' : 'password'" class="key-input" :class="{ 'input-error': baiduKeyError }" placeholder="APP Key" v-model="translatorKeys['baidu_appkey']">
                  <button class="eye-btn" @click="toggleFieldVisibility('baidu_appkey')" type="button">
                    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                      <use :href="`/icons.svg#icon-eye-${fieldVisibility['baidu_appkey'] ? 'off' : 'open'}`"></use>
                    </svg>
                  </button>
                </div>
              </div>
              <div class="error-hint" v-if="baiduKeyError">{{ baiduKeyError }}</div>
            </div>
            <div class="switch" :class="{ active: activeTranslator === t.key }" @click="toggleTranslator(t.key)">
              <div class="switch-knob"></div>
            </div>
          </template>
          <template v-else-if="t.key === 'youdao'">
            <div class="microsoft-wrap" :style="{ display: activeTranslator === t.key ? 'block' : 'none' }">
              <div style="display: flex; gap: 6px; align-items: center;">
                <div class="password-field">
                  <input :type="fieldVisibility['youdao_appid'] ? 'text' : 'password'" class="key-input" :class="{ 'input-error': youdaoKeyError }" placeholder="应用ID" v-model="translatorKeys['youdao_appid']">
                  <button class="eye-btn" @click="toggleFieldVisibility('youdao_appid')" type="button">
                    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                      <use :href="`/icons.svg#icon-eye-${fieldVisibility['youdao_appid'] ? 'off' : 'open'}`"></use>
                    </svg>
                  </button>
                </div>
                <div class="password-field">
                  <input :type="fieldVisibility['youdao_appsecret'] ? 'text' : 'password'" class="key-input" :class="{ 'input-error': youdaoKeyError }" placeholder="应用密钥" v-model="translatorKeys['youdao_appsecret']">
                  <button class="eye-btn" @click="toggleFieldVisibility('youdao_appsecret')" type="button">
                    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                      <use :href="`/icons.svg#icon-eye-${fieldVisibility['youdao_appsecret'] ? 'off' : 'open'}`"></use>
                    </svg>
                  </button>
                </div>
              </div>
              <div class="error-hint" v-if="youdaoKeyError">{{ youdaoKeyError }}</div>
            </div>
            <div class="switch" :class="{ active: activeTranslator === t.key }" @click="toggleTranslator(t.key)">
              <div class="switch-knob"></div>
            </div>
          </template>
          <template v-else-if="t.key === 'google'">
            <div class="switch" :class="{ active: activeTranslator === t.key }" @click="toggleTranslator(t.key)">
              <div class="switch-knob"></div>
            </div>
          </template>
          <template v-else-if="t.key === 'openai'">
            <div class="microsoft-wrap" :style="{ display: activeTranslator === t.key ? 'block' : 'none' }">
              <div style="display: flex; gap: 6px; align-items: center;">
                <div class="password-field">
                  <input :type="fieldVisibility['openai_key'] ? 'text' : 'password'" class="key-input" :class="{ 'input-error': openaiKeyError }" placeholder="API Key" style="width: 120px;" v-model="translatorKeys['openai_key']">
                  <button class="eye-btn" @click="toggleFieldVisibility('openai_key')" type="button">
                    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><use :href="`/icons.svg#icon-eye-${fieldVisibility['openai_key'] ? 'off' : 'open'}`"></use></svg>
                  </button>
                </div>
                <input type="text" class="key-input" :class="{ 'input-error': openaiKeyError }" placeholder="模型名" style="width: 100px;" v-model="translatorKeys['openai_model']">
                <input type="text" class="key-input" placeholder="接口地址 (可选)" style="width: 140px;" v-model="translatorKeys['openai_endpoint']">
              </div>
              <div class="error-hint" v-if="openaiKeyError">{{ openaiKeyError }}</div>
            </div>
            <div class="switch" :class="{ active: activeTranslator === t.key }" @click="toggleTranslator(t.key)">
              <div class="switch-knob"></div>
            </div>
          </template>
          <template v-else-if="t.key === 'ollama'">
            <div class="microsoft-wrap" :style="{ display: activeTranslator === t.key ? 'block' : 'none' }">
              <div style="display: flex; gap: 6px; align-items: center;">
                <input type="text" class="key-input" :class="{ 'input-error': ollamaKeyError }" placeholder="接口地址" style="width: 200px;" v-model="translatorKeys['ollama_base_url']">
                <input type="text" class="key-input" :class="{ 'input-error': ollamaKeyError }" placeholder="模型名" style="width: 160px;" v-model="translatorKeys['ollama_model']">
              </div>
              <div class="error-hint" v-if="ollamaKeyError">{{ ollamaKeyError }}</div>
            </div>
            <div class="switch" :class="{ active: activeTranslator === t.key }" @click="toggleTranslator(t.key)">
              <div class="switch-knob"></div>
            </div>
          </template>
          <template v-else-if="t.key === 'deeplx'">
            <div class="microsoft-wrap" :style="{ display: activeTranslator === t.key ? 'block' : 'none' }">
              <div style="display: flex; gap: 6px; align-items: center;">
                <input type="text" class="key-input" :class="{ 'input-error': deeplxKeyError }" placeholder="地址" style="width: 200px;" v-model="translatorKeys['deeplx_endpoint']">
              </div>
              <div class="error-hint" v-if="deeplxKeyError">{{ deeplxKeyError }}</div>
            </div>
            <div class="switch" :class="{ active: activeTranslator === t.key }" @click="toggleTranslator(t.key)">
              <div class="switch-knob"></div>
            </div>
          </template>
          <div v-else-if="t.key !== 'youdao'" class="password-field" :style="{ display: activeTranslator === t.key ? 'flex' : 'none' }">
            <input :type="fieldVisibility[t.key] ? 'text' : 'password'" class="key-input" :placeholder="t.keyPlaceholder" v-model="translatorKeys[t.key]">
            <button class="eye-btn" @click="toggleFieldVisibility(t.key)" type="button">
              <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                <use :href="`/icons.svg#icon-eye-${fieldVisibility[t.key] ? 'off' : 'open'}`"></use>
              </svg>
            </button>
          </div>
          <div class="switch" v-if="t.key !== 'microsoft_free' && t.key !== 'microsoft' && t.key !== 'baidu' && t.key !== 'youdao' && t.key !== 'google' && t.key !== 'openai' && t.key !== 'ollama' && t.key !== 'deeplx'" :class="{ active: activeTranslator === t.key }" @click="toggleTranslator(t.key)">
            <div class="switch-knob"></div>
          </div>
        </div>
      </div>
    </template>

    <template v-if="translatorTab === 'ocr'">
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
    </template>
    </div>
  </div>
</template>

<script setup>
import { ref, reactive, watch } from 'vue'
import { useSettings } from '../../composables/useSettings.js'

const {
  settings,
  activeTranslator,
  activeOcr,
  translatorKeys,
  ocrKeys,
} = useSettings()

const translatorTab = ref('translate')

const translatorList = [
  { key: 'microsoft_free', name: '微软翻译（免费）', desc: 'Edge 接口，无需密钥，国内需代理' },
  { key: 'microsoft', name: '微软翻译（Azure）', desc: 'Azure 订阅，稳定高配额，支持指定区域' },
  { key: 'google', name: '谷歌翻译', desc: 'Google 公共接口，无需密钥，国内需代理' },
  { key: 'openai', name: 'OpenAI', desc: 'GPT 模型翻译，质量高，需 API Key' },
  { key: 'deeplx', name: 'DeepLX', desc: 'DeepL 免费接口，需自建 DeepLX 服务' },
  { key: 'ollama', name: 'Ollama', desc: '本地 LLM 翻译，完全免费，需部署 Ollama' },
  { key: 'baidu', name: '百度翻译', desc: '百度翻译 API，国内速度快' },
  { key: 'ali', name: '阿里翻译', desc: '阿里云翻译，企业级稳定' },
  { key: 'youdao', name: '有道翻译', desc: '有道智云翻译，老牌翻译服务' },
]

const ocrList = [
  { key: 'paddle_ocr', name: 'PaddleOCR', desc: '本地 PaddleOCR 模型识别，无需 API Key，激活即用' },
  { key: 'ollama_ocr', name: 'Ollama OCR', desc: '本地视觉模型 OCR，需部署 Ollama 和视觉模型' },
  { key: 'baidu_ocr', name: '百度云OCR', desc: '通用文字识别(标准版)' },
  { key: 'xunfei', name: '讯飞OCR', desc: '通用文档识别(OCR大模型)' },
  { key: 'tencent', name: '腾讯云OCR', desc: '通用文字识别Agent(RecognizeAgent)' }
]

const microsoftKeyError = ref('')
const baiduKeyError = ref('')
const youdaoKeyError = ref('')
const baiduOcrKeyError = ref('')
const xunfeiOcrKeyError = ref('')
const tencentOcrKeyError = ref('')
const openaiKeyError = ref('')
const ollamaKeyError = ref('')
const deeplxKeyError = ref('')
const ollamaOcrKeyError = ref('')

watch([() => translatorKeys.value['microsoft'], () => translatorKeys.value['microsoft_region']], ([key, region]) => {
  const filled = k => k && k.trim()
  if ((filled(key) && !filled(region)) || (!filled(key) && filled(region))) {
    microsoftKeyError.value = '密钥 Key 和区域 Region 必须同时填写'
  } else {
    microsoftKeyError.value = ''
  }
})

watch([() => translatorKeys.value['baidu_appid'], () => translatorKeys.value['baidu_appkey']], ([appid, appkey]) => {
  const filled = k => k && k.trim()
  if ((filled(appid) && !filled(appkey)) || (!filled(appid) && filled(appkey))) {
    baiduKeyError.value = 'APP ID 和 APP Key 必须同时填写'
  } else {
    baiduKeyError.value = ''
  }
})

watch([() => translatorKeys.value['youdao_appid'], () => translatorKeys.value['youdao_appsecret']], ([appid, appsecret]) => {
  const filled = k => k && k.trim()
  if ((filled(appid) && !filled(appsecret)) || (!filled(appid) && filled(appsecret))) {
    youdaoKeyError.value = '应用ID和应用密钥必须同时填写'
  } else {
    youdaoKeyError.value = ''
  }
})

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

watch([() => translatorKeys.value['openai_key'], () => translatorKeys.value['openai_model'], activeTranslator], ([key, model, active]) => {
  const filled = k => k && k.trim()
  if (active === 'openai' && (!filled(key) || !filled(model))) {
    openaiKeyError.value = 'API Key 和模型名必须填写'
  } else {
    openaiKeyError.value = ''
  }
})

watch([() => translatorKeys.value['ollama_base_url'], () => translatorKeys.value['ollama_model'], activeTranslator], ([url, model, active]) => {
  const filled = k => k && k.trim()
  if (active === 'ollama' && (!filled(url) || !filled(model))) {
    ollamaKeyError.value = '接口地址和模型名必须填写'
  } else {
    ollamaKeyError.value = ''
  }
})

watch([() => translatorKeys.value['deeplx_endpoint'], activeTranslator], ([endpoint, active]) => {
  const filled = k => k && k.trim()
  if (active === 'deeplx' && !filled(endpoint)) {
    deeplxKeyError.value = '接口地址必须填写'
  } else {
    deeplxKeyError.value = ''
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

function clearTranslatorVisibility(key) {
  if (key === 'ali') {
    fieldVisibility['ali-accesskey'] = false
    fieldVisibility['ali-secretkey'] = false
  } else if (key === 'baidu') {
    fieldVisibility['baidu_appid'] = false
    fieldVisibility['baidu_appkey'] = false
  } else if (key === 'youdao') {
    fieldVisibility['youdao_appid'] = false
    fieldVisibility['youdao_appsecret'] = false
  } else {
    fieldVisibility[key] = false
  }
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

function toggleTranslator(key) {
  if (activeTranslator.value && activeTranslator.value !== key) {
    clearTranslatorVisibility(activeTranslator.value)
  }
  activeTranslator.value = activeTranslator.value === key ? null : key
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

.tab-bar {
  display: flex;
  gap: 4px;
  padding: 6px 4px 0 4px;
  background: var(--bg-sidebar);
  border: 1px solid var(--border-strong);
  border-bottom: none;
  border-radius: 8px 8px 0 0;
}

.tab-item {
  padding: 6px 16px;
  font-size: 14px;
  color: var(--text-secondary);
  cursor: default;
  border-radius: 6px 6px 0 0;
  border: 1px solid transparent;
  border-bottom: none;
  margin-bottom: 0;
  user-select: none;
  transition: background 0.15s, color 0.15s;
}

.tab-item:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

.tab-item.active {
  color: var(--accent);
  font-weight: 500;
  background: var(--bg-card);
  border-color: var(--border-strong);
  border-bottom-color: var(--bg-card);
  transform: translateY(-1px);
}

.tab-panel {
  border: 1px solid var(--border-strong);
  border-top: none;
  border-radius: 0 0 8px 8px;
  padding: 14px 18px 10px;
  background: var(--bg-card);
  box-shadow: inset 0 1px 2px rgba(0,0,0,0.03);
  margin-bottom: 4px;
}

.tab-hint {
  margin-bottom: 10px;
  font-size: 11px;
  color: var(--text-tertiary);
}
</style>
