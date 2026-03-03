<script lang="ts">
  import { onMount, onDestroy, createEventDispatcher } from 'svelte';
  import { createEditor, setVimMode, setSoftWrap, getContent, destroyEditor } from '../editor/createEditor';
  import type { EditorView } from '@codemirror/view';

  export let vimEnabled = true;
  export let softWrap = false;
  export let initialContent = '';

  const dispatch = createEventDispatcher<{
    change: string;
    vimModeChange: string;
  }>();

  let editorElement: HTMLElement;
  let editorView: EditorView | null = null;

  onMount(() => {
    if (!editorElement) return;

    editorView = createEditor({
      element: editorElement,
      vimEnabled,
      softWrap,
      initialContent,
      onChange: (content) => {
        dispatch('change', content);
      },
      onVimModeChange: (mode) => {
        dispatch('vimModeChange', mode);
      },
    });

    setTimeout(() => {
      editorView?.focus();
    }, 0);

    return () => {
      if (editorView) {
        destroyEditor(editorView);
        editorView = null;
      }
    };
  });

  // React to vimEnabled prop changes
  $: if (editorView) {
    setVimMode(editorView, vimEnabled, (mode) => {
      dispatch('vimModeChange', mode);
    });
  }

  // React to softWrap prop changes
  $: if (editorView) {
    setSoftWrap(editorView, softWrap);
  }

  // Expose methods for parent component
  export function getValue(): string {
    return editorView ? getContent(editorView) : '';
  }

  export function setValue(content: string): void {
    if (editorView) {
      // Use the setContent function from createEditor
      import('../editor/createEditor').then(({ setContent }) => {
        if (editorView) setContent(editorView, content);
      });
    }
  }

  export function clear(): void {
    if (editorView) {
      import('../editor/createEditor').then(({ clearContent }) => {
        if (editorView) clearContent(editorView);
      });
    }
  }

  export function focus(): void {
    editorView?.focus();
  }
</script>

<div bind:this={editorElement} class="editor-container"></div>

<style>
  .editor-container {
    height: 100%;
    width: 100%;
    overflow: hidden;
  }

  :global(.cm-editor) {
    height: 100% !important;
  }

  :global(.cm-scroller) {
    overflow: auto;
  }

  :global(.cm-content) {
    min-height: 100%;
  }
</style>
