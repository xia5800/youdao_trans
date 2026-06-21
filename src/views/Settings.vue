<template>
  <div class="settings-container">
    <div class="settings-sidebar">
      <div class="settings-category">基础</div>
      <div
        v-for="item in navItems"
        :key="item.id"
        class="settings-nav-item"
        :class="{ active: activeSection === item.id }"
        @click="activeSection = item.id"
      >{{ item.label }}</div>
    </div>

    <div class="settings-content">
      <!-- 偏好设置 -->
      <div class="setting-section" :class="{ 'active-section': activeSection === 'preference' }">
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
      </div>

      <!-- 高级设置 -->
      <div class="setting-section" :class="{ 'active-section': activeSection === 'advanced' }">
        <div class="section-title">高级设置</div>

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

      <!-- 全局快捷键设置 -->
      <div class="setting-section" :class="{ 'active-section': activeSection === 'hotkey' }">
        <div class="section-title">全局快捷键设置</div>
        <div class="setting-desc" style="margin-bottom:12px;color:var(--text-tertiary);font-size:12px;">点击快捷键组合即可录制修改，仅按修饰键无效</div>
        <div class="setting-item" v-for="hk in hotkeys" :key="hk.id">
          <div>
            <div class="setting-label">{{ hk.label }}</div>
            <div class="setting-desc">{{ hk.desc }}</div>
          </div>
          <div
            class="hotkey-recorder"
            :class="{ recording: isRecording(hk.id) }"
            @click="startRecording(hk.id)"
            tabindex="0"
          >
            <span v-if="isRecording(hk.id) && !tempCombo">按下按键...</span>
            <span v-else-if="isRecording(hk.id) && tempCombo">{{ tempCombo }}</span>
            <span v-else>{{ hk.combo }}</span>
          </div>
        </div>
      </div>

      <!-- 翻译源设置 -->
      <div class="setting-section" :class="{ 'active-section': activeSection === 'translator' }">
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

      <!-- 关于 -->
      <div class="setting-section about-section" :class="{ 'active-section': activeSection === 'about' }">
        <div class="about-container">
          <div class="about-logo">Y</div>
          <div class="about-name">优道翻译</div>
          <div class="about-subtitle">不止翻译，更是词典</div>
          <div class="about-version">版本 v{{ version }}</div>
          <div class="about-copyright">© 2026 GCC. All rights reserved.</div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, reactive, computed, watch, onMounted, onUnmounted } from 'vue'
import { useRoute } from 'vue-router'
import { invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'
import { useTheme } from '../composables/useTheme.js'
import { useHotkey } from '../composables/useHotkey.js'
import { useSettings } from '../composables/useSettings.js'
import { useToast } from '../composables/useToast.js'
import { version } from '../../package.json'

const { themeMode } = useTheme()
const { hotkeys, recordingId, startRecording, stopRecording, isRecording, updateCombo, eventToCombo } = useHotkey()
const { showToast } = useToast()

const translateCombo = computed(() => hotkeys.value.find(h => h.id === 'translate')?.combo || 'Ctrl+Enter')

const tempCombo = ref('')
const ignoreKeydownUntil = ref(0)
const defaultConfigDirPlaceholder = ref('')
const defaultDbDirPlaceholder = ref('')

const route = useRoute()
const activeSection = ref((route.query.section) || 'preference')

const navItems = [
  { id: 'preference', label: '偏好设置' },
  { id: 'advanced', label: '高级设置' },
  { id: 'hotkey', label: '全局快捷键设置' },
  { id: 'translator', label: '翻译源设置' },
  { id: 'about', label: '关于' }
]

function onRecordKeydown(e) {
  // Ignore events briefly after a recording (prevents simulated Ctrl+C from
  // being captured if a new recording starts before the guard expires)
  if (Date.now() < ignoreKeydownUntil.value) return
  if (!recordingId.value) return
  if (e.key === 'Escape') {
    stopRecording()
    return
  }
  const combo = eventToCombo(e)
  if (!combo) return
  e.preventDefault()
  // Check conflict before saving
  const conflict = hotkeys.value.find(h => h.id !== recordingId.value && h.combo === combo)
  if (conflict) {
    showToast(`快捷键冲突：${conflict.label} 已使用 ${combo}`)
    stopRecording()
    return
  }
  updateCombo(recordingId.value, combo)
  settings.hotkeys[recordingId.value] = combo
  ignoreKeydownUntil.value = Date.now() + 600
  stopRecording()
}

onMounted(async () => {
  document.addEventListener('keydown', onRecordKeydown)
  // Sync displayed hotkey combos from saved settings
  for (const hk of hotkeys.value) {
    if (settings.hotkeys[hk.id]) {
      hk.combo = settings.hotkeys[hk.id]
    }
  }
  try {
    defaultConfigDirPlaceholder.value = await invoke('default_config_dir')
    defaultDbDirPlaceholder.value = await invoke('default_db_dir')
  } catch (e) {
    console.warn('failed to fetch default paths:', e)
  }
})
onUnmounted(() => document.removeEventListener('keydown', onRecordKeydown))

const {
  settings,
  configPath,
  dbPath,
  autoStart,
  delayTime,
  volume,
  speed,
  storeRecords,
  replaceNewlines,
  autoTranslate,
  autoTranslateDelay,
  activeTranslator,
  activeOcr,
  translatorKeys,
  ocrKeys,
} = useSettings()

watch(themeMode, (val) => { settings.theme = val }, { immediate: true })

const VOL_RANGE = { min: 0, max: 100, unit: '%', step: 1 }
const SPEED_RANGE = { min: 0.5, max: 2, unit: 'x', step: 0.1 }

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
  { key: 'ollama_ocr', name: 'Ollama OCR', desc: '本地视觉模型 OCR，需部署 Ollama 和视觉模型' },
  { key: 'baidu_ocr', name: '百度云OCR', desc: '通用文字识别(标准版)' },
  { key: 'xunfei', name: '讯飞OCR', desc: '通用文档识别(OCR大模型)' },
  { key: 'tencent', name: '腾讯云OCR', desc: '通用文字识别Agent(RecognizeAgent)' }
]

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

const microsoftKeyError = ref('')
const baiduKeyError = ref('')

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

const youdaoKeyError = ref('')

watch([() => translatorKeys.value['youdao_appid'], () => translatorKeys.value['youdao_appsecret']], ([appid, appsecret]) => {
  const filled = k => k && k.trim()
  if ((filled(appid) && !filled(appsecret)) || (!filled(appid) && filled(appsecret))) {
    youdaoKeyError.value = '应用ID和应用密钥必须同时填写'
  } else {
    youdaoKeyError.value = ''
  }
})

const baiduOcrKeyError = ref('')
const xunfeiOcrKeyError = ref('')
const tencentOcrKeyError = ref('')

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

const translatorTab = ref('translate')

const openaiKeyError = ref('')
const ollamaKeyError = ref('')
const deeplxKeyError = ref('')
const ollamaOcrKeyError = ref('')

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
.settings-container {
  flex: 1;
  display: flex;
  margin: 12px;
  background: var(--bg-card);
  border-radius: 12px;
  box-shadow: var(--shadow-card);
  overflow: hidden;
  min-height: 0;
}

.settings-sidebar {
  width: 140px;
  background-color: var(--bg-sidebar);
  border-right: 1px solid var(--border);
  padding: 20px 0;
  flex-shrink: 0;
  overflow-y: auto;
}

.settings-category {
  padding: 6px 16px;
  font-size: 14px;
  font-weight: 600;
  color: var(--text-tertiary);
  letter-spacing: 0.5px;
}

.settings-nav-item {
  padding: 8px 16px;
  font-size: 13px;
  color: var(--text-secondary);
  cursor: default;
  transition: all 0.2s;
  border-left: 3px solid transparent;
}

.settings-nav-item.active {
  background-color: var(--bg-active);
  color: var(--accent);
  border-left-color: var(--accent);
}

.settings-nav-item:hover:not(.active) {
  background-color: var(--bg-hover);
  color: var(--text-primary);
}

.settings-content {
  flex: 1;
  padding: 20px 24px;
  overflow-y: auto;
  background: var(--bg-card);
}

.setting-section {
  display: none;
}

.setting-section.active-section {
  display: block;
}

.setting-section.about-section.active-section {
  display: flex;
  align-items: center;
  justify-content: center;
  min-height: 400px;
}

.section-title {
  font-size: 18px;
  font-weight: 600;
  color: var(--text-primary);
  margin-bottom: 20px;
  padding-bottom: 10px;
  border-bottom: 2px solid var(--border);
}

.subsection-title {
  font-size: 15px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 20px 0 12px 0;
  padding-left: 8px;
  border-left: 3px solid var(--accent);
}

.setting-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px 0;
  border-bottom: 1px solid var(--border);
  flex-wrap: wrap;
  gap: 24px;
}

.setting-item:last-child {
  border-bottom: none;
}

.setting-label {
  font-size: 14px;
  color: var(--text-primary);
  font-weight: 500;
}

.setting-desc {
  font-size: 12px;
  color: var(--text-tertiary);
  margin-top: 2px;
}

.setting-control {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  gap: 12px;
  flex-wrap: wrap;
}

.radio-group {
  display: flex;
  gap: 12px;
  flex-wrap: wrap;
}

.radio-label {
  display: flex;
  align-items: center;
  gap: 4px;
  font-size: 13px;
  color: var(--text-secondary);
  cursor: default;
}

.switch {
  width: 40px;
  height: 22px;
  background-color: var(--bg-switch-off);
  border-radius: 22px;
  position: relative;
  cursor: default;
  transition: 0.2s;
  flex-shrink: 0;
}

.switch.active {
  background-color: var(--accent);
}

.switch-knob {
  width: 18px;
  height: 18px;
  background-color: var(--text-inverse);
  border-radius: 50%;
  position: absolute;
  top: 2px;
  left: 2px;
  transition: 0.2s;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.2);
}

.switch.active .switch-knob {
  left: 20px;
}

.number-input {
  width: 90px;
  padding: 4px 8px;
  border: 1px solid var(--border-strong);
  border-radius: 6px;
  font-size: 13px;
  outline: none;
  background: var(--bg-card);
  color: var(--text-primary);
}

.number-input:focus {
  border-color: var(--accent);
}

.path-group {
  display: flex;
  gap: 6px;
  align-items: center;
  max-width: 360px;
}

.path-display {
  flex: 1;
  min-width: 0;
  overflow: auto;
  white-space: nowrap;
  padding: 6px 10px;
  border: 1px solid var(--border-strong);
  border-radius: 6px;
  font-size: 12px;
  background: var(--bg-input);
  color: var(--text-tertiary);
  cursor: text;
  user-select: text;
}

.path-display::-webkit-scrollbar {
  height: 4px;
}

.path-display::-webkit-scrollbar-thumb {
  background: var(--scrollbar-thumb);
  border-radius: 2px;
}

.btn-select {
  padding: 4px 12px;
  background: var(--bg-hover);
  border: 1px solid var(--border-strong);
  border-radius: 6px;
  font-size: 12px;
  cursor: default;
  transition: 0.2s;
  color: var(--text-secondary);
}

.btn-select:hover {
  background: var(--border-strong);
}

.slider {
  width: 150px;
  height: 4px;
  -webkit-appearance: none;
  background: var(--border-strong);
  border-radius: 2px;
  outline: none;
}

.slider::-webkit-slider-thumb {
  -webkit-appearance: none;
  width: 14px;
  height: 14px;
  background: var(--accent);
  border-radius: 50%;
  cursor: default;
}

.volume-value {
  font-size: 12px;
  color: var(--text-secondary);
  min-width: 35px;
}

.checkbox-label {
  display: flex;
  align-items: center;
  gap: 6px;
  cursor: default;
  color: var(--text-primary);
}

.checkbox-label input,
.radio-label input {
  accent-color: var(--accent);
  width: 14px;
  height: 14px;
  cursor: default;
}

.hotkey-recorder {
  padding: 6px 16px;
  border: 1px solid var(--border-strong);
  border-radius: 6px;
  font-size: 12px;
  font-family: monospace;
  background: var(--bg-input);
  min-width: 120px;
  text-align: center;
  color: var(--text-primary);
  cursor: pointer;
  transition: border-color 0.15s, box-shadow 0.15s;
  user-select: none;
}

.hotkey-recorder:hover {
  border-color: var(--accent);
}

.hotkey-recorder.recording {
  border-color: var(--accent);
  box-shadow: 0 0 0 3px var(--accent-transparent);
  animation: recorder-pulse 1s ease-in-out infinite;
}

@keyframes recorder-pulse {
  50% { box-shadow: 0 0 0 6px var(--accent-transparent); }
}

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

.microsoft-wrap {
  display: block;
}

.password-field {
  position: relative;
  display: inline-flex;
  align-items: center;
}

.password-field input[type="password"]::-ms-reveal,
.password-field input[type="password"]::-ms-clear {
  display: none;
}

.password-field input[type="password"]::-webkit-reveal {
  display: none;
}

.password-field .key-input {
  padding-right: 26px;
}

.eye-btn {
  position: absolute;
  right: 4px;
  display: flex;
  align-items: center;
  justify-content: center;
  width: 20px;
  height: 20px;
  border: none;
  background: transparent;
  color: var(--text-tertiary);
  cursor: pointer;
  padding: 0;
  border-radius: 4px;
  transition: color 0.15s;
}

.eye-btn:hover {
  color: var(--text-primary);
}

.key-input {
  width: 160px;
  padding: 4px 8px;
  border: 1px solid var(--border-strong);
  border-radius: 6px;
  font-size: 11px;
  background: var(--bg-card);
  color: var(--text-primary);
}

.key-input.input-error {
  border-color: #e74c3c;
}

.error-hint {
  flex-basis: 100%;
  order: 99;
  font-size: 11px;
  color: #e74c3c;
  line-height: 1.4;
}

.about-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
  min-height: 300px;
}

.about-logo {
  width: 60px;
  height: 60px;
  background: var(--accent);
  border-radius: 20px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 36px;
  font-weight: 700;
  color: var(--text-inverse);
  margin-bottom: 20px;
  box-shadow: 0 8px 20px rgba(45, 122, 255, 0.3);
}

.about-name {
  font-size: 24px;
  font-weight: 600;
  color: var(--text-primary);
  margin-bottom: 4px;
}

.about-subtitle {
  font-size: 12px;
  color: var(--text-tertiary);
  margin-bottom: 16px;
}

.about-version {
  font-size: 12px;
  color: var(--text-tertiary);
}

.about-copyright {
  margin-top: 24px;
  font-size: 11px;
  color: var(--scrollbar-thumb);
}

@media (max-width: 1000px) {
  .settings-container {
    margin: 16px 20px 20px 20px;
  }
  .settings-sidebar {
    width: 140px;
  }
  .settings-nav-item {
    padding: 6px 12px;
    font-size: 12px;
  }
  .settings-category {
    padding: 4px 12px;
    font-size: 10px;
  }
  .settings-content {
    padding: 16px;
  }
  .section-title {
    font-size: 16px;
    margin-bottom: 16px;
  }
  .key-input {
    width: 120px;
  }
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

.select-input {
  padding: 4px 8px;
  border: 1px solid var(--border-strong);
  border-radius: 6px;
  font-size: 13px;
  background: var(--bg-card);
  color: var(--text-primary);
  outline: none;
  cursor: default;
}

.select-input:focus {
  border-color: var(--accent);
}
</style>
