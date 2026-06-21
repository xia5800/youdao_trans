import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import { resolve } from 'path'

export default defineConfig({
  plugins: [vue()],
  clearScreen: false,
  build: {
    rollupOptions: {
      input: {
        main: resolve(__dirname, 'index.html'),
        screenshot: resolve(__dirname, 'screenshot.html'),
        popup: resolve(__dirname, 'popup.html'),
      }
    }
  },
  server: {
    port: 1420,
    strictPort: false,
    watch: {
      ignored: ['**/src-tauri/**']
    }
  }
})
