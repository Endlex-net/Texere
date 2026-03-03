<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { tick } from 'svelte';
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import WindowContainer from './lib/components/WindowContainer.svelte';
  import Editor from './lib/components/Editor.svelte';
  import StatusBar from './lib/components/StatusBar.svelte';
  import TemplateSelector from './lib/components/TemplateSelector.svelte';
  import { copyAndDismiss, dismissWithoutCopy } from './lib/clipboard';
  import { loadSettings } from './lib/settings';
  import { getTemplates } from './lib/templates';
  import { initAppearance } from './lib/appearance';
  import type { Template } from './lib/types';

  let editorComponent: Editor;
  let vimMode = 'NORMAL';
  let wordCount = 0;
  let charCount = 0;
  let hint = ''; // For showing ESC double-press hint
  let templates: Template[] = [];
  let showTemplateSelector = false;
  let vimEnabled = true;
  let softWrap = false;
  let fatalError = '';
  
  // Double ESC to close
  let lastEscTime = 0;
  const ESC_DOUBLE_CLICK_DELAY = 1000; // 1 second
  let escHintTimeout: ReturnType<typeof setTimeout> | null = null;

  // Load templates and initialize appearance on mount
  async function refreshRuntimeSettings() {
    const settings = await loadSettings();
    initAppearance(settings.appearance);
    vimEnabled = settings.vim.enabled;
    softWrap = settings.softWrap;
  }

  async function refreshTemplates() {
    templates = await getTemplates();
  }

  async function loadInitialData() {
    await refreshRuntimeSettings();
    await refreshTemplates();
  }

  let unlistenRefresh: (() => void) | null = null;
  let unlistenForceFocus: (() => void) | null = null;

  onMount(async () => {
    try {
      await loadInitialData();

      unlistenRefresh = await getCurrentWindow().listen('texere://refresh-settings', async () => {
        await refreshRuntimeSettings();
        await refreshTemplates();
        await focusEditor();
      });

      unlistenForceFocus = await getCurrentWindow().listen('texere://force-focus', async () => {
        await focusEditor();
      });

      window.dispatchEvent(new CustomEvent('texere://ready'));
    } catch (err) {
      fatalError = err instanceof Error ? err.message : String(err);
      window.dispatchEvent(new CustomEvent('texere://ready'));
    }
  });

  onDestroy(() => {
    unlistenRefresh?.();
    unlistenForceFocus?.();
  });

  async function focusEditor() {
    await tick();
    editorComponent?.focus?.();
  }

  async function handleWindowFocus() {
    await refreshRuntimeSettings();
    await refreshTemplates();
    await focusEditor();
  }

  function handleChange(event: CustomEvent<string>) {
    const content = event.detail;
    charCount = content.length;
    wordCount = content.trim() ? content.trim().split(/\s+/).length : 0;
  }
  function handleVimModeChange(event: CustomEvent<string>) {
    const mode = event.detail;
    if (mode === 'disabled') {
      vimMode = '-- VIM OFF --';
    } else {
      vimMode = mode.toUpperCase();
    }
  }


  async function handleKeydown(event: KeyboardEvent) {
    // Cmd+T → open template selector
    if (event.key === 't' && (event.metaKey || event.ctrlKey)) {
      event.preventDefault();
      showTemplateSelector = true;
      return;
    }

    // Cmd+Enter → copy + dismiss
    if (event.key === 'Enter' && (event.metaKey || event.ctrlKey)) {
      event.preventDefault();
      const content = editorComponent?.getValue() ?? '';
      if (content) {
        const settings = await loadSettings();
        copyAndDismiss(content, settings.autoPaste);
      }
      return;
    }

    // Escape → double press to close (but not when template selector is open)
    if (event.key === 'Escape' && !showTemplateSelector) {
      event.preventDefault();
      
      const now = Date.now();
      if (now - lastEscTime < ESC_DOUBLE_CLICK_DELAY) {
        // Second ESC press - close window
        if (escHintTimeout) clearTimeout(escHintTimeout);
        dismissWithoutCopy();
      } else {
        // First ESC press - show hint
        hint = 'Press ESC again to close';
        
        // Clear hint after delay
        if (escHintTimeout) clearTimeout(escHintTimeout);
        escHintTimeout = setTimeout(() => {
          hint = '';
        }, ESC_DOUBLE_CLICK_DELAY);
      }
      lastEscTime = now;
      return;
    }
  }

  function handleTemplateSelect(template: Template) {
    // Insert template content into editor
    if (editorComponent) {
      const currentContent = editorComponent.getValue();
      const newContent = currentContent ? currentContent + '\n' + template.content : template.content;
      editorComponent.setValue(newContent);
    }
    showTemplateSelector = false;
  }

  function closeTemplateSelector() {
    showTemplateSelector = false;
  }

  async function openTemplateSelector() {
    await refreshTemplates();
    showTemplateSelector = true;
  }
</script>

<svelte:window on:keydown={handleKeydown} on:focus={handleWindowFocus} />

{#if fatalError}
  <div class="fatal-error">Failed to initialize editor: {fatalError}</div>
{/if}

<WindowContainer
  getEditorContent={() => editorComponent?.getValue() ?? ''}
  setEditorContent={(content) => editorComponent?.setValue(content)}
  hasEditorContent={wordCount > 0}
  onOpenTemplates={openTemplateSelector}
>
  <div class="editor-wrapper">
    <Editor 
      bind:this={editorComponent}
      {vimEnabled}
      {softWrap}
      on:change={handleChange}
      on:vimModeChange={handleVimModeChange}
    />
  </div>
  
  <div slot="statusbar">
    <StatusBar {vimMode} {wordCount} {charCount} {hint} />
  </div>
</WindowContainer>

{#if showTemplateSelector}
  <TemplateSelector
    {templates}
    onSelect={handleTemplateSelect}
    onClose={closeTemplateSelector}
  />
{/if}

<style>
  :global(body) { margin: 0; padding: 0; overflow: hidden; }
  .editor-wrapper { height: 100%; width: 100%; }
  .fatal-error {
    position: fixed;
    top: 12px;
    left: 12px;
    right: 12px;
    z-index: 9999;
    padding: 10px 12px;
    border-radius: 8px;
    background: #3b1018;
    color: #ffb4c4;
    font-size: 12px;
  }
</style>
