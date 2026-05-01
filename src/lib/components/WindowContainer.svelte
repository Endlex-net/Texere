<script lang="ts">
  import WindowChrome from './WindowChrome.svelte';

  export let getEditorContent: () => string;
  export let setEditorContent: (content: string) => void;
  export let hasEditorContent: boolean = false;
  export let onOpenTemplates: () => void;
  export let collapsed: boolean = false;
  export let onToggleCollapse: () => void = () => {};

  // Note props passed through to WindowChrome
  export let noteId: string = '';
  export let noteName: string = '';
  export let onNoteNamed: (name: string) => void = () => {};
  export let onNoteDeleted: () => void = () => {};
  export let onClose: () => void;

  let chromeRef: WindowChrome;

  export function showDuplicateError() {
    chromeRef?.showDuplicateError();
  }
</script>

<div class="window-container" class:collapsed>
  <WindowChrome
    bind:this={chromeRef}
    {getEditorContent}
    {setEditorContent}
    {hasEditorContent}
    {onOpenTemplates}
    {collapsed}
    {onToggleCollapse}
    {noteId}
    {noteName}
    {onNoteNamed}
    {onNoteDeleted}
    {onClose}
  />
  <div class="content-area" class:hidden={collapsed}>
    <slot />
  </div>
  <div class="status-bar-area" class:hidden={collapsed}>
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

  .window-container.collapsed {
    height: 24px;
    box-shadow: none;
  }

  .hidden {
    display: none;
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
