import { readFileSync } from 'node:fs'
import { defineConfig } from 'vitest/config'
import { svelte } from '@sveltejs/vite-plugin-svelte'
import { svelteTesting } from '@testing-library/svelte/vite'

const tauriConf = JSON.parse(readFileSync('src-tauri/tauri.conf.json', 'utf-8'))
const APP_VERSION = tauriConf.version as string

export default defineConfig({
  plugins: [svelte(), svelteTesting()],
  define: {
    __APP_VERSION__: JSON.stringify(APP_VERSION)
  },
  test: {
    environment: 'jsdom',
    globals: true,
    setupFiles: ['./vitest.setup.ts'],
    coverage: {
      provider: 'v8',
      reporter: ['text', 'html'],
      reportsDirectory: './coverage',
      exclude: [
        '**/*.test.ts',
        '**/*.e2e.test.ts',
        'src/main.ts',
        'vite.config.ts',
        'vitest.config.ts',
        'vitest.setup.ts',
      ],
    },
  },
})
