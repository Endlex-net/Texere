<script lang="ts">
  import { onMount } from 'svelte';
  import {
    canEnableAutoPaste,
    DEFAULT_AI_MODEL,
    DEFAULT_AI_SYSTEM_PROMPT,
    DEFAULT_OPENAI_BASE_URL,
    loadSettings,
    saveSettings,
  } from '../settings';
  import { applyAppearance } from '../appearance';
  import type { TexereSettings, Mode, StyleId } from '../types';
  import HotkeyRecorder from './HotkeyRecorder.svelte';
  import TemplateManager from './TemplateManager.svelte';
  import { getTemplates, saveTemplate, deleteTemplate } from '../templates';
  import type { Template } from '../types';
  
  const appVersion = '0.1.0';

  let settings: TexereSettings = {
    hotkeys: { summon: '', copyAndDismiss: '' },
    vim: { enabled: true },
    softWrap: false,
    ai: {
      apiKey: '',
      model: DEFAULT_AI_MODEL,
      baseUrl: DEFAULT_OPENAI_BASE_URL,
      systemPrompt: DEFAULT_AI_SYSTEM_PROMPT,
    },
    appearance: { bgColor: '#1e1e2e', opacity: 0.95, mode: 'auto', style: 'tokyo-night' },
    autoPaste: false
  };

  let loaded = false;
  let saveTimeout: ReturnType<typeof setTimeout>;
  let saveStatus: 'idle' | 'saving' | 'saved' | 'error' = 'idle';
  let saveError = '';
  let settingsSnapshot = '';
  let templates: Template[] = [];
  let autoPastePermissionError = '';

  onMount(async () => {
    settings = await loadSettings();
    applyAppearance(settings.appearance);
    templates = await getTemplates();
    loaded = true;
  });

  async function handleTemplateCreate(event: CustomEvent<Template>) {
    const template = event.detail;
    try {
      saveStatus = 'saving';
      saveError = '';
      await saveTemplate(template);
      templates = await getTemplates();
      saveStatus = 'saved';
    } catch (err) {
      console.error('Failed to create template:', err);
      saveStatus = 'error';
      saveError = err instanceof Error ? err.message : String(err);
    }
  }

  async function handleTemplateUpdate(event: CustomEvent<Template>) {
    const template = event.detail;
    try {
      saveStatus = 'saving';
      saveError = '';
      await saveTemplate(template);
      templates = await getTemplates();
      saveStatus = 'saved';
    } catch (err) {
      console.error('Failed to update template:', err);
      saveStatus = 'error';
      saveError = err instanceof Error ? err.message : String(err);
    }
  }

  async function handleTemplateDeleteById(id: string) {
    try {
      saveStatus = 'saving';
      saveError = '';
      await deleteTemplate(id);
      templates = await getTemplates();
      saveStatus = 'saved';
    } catch (err) {
      console.error('Failed to delete template:', err);
      saveStatus = 'error';
      saveError = err instanceof Error ? err.message : String(err);
    }
  }

  // Auto-save logic
  function scheduleSave() {
    if (!loaded) return;
    clearTimeout(saveTimeout);
    saveTimeout = setTimeout(async () => {
      try {
        saveStatus = 'saving';
        saveError = '';
        await saveSettings(settings);
        saveStatus = 'saved';
      } catch (err) {
        console.error('Failed to save settings:', err);
        saveStatus = 'error';
        saveError = err instanceof Error ? err.message : String(err);
      }
    }, 500); // Debounce saves
  }

  async function handleAutoPasteToggle(event: Event) {
    const input = event.currentTarget as HTMLInputElement;
    const nextValue = input.checked;

    if (!nextValue) {
      settings.autoPaste = false;
      autoPastePermissionError = '';
      return;
    }

    const granted = await canEnableAutoPaste();
    if (granted) {
      settings.autoPaste = true;
      autoPastePermissionError = '';
      return;
    }

    settings.autoPaste = false;
    input.checked = false;
    saveStatus = 'error';
    saveError = 'Auto-paste requires Accessibility permission (System Settings > Privacy & Security > Accessibility).';
    autoPastePermissionError = saveError;
  }

  $: settingsSnapshot = loaded ? JSON.stringify(settings) : '';
  $: if (settingsSnapshot) scheduleSave();
  $: if (loaded) applyAppearance(settings.appearance);

</script>

<div class="settings-container">
  <h1>Settings</h1>
  {#if saveStatus !== 'idle'}
    <p class="save-status" class:error={saveStatus === 'error'}>
      {saveStatus === 'saving' ? 'Saving...' : saveStatus === 'saved' ? 'Saved' : `Save failed: ${saveError || 'unknown error'}`}
    </p>
  {/if}

  {#if loaded}
    <!-- Hotkeys Section -->
    <section>
      <h2>Hotkeys</h2>
      <div class="setting-row">
        <label for="summon-recorder">Summon Texere</label>
        <HotkeyRecorder id="summon-recorder" bind:value={settings.hotkeys.summon} />
      </div>
      <div class="setting-row">
        <label for="copy-recorder">Copy & Dismiss</label>
        <HotkeyRecorder id="copy-recorder" bind:value={settings.hotkeys.copyAndDismiss} />
      </div>
    </section>

    <!-- Editor Section -->
    <section>
      <h2>Editor</h2>
      <div class="setting-row">
        <label for="vim-toggle">Vim Mode</label>
        <label class="switch">
          <input id="vim-toggle" type="checkbox" bind:checked={settings.vim.enabled} />
          <span class="slider round"></span>
        </label>
      </div>
      <div class="setting-row">
        <label for="soft-wrap-toggle">Soft Wrap (display only)</label>
        <label class="switch">
          <input id="soft-wrap-toggle" type="checkbox" bind:checked={settings.softWrap} />
          <span class="slider round"></span>
        </label>
      </div>
    </section>

    <!-- Auto-paste Section -->
    <section>
      <h2>Auto-paste</h2>
      <div class="setting-row">
        <label for="autopaste-toggle">Paste on copy & dismiss</label>
        <label class="switch">
          <input
            id="autopaste-toggle"
            type="checkbox"
            checked={settings.autoPaste}
            on:change={handleAutoPasteToggle}
          />
          <span class="slider round"></span>
        </label>
      </div>
      {#if autoPastePermissionError}
        <p class="permission-error">{autoPastePermissionError}</p>
      {/if}
    </section>

    <!-- AI Section -->
    <section>
      <h2>AI Assistant</h2>
      <div class="setting-row">
        <label for="api-key">OpenAI API Key</label>
        <input id="api-key" type="password" placeholder="sk-..." bind:value={settings.ai.apiKey} />
      </div>
      <div class="setting-row">
        <label for="base-url">OpenAI Base URL</label>
        <input id="base-url" type="text" placeholder={DEFAULT_OPENAI_BASE_URL} bind:value={settings.ai.baseUrl} />
      </div>
      <div class="setting-row">
        <label for="model-input">Model</label>
        <input id="model-input" type="text" placeholder={DEFAULT_AI_MODEL} bind:value={settings.ai.model} />
      </div>
      <div class="setting-row prompt-row">
        <label for="system-prompt">System Prompt</label>
        <textarea id="system-prompt" rows="4" placeholder={DEFAULT_AI_SYSTEM_PROMPT} bind:value={settings.ai.systemPrompt}></textarea>
      </div>
    </section>

    <!-- Appearance Section -->
    <section>
      <h2>Appearance</h2>
      <div class="setting-row">
        <label for="mode-select">模式</label>
        <select id="mode-select" bind:value={settings.appearance.mode}>
          <option value="dark">深色</option>
          <option value="light">浅色</option>
          <option value="auto">自动</option>
        </select>
      </div>
      <div class="setting-row">
        <label for="style-select">风格</label>
        <select id="style-select" bind:value={settings.appearance.style}>
          <option value="tokyo-night">Tokyo Night</option>
          <option value="catppuccin">Catppuccin</option>
          <option value="nord">Nord</option>
          <option value="rose-pine">Rosé Pine</option>
        </select>
      </div>
    </section>

    <!-- Templates Section -->
    <section class="templates-section">
      <h2>Templates</h2>
      <TemplateManager
        {templates}
        onDelete={handleTemplateDeleteById}
        on:create={handleTemplateCreate}
        on:update={handleTemplateUpdate}
      />
    </section>

    <!-- About Section -->
    <section class="about-section">
      <h2>About Texere</h2>
      <p>Version {appVersion}</p>
      <p class="branding">Texere everything together</p>
    </section>
  {:else}
    <p>Loading settings...</p>
  {/if}
</div>

<style>
  .settings-container {
    max-width: 600px;
    margin: 0 auto;
    padding: 20px;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
    color: var(--texere-text, #cdd6f4);
    background-color: var(--texere-bg, rgba(30, 30, 46, 0.95));
    height: 100vh;
    overflow-y: auto;
    box-sizing: border-box;
  }

  h1 {
    font-size: 24px;
    margin-bottom: 30px;
    font-weight: 600;
  }

  .save-status {
    margin-top: -18px;
    margin-bottom: 20px;
    font-size: 12px;
    color: var(--texere-muted, #a6accd);
  }

  .save-status.error {
    color: #dc2626;
  }

  .permission-error {
    margin: -6px 0 0;
    font-size: 12px;
    color: #dc2626;
  }

  h2 {
    font-size: 16px;
    margin-bottom: 15px;
    color: var(--texere-accent, #a6accd);
    text-transform: uppercase;
    letter-spacing: 0.05em;
    border-bottom: 1px solid var(--texere-border, rgba(255,255,255,0.1));
    padding-bottom: 8px;
  }

  section {
    margin-bottom: 30px;
  }

  .setting-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 15px;
  }

  label {
    font-size: 14px;
  }

  input[type="password"], input[type="text"], select, textarea {
    background: var(--texere-surface, rgba(0,0,0,0.2));
    border: 1px solid var(--texere-border, rgba(255,255,255,0.1));
    color: var(--texere-text, #cdd6f4);
    padding: 6px 10px;
    border-radius: 4px;
    font-size: 14px;
    width: 200px;
  }

  textarea {
    resize: vertical;
    min-height: 72px;
  }

  .prompt-row {
    align-items: flex-start;
  }

  input[type="password"]:focus, input[type="text"]:focus, select:focus, textarea:focus {
    outline: none;
    border-color: var(--texere-accent, #89b4fa);
  }
  
  /* Toggle Switch */
  .switch {
    position: relative;
    display: inline-block;
    width: 44px;
    height: 24px;
  }

  .switch input { 
    opacity: 0;
    width: 0;
    height: 0;
  }

  .slider {
    position: absolute;
    cursor: pointer;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background-color: #ccc;
    transition: .4s;
  }

  .slider:before {
    position: absolute;
    content: "";
    height: 18px;
    width: 18px;
    left: 3px;
    bottom: 3px;
    background-color: white;
    transition: .4s;
  }

  input:checked + .slider {
    background-color: var(--texere-accent, #a6d189);
  }

  input:checked + .slider:before {
    transform: translateX(20px);
  }

  .slider.round {
    border-radius: 24px;
  }

  .slider.round:before {
    border-radius: 50%;
  }

  .about-section {
    text-align: center;
    padding: 20px 0;
    margin-top: 40px;
    border-top: 1px solid var(--texere-border, rgba(255,255,255,0.1));
  }

  .about-section p {
    margin: 5px 0;
    font-size: 14px;
    color: var(--texere-muted, #a6accd);
  }

  .branding {
    font-style: italic;
    opacity: 0.7;
  }
  
  .templates-section {
    background: var(--texere-surface-2, rgba(0,0,0,0.1));
    padding: 15px;
    border-radius: 8px;
  }
</style>
