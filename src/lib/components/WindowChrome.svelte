<script lang="ts">
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import AIFormatButton from './AIFormatButton.svelte';

  export let getEditorContent: () => string;
  export let setEditorContent: (content: string) => void;
  export let hasEditorContent: boolean = false;
  export let onOpenTemplates: () => void;

  function closeWindow(event: MouseEvent) {
    event.stopPropagation();
    getCurrentWindow().close();
  }

  function startDrag(event: MouseEvent) {
    const target = event.target as HTMLElement;
    if (target.closest('.control-btn, .icon-btn, .format-button, .undo-button')) return;

    getCurrentWindow().startDragging().catch(() => {
      // noop
    });
  }

  function openTemplates(event: MouseEvent) {
    event.stopPropagation();
    onOpenTemplates();
  }
</script>

<div class="titlebar" role="toolbar" tabindex="0" on:mousedown={startDrag}>
  <div class="left-controls">
    <button class="control-btn close" on:click={closeWindow} aria-label="Close" type="button"></button>
  </div>
  
  <div class="title">Texere</div>
  
  <div class="right-controls">
    <button
      class="icon-btn"
      type="button"
      on:click={openTemplates}
      title="Templates"
      aria-label="Templates"
    >
      📄
    </button>
    <AIFormatButton
      getContent={getEditorContent}
      setContent={setEditorContent}
      hasContent={hasEditorContent}
    />
  </div>
</div>

<style>
  .titlebar {
    height: 24px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    background: var(--texere-surface, rgba(30, 30, 46, 0.8));
    border-bottom: 1px solid var(--texere-border, rgba(255, 255, 255, 0.1));
    padding: 0 10px;
    cursor: move;
    border-top-left-radius: 12px;
    border-top-right-radius: 12px;
  }

  .left-controls, .right-controls {
    display: flex;
    align-items: center;
    gap: 8px;
    width: auto;
  }

  .right-controls { justify-content: flex-end; }

  .icon-btn {
    border: 1px solid var(--texere-border, rgba(255, 255, 255, 0.2));
    background: var(--texere-surface-2, rgba(255, 255, 255, 0.06));
    color: var(--texere-accent, #1e3a8a);
    font-weight: 700;
    width: 24px;
    height: 24px;
    padding: 0;
    border-radius: 6px;
    font-size: 12px;
    line-height: 1;
    display: grid;
    place-items: center;
    -webkit-app-region: no-drag;
  }

  .icon-btn:hover {
    background: var(--texere-hover, rgba(255, 255, 255, 0.15));
    border-color: var(--texere-accent, #89b4fa);
    color: var(--texere-accent, #89b4fa);
  }

  .icon-btn:focus-visible {
    outline: 2px solid var(--texere-accent, #1e3a8a);
    outline-offset: 1px;
  }

  .control-btn {
    width: 10px;
    height: 10px;
    border-radius: 50%;
    border: none;
    cursor: pointer;
    padding: 0;
    margin: 0;
    transition: transform 0.15s ease;
  }

  .control-btn:hover { transform: scale(1.1); }
  .control-btn.close { background: #ff5f57; }
  .control-btn.close:hover { background: #ff453a; }

  .title {
    font-size: 12px;
    font-weight: 500;
    color: var(--texere-text, rgba(255, 255, 255, 0.9));
    pointer-events: none;
  }
</style>
