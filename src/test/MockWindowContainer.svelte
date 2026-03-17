<script lang="ts">
  export let getEditorContent: () => string;
  export let setEditorContent: (content: string) => void;
  export let hasEditorContent = false;
  export let onOpenTemplates: () => void;
</script>

<div data-testid="window-container">
  <div hidden data-has-editor-content={hasEditorContent} data-get-editor-content={typeof getEditorContent} data-set-editor-content={typeof setEditorContent} data-open-templates={typeof onOpenTemplates}></div>
  <slot />
  <slot name="statusbar" />
</div>
