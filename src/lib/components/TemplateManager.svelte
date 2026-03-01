<script lang="ts">
  import { tick } from 'svelte';
  import type { Template } from '../types';
  import { createEventDispatcher } from 'svelte';

  export let templates: Template[] = [];
  export let onDelete: ((id: string) => void | Promise<void>) | undefined = undefined;

  const dispatch = createEventDispatcher();

  let editingId: string | null = null;
  let editingName = '';
  let editingContent = '';
  let isCreating = false;
  let newName = '';
  let newContent = '';
  let newNameInput: HTMLInputElement | null = null;
  let deleteCandidate: Template | null = null;
  let deleteCancelButton: HTMLButtonElement | null = null;

  async function startCreate() {
    isCreating = true;
    newName = '';
    newContent = '';
    await tick();
    newNameInput?.focus();
  }

  function cancelCreate() {
    isCreating = false;
  }

  function saveNew() {
    if (!newName.trim()) {
      alert('Template name is required');
      return;
    }

    const template: Template = {
      id: crypto.randomUUID(),
      name: newName.trim(),
      content: newContent,
      createdAt: Date.now(),
      updatedAt: Date.now(),
    };

    dispatch('create', template);
    isCreating = false;
  }

  function startEdit(template: Template) {
    editingId = template.id;
    editingName = template.name;
    editingContent = template.content;
  }

  function cancelEdit() {
    editingId = null;
  }

  function saveEdit(template: Template) {
    if (!editingName.trim()) {
      alert('Template name is required');
      return;
    }

    const updated: Template = {
      ...template,
      name: editingName.trim(),
      content: editingContent,
      updatedAt: Date.now(),
    };

    dispatch('update', updated);
    editingId = null;
  }

  function openDeleteConfirmation(template: Template) {
    deleteCandidate = template;
  }

  function closeDeleteConfirmation() {
    deleteCandidate = null;
  }

  async function confirmDelete() {
    if (!deleteCandidate) return;

    const deleteId = deleteCandidate.id;
    deleteCandidate = null;

    if (onDelete) {
      await onDelete(deleteId);
      return;
    }

    dispatch('delete', deleteId);
    dispatch('templateDelete', deleteId);
  }

  function handleDeleteDialogKeydown(event: KeyboardEvent) {
    if (event.key === 'Escape') {
      event.preventDefault();
      closeDeleteConfirmation();
      return;
    }

    if (event.key === 'Enter') {
      event.preventDefault();
      void confirmDelete();
    }
  }

  $: if (deleteCandidate) {
    tick().then(() => {
      deleteCancelButton?.focus();
    });
  }
</script>

<div class="manager">
  <div class="header">
    <h2>Template Manager</h2>
    <button class="btn-new" on:click={startCreate} disabled={isCreating} type="button">
      + New Template
    </button>
  </div>

  {#if isCreating}
    <div class="template-form">
      <input
        type="text"
        class="input-name"
        placeholder="Template name..."
        bind:value={newName}
        bind:this={newNameInput}
      />
      <textarea
        class="input-content"
        placeholder="Template content..."
        bind:value={newContent}
        rows="4"
      ></textarea>
      <div class="form-actions">
        <button class="btn-save" on:click={saveNew} type="button">Save</button>
        <button class="btn-cancel" on:click={cancelCreate} type="button">Cancel</button>
      </div>
    </div>
  {/if}

  <div class="template-list">
    {#if templates.length === 0}
      <div class="empty-state">
        No templates yet. Click "New Template" to create one.
      </div>
    {:else}
      {#each templates as template (template.id)}
        <div class="template-card">
          {#if editingId === template.id}
            <div class="template-form">
              <input
                type="text"
                class="input-name"
                bind:value={editingName}
              />
              <textarea
                class="input-content"
                bind:value={editingContent}
                rows="4"
              ></textarea>
              <div class="form-actions">
                <button class="btn-save" on:click={() => saveEdit(template)} type="button">Save</button>
                <button class="btn-cancel" on:click={cancelEdit} type="button">Cancel</button>
              </div>
            </div>
          {:else}
            <div class="template-header">
              <button class="template-name" on:click={() => startEdit(template)} type="button">
                {template.name}
              </button>
              <button class="btn-delete" on:click|stopPropagation={() => openDeleteConfirmation(template)} type="button" aria-label="Delete">
                🗑️
              </button>
            </div>
            <div class="template-preview">
              {template.content.substring(0, 100)}{template.content.length > 100 ? '...' : ''}
            </div>
            <div class="template-meta">
              Created: {new Date(template.createdAt).toLocaleDateString()}
            </div>
          {/if}
        </div>
      {/each}
    {/if}
  </div>

  {#if deleteCandidate}
    <div class="delete-overlay" role="presentation" on:mousedown={closeDeleteConfirmation}>
      <div
        class="delete-dialog"
        role="alertdialog"
        aria-modal="true"
        aria-labelledby="delete-dialog-title"
        aria-describedby="delete-dialog-desc"
        tabindex="-1"
        on:mousedown|stopPropagation
        on:keydown={handleDeleteDialogKeydown}
      >
        <h3 id="delete-dialog-title">Confirm Delete</h3>
        <p id="delete-dialog-desc">
          Delete template "{deleteCandidate.name}"? This action cannot be undone.
        </p>
        <div class="delete-actions">
          <button class="btn-cancel" bind:this={deleteCancelButton} on:click={closeDeleteConfirmation} type="button">
            Cancel
          </button>
          <button class="btn-danger" on:click={() => void confirmDelete()} type="button">
            Delete
          </button>
        </div>
      </div>
    </div>
  {/if}
</div>

<style>
  .manager {
    padding: 20px;
    max-width: 800px;
    margin: 0 auto;
  }

  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 20px;
  }

  h2 {
    margin: 0;
    font-size: 20px;
    color: var(--texere-text, rgba(255, 255, 255, 0.9));
  }

  .btn-new {
    padding: 8px 16px;
    background: var(--texere-hover, rgba(88, 166, 255, 0.2));
    border: 1px solid var(--texere-accent, rgba(88, 166, 255, 0.5));
    border-radius: 6px;
    color: var(--texere-accent, rgba(88, 166, 255, 1));
    cursor: pointer;
    font-size: 14px;
    transition: background 0.15s ease;
  }

  .btn-new:hover:not(:disabled) {
    background: color-mix(in srgb, var(--texere-accent, #89b4fa) 20%, var(--texere-surface, transparent));
  }

  .btn-new:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .template-form {
    background: var(--texere-surface, rgba(255, 255, 255, 0.05));
    border: 1px solid var(--texere-border, rgba(255, 255, 255, 0.1));
    border-radius: 8px;
    padding: 16px;
    margin-bottom: 16px;
  }

  .input-name {
    width: 100%;
    padding: 10px 12px;
    background: var(--texere-surface-2, rgba(255, 255, 255, 0.05));
    border: 1px solid var(--texere-border, rgba(255, 255, 255, 0.1));
    border-radius: 6px;
    color: var(--texere-text, rgba(255, 255, 255, 0.9));
    font-size: 14px;
    margin-bottom: 12px;
    outline: none;
  }

  .input-name:focus {
    border-color: var(--texere-accent, rgba(88, 166, 255, 0.5));
  }

  .input-content {
    width: 100%;
    padding: 10px 12px;
    background: var(--texere-surface-2, rgba(255, 255, 255, 0.05));
    border: 1px solid var(--texere-border, rgba(255, 255, 255, 0.1));
    border-radius: 6px;
    color: var(--texere-text, rgba(255, 255, 255, 0.9));
    font-size: 13px;
    font-family: 'SF Mono', 'Monaco', 'Courier New', monospace;
    margin-bottom: 12px;
    outline: none;
    resize: vertical;
  }

  .input-content:focus {
    border-color: var(--texere-accent, rgba(88, 166, 255, 0.5));
  }

  .form-actions {
    display: flex;
    gap: 8px;
    justify-content: flex-end;
  }

  .btn-save {
    padding: 6px 12px;
    background: var(--texere-hover, rgba(88, 166, 255, 0.2));
    border: 1px solid var(--texere-accent, rgba(88, 166, 255, 0.5));
    border-radius: 4px;
    color: var(--texere-accent, rgba(88, 166, 255, 1));
    cursor: pointer;
    font-size: 13px;
  }

  .btn-save:hover {
    background: color-mix(in srgb, var(--texere-accent, #89b4fa) 20%, var(--texere-surface, transparent));
  }

  .btn-cancel {
    padding: 6px 12px;
    background: transparent;
    border: 1px solid var(--texere-border, rgba(255, 255, 255, 0.2));
    border-radius: 4px;
    color: var(--texere-muted, rgba(255, 255, 255, 0.7));
    cursor: pointer;
    font-size: 13px;
  }

  .btn-cancel:hover {
    background: var(--texere-hover, rgba(255, 255, 255, 0.05));
  }

  .template-list {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .template-card {
    background: var(--texere-surface, rgba(255, 255, 255, 0.05));
    border: 1px solid var(--texere-border, rgba(255, 255, 255, 0.1));
    border-radius: 8px;
    padding: 16px;
  }

  .template-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 8px;
  }

  .template-name {
    background: transparent;
    border: none;
    color: var(--texere-text, rgba(255, 255, 255, 0.9));
    font-size: 16px;
    font-weight: 500;
    cursor: pointer;
    padding: 0;
    text-align: left;
    transition: color 0.15s ease;
  }

  .template-name:hover {
    color: var(--texere-accent, rgba(88, 166, 255, 1));
  }

  .btn-delete {
    background: transparent;
    border: none;
    cursor: pointer;
    font-size: 16px;
    opacity: 0.6;
    transition: opacity 0.15s ease;
    padding: 4px 8px;
  }

  .btn-delete:hover {
    opacity: 1;
  }

  .delete-overlay {
    position: fixed;
    inset: 0;
    z-index: 1000;
    background: color-mix(in srgb, #000 55%, transparent);
    backdrop-filter: blur(4px);
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 20px;
  }

  .delete-dialog {
    width: min(440px, 100%);
    background: color-mix(in srgb, var(--texere-surface, #1b1e2b) 85%, #000 15%);
    border: 1px solid var(--texere-border, rgba(255, 255, 255, 0.16));
    border-radius: 10px;
    box-shadow: 0 22px 50px rgba(0, 0, 0, 0.45);
    padding: 18px;
  }

  .delete-dialog h3 {
    margin: 0 0 10px;
    color: var(--texere-text, rgba(255, 255, 255, 0.9));
    font-size: 16px;
  }

  .delete-dialog p {
    margin: 0;
    color: var(--texere-muted, rgba(255, 255, 255, 0.68));
    font-size: 13px;
    line-height: 1.45;
  }

  .delete-actions {
    margin-top: 16px;
    display: flex;
    justify-content: flex-end;
    gap: 8px;
  }

  .btn-danger {
    padding: 6px 12px;
    border-radius: 4px;
    border: 1px solid color-mix(in srgb, #ef4444 60%, #000 40%);
    background: color-mix(in srgb, #ef4444 26%, var(--texere-surface, transparent));
    color: #fecaca;
    cursor: pointer;
    font-size: 13px;
  }

  .btn-danger:hover {
    background: color-mix(in srgb, #ef4444 34%, var(--texere-surface, transparent));
  }

  .template-preview {
    color: var(--texere-muted, rgba(255, 255, 255, 0.6));
    font-size: 13px;
    font-family: 'SF Mono', 'Monaco', 'Courier New', monospace;
    margin-bottom: 8px;
    white-space: pre-wrap;
  }

  .template-meta {
    color: color-mix(in srgb, var(--texere-muted, #a6adc8) 70%, transparent);
    font-size: 11px;
  }

  .empty-state {
    padding: 40px 20px;
    text-align: center;
    color: var(--texere-muted, rgba(255, 255, 255, 0.5));
    font-size: 14px;
  }
</style>
