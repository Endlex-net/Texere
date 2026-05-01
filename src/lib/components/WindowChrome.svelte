<script lang="ts">
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import AIFormatButton from './AIFormatButton.svelte';

  // ── existing props ──────────────────────────────────────────────────────────
  export let getEditorContent: () => string;
  export let setEditorContent: (content: string) => void;
  export let hasEditorContent: boolean = false;
  export let onOpenTemplates: () => void;
  export let collapsed: boolean = false;
  export let onToggleCollapse: () => void = () => {};

  // ── note props ──────────────────────────────────────────────────────────────
  /** noteId of the note bound to this window (empty = temporary) */
  export let noteId: string = '';
  /** Display name of the bound note */
  export let noteName: string = '';
  /** Called when the user confirms a new/updated name via the inline editor */
  export let onNoteNamed: (name: string) => void = () => {};
  /** Called when the user confirms the delete-note action */
  export let onNoteDeleted: () => void = () => {};
  /** Called when the close button is clicked (replaces direct window.close) */
  export let onClose: () => void = () => getCurrentWindow().close();

  // ── title inline-edit state ─────────────────────────────────────────────────
  let editingTitle = false;
  let editValue = '';
  let titleInputEl: HTMLInputElement | null = null;
  let duplicateError = false;

  // ── 📌 delete-confirm state ─────────────────────────────────────────────────
  let confirmingDelete = false;

  // ── derived ─────────────────────────────────────────────────────────────────
  $: displayTitle = noteId ? noteName : 'Texere';
  $: isPinned = !!noteId;

  // ── close ───────────────────────────────────────────────────────────────────
  function closeWindow(event: MouseEvent) {
    event.stopPropagation();
    onClose();
  }

  // ── drag ────────────────────────────────────────────────────────────────────
  function startDrag(event: MouseEvent) {
    const target = event.target as HTMLElement;
    if (target.closest('.control-btn, .icon-btn, .title, .pin-btn, .confirm-delete')) return;
    getCurrentWindow().startDragging().catch(() => {});
  }

  // ── templates ───────────────────────────────────────────────────────────────
  function openTemplates(event: MouseEvent) {
    event.stopPropagation();
    onOpenTemplates();
  }

  // ── titlebar double-click: collapse (only if NOT on the title text) ─────────
  function handleDblClick(event: MouseEvent) {
    const target = event.target as HTMLElement;
    // Double-clicking the title text itself is handled separately (inline edit).
    // Double-clicking any button is a no-op.
    if (target.closest('.control-btn, .icon-btn, .title, .pin-btn, .confirm-delete')) return;
    onToggleCollapse();
  }

  // ── title double-click: enter inline edit ───────────────────────────────────
  async function handleTitleDblClick(event: MouseEvent) {
    event.stopPropagation(); // prevent titlebar collapse
    if (editingTitle) return;
    editValue = displayTitle === 'Texere' ? '' : displayTitle;
    duplicateError = false;
    editingTitle = true;
    // wait for the input to mount, then focus it
    await tick();
    titleInputEl?.focus();
    titleInputEl?.select();
  }

  function handleTitleKeydown(event: KeyboardEvent) {
    if (event.key === 'Enter') {
      event.preventDefault();
      commitTitleEdit();
    } else if (event.key === 'Escape') {
      event.preventDefault();
      cancelTitleEdit();
    }
  }

  function commitTitleEdit() {
    const trimmed = editValue.trim();
    if (!trimmed) return; // empty → stay in edit mode
    duplicateError = false;
    editingTitle = false;
    onNoteNamed(trimmed);
  }

  function cancelTitleEdit() {
    editingTitle = false;
    duplicateError = false;
  }

  function handleTitleBlur() {
    const trimmed = editValue.trim();
    if (!trimmed) {
      cancelTitleEdit();
    } else {
      commitTitleEdit();
    }
  }

  /** Called from App.svelte when save_note returns a duplicate-name error */
  export function showDuplicateError() {
    duplicateError = true;
    editingTitle = true;
    tick().then(() => titleInputEl?.focus());
  }

  // ── 📌 click: enter delete-confirm mode ────────────────────────────────────
  function handlePinClick(event: MouseEvent) {
    event.stopPropagation();
    if (!isPinned) return;
    confirmingDelete = true;
  }

  function confirmDelete(event: MouseEvent) {
    event.stopPropagation();
    confirmingDelete = false;
    onNoteDeleted();
  }

  function cancelDelete(event: MouseEvent) {
    event.stopPropagation();
    confirmingDelete = false;
  }

  // needed for async focus after editingTitle = true
  import { tick } from 'svelte';
</script>

<div
  class="titlebar"
  class:collapsed
  role="toolbar"
  tabindex="0"
  on:mousedown={startDrag}
  on:dblclick={handleDblClick}
>
  <!-- Left: close button -->
  <div class="left-controls">
    <button class="control-btn close" on:click={closeWindow} aria-label="Close" type="button"></button>
  </div>

  <!-- Centre: title text or inline editor or delete-confirm -->
  {#if confirmingDelete}
  <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
    <div class="confirm-delete" role="group" aria-label="Delete confirmation" on:mousedown|stopPropagation>
      <span class="confirm-label">Delete '{noteName}'?</span>
      <button class="confirm-btn cancel" type="button" on:click={cancelDelete} aria-label="Cancel">✕</button>
      <button class="confirm-btn ok" type="button" on:click={confirmDelete} aria-label="Confirm delete">✓</button>
    </div>
  {:else if editingTitle}
  <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
    <div class="title-edit-wrap" role="group" aria-label="Title editor" on:mousedown|stopPropagation>
      <input
        bind:this={titleInputEl}
        bind:value={editValue}
        class="title-input"
        class:error={duplicateError}
        type="text"
        placeholder="Note name…"
        maxlength="80"
        on:keydown={handleTitleKeydown}
        on:blur={handleTitleBlur}
      />
    </div>
  {:else}
    <!-- svelte-ignore a11y-no-static-element-interactions -->
    <div
      class="title"
      title={displayTitle}
      on:dblclick={handleTitleDblClick}
    >
      {displayTitle}
    </div>
  {/if}

  <!-- Right: 📌 (if pinned) + template + AI -->
  <div class="right-controls">
    {#if isPinned && !confirmingDelete && !editingTitle}
      <button
        class="icon-btn pin-btn"
        type="button"
        on:click={handlePinClick}
        title="Delete this note"
        aria-label="Delete note"
      >📌</button>
    {/if}
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

  /* ── title text ─────────────────────────────────────────────── */
  .title {
    font-size: 12px;
    font-weight: 500;
    color: var(--texere-text, rgba(255, 255, 255, 0.9));
    pointer-events: auto;
    cursor: default;
    max-width: 160px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    user-select: none;
  }

  /* ── inline title editor ────────────────────────────────────── */
  .title-edit-wrap {
    flex: 1;
    display: flex;
    justify-content: center;
    padding: 0 4px;
  }

  .title-input {
    font-size: 12px;
    font-weight: 500;
    color: var(--texere-text, rgba(255, 255, 255, 0.9));
    background: var(--texere-surface-2, rgba(255, 255, 255, 0.08));
    border: 1px solid var(--texere-accent, #89b4fa);
    border-radius: 4px;
    padding: 1px 6px;
    width: 150px;
    outline: none;
  }

  .title-input.error {
    border-color: #ff5f57;
    animation: shake 0.2s ease;
  }

  @keyframes shake {
    0%, 100% { transform: translateX(0); }
    25%       { transform: translateX(-4px); }
    75%       { transform: translateX(4px); }
  }

  /* ── 📌 delete confirm strip ────────────────────────────────── */
  .confirm-delete {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 4px;
    font-size: 11px;
    color: var(--texere-text, rgba(255, 255, 255, 0.85));
    overflow: hidden;
  }

  .confirm-label {
    max-width: 120px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    flex-shrink: 1;
  }

  .confirm-btn {
    flex-shrink: 0;
    border: none;
    background: var(--texere-surface-2, rgba(255, 255, 255, 0.06));
    color: var(--texere-text, rgba(255, 255, 255, 0.8));
    border-radius: 4px;
    width: 20px;
    height: 20px;
    font-size: 11px;
    padding: 0;
    display: grid;
    place-items: center;
    cursor: pointer;
    -webkit-app-region: no-drag;
  }

  .confirm-btn.ok:hover  { background: #ff5f57; color: #fff; }
  .confirm-btn.cancel:hover { background: var(--texere-hover, rgba(255,255,255,0.15)); }

  /* ── icon buttons ───────────────────────────────────────────── */
  .icon-btn {
    border: none;
    background: transparent;
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
    cursor: pointer;
    -webkit-app-region: no-drag;
    transition: background 0.15s ease;
  }

  .icon-btn:hover {
    background: rgba(255, 255, 255, 0.08);
    color: var(--texere-accent, #89b4fa);
  }

  .icon-btn:focus-visible {
    outline: 2px solid var(--texere-accent, #89b4fa);
    outline-offset: 1px;
  }

  /* ── traffic-light close button ─────────────────────────────── */
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

  .titlebar.collapsed {
    border-bottom-left-radius: 12px;
    border-bottom-right-radius: 12px;
  }
</style>
