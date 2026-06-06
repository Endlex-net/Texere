import { readFileSync } from 'node:fs'
import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'

const tauriConf = JSON.parse(readFileSync('src-tauri/tauri.conf.json', 'utf-8'))
const APP_VERSION = tauriConf.version as string

// https://vite.dev/config/
export default defineConfig({
  plugins: [svelte()],
  define: {
    __APP_VERSION__: JSON.stringify(APP_VERSION)
  },
  build: {
    rollupOptions: {
      input: {
        main: 'index.html',
        settings: 'settings.html'
      }
    }
  }
})
