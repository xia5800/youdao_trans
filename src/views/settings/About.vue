<template>
  <div class="about-container">
    <div class="about-logo">Y</div>
    <div class="about-name">优道翻译</div>
    <div class="about-subtitle">不止翻译，更是词典</div>
    <div class="about-version">版本 v{{ version }}</div>
    <div class="about-copyright">© 2026 GCC. All rights reserved.</div>
  </div>
</template>

<script setup>
import { version } from '../../../package.json'
</script>

<style scoped>
.about-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
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
</style>
