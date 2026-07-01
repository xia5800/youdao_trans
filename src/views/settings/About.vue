<template>
  <div class="about-container">
    <div class="about-hero">
      <div class="about-logo">Y</div>
      <div class="about-name">优道翻译</div>
      <div class="about-subtitle">不止翻译，更是词典</div>
      <div class="about-version">版本 v{{ version }}</div>
    </div>

    <div class="about-section">
      <div class="section-line"></div>
      <div class="section-label">核心功能</div>
      <div class="section-line"></div>
    </div>

    <div class="feature-grid">
      <div class="feature-card" v-for="f in features" :key="f.title">
        <div class="feature-icon">
          <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <use :href="`/icons.svg#icon-${f.icon}`"></use>
          </svg>
        </div>
        <div class="feature-title">{{ f.title }}</div>
        <div class="feature-desc">{{ f.desc }}</div>
      </div>
    </div>

    <div class="about-section">
      <div class="section-line"></div>
      <div class="section-label">技术栈</div>
      <div class="section-line"></div>
    </div>

    <div class="tech-row">
      <span class="tech-badge" v-for="t in techList" :key="t">{{ t }}</span>
      <span class="tech-badge tech-etc">...</span>
    </div>

    <div class="about-copyright">Made with <span class="copyright-heart">♥</span> by GCC · 2026</div>
  </div>
</template>

<script setup>
import { version } from '../../../package.json'

const features = [
  { icon: 'layers', title: '多引擎翻译', desc: '聚合百度、有道、微软、OpenAI 等九大翻译引擎，自由切换' },
  { icon: 'image', title: '截图 OCR', desc: '支持百度云、讯飞、腾讯云 OCR 及本地 PaddleOCR 模型' },
  { icon: 'search', title: '划词翻译', desc: '鼠标选中即译，悬浮窗展示结果，一键复制朗读' },
  { icon: 'book', title: '词典查询', desc: '输入即查，展示音标、释义、例句，支持收藏生词' },
  { icon: 'activity', title: '语音朗读', desc: 'TTS 朗读翻译结果，支持语速调节和自动发音' },
  { icon: 'clipboard', title: '历史记录', desc: '自动保存翻译记录，支持搜索、收藏、批量管理' }
]

const techList = ['Tauri 2', 'Rust', 'Vue 3', 'Vite', 'SQLite']
</script>

<style scoped>
.about-container {
  position: relative;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
  overflow: hidden;
  padding: 20px 24px;
  gap: 16px;
}

/* Hero */
.about-hero {
  display: flex;
  flex-direction: column;
  align-items: center;
  position: relative;
  z-index: 1;
}

.about-logo {
  width: 64px;
  height: 64px;
  background: linear-gradient(135deg, var(--accent), var(--accent-hover));
  border-radius: 18px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 34px;
  font-weight: 700;
  color: var(--text-inverse);
  margin-bottom: 16px;
  box-shadow: 0 8px 24px rgba(45, 122, 255, 0.35);
}

.about-name {
  font-size: 22px;
  font-weight: 600;
  color: var(--text-primary);
  margin-bottom: 4px;
}

.about-subtitle {
  font-size: 12px;
  color: var(--text-tertiary);
  margin-bottom: 8px;
}

.about-version {
  font-size: 14px;
  font-weight: 500;
  color: var(--text-secondary);
  background: var(--bg-hover);
  padding: 4px 14px;
  border-radius: 10px;
  letter-spacing: 0.3px;
}

/* Section divider */
.about-section {
  display: flex;
  align-items: center;
  gap: 12px;
  position: relative;
  z-index: 1;
  width: 100%;
  max-width: 440px;
}

.section-line {
  flex: 1;
  height: 1px;
  background: linear-gradient(to right, transparent, var(--border-strong), transparent);
}

.section-label {
  font-size: 11px;
  font-weight: 600;
  color: var(--text-tertiary);
  text-transform: uppercase;
  letter-spacing: 1.5px;
  white-space: nowrap;
}

/* Feature grid */
.feature-grid {
  display: grid;
  grid-template-columns: 1fr 1fr 1fr;
  gap: 10px;
  width: 100%;
  max-width: 520px;
  position: relative;
  z-index: 1;
}

.feature-card {
  display: flex;
  flex-direction: column;
  gap: 6px;
  padding: 14px;
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: 10px;
  transition: all 0.2s;
}

.feature-card:hover {
  border-color: var(--accent-transparent);
  box-shadow: 0 4px 16px rgba(45, 122, 255, 0.08);
}

.feature-icon {
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 8px;
  background: var(--accent-transparent);
  color: var(--accent);
}

.feature-title {
  font-size: 13px;
  font-weight: 600;
  color: var(--text-primary);
}

.feature-desc {
  font-size: 11px;
  color: var(--text-tertiary);
  line-height: 1.4;
}

/* Tech stack */
.tech-row {
  display: flex;
  flex-wrap: wrap;
  justify-content: center;
  gap: 6px;
  max-width: 440px;
  position: relative;
  z-index: 1;
}

.tech-badge {
  font-size: 10px;
  padding: 3px 10px;
  border-radius: 10px;
  background: var(--bg-hover);
  color: var(--text-secondary);
  border: 1px solid var(--border);
  transition: all 0.2s;
}

.tech-badge:hover {
  background: var(--accent-transparent);
  color: var(--accent);
  border-color: var(--accent-transparent);
}

/* Copyright */
.about-copyright {
  font-size: 12px;
  color: var(--text-tertiary);
  position: relative;
  z-index: 1;
  padding-top: 4px;
  letter-spacing: 0.5px;
}

.copyright-heart {
  color: #e74c3c;
  display: inline-block;
  animation: heartbeat 1.4s ease-in-out infinite;
}

@keyframes heartbeat {
  0%, 100% { transform: scale(1); }
  25% { transform: scale(1.2); }
  35% { transform: scale(1); }
  45% { transform: scale(1.15); }
  55% { transform: scale(1); }
}
</style>
