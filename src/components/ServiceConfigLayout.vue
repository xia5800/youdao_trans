<template>
  <div class="translator-page">
    <div class="translator-layout">
      <div class="translator-list">
        <slot name="list" />
      </div>
      <div class="translator-detail">
        <slot name="detail" />
      </div>
    </div>
  </div>
</template>

<style scoped>
.translator-page {
  flex: 1;
  min-height: 0;
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
  overflow-y: auto;
}

.translator-detail {
  flex: 1;
  padding: 20px 24px;
  background: var(--bg-card);
  border-radius: 0 8px 8px 0;
  overflow-y: auto;
}
</style>
