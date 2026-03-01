<script lang="ts">
  import { backOut } from 'svelte/easing';
  import { onMount } from 'svelte';
  import { scale } from 'svelte/transition';
  import { formatText } from '../ai';
  import { loadSettings } from '../settings';

  export let getContent: () => string;
  export let setContent: (content: string) => void;
  export let hasContent: boolean = false;

  let isLoading = false;
  let errorMessage = '';
  let lastBeforeFormatContent: string | null = null;
  let hasApiKey = false;

  async function checkApiKey() {
    const settings = await loadSettings();
    hasApiKey = !!settings.ai.apiKey;
  }

  onMount(() => {
    checkApiKey();
  });

  async function handleFormat() {
    if (!hasContent || !hasApiKey || isLoading) return;

    const content = getContent();
    if (!content.trim()) return;

    isLoading = true;
    errorMessage = '';
    lastBeforeFormatContent = content;

    try {
      const formatted = await formatText(content);
      setContent(formatted.formatted);
    } catch (error) {
      errorMessage = error instanceof Error ? error.message : 'Failed to format text';
      lastBeforeFormatContent = null;
      setTimeout(() => {
        errorMessage = '';
      }, 3000);
    } finally {
      isLoading = false;
    }
  }

  function handleUndoFormat() {
    if (!lastBeforeFormatContent || isLoading) return;
    setContent(lastBeforeFormatContent);
    lastBeforeFormatContent = null;
  }
</script>

<div class="ai-format-container">
  <button
    class="format-button"
    on:click={handleFormat}
    disabled={!hasContent || !hasApiKey || isLoading}
    title={!hasApiKey ? 'OpenAI API key required' : !hasContent ? 'Enter some text first' : 'Format with AI'}
    aria-label={!hasApiKey ? 'OpenAI API key required' : !hasContent ? 'Enter some text first' : 'Format with AI'}
  >
    {#if isLoading}
      <span class="spinner">⟳</span>
    {:else}
      <span class="icon">✨</span>
    {/if}
  </button>

  {#if lastBeforeFormatContent}
    <button
      class="undo-button"
      on:click={handleUndoFormat}
      disabled={isLoading}
      title="Revert AI format"
      aria-label="Revert AI format"
      type="button"
      transition:scale={{ duration: 220, easing: backOut, start: 0.75 }}
    >
      ↶
    </button>
  {/if}

  {#if errorMessage}
    <div class="error-toast">{errorMessage}</div>
  {/if}
</div>

<style>
  .ai-format-container {
    position: relative;
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .format-button {
    display: flex;
    align-items: center;
    gap: 4px;
    justify-content: center;
    width: 24px;
    height: 24px;
    padding: 0;
    background: var(--texere-surface-2, rgba(255, 255, 255, 0.08));
    border: 1px solid var(--texere-border, rgba(255, 255, 255, 0.2));
    border-radius: 6px;
    color: var(--texere-accent, #1e3a8a);
    font-weight: 700;
    font-size: 13px;
    cursor: pointer;
    transition: all 0.2s;
    -webkit-app-region: no-drag;
  }

  .undo-button {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 24px;
    height: 24px;
    padding: 0;
    background: rgba(250, 204, 21, 0.22);
    border: 1px solid rgba(250, 204, 21, 0.55);
    border-radius: 6px;
    color: #fde68a;
    font-size: 13px;
    cursor: pointer;
    transition: all 0.2s;
    -webkit-app-region: no-drag;
  }

  .undo-button:hover:not(:disabled) {
    background: rgba(250, 204, 21, 0.3);
    border-color: rgba(250, 204, 21, 0.8);
    color: #fef3c7;
    transform: translateY(-1px);
  }

  .undo-button:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  .format-button:hover:not(:disabled) {
    background: var(--texere-hover, rgba(255, 255, 255, 0.15));
    border-color: var(--texere-accent, #89b4fa);
    color: var(--texere-accent, #1e3a8a);
  }

  .format-button:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  .icon {
    font-size: 14px;
    filter: saturate(1.2) contrast(1.1);
  }

  .spinner {
    display: inline-block;
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    from { transform: rotate(0deg); }
    to { transform: rotate(360deg); }
  }

  .error-toast {
    position: absolute;
    top: 100%;
    right: 0;
    margin-top: 8px;
    padding: 8px 12px;
    background: rgba(255, 100, 100, 0.9);
    color: white;
    font-size: 12px;
    border-radius: 4px;
    white-space: nowrap;
    z-index: 100;
  }
</style>
