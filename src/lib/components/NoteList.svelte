<script lang="ts">
  import { FileText, Plus, Trash2, Pin } from 'lucide-svelte';
  import { notes, refreshNotes } from '$lib/stores/notes';
  import { activeNoteId, activeNotebookId, activeTagId, searchQuery } from '$lib/stores/app';
  import { createNote, deleteNote, searchNotes } from '$lib/commands';
  import type { Note, SearchResult } from '$lib/commands';
  import { onMount } from 'svelte';

  let searchResults: SearchResult[] = [];
  let debounceTimer: ReturnType<typeof setTimeout>;

  $: {
    clearTimeout(debounceTimer);
    if ($searchQuery.trim()) {
      debounceTimer = setTimeout(async () => {
        try { searchResults = await searchNotes($searchQuery); } catch {}
      }, 300);
    } else {
      searchResults = [];
    }
  }

  $: {
    refreshNotes($activeNotebookId ?? undefined, $activeTagId ?? undefined);
  }

  async function newNote() {
    const note = await createNote('Untitled', $activeNotebookId ?? undefined);
    await refreshNotes($activeNotebookId ?? undefined);
    activeNoteId.set(note.id);
  }

  async function remove(id: string, e: MouseEvent) {
    e.stopPropagation();
    if (!confirm('Delete this note?')) return;
    await deleteNote(id);
    await refreshNotes($activeNotebookId ?? undefined);
    if ($activeNoteId === id) activeNoteId.set(null);
  }

  $: displayNotes = $searchQuery.trim()
    ? searchResults.map(r => ({ id: r.id, title: r.title, snippet: r.snippet }))
    : $notes.map(n => ({ id: n.id, title: n.title, snippet: n.content?.slice(0, 80) ?? '' }));
</script>

<div class="note-list">
  <div class="list-header">
    <span class="list-title">Notes</span>
    <button class="icon-btn" title="New note" on:click={newNote}><Plus size={14}/></button>
  </div>
  {#each displayNotes as item (item.id)}
    <div
      class="note-item"
      class:active={$activeNoteId === item.id}
      role="button"
      tabindex="0"
      on:click={() => activeNoteId.set(item.id)}
      on:keydown={e => e.key === 'Enter' && activeNoteId.set(item.id)}
    >
      <FileText size={13} color="var(--text-muted)"/>
      <div class="note-info">
        <span class="note-title">{item.title || 'Untitled'}</span>
        {#if item.snippet}
          <span class="note-snippet">{@html item.snippet}</span>
        {/if}
      </div>
      <button class="icon-btn danger" on:click={e => remove(item.id, e)}><Trash2 size={11}/></button>
    </div>
  {/each}
  {#if displayNotes.length === 0}
    <div class="empty">
      {$searchQuery ? 'No results' : 'No notes yet'}
    </div>
  {/if}
</div>

<style>
  .note-list { display: flex; flex-direction: column; height: 100%; overflow: hidden; }
  .list-header {
    display: flex; align-items: center; justify-content: space-between;
    padding: 8px 12px; border-bottom: 1px solid var(--border-subtle);
    flex-shrink: 0;
  }
  .list-title { font-size: 11px; font-family: var(--font-mono); text-transform: uppercase; letter-spacing: 1.2px; color: var(--text-muted); }
  .note-item {
    display: flex; align-items: flex-start; gap: 8px;
    padding: 10px 12px; border-bottom: 1px solid var(--border-subtle);
    cursor: pointer; transition: background 0.1s;
  }
  .note-item:hover { background: var(--border-subtle); }
  .note-item.active { background: var(--border-standard); border-left: 2px solid var(--green-brand); }
  .note-info { flex: 1; min-width: 0; }
  .note-title { display: block; font-size: 13px; color: var(--text-primary); overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
  .note-snippet { display: block; font-size: 11px; color: var(--text-muted); overflow: hidden; text-overflow: ellipsis; white-space: nowrap; margin-top: 2px; }
  .note-snippet :global(mark) { background: rgba(62, 207, 142, 0.2); color: var(--green-brand); border-radius: 2px; }
  .icon-btn { background: none; border: none; cursor: pointer; color: var(--text-muted); padding: 2px; border-radius: 4px; display: flex; align-items: center; flex-shrink: 0; }
  .icon-btn:hover { color: var(--text-primary); }
  .icon-btn.danger:hover { color: #ff6b6b; }
  .empty { padding: 16px 12px; font-size: 12px; color: var(--text-muted); text-align: center; }
</style>
