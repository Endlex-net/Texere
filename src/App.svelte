<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { tick } from 'svelte';
  import { getCurrentWindow, LogicalSize } from '@tauri-apps/api/window';
  import { invoke } from '@tauri-apps/api/core';
  import WindowContainer from './lib/components/WindowContainer.svelte';
  import Editor from './lib/components/Editor.svelte';
  import StatusBar from './lib/components/StatusBar.svelte';
  import TemplateSelector from './lib/components/TemplateSelector.svelte';
  import { copyAndDismiss, dismissWithoutCopy } from './lib/clipboard';
  import { loadSettings } from './lib/settings';
  import { getTemplates } from './lib/templates';
  import { initAppearance } from './lib/appearance';
  import { saveNote, deleteNote } from './lib/notes';
  import type { Template, Note } from './lib/types';

  let editorComponent: Editor;
  let containerRef: WindowContainer;
  let vimMode = 'NORMAL';
  let wordCount = 0;
  let charCount = 0;
  let hint = '';
  let templates: Template[] = [];
  let showTemplateSelector = false;
  let persistedVimEnabled = true;
  let windowVimOverride: boolean | null = null;
  let vimEnabled = true;
  let softWrap = false;
  let collapsed = false;
  let expandedHeight = 360;
  let fatalError = '';

  // ── Note state ──────────────────────────────────────────────────────────────
  let noteId = '';
  let noteName = '';
  /** Debounce timer for auto-save */
  let autoSaveTimer: ReturnType<typeof setTimeout> | null = null;
  const AUTOSAVE_DEBOUNCE_MS = 500;

  // Double ESC to close
  let lastEscTime = 0;
  const ESC_DOUBLE_CLICK_DELAY = 1000;
  let escHintTimeout: ReturnType<typeof setTimeout> | null = null;

  async function refreshRuntimeSettings() {
    const settings = await loadSettings();
    initAppearance(settings.appearance);
    persistedVimEnabled = settings.vim.enabled;
    if (windowVimOverride === persistedVimEnabled) {
      windowVimOverride = null;
    }
    softWrap = settings.softWrap;
  }

  $: vimEnabled = windowVimOverride ?? persistedVimEnabled;

  async function refreshTemplates() {
    templates = await getTemplates();
  }

  async function loadInitialData() {
    await refreshRuntimeSettings();
    await refreshTemplates();
  }

  let unlistenRefresh: (() => void) | null = null;
  let unlistenForceFocus: (() => void) | null = null;
  let unlistenLoadNote: (() => void) | null = null;

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

      // Receive note data when this window is opened from the Tray for a named note
      unlistenLoadNote = await getCurrentWindow().listen<{ id: string; name: string; content: string }>(
        'texere://load-note',
        async (event) => {
          const { id, name, content } = event.payload;
          // Cancel any pending autosave from previous note
          if (autoSaveTimer) { clearTimeout(autoSaveTimer); autoSaveTimer = null; }
          noteId = id;
          noteName = name;
          await tick();
          editorComponent?.setValue(content);
          await focusEditor();
        }
      );

      window.dispatchEvent(new CustomEvent('texere://ready'));
    } catch (err) {
      fatalError = err instanceof Error ? err.message : String(err);
      window.dispatchEvent(new CustomEvent('texere://ready'));
    }
  });

  onDestroy(() => {
    unlistenRefresh?.();
    unlistenForceFocus?.();
    unlistenLoadNote?.();
    if (autoSaveTimer) clearTimeout(autoSaveTimer);
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

  // ── Auto-save on content change ─────────────────────────────────────────────
  function handleChange(event: CustomEvent<string>) {
    const content = event.detail;
    charCount = content.length;
    wordCount = content.trim() ? content.trim().split(/\s+/).length : 0;

    // Auto-save only for named notes
    if (noteId) {
      if (autoSaveTimer) clearTimeout(autoSaveTimer);
      autoSaveTimer = setTimeout(async () => {
        await saveNote({ id: noteId, name: noteName, content });
      }, AUTOSAVE_DEBOUNCE_MS);
    }
  }

  function handleVimModeChange(event: CustomEvent<string>) {
    const mode = event.detail;
    vimMode = mode === 'disabled' ? 'NORMAL' : mode.toUpperCase();
  }

  function handleToggleVim() {
    const nextVimEnabled = !vimEnabled;
    windowVimOverride = nextVimEnabled === persistedVimEnabled ? null : nextVimEnabled;
  }

  // ── Naming callback (from WindowChrome → WindowContainer → here) ────────────
  async function handleNoteNamed(name: string) {
    const content = editorComponent?.getValue() ?? '';
    const isNewNote = !noteId;
    try {
      // For new notes, pass the current window label so Rust can register the
      // note→window mapping atomically before rebuilding the tray (prevents C3 race).
      const label = isNewNote ? getCurrentWindow().label : undefined;
      const saved = await saveNote({ id: noteId, name, content }, label);
      noteId = saved.id;
      noteName = saved.name;
    } catch (err: unknown) {
      const msg = typeof err === 'string' ? err : String(err);
      if (msg.includes('DUPLICATE_NAME:')) {
        containerRef?.showDuplicateError();
      } else {
        // Non-duplicate save failure (disk full, serialization error, etc.)
        console.error('Failed to save note:', msg);
      }
    }
  }

  // ── Delete callback (from WindowChrome) ─────────────────────────────────────
  async function handleNoteDeleted() {
    if (!noteId) return;
    // Cancel any pending autosave BEFORE clearing noteId — otherwise the timer
    // fires after deletion with noteId='' and recreates the deleted note (B5).
    if (autoSaveTimer) { clearTimeout(autoSaveTimer); autoSaveTimer = null; }
    try {
      await deleteNote(noteId);
      noteId = '';
      noteName = '';
      // Content stays in editor for this session
    } catch (err) {
      console.error('Failed to delete note:', err);
    }
  }

  // ── Close handler ───────────────────────────────────────────────────────────
  async function handleClose() {
    // Cancel any pending autosave debounce
    if (autoSaveTimer) { clearTimeout(autoSaveTimer); autoSaveTimer = null; }

    if (noteId) {
      // Save current content synchronously before closing
      const content = editorComponent?.getValue() ?? '';
      try {
        await saveNote({ id: noteId, name: noteName, content });
      } catch (err) {
        console.error('Failed to save note before close:', err);
      }
    }

    await invoke('close_window');
  }

  // ── Keyboard shortcuts ───────────────────────────────────────────────────────
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
        // Save named note before dismissing
        if (noteId) {
          if (autoSaveTimer) { clearTimeout(autoSaveTimer); autoSaveTimer = null; }
          try { await saveNote({ id: noteId, name: noteName, content }); } catch (_) {}
        }
        const settings = await loadSettings();
        copyAndDismiss(content, settings.autoPaste);
      }
      return;
    }

    // Escape → double press to close
    if (event.key === 'Escape' && !showTemplateSelector) {
      event.preventDefault();
      const now = Date.now();
      if (now - lastEscTime < ESC_DOUBLE_CLICK_DELAY) {
        if (escHintTimeout) clearTimeout(escHintTimeout);
        // Save named note before dismissing
        if (noteId) {
          if (autoSaveTimer) { clearTimeout(autoSaveTimer); autoSaveTimer = null; }
          const content = editorComponent?.getValue() ?? '';
          try { await saveNote({ id: noteId, name: noteName, content }); } catch (_) {}
        }
        dismissWithoutCopy();
      } else {
        hint = 'Press ESC again to close';
        if (escHintTimeout) clearTimeout(escHintTimeout);
        escHintTimeout = setTimeout(() => { hint = ''; }, ESC_DOUBLE_CLICK_DELAY);
      }
      lastEscTime = now;
      return;
    }
  }

  function handleTemplateSelect(template: Template) {
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

  async function toggleCollapse() {
    if (collapsed) {
      collapsed = false;
      await getCurrentWindow().setSize(new LogicalSize(480, expandedHeight));
    } else {
      const size = await getCurrentWindow().innerSize();
      const scale = await getCurrentWindow().scaleFactor();
      expandedHeight = Math.round(size.height / scale);
      collapsed = true;
      await getCurrentWindow().setSize(new LogicalSize(480, 24));
    }
  }
</script>

<svelte:window on:keydown={handleKeydown} on:focus={handleWindowFocus} />

{#if fatalError}
  <div class="fatal-error">Failed to initialize editor: {fatalError}</div>
{/if}

<WindowContainer
  bind:this={containerRef}
  getEditorContent={() => editorComponent?.getValue() ?? ''}
  setEditorContent={(content) => editorComponent?.setValue(content)}
  hasEditorContent={wordCount > 0}
  onOpenTemplates={openTemplateSelector}
  {collapsed}
  onToggleCollapse={toggleCollapse}
  {noteId}
  {noteName}
  onNoteNamed={handleNoteNamed}
  onNoteDeleted={handleNoteDeleted}
  onClose={handleClose}
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
    <StatusBar {vimMode} {vimEnabled} {wordCount} {charCount} {hint} on:toggleVim={handleToggleVim} />
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
