import { createApp } from 'vue'
import SelectionPopupStandalone from './views/SelectionPopupStandalone.vue'
import { applySavedTheme } from './composables/useTheme.js'

applySavedTheme()

const app = createApp(SelectionPopupStandalone)
app.mount('#app')
