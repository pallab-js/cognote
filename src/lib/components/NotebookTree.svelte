<script lang="ts">
  import { ChevronRight, ChevronDown, Folder, FolderOpen, Plus, Trash2, Pencil } from 'lucide-svelte';
  import type { Notebook } from '$lib/commands';
  import { createNotebook, renameNotebook, deleteNotebook } from '$lib/commands';
  import { notebooks, refreshNotebooks } from '$lib/stores/notebooks';
  import { activeNotebookId, activeTagId } from '$lib/stores/app';

  export let items: Notebook[] = [];
  export let parentId: string | null = null;
  export let depth = 0;

  let expanded: Record<string, boolean> = {};
  let editing: string | null = null;
  let editName = '';

  $: children = items.filter(n => n.parent_id === parentId);

  async function addNotebook(parent: string | null) {
    const name = prompt('Notebook name:');
    if (!name?.trim()) return;
    await createNotebook(name.trim(), parent ?? undefined);
    await refreshNotebooks();
  }

  function startEdit(nb: Notebook) {
    editing = nb.id;
    editName = nb.name;
  }

  async function commitEdit(id: string) {
    if (editName.trim()) await renameNotebook(id, editName.trim());
    editing = null;
    await refreshNotebooks();
  }

  async function remove(id: string) {
    if (!confirm('Delete notebook and all its children?')) return;
    await deleteNotebook(id);
    await refreshNotebooks();
    if ($activeNotebookId === id) activeNotebookId.set(null);
  }
</script>

<ul style="padding-left: {depth > 0 ? 12 : 0}px">
  {#each children as nb (nb.id)}
    {@const hasChildren = items.some(n => n.parent_id === nb.id)}
    {@const isActive = $activeNotebookId === nb.id}
    <li>
      <div
        class="nb-row"
        class:active={isActive}
        role="button"
        tabindex="0"
        onclick={() => { activeNotebookId.set(nb.id); activeTagId.set(null); }}
        onkeydown={e => e.key === 'Enter' && activeNotebookId.set(nb.id)}
      >
        <button class="icon-btn" onclick={(e) => { e.stopPropagation(); expanded[nb.id] = !expanded[nb.id]; }}>
          {#if hasChildren}
            {#if expanded[nb.id]}<ChevronDown size={12}/>{:else}<ChevronRight size={12}/>{/if}
          {:else}
            <span style="width:12px;display:inline-block"></span>
          {/if}
        </button>
        {#if expanded[nb.id]}<FolderOpen size={14} color="var(--green-brand)"/>{:else}<Folder size={14} color="var(--text-muted)"/>{/if}
        {#if editing === nb.id}
          <input
            class="edit-input"
            bind:value={editName}
            onblur={() => commitEdit(nb.id)}
            onkeydown={e => e.key === 'Enter' && commitEdit(nb.id)}
          />
        {:else}
          <span class="nb-name">{nb.name}</span>
        {/if}
        <div class="nb-actions">
          <button class="icon-btn" title="Add child" onclick={(e) => { e.stopPropagation(); addNotebook(nb.id); }}><Plus size={11}/></button>
          <button class="icon-btn" title="Rename" onclick={(e) => { e.stopPropagation(); startEdit(nb); }}><Pencil size={11}/></button>
          <button class="icon-btn danger" title="Delete" onclick={(e) => { e.stopPropagation(); remove(nb.id); }}><Trash2 size={11}/></button>
        </div>
      </div>
      {#if expanded[nb.id]}
        <svelte:self {items} parentId={nb.id} depth={depth + 1} />
      {/if}
    </li>
  {/each}
</ul>

{#if depth === 0}
  <button class="add-root-btn" onclick={() => addNotebook(null)}>
    <Plus size={12}/> New notebook
  </button>
{/if}

<style>
  ul { list-style: none; }
  .nb-row {
    display: flex; align-items: center; gap: 4px;
    padding: 4px 6px; border-radius: 6px; cursor: pointer;
    color: var(--text-secondary); font-size: 13px;
    transition: background 0.1s;
  }
  .nb-row:hover { background: var(--border-subtle); }
  .nb-row.active { background: var(--border-standard); color: var(--text-primary); }
  .nb-name { flex: 1; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
  .nb-actions { display: none; gap: 2px; }
  .nb-row:hover .nb-actions { display: flex; }
  .icon-btn {
    background: none; border: none; cursor: pointer;
    color: var(--text-muted); padding: 2px; border-radius: 4px;
    display: flex; align-items: center;
  }
  .icon-btn:hover { color: var(--text-primary); background: var(--border-prominent); }
  .icon-btn.danger:hover { color: #ff6b6b; }
  .edit-input {
    flex: 1; background: var(--bg-button); border: 1px solid var(--green-brand);
    color: var(--text-primary); font-size: 13px; padding: 1px 4px; border-radius: 4px;
  }
  .add-root-btn {
    display: flex; align-items: center; gap: 4px;
    margin-top: 4px; padding: 4px 6px; border-radius: 6px;
    background: none; border: none; cursor: pointer;
    color: var(--text-muted); font-size: 12px; width: 100%;
  }
  .add-root-btn:hover { color: var(--green-brand); background: var(--border-subtle); }
</style>
