import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'

// https://vite.dev/config/
export default defineConfig({
  plugins: [svelte()],
  define: {
    __APP_VERSION__: JSON.stringify('0.1.0')
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
