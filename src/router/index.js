import { createRouter, createWebHashHistory } from 'vue-router'
import Translate from '../views/Translate.vue'
import Dictionary from '../views/Dictionary.vue'
import History from '../views/History.vue'
import ScreenshotOverlay from '../views/ScreenshotOverlay.vue'

const routes = [
  { path: '/', redirect: '/translate' },
  { path: '/translate', name: 'translate', component: Translate },
  { path: '/dictionary', name: 'dictionary', component: Dictionary },
  { path: '/history', name: 'history', component: History },
  {
    path: '/settings',
    component: () => import('../views/settings/index.vue'),
    children: [
      { path: '', redirect: '/settings/preference' },
      { path: 'preference', name: 'settings-preference', component: () => import('../views/settings/Preference.vue') },
      { path: 'advanced', name: 'settings-advanced', component: () => import('../views/settings/Advanced.vue') },
      { path: 'hotkey', name: 'settings-hotkey', component: () => import('../views/settings/Hotkey.vue') },
      { path: 'translator', name: 'settings-translator', component: () => import('../views/settings/Translator.vue') },
      { path: 'about', name: 'settings-about', component: () => import('../views/settings/About.vue') },
    ]
  },
  { path: '/screenshot-overlay', name: 'screenshot-overlay', component: ScreenshotOverlay }
]

const router = createRouter({
  history: createWebHashHistory(),
  routes
})

export default router
