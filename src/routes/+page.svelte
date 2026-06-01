<!-- ANCHOR: SHELL_READY -->
<script lang="ts">
  import { onMount } from 'svelte';
  import NotebookTree from '$lib/components/NotebookTree.svelte';
  import NoteList from '$lib/components/NoteList.svelte';
  import NoteEditor from '$lib/components/NoteEditor.svelte';
  import KnowledgeGraphView from '$lib/components/KnowledgeGraphView.svelte';
  import MindMapView from '$lib/components/MindMapView.svelte';
  import Dashboard from '$lib/components/Dashboard.svelte';
  import FileBrowser from '$lib/components/FileBrowser.svelte';
  import TaskView from '$lib/components/TaskView.svelte';
  import CommandPalette from '$lib/components/layout/CommandPalette.svelte';
  import LayoutManager from '$lib/components/layout/LayoutManager.svelte';
  import { notebooks, refreshNotebooks } from '$lib/stores/notebooks';
  import { currentView, appConfig, showToast, activeNoteId, activeNotebookId, commandPaletteOpen } from '$lib/stores/app';
  import { updateAppConfig, createNote } from '$lib/commands';
  import { refreshNotes } from '$lib/stores/notes';

  onMount(() => {
    refreshNotebooks();
    window.addEventListener('keydown', handleKeydown);
    return () => window.removeEventListener('keydown', handleKeydown);
  });

  async function handleKeydown(e: KeyboardEvent) {
    const meta = e.metaKey || e.ctrlKey;
    if (!meta) return;
    switch (e.key) {
      case 'n':
        e.preventDefault();
        try {
          const note = await createNote('Untitled', $activeNotebookId ?? undefined);
          await refreshNotes($activeNotebookId ?? undefined);
          activeNoteId.set(note.id);
          currentView.set('editor');
        } catch (err) {
          showToast('Failed to create note', 'error');
        }
        break;
      case 'k':
        e.preventDefault();
        commandPaletteOpen.update(v => !v);
        break;
      case 'g':
        e.preventDefault();
        currentView.set('graph');
        break;
      case 'm':
        e.preventDefault();
        currentView.set('mindmap');
        break;
    }
  }
</script>

<CommandPalette />

<LayoutManager>
  <div class="view-container">
    {#if $currentView === 'editor'}
      <div class="editor-workspace">
        <div class="note-list-pane">
          <NoteList />
        </div>
        <div class="editor-pane">
          <NoteEditor />
        </div>
      </div>
    {:else if $currentView === 'graph'}
      <KnowledgeGraphView />
    {:else if $currentView === 'mindmap'}
      <MindMapView />
    {:else if $currentView === 'dashboard'}
      <Dashboard />
    {:else if $currentView === 'files'}
      <FileBrowser />
    {:else if $currentView === 'tasks'}
      <TaskView />
    {/if}
  </div>
</LayoutManager>

<style>
  .view-container {
    height: 100%;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .editor-workspace {
    display: flex;
    height: 100%;
    overflow: hidden;
  }

  .note-list-pane {
    width: 280px;
    flex-shrink: 0;
    border-right: 1px solid var(--border-subtle);
    background: var(--bg-primary);
  }

  .editor-pane {
    flex: 1;
    overflow: hidden;
    background: var(--bg-primary);
  }
</style>
