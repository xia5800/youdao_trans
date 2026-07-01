<template>
  <div class="translator-setting">
    <div class="section-title">翻译源设置</div>
    <ServiceConfigLayout>
    <template #list>
        <div
          v-for="t in translatorList"
          :key="t.key"
          class="translator-row"
          :class="{ selected: selectedKey === t.key }"
          @click="selectedKey = t.key"
        >
          <span class="translator-row-name">{{ t.name }}</span>
          <SwitchToggle :modelValue="activeTranslator === t.key" @update:modelValue="toggleTranslator(t.key)" />
        </div>
    </template>
    <template #detail>
      <template v-if="selectedTranslator">
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
              <PasswordField v-model="translatorKeys['microsoft']" placeholder="请输入密钥 Key" />
            </div>
            <div class="detail-field">
              <label>区域 Region <span class="required">*</span></label>
              <input type="text" class="key-input detail-input" :class="{ 'input-error': microsoftKeyError }" placeholder="例如: eastasia" v-model="translatorKeys['microsoft_region']">
            </div>
            <div class="error-hint" v-if="microsoftKeyError">{{ microsoftKeyError }}</div>
          </div>
          <div class="btn-save-wrap">
            <button class="btn-save" @click="saveConfig">保存</button>
          </div>
          <div class="detail-help">
            <div class="detail-help-title">如何获取：</div>
            <p>前往 <a class="link" @click.prevent="openUrl('https://portal.azure.com')">Azure Portal</a> 创建 Translator 资源，在"密钥和终结点"页面可以获取密钥 Key 和区域 Region。</p>
          </div>
        </template>

        <template v-else-if="selectedTranslator.key === 'baidu'">
          <div class="detail-fields">
            <div class="detail-field">
              <label>APP ID <span class="required">*</span></label>
              <PasswordField v-model="translatorKeys['baidu_appid']" placeholder="请输入 APP ID" />
            </div>
            <div class="detail-field">
              <label>APP Key <span class="required">*</span></label>
              <PasswordField v-model="translatorKeys['baidu_appkey']" placeholder="请输入 APP Key" />
            </div>
            <div class="error-hint" v-if="baiduKeyError">{{ baiduKeyError }}</div>
          </div>
          <div class="btn-save-wrap">
            <button class="btn-save" @click="saveConfig">保存</button>
          </div>
          <div class="detail-help">
            <div class="detail-help-title">如何获取：</div>
            <p>前往 <a class="link" @click.prevent="openUrl('https://fanyi-api.baidu.com')">百度翻译开放平台</a> 注册开发者账号，创建应用后即可获取 APP ID 和 APP Key。</p>
          </div>
        </template>

        <template v-else-if="selectedTranslator.key === 'youdao'">
          <div class="detail-fields">
            <div class="detail-field">
              <label>应用 ID <span class="required">*</span></label>
              <PasswordField v-model="translatorKeys['youdao_appid']" placeholder="请输入应用 ID" />
            </div>
            <div class="detail-field">
              <label>应用密钥 <span class="required">*</span></label>
              <PasswordField v-model="translatorKeys['youdao_appsecret']" placeholder="请输入应用密钥" />
            </div>
            <div class="error-hint" v-if="youdaoKeyError">{{ youdaoKeyError }}</div>
          </div>
          <div class="btn-save-wrap">
            <button class="btn-save" @click="saveConfig">保存</button>
          </div>
          <div class="detail-help">
            <div class="detail-help-title">如何获取：</div>
            <p>前往 <a class="link" @click.prevent="openUrl('https://ai.youdao.com')">有道智云</a> 注册开发者账号，创建文本翻译应用后获取应用 ID 和应用密钥。</p>
          </div>
        </template>

        <template v-else-if="selectedTranslator.key === 'openai'">
          <div class="detail-fields">
            <div class="detail-field">
              <label>API Key <span class="required">*</span></label>
              <PasswordField v-model="translatorKeys['openai_key']" placeholder="sk-..." />
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
          <div class="btn-save-wrap">
            <button class="btn-save" @click="saveConfig">保存</button>
          </div>
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
          <div class="btn-save-wrap">
            <button class="btn-save" @click="saveConfig">保存</button>
          </div>
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
          <div class="btn-save-wrap">
            <button class="btn-save" @click="saveConfig">保存</button>
          </div>
          <div class="detail-help">
            <div class="detail-help-title">如何获取：</div>
            <p>需自行部署 DeepLX 服务，部署成功后填写服务地址即可使用。</p>
          </div>
        </template>

        <template v-else-if="selectedTranslator.key === 'ali'">
          <div class="detail-fields">
            <div class="detail-field">
              <label>AccessKey <span class="required">*</span></label>
              <PasswordField v-model="translatorKeys['ali-accesskey']" placeholder="请输入 AccessKey" />
            </div>
            <div class="detail-field">
              <label>SecretKey <span class="required">*</span></label>
              <PasswordField v-model="translatorKeys['ali-secretkey']" placeholder="请输入 SecretKey" />
            </div>
          </div>
          <div class="btn-save-wrap">
            <button class="btn-save" @click="saveConfig">保存</button>
          </div>
          <div class="detail-help">
            <div class="detail-help-title">如何获取：</div>
            <p>前往 <a class="link" @click.prevent="openUrl('https://aliyun.com')">阿里云控制台</a> 的 RAM 访问控制中创建 AccessKey。</p>
          </div>
        </template>
      </template>
    </template>
  </ServiceConfigLayout>
</div>
</template>

<script setup>
import PasswordField from '../../components/PasswordField.vue'
import SwitchToggle from '../../components/SwitchToggle.vue'
import ServiceConfigLayout from '../../components/ServiceConfigLayout.vue'
import { useSettings } from '../../composables/useSettings.js'
import { useUtils } from '../../composables/useUtils.js'
import { useServiceConfig } from '../../composables/useServiceConfig.js'

const { settings, activeTranslator, translatorKeys } = useSettings()
const { showToastOnce, showToast, openUrl } = useUtils()

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

const { selectedKey, selectedService: selectedTranslator, errors, toggleService, saveConfig: saveConfigFn } = useServiceConfig({
  serviceList: translatorList,
  activeService: activeTranslator,
  keys: translatorKeys,
  validators: [
    { key: 'microsoft', activeKey: 'microsoft', fields: ['microsoft', 'microsoft_region'],
      message: '密钥 Key 和区域 Region 必须同时填写',
      saveCheck: (k) => { const a = k['microsoft'], b = k['microsoft_region']; return (!a || !a.trim()) == (!b || !b.trim()) } },
    { key: 'baidu', activeKey: 'baidu', fields: ['baidu_appid', 'baidu_appkey'],
      message: 'APP ID 和 APP Key 必须同时填写',
      saveCheck: (k) => { const a = k['baidu_appid'], b = k['baidu_appkey']; return (!a || !a.trim()) == (!b || !b.trim()) } },
    { key: 'youdao', activeKey: 'youdao', fields: ['youdao_appid', 'youdao_appsecret'],
      message: '应用 ID 和应用密钥必须同时填写',
      saveCheck: (k) => { const a = k['youdao_appid'], b = k['youdao_appsecret']; return (!a || !a.trim()) == (!b || !b.trim()) } },
    { key: 'openai', activeKey: 'openai', fields: ['openai_key', 'openai_model'], message: 'API Key 和模型名必须填写' },
    { key: 'ollama', activeKey: 'ollama', fields: ['ollama_base_url', 'ollama_model'], message: '接口地址和模型名必须填写' },
    { key: 'deeplx', activeKey: 'deeplx', fields: ['deeplx_endpoint'], message: '接口地址必须填写' },
  ],
})

const microsoftKeyError = errors.microsoft
const baiduKeyError = errors.baidu
const youdaoKeyError = errors.youdao
const openaiKeyError = errors.openai
const ollamaKeyError = errors.ollama
const deeplxKeyError = errors.deeplx

function toggleTranslator(key) { toggleService(key) }

function saveConfig() { saveConfigFn({ settings, showToastOnce, showToast }) }
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
</style>
