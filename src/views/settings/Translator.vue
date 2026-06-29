<template>
  <div class="translator-page">
    <div class="section-title">翻译源设置</div>
    <div class="translator-layout">
      <div class="translator-list">
        <div
          v-for="t in translatorList"
          :key="t.key"
          class="translator-row"
          :class="{ selected: selectedKey === t.key }"
          @click="selectedKey = t.key"
        >
          <span class="translator-row-name">{{ t.name }}</span>
          <div class="switch" :class="{ active: activeTranslator === t.key }" @click.stop="toggleTranslator(t.key)">
            <div class="switch-knob"></div>
          </div>
        </div>
      </div>
      <div class="translator-detail" v-if="selectedTranslator">
        <div class="detail-desc">{{ selectedTranslator.desc }}</div>

        <template v-if="selectedTranslator.key === 'microsoft_free' || selectedTranslator.key === 'google'">
          <div class="detail-fields">
            <div class="detail-no-config">无需额外配置，直接启用即可使用。</div>
          </div>
        </template>

        <template v-else-if="selectedTranslator.key === 'microsoft'">
          <div class="detail-fields">
            <div class="detail-field">
              <label>密钥 Key <span class="required">*</span></label>
              <div class="password-field">
                <input :type="fieldVisibility['microsoft'] ? 'text' : 'password'" class="key-input detail-input" :class="{ 'input-error': microsoftKeyError }" placeholder="请输入密钥 Key" v-model="translatorKeys['microsoft']">
                <button class="eye-btn" @click="toggleFieldVisibility('microsoft')" type="button">
                  <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                    <use :href="`/icons.svg#icon-eye-${fieldVisibility['microsoft'] ? 'off' : 'open'}`"></use>
                  </svg>
                </button>
              </div>
            </div>
            <div class="detail-field">
              <label>区域 Region <span class="required">*</span></label>
              <input type="text" class="key-input detail-input" :class="{ 'input-error': microsoftKeyError }" placeholder="例如: eastasia" v-model="translatorKeys['microsoft_region']">
            </div>
            <div class="error-hint" v-if="microsoftKeyError">{{ microsoftKeyError }}</div>
          </div>
          <button class="btn-save" @click="saveConfig">保存</button>
          <div class="detail-help">
            <div class="detail-help-title">如何获取：</div>
            <p>前往 <a class="link" @click.prevent="openUrl('https://portal.azure.com')">Azure Portal</a> 创建 Translator 资源，在"密钥和终结点"页面可以获取密钥 Key 和区域 Region。</p>
          </div>
        </template>

        <template v-else-if="selectedTranslator.key === 'baidu'">
          <div class="detail-fields">
            <div class="detail-field">
              <label>APP ID <span class="required">*</span></label>
              <div class="password-field">
                <input :type="fieldVisibility['baidu_appid'] ? 'text' : 'password'" class="key-input detail-input" :class="{ 'input-error': baiduKeyError }" placeholder="请输入 APP ID" v-model="translatorKeys['baidu_appid']">
                <button class="eye-btn" @click="toggleFieldVisibility('baidu_appid')" type="button">
                  <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                    <use :href="`/icons.svg#icon-eye-${fieldVisibility['baidu_appid'] ? 'off' : 'open'}`"></use>
                  </svg>
                </button>
              </div>
            </div>
            <div class="detail-field">
              <label>APP Key <span class="required">*</span></label>
              <div class="password-field">
                <input :type="fieldVisibility['baidu_appkey'] ? 'text' : 'password'" class="key-input detail-input" :class="{ 'input-error': baiduKeyError }" placeholder="请输入 APP Key" v-model="translatorKeys['baidu_appkey']">
                <button class="eye-btn" @click="toggleFieldVisibility('baidu_appkey')" type="button">
                  <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                    <use :href="`/icons.svg#icon-eye-${fieldVisibility['baidu_appkey'] ? 'off' : 'open'}`"></use>
                  </svg>
                </button>
              </div>
            </div>
            <div class="error-hint" v-if="baiduKeyError">{{ baiduKeyError }}</div>
          </div>
          <button class="btn-save" @click="saveConfig">保存</button>
          <div class="detail-help">
            <div class="detail-help-title">如何获取：</div>
            <p>前往 <a class="link" @click.prevent="openUrl('https://fanyi-api.baidu.com')">百度翻译开放平台</a> 注册开发者账号，创建应用后即可获取 APP ID 和 APP Key。</p>
          </div>
        </template>

        <template v-else-if="selectedTranslator.key === 'youdao'">
          <div class="detail-fields">
            <div class="detail-field">
              <label>应用 ID <span class="required">*</span></label>
              <div class="password-field">
                <input :type="fieldVisibility['youdao_appid'] ? 'text' : 'password'" class="key-input detail-input" :class="{ 'input-error': youdaoKeyError }" placeholder="请输入应用 ID" v-model="translatorKeys['youdao_appid']">
                <button class="eye-btn" @click="toggleFieldVisibility('youdao_appid')" type="button">
                  <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                    <use :href="`/icons.svg#icon-eye-${fieldVisibility['youdao_appid'] ? 'off' : 'open'}`"></use>
                  </svg>
                </button>
              </div>
            </div>
            <div class="detail-field">
              <label>应用密钥 <span class="required">*</span></label>
              <div class="password-field">
                <input :type="fieldVisibility['youdao_appsecret'] ? 'text' : 'password'" class="key-input detail-input" :class="{ 'input-error': youdaoKeyError }" placeholder="请输入应用密钥" v-model="translatorKeys['youdao_appsecret']">
                <button class="eye-btn" @click="toggleFieldVisibility('youdao_appsecret')" type="button">
                  <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                    <use :href="`/icons.svg#icon-eye-${fieldVisibility['youdao_appsecret'] ? 'off' : 'open'}`"></use>
                  </svg>
                </button>
              </div>
            </div>
            <div class="error-hint" v-if="youdaoKeyError">{{ youdaoKeyError }}</div>
          </div>
          <button class="btn-save" @click="saveConfig">保存</button>
          <div class="detail-help">
            <div class="detail-help-title">如何获取：</div>
            <p>前往 <a class="link" @click.prevent="openUrl('https://ai.youdao.com')">有道智云</a> 注册开发者账号，创建文本翻译应用后获取应用 ID 和应用密钥。</p>
          </div>
        </template>

        <template v-else-if="selectedTranslator.key === 'openai'">
          <div class="detail-fields">
            <div class="detail-field">
              <label>API Key <span class="required">*</span></label>
              <div class="password-field">
                <input :type="fieldVisibility['openai_key'] ? 'text' : 'password'" class="key-input detail-input" :class="{ 'input-error': openaiKeyError }" placeholder="sk-..." v-model="translatorKeys['openai_key']">
                <button class="eye-btn" @click="toggleFieldVisibility('openai_key')" type="button">
                  <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                    <use :href="`/icons.svg#icon-eye-${fieldVisibility['openai_key'] ? 'off' : 'open'}`"></use>
                  </svg>
                </button>
              </div>
            </div>
            <div class="detail-field">
              <label>模型名 <span class="required">*</span></label>
              <input type="text" class="key-input detail-input" :class="{ 'input-error': openaiKeyError }" placeholder="例如: gpt-4o-mini" v-model="translatorKeys['openai_model']">
            </div>
            <div class="detail-field">
              <label>接口地址</label>
              <input type="text" class="key-input detail-input" placeholder="可选，默认 https://api.openai.com" v-model="translatorKeys['openai_endpoint']">
            </div>
            <div class="error-hint" v-if="openaiKeyError">{{ openaiKeyError }}</div>
          </div>
          <button class="btn-save" @click="saveConfig">保存</button>
          <div class="detail-help">
            <div class="detail-help-title">如何获取：</div>
            <p>前往 <a class="link" @click.prevent="openUrl('https://platform.openai.com')">OpenAI Platform</a> 的 API Keys 页面创建 API Key，模型名可在模型列表中查看。如使用代理或中转服务，可修改接口地址。</p>
          </div>
        </template>

        <template v-else-if="selectedTranslator.key === 'ollama'">
          <div class="detail-fields">
            <div class="detail-field">
              <label>接口地址 <span class="required">*</span></label>
              <input type="text" class="key-input detail-input" :class="{ 'input-error': ollamaKeyError }" placeholder="例如: http://localhost:11434" v-model="translatorKeys['ollama_base_url']">
            </div>
            <div class="detail-field">
              <label>模型名 <span class="required">*</span></label>
              <input type="text" class="key-input detail-input" :class="{ 'input-error': ollamaKeyError }" placeholder="例如: llama3.2" v-model="translatorKeys['ollama_model']">
            </div>
            <div class="error-hint" v-if="ollamaKeyError">{{ ollamaKeyError }}</div>
          </div>
          <button class="btn-save" @click="saveConfig">保存</button>
          <div class="detail-help">
            <div class="detail-help-title">如何获取：</div>
            <p>确保已部署 Ollama 服务并拉取了翻译模型。可在 Ollama 官网查看支持的模型列表。</p>
          </div>
        </template>

        <template v-else-if="selectedTranslator.key === 'deeplx'">
          <div class="detail-fields">
            <div class="detail-field">
              <label>接口地址 <span class="required">*</span></label>
              <input type="text" class="key-input detail-input" :class="{ 'input-error': deeplxKeyError }" placeholder="例如: http://localhost:1188" v-model="translatorKeys['deeplx_endpoint']">
            </div>
            <div class="error-hint" v-if="deeplxKeyError">{{ deeplxKeyError }}</div>
          </div>
          <button class="btn-save" @click="saveConfig">保存</button>
          <div class="detail-help">
            <div class="detail-help-title">如何获取：</div>
            <p>需自行部署 DeepLX 服务，部署成功后填写服务地址即可使用。</p>
          </div>
        </template>

        <template v-else-if="selectedTranslator.key === 'ali'">
          <div class="detail-fields">
            <div class="detail-field">
              <label>AccessKey <span class="required">*</span></label>
              <div class="password-field">
                <input :type="fieldVisibility['ali-accesskey'] ? 'text' : 'password'" class="key-input detail-input" placeholder="请输入 AccessKey" v-model="translatorKeys['ali-accesskey']">
                <button class="eye-btn" @click="toggleFieldVisibility('ali-accesskey')" type="button">
                  <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                    <use :href="`/icons.svg#icon-eye-${fieldVisibility['ali-accesskey'] ? 'off' : 'open'}`"></use>
                  </svg>
                </button>
              </div>
            </div>
            <div class="detail-field">
              <label>SecretKey <span class="required">*</span></label>
              <div class="password-field">
                <input :type="fieldVisibility['ali-secretkey'] ? 'text' : 'password'" class="key-input detail-input" placeholder="请输入 SecretKey" v-model="translatorKeys['ali-secretkey']">
                <button class="eye-btn" @click="toggleFieldVisibility('ali-secretkey')" type="button">
                  <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                    <use :href="`/icons.svg#icon-eye-${fieldVisibility['ali-secretkey'] ? 'off' : 'open'}`"></use>
                  </svg>
                </button>
              </div>
            </div>
          </div>
          <button class="btn-save" @click="saveConfig">保存</button>
          <div class="detail-help">
            <div class="detail-help-title">如何获取：</div>
            <p>前往 <a class="link" @click.prevent="openUrl('https://aliyun.com')">阿里云控制台</a> 的 RAM 访问控制中创建 AccessKey。</p>
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

const { settings, activeTranslator, translatorKeys, configPath } = useSettings()
const { showToastOnce, openUrl } = useUtils()

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

const selectedKey = ref(activeTranslator.value || translatorList[0].key)
const selectedTranslator = computed(() => translatorList.find(t => t.key === selectedKey.value))

const microsoftKeyError = ref('')
const baiduKeyError = ref('')
const youdaoKeyError = ref('')
const openaiKeyError = ref('')
const ollamaKeyError = ref('')
const deeplxKeyError = ref('')

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

function toggleTranslator(key) {
  selectedKey.value = key
  if (activeTranslator.value && activeTranslator.value !== key) {
    clearTranslatorVisibility(activeTranslator.value)
  }
  activeTranslator.value = activeTranslator.value === key ? null : key
}

async function saveConfig() {
  const filled = v => v && v.trim()
  const key = selectedKey.value

  if (key === 'microsoft') {
    const k = translatorKeys.value['microsoft']
    const r = translatorKeys.value['microsoft_region']
    if ((filled(k) && !filled(r)) || (!filled(k) && filled(r))) {
      showToastOnce('密钥 Key 和区域 Region 必须同时填写')
      return
    }
  } else if (key === 'baidu') {
    const a = translatorKeys.value['baidu_appid']
    const k = translatorKeys.value['baidu_appkey']
    if ((filled(a) && !filled(k)) || (!filled(a) && filled(k))) {
      showToastOnce('APP ID 和 APP Key 必须同时填写')
      return
    }
  } else if (key === 'youdao') {
    const a = translatorKeys.value['youdao_appid']
    const s = translatorKeys.value['youdao_appsecret']
    if ((filled(a) && !filled(s)) || (!filled(a) && filled(s))) {
      showToastOnce('应用 ID 和应用密钥必须同时填写')
      return
    }
  } else if (key === 'openai') {
    const k = translatorKeys.value['openai_key']
    const m = translatorKeys.value['openai_model']
    if (!filled(k) || !filled(m)) {
      showToastOnce('API Key 和模型名必须填写')
      return
    }
  } else if (key === 'ollama') {
    const u = translatorKeys.value['ollama_base_url']
    const m = translatorKeys.value['ollama_model']
    if (!filled(u) || !filled(m)) {
      showToastOnce('接口地址和模型名必须填写')
      return
    }
  } else if (key === 'deeplx') {
    const e = translatorKeys.value['deeplx_endpoint']
    if (!filled(e)) {
      showToastOnce('接口地址必须填写')
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
