import { mount } from 'svelte'
import SettingsPanel from './lib/components/SettingsPanel.svelte'

document.body.style.margin = '0';
document.body.style.padding = '0';

const app = mount(SettingsPanel, {
  target: document.getElementById('settings-app')!,
})

export default app
