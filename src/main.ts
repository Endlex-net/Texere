import { mount } from 'svelte'
import './app.css'
import App from './App.svelte'

const probe = document.getElementById('boot-probe')

window.addEventListener('error', (event) => {
  if (probe) {
    probe.textContent = `Runtime error: ${event.message}`
    probe.style.color = '#ff6b6b'
  }
})

window.addEventListener('unhandledrejection', (event) => {
  if (probe) {
    const reason = event.reason instanceof Error ? event.reason.message : String(event.reason)
    probe.textContent = `Unhandled rejection: ${reason}`
    probe.style.color = '#ff6b6b'
  }
})

const app = mount(App, {
  target: document.getElementById('app')!,
})

window.addEventListener('texere://ready', () => {
  if (probe) {
    probe.remove()
  }
}, { once: true })

setTimeout(() => {
  if (probe && probe.isConnected) {
    probe.textContent = 'Texere is still loading...'
  }
}, 3000)

export default app
