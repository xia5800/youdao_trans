<template>
  <div class="dictionary-container">
    <div class="search-bar">
      <div class="search-input-wrapper">
         <input type="text" class="search-input" v-model="searchWord" placeholder="搜索单词..."
          @input="onInput" @keydown.down.prevent="suggestIndex = Math.min(suggestIndex + 1, suggestions.length - 1)"
          @keydown.up.prevent="suggestIndex = Math.max(suggestIndex - 1, 0)"
          @keydown.enter.prevent="onEnter" @keydown.escape="suggestions = []"
          @focus="onFocus" @blur="onBlur">
        <svg v-if="searchWord" class="clear-icon" viewBox="0 0 20 20" fill="none" @click="clearSearch">
          <circle cx="10" cy="10" r="8" stroke="currentColor" stroke-width="1.5"/>
          <path d="M7 7l6 6M13 7l-6 6" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
        </svg>
        <ul v-if="suggestions.length" class="suggest-dropdown">
          <li v-for="(w, i) in suggestions" :key="w"
            :class="{ active: i === suggestIndex }"
            @mousedown.prevent="selectSuggestion(w)"
            @mouseenter="suggestIndex = i">{{ w }}</li>
        </ul>
      </div>
      <button class="search-btn" @click="doSearch">查词</button>
    </div>

    <div class="dictionary-content">
      <div class="definition-area">
        <div v-if="loading" class="status-msg">查询中...</div>
        <div v-else-if="errorMsg" class="status-msg error">{{ errorMsg }}</div>
        <div v-else-if="notFound" class="empty-state" style="position:absolute;inset:0;display:flex;flex-direction:column;align-items:center;justify-content:center;gap:12px;animation:fadeIn 0.4s ease">
          <svg class="empty-icon" viewBox="0 0 64 64" fill="none">
            <circle cx="32" cy="28" r="14" stroke="currentColor" stroke-width="2"/>
            <path d="M40 40l10 10" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
            <text x="32" y="33" text-anchor="middle" font-size="18" font-weight="700" fill="currentColor">?</text>
          </svg>
          <div class="empty-text">未找到该单词</div>
          <div class="empty-hint">请检查拼写或尝试其他关键词</div>
        </div>
        <div v-else-if="result" class="word-detail">
          <div class="word-header">
            <div class="word-title">{{ result.word }}</div>
            <div class="word-phonetic">
              <span>{{ result.phonetic }}</span>
              <span v-if="result.phonetic" class="pronounce-btn" @click="pronounce">
                <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                  <use href="/icons.svg#icon-speaker"></use>
                </svg>
                发音
              </span>
            </div>
            <div class="word-tags">
              <span v-for="t in result.tags" :key="t" class="tag-badge">{{ tagLabel(t) }}</span>
              <span v-if="result.collins > 0" class="tag-badge collins">柯林斯 {{ result.collins }}★</span>
              <span v-if="result.oxford > 0" class="tag-badge oxford">牛津</span>
            </div>
          </div>

          <div v-for="(m, ci) in result.meanings" :key="ci" class="meaning-card">
            <div v-if="m.pos" class="meaning-header">
              <span class="part-of-speech">{{ m.pos }}<span v-if="m.pct" class="pos-pct">&nbsp;{{ m.pct }}%</span></span>
            </div>
            <div class="meaning-cn-lines">
              <div v-for="(part, ti) in splitTranslation(m.translation)" :key="ti" class="meaning-cn">{{ part }}</div>
            </div>
          </div>

          <div v-if="result.definitions && result.definitions.length" class="definitions-section">
            <div class="section-title">英文释义</div>
            <div v-for="(def, di) in result.definitions" :key="di" class="definition-line">{{ def }}</div>
          </div>

          <div v-if="result.exchange && result.exchange.length" class="exchange-section">
            <div class="section-title">词形变化</div>
            <div class="exchange-grid">
              <div v-for="ex in result.exchange" :key="ex.label" class="exchange-item">
                <span class="exchange-label">{{ ex.label }}</span>
                <span class="exchange-value">{{ ex.value }}</span>
              </div>
            </div>
          </div>
        </div>
        <div v-else class="empty-state" style="position:absolute;inset:0;display:flex;flex-direction:column;align-items:center;justify-content:center;gap:12px;animation:fadeIn 0.4s ease">
          <svg class="empty-icon" viewBox="0 0 64 64" fill="none">
            <circle cx="28" cy="28" r="16" stroke="currentColor" stroke-width="2"/>
            <path d="M40 40l10 10" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
            <path d="M22 28h12M28 22v12" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
          </svg>
          <div class="empty-text">搜索单词开始查询</div>
        </div>
      </div>

      <div v-if="result || loading || errorMsg" class="sidebar-area">
        <div v-if="result" class="info-card">
          <div class="info-card-title">词频信息</div>
          <div class="freq-row"><span class="freq-label">BNC 词频</span><span>{{ result.bnc ?? '无' }}</span></div>
          <div class="freq-row"><span class="freq-label">COCA 词频</span><span>{{ result.frq ?? '无' }}</span></div>
        </div>
        <div v-if="result && hasDetail" class="info-card">
          <div class="info-card-title">详细信息</div>
          <pre class="detail-text">{{ result.detail }}</pre>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, reactive, computed, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useTts } from '../composables/useTts.js'

const searchWord = ref('')
const suggestions = ref([])
const suggestIndex = ref(-1)
const result = ref(null)
const loading = ref(false)
const notFound = ref(false)
const errorMsg = ref('')

const hasDetail = computed(() => {
  const d = result.value?.detail
  return d && d !== '' && d !== '""'
})

let debounceTimer = null

onMounted(() => {
  searchWord.value = 'abandon'
  doSearch()
})

async function onInput() {
  clearTimeout(debounceTimer)
  const w = searchWord.value.trim()
  if (w.length < 1) { suggestions.value = []; result.value = null; notFound.value = false; errorMsg.value = ''; return }
  debounceTimer = setTimeout(async () => {
    try {
      suggestions.value = await invoke('dict_suggestions', { prefix: w, limit: 10 })
      suggestIndex.value = -1
    } catch { suggestions.value = [] }
  }, 200)
}

function onFocus() {
  if (searchWord.value.trim().length >= 1) onInput()
}

function onBlur() {
  setTimeout(() => { suggestions.value = []; }, 200)
}

function clearSearch() {
  searchWord.value = ''
  suggestions.value = []
  result.value = null
  notFound.value = false
  errorMsg.value = ''
}

function splitTranslation(text) {
  return text.split(/[；;]/).map(s => s.trim()).filter(Boolean)
}

function selectSuggestion(word) {
  searchWord.value = word
  suggestions.value = []
  doSearch()
}

function onEnter() {
  if (suggestIndex.value >= 0 && suggestions.value[suggestIndex.value]) {
    selectSuggestion(suggestions.value[suggestIndex.value])
  } else {
    doSearch()
  }
}

async function doSearch() {
  const w = searchWord.value.trim()
  suggestions.value = []
  result.value = null
  notFound.value = false
  errorMsg.value = ''
  if (!w) return
  loading.value = true
  try {
    const data = await invoke('dict_lookup', { word: w })
    if (data) {
      result.value = data
    } else {
      notFound.value = true
    }
  } catch (e) {
    errorMsg.value = String(e)
  }
  loading.value = false
}

const TAG_MAP = {
  zk: '中考', gk: '高考', ky: '考研',
  cet4: '四级', cet6: '六级',
  toefl: '托福', gre: 'GRE', ielts: '雅思', sat: 'SAT',
  bec: '商务英语', aptis: '艾普思',
  tem4: '专四', tem8: '专八',
  toeic: '托业', pte: 'PTE',
}

function tagLabel(t) {
  return TAG_MAP[t] || t
}

function pronounce() {
  const { speak } = useTts()
  if (result.value?.word) speak(result.value.word, 'en')
}
</script>

<style scoped>
.dictionary-container {
  flex: 1;
  margin: 12px;
  background: var(--bg-card);
  border-radius: 12px;
  box-shadow: var(--shadow-card);
  overflow: hidden;
  display: flex;
  flex-direction: column;
  min-height: 0;
}

.search-bar {
  padding: 20px 28px;
  border-bottom: 1px solid var(--border);
  display: flex;
  gap: 12px;
  align-items: center;
  flex-shrink: 0;
  flex-wrap: wrap;
}

.search-input-wrapper {
  flex: 1;
  position: relative;
  min-width: 150px;
}

.search-input {
  width: 100%;
  padding: 10px 36px 10px 16px;
  font-size: 14px;
  border: 1px solid var(--border-strong);
  border-radius: 48px;
  outline: none;
  background: var(--bg-input);
  transition: all 0.2s;
  color: var(--text-primary);
}

.search-input:focus {
  border-color: var(--accent);
  background: var(--bg-card);
}

.clear-icon {
  position: absolute;
  right: 10px;
  top: 50%;
  transform: translateY(-50%);
  width: 18px;
  height: 18px;
  color: var(--text-tertiary);
  cursor: default;
  z-index: 10;
}

.clear-icon:hover {
  color: var(--text-secondary);
}

.suggest-dropdown {
  position: absolute;
  top: 100%;
  left: 0;
  right: 0;
  margin-top: 4px;
  background: var(--bg-card);
  border: 1px solid var(--border-strong);
  border-radius: 10px;
  list-style: none;
  padding: 4px 0;
  max-height: 260px;
  overflow-y: auto;
  z-index: 100;
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.15);
}

.suggest-dropdown li {
  padding: 8px 16px;
  font-size: 14px;
  color: var(--text-primary);
  cursor: default;
}

.suggest-dropdown li.active,
.suggest-dropdown li:hover {
  background: var(--bg-hover);
  color: var(--accent);
}

.search-btn {
  padding: 8px 20px;
  background: var(--accent);
  color: var(--text-inverse);
  border: none;
  border-radius: 48px;
  font-size: 14px;
  font-weight: 500;
  cursor: default;
  transition: 0.2s;
  white-space: nowrap;
}

.search-btn:hover {
  background: var(--accent-hover);
}

.dictionary-content {
  flex: 1;
  display: flex;
  overflow: hidden;
  min-height: 0;
  position: relative;
}

.definition-area {
  flex: 1.2;
  padding: 20px 24px;
  overflow-y: auto;
  border-right: 1px solid var(--border);
  min-width: 200px;
}

.empty-icon {
  width: 48px;
  height: 48px;
  color: var(--text-tertiary);
  opacity: 0.5;
}

.empty-text {
  font-size: 15px;
  color: var(--text-secondary);
  font-weight: 500;
}

.empty-hint {
  font-size: 12px;
  color: var(--text-tertiary);
}

.status-msg {
  text-align: center;
  color: var(--text-tertiary);
  font-size: 14px;
  padding: 40px 0;
}
.status-msg.error {
  color: #e74c3c;
}

.word-header {
  margin-bottom: 20px;
}

.word-title {
  font-size: 28px;
  font-weight: 700;
  color: var(--text-primary);
  margin-bottom: 6px;
  user-select: text;
}

.word-phonetic {
  display: flex;
  gap: 12px;
  color: var(--text-tertiary);
  font-size: 13px;
  flex-wrap: wrap;
  align-items: center;
  margin-bottom: 10px;
}

.pronounce-btn {
  color: var(--accent);
  cursor: default;
}

.word-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
  margin-top: 4px;
}

.tag-badge {
  font-size: 11px;
  padding: 2px 8px;
  border-radius: 6px;
  background: var(--bg-hover);
  color: var(--text-secondary);
  border: 1px solid var(--border-strong);
}

.tag-badge.collins { color: #e67e22; border-color: #e67e22; }
.tag-badge.oxford { color: var(--accent); border-color: var(--accent); }

.meaning-card {
  background: var(--bg-hover);
  border-radius: 20px;
  padding: 16px;
  margin-bottom: 12px;
}

.meaning-header {
  margin-bottom: 8px;
}

.part-of-speech {
  font-size: 12px;
  font-weight: 600;
  color: var(--accent);
  background: color-mix(in srgb, var(--accent) 12%, transparent);
  display: inline-block;
  padding: 2px 10px;
  border-radius: 24px;
}

.pos-pct {
  font-size: 10px;
  color: var(--text-tertiary);
  font-weight: 400;
}

.meaning-cn {
  font-size: 15px;
  color: var(--text-primary);
  line-height: 1.6;
  user-select: text;
}

.definitions-section {
  margin-top: 16px;
}

.definition-line {
  font-size: 13px;
  color: var(--text-secondary);
  padding: 6px 12px;
  border-left: 3px solid var(--border-strong);
  margin-bottom: 6px;
  line-height: 1.5;
  user-select: text;
}

.exchange-section {
  margin-top: 16px;
}

.section-title {
  font-size: 15px;
  font-weight: 600;
  color: var(--text-primary);
  margin-bottom: 10px;
  padding-left: 8px;
  border-left: 3px solid var(--accent);
}

.exchange-grid {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}

.exchange-item {
  background: var(--bg-hover);
  border-radius: 8px;
  padding: 6px 12px;
  font-size: 13px;
  display: flex;
  gap: 6px;
}

.exchange-label {
  color: var(--text-tertiary);
}

.exchange-value {
  color: var(--text-primary);
  font-weight: 500;
}

.sidebar-area {
  flex: 0.8;
  padding: 20px 24px;
  overflow-y: auto;
  background: var(--bg-card);
  min-width: 180px;
}

.info-card {
  background: var(--bg-hover);
  border-radius: 20px;
  padding: 16px;
  margin-bottom: 20px;
}

.info-card-title {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
  margin-bottom: 12px;
  display: flex;
  align-items: center;
  gap: 6px;
}

.freq-row {
  display: flex;
  justify-content: space-between;
  font-size: 13px;
  padding: 4px 0;
  color: var(--text-primary);
}

.freq-label {
  color: var(--text-tertiary);
}

.detail-text {
  white-space: pre-wrap;
  font-size: 12px;
  color: var(--text-secondary);
  font-family: monospace;
  line-height: 1.4;
}

@media (max-width: 1000px) {
  .dictionary-container {
    margin: 16px 20px 20px 20px;
  }
  .search-bar {
    padding: 16px 20px;
  }
  .definition-area {
    padding: 16px;
  }
  .sidebar-area {
    padding: 16px;
  }
  .word-title {
    font-size: 24px;
  }
  .meaning-card {
    padding: 12px;
  }
}
</style>