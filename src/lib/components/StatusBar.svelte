<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { getCurrentWindow } from '@tauri-apps/api/window';

  export let vimMode = 'NORMAL';
  export let wordCount = 0;
  export let charCount = 0;
  export let vimEnabled = true;
  export let hint = ''; // ESC double-press hint

  const dispatch = createEventDispatcher<{
    toggleVim: void;
  }>();

  function startDrag() {
    getCurrentWindow().startDragging().catch(() => {
      // ignore
    });
  }

  function toggleVim() {
    dispatch('toggleVim');
  }

</script>

<div class="status-bar" role="toolbar" tabindex="-1" aria-label="Status bar" on:mousedown={startDrag}>
  <div class="left">
    <button
      type="button"
      class:vim-mode={vimEnabled}
      class:vim-disabled={!vimEnabled}
      class="vim-toggle"
      aria-pressed={vimEnabled}
      aria-label={vimEnabled ? 'Disable Vim mode for this window' : 'Enable Vim mode for this window'}
      on:mousedown|stopPropagation
      on:click|stopPropagation={toggleVim}
    >
      {vimEnabled ? vimMode : '-- VIM OFF --'}
    </button>
  </div>
  
  <div class="center">
    {#if hint}
      <span class="hint">{hint}</span>
    {:else}
      <span class="drag-hint">⋮⋮</span>
    {/if}
  </div>
  
  <div class="right">
    <span class="count">{wordCount} words · {charCount} chars</span>
  </div>
</div>

<style>
  .status-bar {
    height: 24px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 12px;
    background: var(--texere-statusbar-bg, rgba(0, 0, 0, 0.2));
    border-top: 1px solid var(--texere-border, rgba(255, 255, 255, 0.05));
    font-size: 11px;
    font-family: Menlo, Monaco, 'Courier New', monospace;
    -webkit-app-region: drag;
    cursor: move;
    border-bottom-left-radius: 12px;
    border-bottom-right-radius: 12px;
  }

  .left, .right, .center {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .center {
    flex: 1;
    justify-content: center;
  }

  .vim-mode {
    color: var(--texere-ok, #a6e3a1);
    font-weight: 600;
    min-width: 60px;
  }

  .vim-disabled {
    color: var(--texere-accent, #1e3a8a);
    font-style: italic;
    font-weight: 600;
    letter-spacing: 0.03em;
  }

  .vim-toggle {
    -webkit-app-region: no-drag;
    appearance: none;
    border: 0;
    padding: 0;
    background: transparent;
    font: inherit;
    cursor: pointer;
  }

  .vim-toggle:focus-visible {
    outline: 1px solid color-mix(in srgb, var(--texere-text, #0b1220) 35%, transparent);
    outline-offset: 4px;
    border-radius: 4px;
  }

  .count {
    color: var(--texere-text, #0b1220);
    opacity: 0.95;
    font-weight: 500;
  }

  .hint {
    color: var(--texere-warn, #f9e2af);
    font-style: italic;
    opacity: 0.95;
  }

  .drag-hint {
    color: var(--texere-muted, #64748b);
    font-size: 10px;
    letter-spacing: 1px;
    opacity: 0.8;
  }
</style>
