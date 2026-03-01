<script lang="ts">
  import WindowChrome from './WindowChrome.svelte';

  export let getEditorContent: () => string;
  export let setEditorContent: (content: string) => void;
  export let hasEditorContent: boolean = false;
  export let onOpenTemplates: () => void;
</script>

<div class="window-container">
  <WindowChrome
    {getEditorContent}
    {setEditorContent}
    {hasEditorContent}
    {onOpenTemplates}
  />
  <div class="content-area">
    <slot />
  </div>
  <div class="status-bar-area">
    <slot name="statusbar" />
  </div>
</div>

<style>
  .window-container {
    display: flex;
    flex-direction: column;
    height: 100vh;
    width: 100vw;
    background: var(--texere-bg, rgba(36, 36, 36, 0.95));
    border-radius: 12px;
    overflow: hidden;
    clip-path: inset(0 round 12px);
    box-shadow: 0 0 15px rgba(0, 0, 0, 0.3);
    color: var(--texere-text, rgba(255, 255, 255, 0.87));
  }

  .content-area {
    flex: 1;
    overflow: auto;
    position: relative;
  }

  .status-bar-area {
    flex-shrink: 0;
  }
</style>
