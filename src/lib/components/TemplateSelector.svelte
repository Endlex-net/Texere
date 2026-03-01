<script lang="ts">
  import { onMount } from 'svelte';
  import type { Template } from '../types';

  export let templates: Template[] = [];
  export let onSelect: (template: Template) => void;
  export let onClose: () => void;

  let searchQuery = '';
  let selectedIndex = 0;
  let searchInput: HTMLInputElement | null = null;

  onMount(() => {
    searchInput?.focus();
  });

  $: filteredTemplates = templates.filter(t =>
    t.name.toLowerCase().includes(searchQuery.toLowerCase())
  );

  $: {
    // Keep selectedIndex in bounds when filter changes
    if (selectedIndex >= filteredTemplates.length) {
      selectedIndex = Math.max(0, filteredTemplates.length - 1);
    }
  }

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === 'Escape') {
      event.preventDefault();
      onClose();
    } else if (event.key === 'ArrowDown') {
      event.preventDefault();
      selectedIndex = Math.min(selectedIndex + 1, filteredTemplates.length - 1);
    } else if (event.key === 'ArrowUp') {
      event.preventDefault();
      selectedIndex = Math.max(0, selectedIndex - 1);
    } else if (event.key === 'Enter') {
      event.preventDefault();
      if (filteredTemplates[selectedIndex]) {
        onSelect(filteredTemplates[selectedIndex]);
      }
    }
  }

  function handleTemplateClick(template: Template) {
    onSelect(template);
  }

  function handleBackdropClick() {
    onClose();
  }

  function handleBackdropKeydown(event: KeyboardEvent) {
    if (event.key === 'Escape' || event.key === 'Enter' || event.key === ' ') {
      event.preventDefault();
      onClose();
    }
  }
</script>

<svelte:window on:keydown={handleKeydown} />

<div
  class="backdrop"
  on:click|self={handleBackdropClick}
  on:keydown={handleBackdropKeydown}
  role="button"
  tabindex="0"
  aria-label="Close template selector"
>
  <div class="selector" role="dialog" aria-modal="true" tabindex="-1">
    <input
      type="text"
      class="search-input"
      placeholder="Search templates..."
      bind:value={searchQuery}
      bind:this={searchInput}
    />
    
    <div class="template-list">
      {#if filteredTemplates.length === 0}
        <div class="empty-state">
          {#if templates.length === 0}
            No templates yet. Create one in Template Manager.
          {:else}
            No templates found matching "{searchQuery}"
          {/if}
        </div>
      {:else}
        {#each filteredTemplates as template, i}
          <button
            class="template-item"
            class:selected={i === selectedIndex}
            on:click={() => handleTemplateClick(template)}
            type="button"
          >
            <div class="template-name">{template.name}</div>
            <div class="template-preview">{template.content.substring(0, 50)}{template.content.length > 50 ? '...' : ''}</div>
          </button>
        {/each}
      {/if}
    </div>
  </div>
</div>

<style>
  .backdrop {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: flex-start;
    justify-content: center;
    padding-top: 100px;
    z-index: 1000;
  }

  .selector {
    background: var(--texere-surface, rgba(30, 30, 46, 0.98));
    border: 1px solid var(--texere-border, rgba(255, 255, 255, 0.1));
    border-radius: 8px;
    width: 500px;
    max-height: 400px;
    display: flex;
    flex-direction: column;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.4);
  }

  .search-input {
    width: 100%;
    padding: 12px 16px;
    background: var(--texere-surface-2, rgba(255, 255, 255, 0.05));
    border: none;
    border-bottom: 1px solid var(--texere-border, rgba(255, 255, 255, 0.1));
    color: var(--texere-text, rgba(255, 255, 255, 0.9));
    font-size: 14px;
    outline: none;
  }

  .search-input::placeholder {
    color: var(--texere-muted, rgba(255, 255, 255, 0.4));
  }

  .template-list {
    overflow-y: auto;
    max-height: 320px;
  }

  .template-item {
    width: 100%;
    padding: 12px 16px;
    background: transparent;
    border: none;
    border-bottom: 1px solid color-mix(in srgb, var(--texere-border, rgba(255,255,255,0.1)) 60%, transparent);
    text-align: left;
    cursor: pointer;
    transition: background 0.15s ease;
  }

  .template-item:hover,
  .template-item.selected {
    background: var(--texere-hover, rgba(255, 255, 255, 0.1));
  }

  .template-name {
    color: var(--texere-text, rgba(255, 255, 255, 0.9));
    font-size: 14px;
    font-weight: 500;
    margin-bottom: 4px;
  }

  .template-preview {
    color: var(--texere-muted, rgba(255, 255, 255, 0.5));
    font-size: 12px;
    font-family: 'SF Mono', 'Monaco', 'Courier New', monospace;
  }

  .empty-state {
    padding: 32px 16px;
    text-align: center;
    color: var(--texere-muted, rgba(255, 255, 255, 0.5));
    font-size: 14px;
  }
</style>
