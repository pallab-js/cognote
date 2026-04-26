<script lang="ts">
  import { Tag as TagIcon } from 'lucide-svelte';
  import { listTags } from '$lib/commands';
  import { activeTagId, activeNotebookId } from '$lib/stores/app';
  import { onMount } from 'svelte';
  import type { Tag } from '$lib/commands';

  let tags: Tag[] = [];

  onMount(async () => {
    try { tags = await listTags(); } catch {}
  });
</script>

<div class="tag-list">
  {#each tags as tag (tag.id)}
    <button
      class="tag-btn"
      class:active={$activeTagId === tag.id}
      on:click={() => { activeTagId.set(tag.id); activeNotebookId.set(null); }}
    >
      <TagIcon size={11}/> {tag.name}
    </button>
  {/each}
  {#if tags.length === 0}
    <span class="empty">No tags yet</span>
  {/if}
</div>

<style>
  .tag-list { display: flex; flex-wrap: wrap; gap: 4px; padding: 4px 0; }
  .tag-btn {
    display: flex; align-items: center; gap: 4px;
    padding: 3px 8px; border-radius: 9999px;
    border: 1px solid var(--border-standard);
    background: none; color: var(--text-muted);
    font-size: 11px; cursor: pointer; font-family: var(--font-mono);
    text-transform: uppercase; letter-spacing: 0.8px;
  }
  .tag-btn:hover { border-color: var(--border-prominent); color: var(--text-secondary); }
  .tag-btn.active { border-color: var(--green-border); color: var(--green-brand); }
  .empty { font-size: 12px; color: var(--text-muted); padding: 4px 6px; }
</style>
