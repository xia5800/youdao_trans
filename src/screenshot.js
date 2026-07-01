import { createApp } from 'vue'
import ScreenshotOverlay from './views/ScreenshotOverlay.vue'
import { applySavedTheme } from './composables/useTheme.js'

applySavedTheme()

const app = createApp(ScreenshotOverlay)
app.mount('#app')
