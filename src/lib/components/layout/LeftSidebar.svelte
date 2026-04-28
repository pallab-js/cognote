<script lang="ts">
  import { 
    LayoutDashboard, 
    FileText, 
    CheckSquare, 
    Files, 
    Network, 
    Tag, 
    ChevronDown, 
    ChevronRight,
    Folder,
    BookOpen
  } from 'lucide-svelte';
  import { currentView, leftSidebarOpen } from '$lib/stores/app';
  import NotebookTree from '$lib/components/NotebookTree.svelte';
  import { notebooks } from '$lib/stores/notebooks';

  const navItems = [
    { id: 'dashboard', label: 'Dashboard', icon: LayoutDashboard },
    { id: 'editor', label: 'Notes', icon: FileText },
    { id: 'research', label: 'Research', icon: BookOpen },
    { id: 'tasks', label: 'Tasks', icon: CheckSquare },
    { id: 'files', label: 'Files', icon: Files },
    { id: 'graph', label: 'Graph', icon: Network },
    { id: 'tags', label: 'Tags', icon: Tag },
  ];

  let foldersExpanded = true;
</script>

<aside class="sidebar" class:collapsed={!$leftSidebarOpen}>
  <div class="nav-section">
    {#each navItems as item}
      <button 
        class="nav-item" 
        class:active={$currentView === item.id}
        onclick={() => currentView.set(item.id as any)}
      >
        <svelte:component this={item.icon} size={18} />
        <span>{item.label}</span>
      </button>
    {/each}
  </div>

  <div class="divider"></div>

  <div class="folders-section">
    <button class="section-header" onclick={() => foldersExpanded = !foldersExpanded}>
      {#if foldersExpanded}<ChevronDown size={14} />{:else}<ChevronRight size={14} />{/if}
      <Folder size={14} />
      <span>FOLDERS</span>
    </button>

    {#if foldersExpanded}
      <div class="folders-content">
        <NotebookTree items={$notebooks} />
      </div>
    {/if}
  </div>
</aside>

<style>
  .sidebar {
    width: 240px;
    height: 100%;
    background: var(--bg-secondary);
    border-right: 1px solid var(--border-subtle);
    display: flex;
    flex-direction: column;
    padding: 12px 8px;
    transition: width 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    overflow-x: hidden;
  }

  .sidebar.collapsed {
    width: 0;
    padding: 0;
    border-right: none;
  }

  .nav-section {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .nav-item {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 8px 12px;
    background: none;
    border: none;
    color: var(--text-secondary);
    cursor: pointer;
    border-radius: 8px;
    font-size: 14px;
    font-weight: 500;
    transition: all 0.2s;
    text-align: left;
  }

  .nav-item:hover {
    color: var(--text-primary);
    background: var(--border-subtle);
  }

  .nav-item.active {
    color: var(--green-brand);
    background: rgba(62, 207, 142, 0.1);
  }

  .divider {
    height: 1px;
    background: var(--border-subtle);
    margin: 16px 12px;
  }

  .folders-section {
    flex: 1;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .section-header {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 12px;
    background: none;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    font-size: 11px;
    font-weight: 700;
    letter-spacing: 0.05em;
    transition: color 0.2s;
  }

  .section-header:hover {
    color: var(--text-secondary);
  }

  .folders-content {
    flex: 1;
    overflow-y: auto;
    padding-left: 8px;
  }

  /* Custom scrollbar for sidebar */
  .folders-content::-webkit-scrollbar {
    width: 4px;
  }
  .folders-content::-webkit-scrollbar-thumb {
    background: var(--border-subtle);
    border-radius: 2px;
  }
</style>
