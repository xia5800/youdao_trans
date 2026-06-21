<template>
  <div class="history-container">
    <div class="history-toolbar">
      <div class="search-wrapper">
        <span class="search-icon">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
            <use href="/icons.svg#icon-search"></use>
          </svg>
        </span>
        <input type="text" class="search-input-history" v-model="searchQuery" @input="onSearch" placeholder="搜索翻译记录...">
      </div>
      <div class="filter-buttons">
        <button
          v-for="f in filters"
          :key="f.key"
          class="filter-btn"
          :class="{ active: currentFilter === f.key }"
          @click="setFilter(f.key)"
        >{{ f.label }}</button>
      </div>
    </div>

    <div class="batch-bar">
      <label class="select-all">
        <input type="checkbox" :checked="isAllSelected" :indeterminate.prop="isIndeterminate" @change="toggleSelectAll"> 全选
      </label>
      <div class="batch-actions">
        <span class="batch-btn" @click="batchDelete">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
            <use href="/icons.svg#icon-delete"></use>
          </svg>
          删除
        </span>
        <span class="batch-btn" @click="batchClear">清除全部</span>
      </div>
    </div>

    <div class="history-list">
      <div v-if="paginatedRecords.length === 0" class="empty-state">
        <div class="empty-icon">
          <svg width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.2" opacity="0.5">
            <rect x="2" y="6" width="20" height="14" rx="2" stroke="currentColor" />
            <polyline points="2 6 12 13 22 6" stroke="currentColor" />
            <line x1="8" y1="14" x2="5" y2="17" stroke="currentColor" stroke-linecap="round" />
            <line x1="16" y1="14" x2="19" y2="17" stroke="currentColor" stroke-linecap="round" />
          </svg>
        </div>
        <div class="empty-text">暂无翻译记录</div>
      </div>

      <div
        v-for="record in paginatedRecords"
        :key="record.id"
        class="history-item"
        :class="{ selected: selectedIds.has(record.id) }"
      >
        <div style="display: flex; align-items: flex-start;">
          <input type="checkbox" class="item-checkbox" :checked="selectedIds.has(record.id)" @change="toggleSelect(record.id)">
          <div style="flex: 1; min-width: 0;">
            <div class="item-header">
              <span class="lang-badge">{{ record.source_lang }} → {{ record.target_lang }}</span>
              <span class="time-badge">{{ formatTime(record.timestamp) }}</span>
              <div class="item-actions">
                <span class="action-btn" @click="toggleFavorite(record.id)">
                  <svg v-if="record.favorite" width="14" height="14" viewBox="0 0 24 24" fill="#f5a623" stroke="#f5a623" stroke-width="1.5">
                    <polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2" />
                  </svg>
                  <svg v-else width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                    <polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2" />
                  </svg>
                </span>
                <span class="action-btn delete" @click="deleteRecord(record.id)">
                  <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                    <use href="/icons.svg#icon-delete"></use>
                  </svg>
                </span>
              </div>
            </div>
            <div class="item-content">
              <div class="source-text clickable-text" :class="{ preview: !isExpanded(record.id) && needsTruncation(record.source_text) }" @click="copyText(record.source_text)">{{ isExpanded(record.id) ? record.source_text : truncateText(record.source_text) }}</div>
              <div class="arrow-icon">→</div>
              <div class="target-text clickable-text" :class="{ preview: !isExpanded(record.id) && needsTruncation(record.target_text) }" @click="copyText(record.target_text)">{{ isExpanded(record.id) ? record.target_text : truncateText(record.target_text) }}</div>
            </div>
            <div v-if="!isExpanded(record.id) && needsTruncation(record.source_text)" class="expand-btn" @click="expandItem(record.id)">
              <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <use href="/icons.svg#icon-plus"></use>
              </svg>
              展开全文
            </div>
            <div v-if="isExpanded(record.id)" class="expand-btn" @click="collapseItem(record.id)">
              <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <use href="/icons.svg#icon-minus"></use>
              </svg>
              收起
            </div>
          </div>
        </div>
      </div>
    </div>

    <div class="history-footer">
      <span>共 {{ filteredRecords.length }} 条记录</span>
      <div class="pagination" v-if="totalPages > 1">
        <span class="page-btn" :class="{ disabled: currentPage === 1 }" @click="goPage(currentPage - 1)">‹</span>
        <template v-for="p in visiblePages" :key="p">
          <span v-if="p === '...'" class="page-btn disabled">...</span>
          <span v-else class="page-btn" :class="{ active: p === currentPage }" @click="goPage(p)">{{ p }}</span>
        </template>
        <span class="page-btn" :class="{ disabled: currentPage === totalPages }" @click="goPage(currentPage + 1)">›</span>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, reactive, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useToast } from '../composables/useToast.js'

const { showToast } = useToast()

const records = ref([])
const loading = ref(true)

async function loadHistory() {
  loading.value = true
  try {
    const raw = await invoke('load_history')
    records.value = JSON.parse(raw)
  } catch (e) {
    console.warn('load history failed:', e)
    records.value = []
  } finally {
    loading.value = false
  }
}

onMounted(loadHistory)

const filters = [
  { key: 'all', label: '全部' },
  { key: 'today', label: '今日' },
  { key: 'week', label: '本周' },
  { key: 'month', label: '本月' },
  { key: 'favorite', label: '收藏' }
]

const currentFilter = ref('all')
const searchQuery = ref('')
const currentPage = ref(1)
const selectedIds = reactive(new Set())
const expandedIds = reactive(new Set())
const itemsPerPage = 6

function formatTime(ts) {
  const date = new Date(ts)
  const now = new Date()
  const today = new Date(now.getFullYear(), now.getMonth(), now.getDate())
  const yesterday = new Date(today.getTime() - 24 * 60 * 60 * 1000)

  if (date >= today) {
    return `今天 ${date.getHours().toString().padStart(2, '0')}:${date.getMinutes().toString().padStart(2, '0')}`
  } else if (date >= yesterday) {
    return `昨天 ${date.getHours().toString().padStart(2, '0')}:${date.getMinutes().toString().padStart(2, '0')}`
  } else {
    return `${date.getMonth() + 1}/${date.getDate()} ${date.getHours().toString().padStart(2, '0')}:${date.getMinutes().toString().padStart(2, '0')}`
  }
}

function truncateText(text, maxLength = 300) {
  if (text.length <= maxLength) return text
  return text.substring(0, maxLength) + '...'
}

function needsTruncation(text) {
  return text.length > 300
}

const todayTs = () => new Date(new Date().getFullYear(), new Date().getMonth(), new Date().getDate()).getTime()
const weekAgoTs = () => todayTs() - 7 * 24 * 60 * 60 * 1000
const monthAgoTs = () => todayTs() - 30 * 24 * 60 * 60 * 1000

const filteredRecords = computed(() => {
  let list = [...records.value]

  switch (currentFilter.value) {
    case 'today': list = list.filter(r => r.timestamp >= todayTs()); break
    case 'week': list = list.filter(r => r.timestamp >= weekAgoTs()); break
    case 'month': list = list.filter(r => r.timestamp >= monthAgoTs()); break
    case 'favorite': list = list.filter(r => r.favorite); break
  }

  if (searchQuery.value) {
    const q = searchQuery.value.toLowerCase()
    list = list.filter(r => r.source_text.toLowerCase().includes(q) || r.target_text.toLowerCase().includes(q))
  }
  return list
})

const totalPages = computed(() => Math.ceil(filteredRecords.value.length / itemsPerPage))

const paginatedRecords = computed(() => {
  const start = (currentPage.value - 1) * itemsPerPage
  return filteredRecords.value.slice(start, start + itemsPerPage)
})

const isAllSelected = computed(() => {
  return paginatedRecords.value.length > 0 && paginatedRecords.value.every(r => selectedIds.has(r.id))
})

const isIndeterminate = computed(() => {
  const some = paginatedRecords.value.some(r => selectedIds.has(r.id))
  return some && !isAllSelected.value
})

const visiblePages = computed(() => {
  const total = totalPages.value
  if (total <= 1) return []
  const pages = []
  const cp = currentPage.value
  for (let i = 1; i <= total; i++) {
    if (i === 1 || i === total || (i >= cp - 1 && i <= cp + 1)) {
      pages.push(i)
    } else if (i === cp - 2 || i === cp + 2) {
      pages.push('...')
    }
  }
  return pages
})

const isExpanded = (id) => expandedIds.has(id)
const expandItem = (id) => expandedIds.add(id)
const collapseItem = (id) => expandedIds.delete(id)

function setFilter(key) {
  currentFilter.value = key
  currentPage.value = 1
  selectedIds.clear()
}

function onSearch() {
  currentPage.value = 1
  selectedIds.clear()
}

function toggleSelect(id) {
  if (selectedIds.has(id)) selectedIds.delete(id)
  else selectedIds.add(id)
}

function toggleSelectAll(e) {
  if (e.target.checked) {
    paginatedRecords.value.forEach(r => selectedIds.add(r.id))
  } else {
    paginatedRecords.value.forEach(r => selectedIds.delete(r.id))
  }
}

function goPage(page) {
  if (page < 1 || page > totalPages.value) return
  currentPage.value = page
}

async function batchDelete() {
  if (selectedIds.size === 0) { alert('请先选择要删除的记录'); return }
  if (!confirm(`确定要删除选中的 ${selectedIds.size} 条记录吗？`)) return
  try {
    await invoke('delete_history_batch', { ids: [...selectedIds] })
    selectedIds.clear()
    currentPage.value = 1
    await loadHistory()
  } catch (e) {
    console.warn('batch delete failed:', e)
  }
}

async function batchClear() {
  const list = filteredRecords.value
  if (list.length === 0) { alert('没有可清除的记录'); return }
  if (!confirm(`确定要删除所有 ${list.length} 条记录吗？此操作不可恢复。`)) return
  try {
    const ids = list.map(r => r.id)
    await invoke('delete_history_batch', { ids })
    selectedIds.clear()
    currentPage.value = 1
    await loadHistory()
  } catch (e) {
    console.warn('batch clear failed:', e)
  }
}

async function toggleFavorite(id) {
  try {
    await invoke('toggle_history_favorite', { id })
    await loadHistory()
  } catch (e) {
    console.warn('toggle favorite failed:', e)
  }
}

async function deleteRecord(id) {
  if (!confirm('确定要删除这条记录吗？')) return
  try {
    await invoke('delete_history', { id })
    selectedIds.delete(id)
    await loadHistory()
  } catch (e) {
    console.warn('delete record failed:', e)
  }
}

function copyText(text) {
  if (!text) return
  navigator.clipboard.writeText(text)
  showToast('已复制')
}
</script>

<style scoped>
.history-container {
  flex: 1;
  margin: 12px;
  background: var(--bg-empty);
  border-radius: 12px;
  box-shadow: var(--shadow-card);
  overflow: hidden;
  display: flex;
  flex-direction: column;
  min-height: 0;
}

.history-toolbar {
  padding: 16px 24px;
  border-bottom: 1px solid var(--border);
  display: flex;
  align-items: center;
  justify-content: space-between;
  flex-wrap: wrap;
  gap: 12px;
  flex-shrink: 0;
}

.search-wrapper {
  flex: 1;
  min-width: 200px;
  position: relative;
}

.search-input-history {
  width: 100%;
  padding: 10px 16px 10px 36px;
  font-size: 14px;
  border: 1px solid var(--border-strong);
  border-radius: 10px;
  outline: none;
  background: var(--bg-card);
  transition: all 0.2s;
  color: var(--text-primary);
}

.search-input-history:focus {
  border-color: var(--accent);
  box-shadow: 0 0 0 2px rgba(45, 122, 255, 0.1);
}

.search-icon {
  position: absolute;
  left: 12px;
  top: 50%;
  transform: translateY(-50%);
  color: var(--text-tertiary);
  display: flex;
}

.filter-buttons {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
}

.filter-btn {
  padding: 6px 14px;
  font-size: 13px;
  border-radius: 20px;
  border: 1px solid var(--border-strong);
  background: var(--bg-card);
  color: var(--text-secondary);
  cursor: default;
  transition: all 0.2s;
}

.filter-btn.active {
  background: var(--accent);
  border-color: var(--accent);
  color: var(--text-inverse);
}

.filter-btn:hover:not(.active) {
  background: var(--bg-hover);
}

.batch-bar {
  padding: 10px 24px;
  background: var(--bg-card);
  border-bottom: 1px solid var(--border);
  display: flex;
  align-items: center;
  justify-content: space-between;
  flex-shrink: 0;
}

.select-all {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 13px;
  color: var(--text-secondary);
  cursor: default;
}

.batch-actions {
  display: flex;
  gap: 16px;
}

.batch-btn {
  font-size: 13px;
  color: var(--text-tertiary);
  cursor: default;
  transition: color 0.2s;
  display: inline-flex;
  align-items: center;
  gap: 4px;
}

.batch-btn:hover {
  color: var(--danger);
}

.history-list {
  flex: 1;
  overflow-y: auto;
  padding: 8px 0;
}

.history-item {
  padding: 16px 24px;
  border-bottom: 1px solid var(--border);
  transition: background 0.2s;
  cursor: default;
}

.history-item:hover {
  background: var(--bg-card);
}

.history-item.selected {
  background: var(--bg-active);
}

.item-checkbox {
  margin-right: 12px;
  flex-shrink: 0;
  margin-top: 2px;
  accent-color: var(--accent);
}

.select-all input[type="checkbox"] {
  accent-color: var(--accent);
}

.item-header {
  display: flex;
  align-items: center;
  margin-bottom: 12px;
  flex-wrap: wrap;
  gap: 8px;
}

.lang-badge {
  font-size: 12px;
  color: var(--accent);
  background: var(--bg-active);
  padding: 2px 10px;
  border-radius: 12px;
  flex-shrink: 0;
}

.time-badge {
  font-size: 12px;
  color: var(--text-tertiary);
  flex-shrink: 0;
}

.item-actions {
  margin-left: auto;
  display: flex;
  gap: 12px;
  opacity: 0;
  transition: opacity 0.2s;
  flex-shrink: 0;
}

.history-item:hover .item-actions {
  opacity: 1;
}

.action-btn {
  cursor: default;
  color: var(--text-tertiary);
  transition: color 0.2s;
  padding: 2px 4px;
  display: inline-flex;
  align-items: center;
}

.action-btn:hover {
  color: var(--accent);
}

.action-btn.delete:hover {
  color: var(--danger);
}

.item-content {
  display: flex;
  gap: 24px;
  min-width: 0;
}

.clickable-text {
  cursor: pointer;
  border-radius: 4px;
  transition: background 0.15s;
}

.clickable-text:hover {
  background: var(--bg-hover);
}

.source-text {
  flex: 1;
  font-size: 14px;
  color: var(--text-primary);
  line-height: 1.6;
  word-wrap: break-word;
  word-break: break-word;
  white-space: normal;
  overflow-wrap: break-word;
  max-height: none;
}

.target-text {
  flex: 1;
  font-size: 14px;
  color: var(--text-secondary);
  line-height: 1.6;
  word-wrap: break-word;
  word-break: break-word;
  white-space: normal;
  overflow-wrap: break-word;
  max-height: none;
}

.arrow-icon {
  color: var(--scrollbar-thumb);
  font-size: 16px;
  flex-shrink: 0;
  align-self: flex-start;
  padding-top: 2px;
}

.expand-btn {
  margin-top: 8px;
  font-size: 12px;
  color: var(--accent);
  cursor: default;
  display: inline-flex;
  align-items: center;
  gap: 4px;
}

.expand-btn:hover {
  text-decoration: underline;
}

.source-text.preview,
.target-text.preview {
  max-height: 4.8em;
  overflow: hidden;
  position: relative;
}

.source-text.preview::after,
.target-text.preview::after {
  content: '...';
  position: absolute;
  bottom: 0;
  right: 0;
  background: linear-gradient(to right, transparent, var(--bg-empty) 50%);
  padding-left: 20px;
}

.history-item:hover .source-text.preview::after,
.history-item:hover .target-text.preview::after {
  background: linear-gradient(to right, transparent, var(--bg-card) 50%);
}

.history-item.selected .source-text.preview::after,
.history-item.selected .target-text.preview::after {
  background: linear-gradient(to right, transparent, var(--bg-active) 50%);
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
  min-height: 300px;
  color: var(--text-tertiary);
}

.empty-icon {
  margin-bottom: 16px;
  display: flex;
}

.empty-text {
  font-size: 14px;
}

.history-footer {
  padding: 12px 24px;
  border-top: 1px solid var(--border);
  display: flex;
  align-items: center;
  justify-content: space-between;
  font-size: 12px;
  color: var(--text-tertiary);
  flex-shrink: 0;
  background: var(--bg-card);
}

.pagination {
  display: flex;
  gap: 8px;
}

.page-btn {
  padding: 4px 10px;
  border-radius: 6px;
  cursor: default;
  transition: all 0.2s;
  color: var(--text-secondary);
}

.page-btn:hover:not(.disabled) {
  background: var(--bg-hover);
}

.page-btn.active {
  background: var(--accent);
  color: var(--text-inverse);
}

.page-btn.disabled {
  opacity: 0.4;
  cursor: default;
}

.history-list::-webkit-scrollbar {
  width: 5px;
}

.history-list::-webkit-scrollbar-track {
  background: var(--scrollbar-track);
}

.history-list::-webkit-scrollbar-thumb {
  background: var(--scrollbar-thumb);
  border-radius: 4px;
}

@media (max-width: 1000px) {
  .history-container {
    margin: 16px 20px 20px 20px;
  }
  .history-toolbar {
    padding: 12px 16px;
  }
  .history-item {
    padding: 12px 16px;
  }
  .item-content {
    flex-direction: column;
    gap: 8px;
  }
  .arrow-icon {
    display: none;
  }
}
</style>
