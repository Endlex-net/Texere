<script lang="ts">
  import TemplateManager from './TemplateManager.svelte';
  import type { Template } from '../types';

  export let templates: Template[] = [];

  let lastCreate = '';
  let lastDelete = '';

  function handleCreate(event: CustomEvent<Template>) {
    lastCreate = JSON.stringify(event.detail);
  }

  function handleDelete(event: CustomEvent<string>) {
    lastDelete = event.detail;
  }
</script>

<TemplateManager {templates} on:create={handleCreate} on:delete={handleDelete} />

<output data-testid="last-create">{lastCreate}</output>
<output data-testid="last-delete">{lastDelete}</output>
