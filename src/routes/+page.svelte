<!-- ANCHOR: SHELL_READY -->
<script lang="ts">
  import { onMount } from 'svelte';
  import { Search, Network, Map, LayoutDashboard, Files, Sun, Moon, BookOpen, Tag } from 'lucide-svelte';
  import NotebookTree from '$lib/components/NotebookTree.svelte';
  import TagList from '$lib/components/TagList.svelte';
  import NoteList from '$lib/components/NoteList.svelte';
  import NoteEditor from '$lib/components/NoteEditor.svelte';
  import KnowledgeGraphView from '$lib/components/KnowledgeGraphView.svelte';
  import MindMapView from '$lib/components/MindMapView.svelte';
  import Dashboard from '$lib/components/Dashboard.svelte';
  import FileBrowser from '$lib/components/FileBrowser.svelte';
  import StatusBar from '$lib/components/StatusBar.svelte';
  import { notebooks, refreshNotebooks } from '$lib/stores/notebooks';
  import { currentView, searchQuery, appConfig } from '$lib/stores/app';
  import { updateAppConfig, createNote } from '$lib/commands';
  import { activeNoteId, activeNotebookId } from '$lib/stores/app';
  import { refreshNotes } from '$lib/stores/notes';

  let sidebarTab: 'notebooks' | 'tags' | 'files' = 'notebooks';
  let searchInput: HTMLInputElement;

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
        const note = await createNote('Untitled', $activeNotebookId ?? undefined);
        await refreshNotes($activeNotebookId ?? undefined);
        activeNoteId.set(note.id);
        currentView.set('editor');
        break;
      case 'k':
        e.preventDefault();
        searchInput?.focus();
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

  async function toggleTheme() {
    const newTheme = $appConfig.theme === 'dark' ? 'light' : 'dark';
    appConfig.update(c => ({ ...c, theme: newTheme }));
    document.body.classList.toggle('light', newTheme === 'light');
    await updateAppConfig(newTheme);
  }
</script>

<div class="app">
  <!-- Top bar -->
  <header class="topbar">
    <div class="brand">
      <span class="brand-dot">●</span>
      <span class="brand-name">Cognote</span>
    </div>
    <div class="search-wrap">
      <Search size={13} color="var(--text-muted)"/>
      <input
        class="search-input"
        type="text"
        placeholder="Search notes... (⌘K)"
        bind:value={$searchQuery}
        bind:this={searchInput}
      />
    </div>
    <div class="topbar-actions">
      <button class="icon-btn" title="Graph" class:active={$currentView === 'graph'} on:click={() => currentView.set('graph')}><Network size={15}/></button>
      <button class="icon-btn" title="Mind Map" class:active={$currentView === 'mindmap'} on:click={() => currentView.set('mindmap')}><Map size={15}/></button>
      <button class="icon-btn" title="Dashboard" class:active={$currentView === 'dashboard'} on:click={() => currentView.set('dashboard')}><LayoutDashboard size={15}/></button>
      <button class="icon-btn" title="Toggle theme" on:click={toggleTheme}>
        {#if $appConfig.theme === 'dark'}<Sun size={15}/>{:else}<Moon size={15}/>{/if}
      </button>
    </div>
  </header>

  <div class="body">
    <!-- Sidebar -->
    <aside class="sidebar">
      <div class="sidebar-tabs">
        <button class="stab" class:active={sidebarTab === 'notebooks'} on:click={() => sidebarTab = 'notebooks'}>
          <BookOpen size={13}/> Notebooks
        </button>
        <button class="stab" class:active={sidebarTab === 'tags'} on:click={() => sidebarTab = 'tags'}>
          <Tag size={13}/> Tags
        </button>
        <button class="stab" class:active={sidebarTab === 'files'} on:click={() => { sidebarTab = 'files'; currentView.set('files'); }}>
          <Files size={13}/> Files
        </button>
      </div>
      <div class="sidebar-content">
        {#if sidebarTab === 'notebooks'}
          <NotebookTree items={$notebooks} />
        {:else if sidebarTab === 'tags'}
          <TagList />
        {:else}
          <FileBrowser />
        {/if}
      </div>
    </aside>

    <!-- Note list -->
    {#if $currentView === 'editor'}
      <div class="note-list-pane">
        <NoteList />
      </div>
    {/if}

    <!-- Main view -->
    <main class="main-view">
      {#if $currentView === 'editor'}
        <NoteEditor />
      {:else if $currentView === 'graph'}
        <KnowledgeGraphView />
      {:else if $currentView === 'mindmap'}
        <MindMapView />
      {:else if $currentView === 'dashboard'}
        <Dashboard />
      {:else if $currentView === 'files'}
        <FileBrowser />
      {/if}
    </main>
  </div>

  <StatusBar />
</div>

<style>
  .app { display: flex; flex-direction: column; height: 100vh; overflow: hidden; }

  .topbar {
    display: flex; align-items: center; gap: 12px;
    padding: 0 16px; height: 44px; flex-shrink: 0;
    border-bottom: 1px solid var(--border-subtle);
    background: var(--bg-button);
  }
  .brand { display: flex; align-items: center; gap: 6px; flex-shrink: 0; }
  .brand-dot { color: var(--green-brand); font-size: 10px; }
  .brand-name { font-size: 14px; font-weight: 500; color: var(--text-primary); letter-spacing: -0.2px; }

  .search-wrap {
    flex: 1; display: flex; align-items: center; gap: 8px;
    background: var(--bg-primary); border: 1px solid var(--border-standard);
    border-radius: 6px; padding: 6px 10px; max-width: 400px;
  }
  .search-input {
    flex: 1; background: none; border: none; outline: none;
    color: var(--text-primary); font-size: 13px; font-family: var(--font-sans);
  }
  .search-input::placeholder { color: var(--text-muted); }
  .search-wrap:focus-within { border-color: var(--green-brand); }

  .topbar-actions { display: flex; align-items: center; gap: 4px; margin-left: auto; }
  .icon-btn {
    background: none; border: 1px solid transparent; cursor: pointer;
    color: var(--text-muted); padding: 6px; border-radius: 6px;
    display: flex; align-items: center;
  }
  .icon-btn:hover { color: var(--text-primary); border-color: var(--border-prominent); }
  .icon-btn.active { color: var(--green-brand); border-color: var(--green-border); }

  .body { display: flex; flex: 1; overflow: hidden; }

  .sidebar {
    width: 220px; flex-shrink: 0; display: flex; flex-direction: column;
    border-right: 1px solid var(--border-subtle); overflow: hidden;
  }
  .sidebar-tabs {
    display: flex; border-bottom: 1px solid var(--border-subtle);
    flex-shrink: 0;
  }
  .stab {
    flex: 1; display: flex; align-items: center; justify-content: center; gap: 4px;
    padding: 8px 4px; background: none; border: none; cursor: pointer;
    color: var(--text-muted); font-size: 11px; font-weight: 500;
    border-bottom: 2px solid transparent; transition: color 0.1s;
  }
  .stab:hover { color: var(--text-secondary); }
  .stab.active { color: var(--text-primary); border-bottom-color: var(--green-brand); }
  .sidebar-content { flex: 1; overflow-y: auto; padding: 8px; }

  .note-list-pane {
    width: 240px; flex-shrink: 0;
    border-right: 1px solid var(--border-subtle);
    overflow: hidden; display: flex; flex-direction: column;
  }

  .main-view { flex: 1; overflow: hidden; display: flex; flex-direction: column; }
</style>
