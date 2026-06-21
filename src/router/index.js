import { createRouter, createWebHashHistory } from 'vue-router'
import Translate from '../views/Translate.vue'
import Dictionary from '../views/Dictionary.vue'
import History from '../views/History.vue'
import Settings from '../views/Settings.vue'
import ScreenshotOverlay from '../views/ScreenshotOverlay.vue'

const routes = [
  { path: '/', redirect: '/translate' },
  { path: '/translate', name: 'translate', component: Translate },
  { path: '/dictionary', name: 'dictionary', component: Dictionary },
  { path: '/history', name: 'history', component: History },
  { path: '/settings', name: 'settings', component: Settings },
  { path: '/screenshot-overlay', name: 'screenshot-overlay', component: ScreenshotOverlay }
]

const router = createRouter({
  history: createWebHashHistory(),
  routes
})

export default router
