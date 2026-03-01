<script lang="ts">
  import { createEventDispatcher } from 'svelte';

export let value: string;
  export let id: string | undefined = undefined;
  let recording = false;
  
  const dispatch = createEventDispatcher();

  function startRecording() {
    recording = true;
    window.addEventListener('keydown', handleKeydown, { capture: true, once: false });
    window.addEventListener('keyup', handleKeyup, { capture: true, once: false });
  }

  function stopRecording() {
    recording = false;
    window.removeEventListener('keydown', handleKeydown, { capture: true });
    window.removeEventListener('keyup', handleKeyup, { capture: true });
  }

  let currentModifiers: Set<string> = new Set();
  
  function handleKeydown(e: KeyboardEvent) {
    if (!recording) return;
    e.preventDefault();
    e.stopPropagation();

    const key = e.key.toLowerCase();
    
    // Ignore simple modifier presses until a real key is pressed
    if (['meta', 'control', 'alt', 'shift'].includes(key)) {
      currentModifiers.add(key);
      return;
    }

    // Build the string
    let parts = [];
    if (e.metaKey || e.ctrlKey) parts.push('CommandOrControl');
    if (e.shiftKey) parts.push('Shift');
    if (e.altKey) parts.push('Alt');

    let keyStr = e.key;
    if (keyStr === ' ') keyStr = 'Space';
    if (keyStr.length === 1) keyStr = keyStr.toUpperCase();
    
    parts.push(keyStr);
    
    value = parts.join('+');
    dispatch('change', value);
    
    stopRecording();
  }

  function handleKeyup(e: KeyboardEvent) {
    if (!recording) return;
    const key = e.key.toLowerCase();
    if (['meta', 'control', 'alt', 'shift'].includes(key)) {
      currentModifiers.delete(key);
    }
  }

  // Formatting display value slightly nicer
  $: displayValue = value
    .replace('CommandOrControl', 'Cmd/Ctrl')
    .replace('+', ' + ');

</script>

<button {id} class="hotkey-recorder" class:recording on:click={startRecording} type="button">
  {#if recording}
    <span>Press keys...</span>
  {:else}
    <span>{displayValue || 'Click to record...'}</span>
  {/if}
</button>

<style>
  .hotkey-recorder {
    padding: 6px 12px;
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.2);
    border-radius: 4px;
    cursor: pointer;
    font-family: monospace;
    font-size: 13px;
    min-width: 150px;
    text-align: center;
    transition: all 0.2s;
    user-select: none;
    color: #a6accd;
    appearance: none;
  }

  .hotkey-recorder:hover {
    border-color: #89b4fa;
    background: rgba(137, 180, 250, 0.1);
  }

  .recording {
    border-color: #f38ba8;
    background: rgba(243, 139, 168, 0.1);
    color: #f38ba8;
    animation: pulse 1.5s infinite;
  }

  @keyframes pulse {
    0% { box-shadow: 0 0 0 0 rgba(243, 139, 168, 0.4); }
    70% { box-shadow: 0 0 0 4px rgba(243, 139, 168, 0); }
    100% { box-shadow: 0 0 0 0 rgba(243, 139, 168, 0); }
  }
</style>
