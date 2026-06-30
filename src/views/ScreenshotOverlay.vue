<template>
  <div
    class="screenshot-overlay"
    :class="{ ready, selecting }"
    ref="overlayRef"
    tabindex="0"
    @mousedown="onMouseDown"
    @mousemove="onMouseMove"
    @mouseup="onMouseUp"
    @contextmenu.prevent="cancel"
    @keydown.escape="cancel"
    @keydown.enter="confirm"
  >
    <canvas ref="canvasRef" class="screenshot-canvas"></canvas>

    <div class="dim-mask" v-show="ready"></div>

    <div v-if="!selected && showCrosshair" class="crosshair-h" v-show="ready" :style="{ top: mouseY + 'px' }"></div>
    <div v-if="!selected && showCrosshair" class="crosshair-v" v-show="ready" :style="{ left: mouseX + 'px' }"></div>

    <div
      v-if="selecting || selected"
      class="selection-rect"
      :class="{ draggable: selected && !isDragging }"
      :style="selectionStyle"
      @mousedown.stop="onSelectionMouseDown"
    >
      <div v-if="selected" class="resize-handle nw" @mousedown.stop="onResizeStart($event, 'nw')"></div>
      <div v-if="selected" class="resize-handle n" @mousedown.stop="onResizeStart($event, 'n')"></div>
      <div v-if="selected" class="resize-handle ne" @mousedown.stop="onResizeStart($event, 'ne')"></div>
      <div v-if="selected" class="resize-handle e" @mousedown.stop="onResizeStart($event, 'e')"></div>
      <div v-if="selected" class="resize-handle se" @mousedown.stop="onResizeStart($event, 'se')"></div>
      <div v-if="selected" class="resize-handle s" @mousedown.stop="onResizeStart($event, 's')"></div>
      <div v-if="selected" class="resize-handle sw" @mousedown.stop="onResizeStart($event, 'sw')"></div>
      <div v-if="selected" class="resize-handle w" @mousedown.stop="onResizeStart($event, 'w')"></div>
    </div>

    <div v-if="selecting || selected" class="size-tip" v-show="ready" :style="sizeTipStyle">
      {{ rectW }} x {{ rectH }}
    </div>

    <div v-if="!selecting && !selected" class="hint" v-show="ready">按住鼠标左键框选区域 · ESC 取消</div>

    <div v-if="selected" class="toolbar" v-show="ready" :style="toolbarStyle" @mousedown.stop>
      <div class="toolbar-btns">
        <div class="toolbar-btn" @click="cancel">取消</div>
        <div class="toolbar-btn primary" @click="confirm">确定</div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, nextTick, onMounted } from 'vue'
import { invoke, convertFileSrc } from '@tauri-apps/api/core'
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow'
import { listen, emit } from '@tauri-apps/api/event'

const appWindow = getCurrentWebviewWindow()

const overlayRef = ref(null)
const canvasRef = ref(null)

const ready = ref(false)
const dpr = ref(1)
const showCrosshair = ref(true)

const mouseX = ref(0)
const mouseY = ref(0)

const selecting = ref(false)
const selected = ref(false)
const startX = ref(0)
const startY = ref(0)
const endX = ref(0)
const endY = ref(0)

const isDragging = ref(false)
const dragOffset = ref({ x: 0, y: 0 })

const isResizing = ref(false)
const resizeDir = ref('')
const resizeStart = ref({ mx: 0, my: 0, sx: 0, sy: 0, ex: 0, ey: 0 })

let canvasWidth = 0
let canvasHeight = 0

function rect() {
  let x1 = startX.value, y1 = startY.value
  let x2 = endX.value,   y2 = endY.value
  if (x1 > x2) [x1, x2] = [x2, x1]
  if (y1 > y2) [y1, y2] = [y2, y1]
  return { x: x1, y: y1, w: x2 - x1, h: y2 - y1 }
}

const rectW = computed(() => rect().w)
const rectH = computed(() => rect().h)

const selectionStyle = computed(() => {
  if (!selecting.value && !selected.value) return { display: 'none' }
  const r = rect()
  return {
    left: r.x + 'px', top: r.y + 'px',
    width: r.w + 'px', height: r.h + 'px',
    cursor: selected.value && !isDragging.value ? 'move' : 'default',
  }
})

const sizeTipStyle = computed(() => {
  const r = rect()
  const tipH = 24, margin = 6
  let top = r.y - tipH - margin
  let left = r.x + margin
  if (top < 0) top = r.y + margin
  if (left + 90 > window.innerWidth) left = window.innerWidth - 90 - margin
  return { top: top + 'px', left: left + 'px' }
})

const toolbarStyle = computed(() => {
  const r = rect()
  const tbH = 42, margin = 8
  let top = r.y + r.h + margin
  if (top + tbH > window.innerHeight - 10) top = r.y - tbH - margin
  return {
    top: top + 'px',
    right: Math.max(8, window.innerWidth - r.x - r.w) + 'px',
  }
})

function onMouseDown(e) {
  if (e.button !== 0 || selected.value) return
  startX.value = e.clientX
  startY.value = e.clientY
  endX.value   = e.clientX
  endY.value   = e.clientY
  selecting.value = true
  selected.value  = false
}

function onMouseMove(e) {
  mouseX.value = e.clientX
  mouseY.value = e.clientY
  if (selecting.value) {
    endX.value = e.clientX
    endY.value = e.clientY
  }
  if (isDragging.value) {
    const r = rect()
    const dx = e.clientX - dragOffset.value.x
    const dy = e.clientY - dragOffset.value.y
    let nx = r.x + dx, ny = r.y + dy
    if (nx < 0) nx = 0
    if (ny < 0) ny = 0
    if (nx + r.w > window.innerWidth) nx = window.innerWidth - r.w
    if (ny + r.h > window.innerHeight) ny = window.innerHeight - r.h
    startX.value = nx; startY.value = ny
    endX.value = nx + r.w; endY.value = ny + r.h
    dragOffset.value = { x: e.clientX, y: e.clientY }
  }
  if (isResizing.value) {
    const dx = e.clientX - resizeStart.value.mx
    const dy = e.clientY - resizeStart.value.my
    let sx = resizeStart.value.sx, sy = resizeStart.value.sy
    let ex = resizeStart.value.ex, ey = resizeStart.value.ey
    if (resizeDir.value.includes('n')) sy += dy
    if (resizeDir.value.includes('s')) ey += dy
    if (resizeDir.value.includes('w')) sx += dx
    if (resizeDir.value.includes('e')) ex += dx
    if (ex - sx < 20) { if (resizeDir.value.includes('w')) sx = ex - 20; else ex = sx + 20 }
    if (ey - sy < 20) { if (resizeDir.value.includes('n')) sy = ey - 20; else ey = sy + 20 }
    if (sx < 0) { ex -= sx; sx = 0 }
    if (sy < 0) { ey -= sy; sy = 0 }
    if (ex > window.innerWidth) { sx -= ex - window.innerWidth; ex = window.innerWidth }
    if (ey > window.innerHeight) { sy -= ey - window.innerHeight; ey = window.innerHeight }
    startX.value = sx; startY.value = sy
    endX.value = ex; endY.value = ey
  }
}

function onMouseUp() {
  if (isResizing.value) { isResizing.value = false; return }
  if (isDragging.value) { isDragging.value = false; return }
  if (!selecting.value) return
  selecting.value = false
  const r = rect()
  if (r.w < 5 || r.h < 5) {
    startX.value = 0; startY.value = 0; endX.value = 0; endY.value = 0
    return
  }
  selected.value = true
}

function onSelectionMouseDown(e) {
  if (e.button !== 0) return
  isDragging.value = true
  dragOffset.value = { x: e.clientX, y: e.clientY }
}

function onResizeStart(e, dir) {
  if (e.button !== 0) return
  isResizing.value = true
  resizeDir.value = dir
  resizeStart.value = {
    mx: e.clientX, my: e.clientY,
    sx: startX.value, sy: startY.value,
    ex: endX.value, ey: endY.value,
  }
}

function resetSelection() {
  selecting.value = false; selected.value = false; isDragging.value = false
  startX.value = 0; startY.value = 0; endX.value = 0; endY.value = 0
}

function clearCanvas() {
  const canvas = canvasRef.value
  if (canvas) {
    const ctx = canvas.getContext('2d')
    if (ctx) ctx.clearRect(0, 0, canvas.width, canvas.height)
  }
  ready.value = false
}

async function confirm() {
  const r = rect()
  if (r.w < 1 || r.h < 1) return

  const canvas = document.createElement('canvas')
  canvas.width = r.w
  canvas.height = r.h
  const ctx = canvas.getContext('2d')
  const srcCanvas = canvasRef.value
  if (!srcCanvas || !ctx) return

  const scale = dpr.value
  ctx.drawImage(srcCanvas, r.x * scale, r.y * scale, r.w * scale, r.h * scale, 0, 0, r.w, r.h)

  try {
    const base64 = canvas.toDataURL('image/png').split(',')[1]
    await invoke('finish_ocr_screenshot', { base64Img: base64 })
  } catch (e) {
    console.error('confirm failed:', e)
  }
  clearCanvas()
  resetSelection()
  await nextTick()
  await new Promise(r => requestAnimationFrame(r))
  await invoke('cleanup_screenshot')
  await appWindow.hide()
}

async function cancel() {
  clearCanvas()
  resetSelection()
  await nextTick()
  await new Promise(r => requestAnimationFrame(r))
  await invoke('cleanup_screenshot')
  await appWindow.hide()
}

function drawScreenshot(imgWidth, imgHeight, pixels) {
  ready.value = false
  resetSelection()
  const canvas = canvasRef.value
  if (!canvas) return

  canvasWidth = imgWidth
  canvasHeight = imgHeight
  canvas.width = canvasWidth
  canvas.height = canvasHeight

  dpr.value = canvasWidth / window.innerWidth

  const ctx = canvas.getContext('2d')
  ctx.clearRect(0, 0, canvas.width, canvas.height)

  const clamped = new Uint8ClampedArray(pixels)
  const imageData = new ImageData(clamped, imgWidth, imgHeight)
  ctx.putImageData(imageData, 0, 0)
  ready.value = true
  overlayRef.value?.focus()
}

onMounted(async () => {
  try {
    const raw = await invoke('load_config', { path: null })
    const data = JSON.parse(raw)
    if (data.showScreenshotCrosshair === false) {
      showCrosshair.value = false
    }
  } catch (e) {
    console.warn('读取设置失败:', e)
  }

  await listen('capture-screen', async (event) => {
    const payload = event.payload
    if (payload.error) {
      console.error('截图失败:', payload.error)
      await appWindow.close()
      return
    }
    if (!payload.path) return

    const url = convertFileSrc(payload.path)
    const response = await fetch(url)
    const buffer = await response.arrayBuffer()

    const view = new DataView(buffer)
    const imgWidth = view.getUint32(0, true)
    const imgHeight = view.getUint32(4, true)
    const pixels = new Uint8Array(buffer, 8)

    drawScreenshot(imgWidth, imgHeight, pixels)

    await nextTick()
    await new Promise(r => requestAnimationFrame(r))
    await appWindow.show()
  })

  await emit('screenshot-ready')
})
</script>

<style>
* { margin: 0; padding: 0; box-sizing: border-box; }
html, body, #app {
  width: 100%; height: 100%;
  overflow: hidden;
  background: transparent !important;
  user-select: none;
}
</style>

<style scoped>
.screenshot-overlay {
  position: relative;
  width: 100vw;
  height: 100vh;
  cursor: default;
  outline: none;
  background: transparent;
}

.screenshot-overlay.ready { cursor: crosshair; }
.screenshot-overlay.ready.selecting { cursor: crosshair; }

.screenshot-canvas {
  display: block;
  width: 100%;
  height: 100%;
  object-fit: contain;
  background: transparent;
}

.dim-mask {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.45);
  pointer-events: none;
  z-index: 1;
}

.crosshair-h,
.crosshair-v {
  position: fixed;
  pointer-events: none;
  z-index: 2;
  background: rgba(255, 255, 255, 0.6);
}
.crosshair-h { left: 0; right: 0; height: 1px; transform: translateY(-0.5px); }
.crosshair-v { top: 0; bottom: 0; width: 1px; transform: translateX(-0.5px); }

.selection-rect {
  position: fixed;
  border: 1.5px solid var(--accent);
  z-index: 5;
  pointer-events: auto;
}

.resize-handle {
  position: absolute;
  width: 7px;
  height: 7px;
  background: #fff;
  border: 2px solid var(--accent);
  z-index: 6;
}
.resize-handle.nw { top: -4px; left: -4px; cursor: nw-resize; }
.resize-handle.n  { top: -4px; left: 50%; margin-left: -3.5px; cursor: n-resize; }
.resize-handle.ne { top: -4px; right: -4px; cursor: ne-resize; }
.resize-handle.e  { right: -4px; top: 50%; margin-top: -3.5px; cursor: e-resize; }
.resize-handle.se { bottom: -4px; right: -4px; cursor: se-resize; }
.resize-handle.s  { bottom: -4px; left: 50%; margin-left: -3.5px; cursor: s-resize; }
.resize-handle.sw { bottom: -4px; left: -4px; cursor: sw-resize; }
.resize-handle.w  { left: -4px; top: 50%; margin-top: -3.5px; cursor: w-resize; }

.size-tip {
  position: fixed;
  z-index: 10;
  background: rgba(0,0,0,0.65);
  color: #fff;
  font-size: 12px;
  font-family: monospace;
  padding: 3px 8px;
  border-radius: 4px;
  pointer-events: none;
  white-space: nowrap;
}

.hint {
  position: fixed;
  bottom: 36px;
  left: 50%;
  transform: translateX(-50%);
  color: rgba(255,255,255,0.9);
  font-size: 13px;
  background: rgba(0,0,0,0.55);
  padding: 7px 20px;
  border-radius: 8px;
  pointer-events: none;
  z-index: 10;
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
  letter-spacing: 0.02em;
}

.toolbar {
  position: fixed;
  display: flex;
  align-items: center;
  gap: 8px;
  background: rgba(28,28,28,0.92);
  backdrop-filter: blur(8px);
  padding: 6px 10px;
  border-radius: 8px;
  z-index: 20;
  box-shadow: 0 4px 20px rgba(0,0,0,0.4);
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
}

.toolbar-btns { display: flex; gap: 6px; }

.toolbar-btn {
  padding: 5px 14px;
  border-radius: 5px;
  font-size: 13px;
  cursor: pointer;
  color: rgba(255,255,255,0.85);
  background: rgba(255,255,255,0.1);
  transition: background 0.12s;
}
.toolbar-btn:hover { background: rgba(255,255,255,0.2); }
.toolbar-btn.primary { background: #2d7aff; color: #fff; }
.toolbar-btn.primary:hover { background: #1a6aff; }
</style>
