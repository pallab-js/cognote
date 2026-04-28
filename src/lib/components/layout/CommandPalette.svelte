<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { 
    commandPaletteOpen, 
    currentView, 
    activeNoteId, 
    showToast,
    leftSidebarOpen,
    rightPanelOpen,
    bottomPanelOpen
  } from '$lib/stores/app';
  import { 
    Search, 
    Plus, 
    FileText, 
    Network, 
    CheckSquare, 
    LayoutDashboard, 
    BookOpen,
    Command,
    Settings,
    Moon,
    Sun,
    PanelLeft,
    PanelRight,
    PanelBottom
  } from 'lucide-svelte';
  import { createNote } from '$lib/commands';
  import { refreshNotes } from '$lib/stores/notes';

  let query = '';
  let selectedIndex = 0;
  let container: HTMLElement;

  const commands = [
    { id: 'new-note', label: 'Create New Note', icon: Plus, shortcut: '⌘ N', action: handleNewNote },
    { id: 'view-editor', label: 'Go to Editor', icon: FileText, action: () => currentView.set('editor') },
    { id: 'view-dashboard', label: 'Go to Dashboard', icon: LayoutDashboard, action: () => currentView.set('dashboard') },
    { id: 'view-research', label: 'Go to Research Mode', icon: BookOpen, action: () => currentView.set('research') },
    { id: 'view-graph', label: 'Go to Knowledge Graph', icon: Network, action: () => currentView.set('graph') },
    { id: 'view-tasks', label: 'Go to Tasks', icon: CheckSquare, action: () => currentView.set('tasks') },
    { id: 'toggle-sidebar', label: 'Toggle Left Sidebar', icon: PanelLeft, action: () => leftSidebarOpen.update(v => !v) },
    { id: 'toggle-right', label: 'Toggle Right Panel', icon: PanelRight, action: () => rightPanelOpen.update(v => !v) },
    { id: 'toggle-bottom', label: 'Toggle Bottom Panel', icon: PanelBottom, action: () => bottomPanelOpen.update(v => !v) },
  ];

  $: filteredCommands = query 
    ? commands.filter(c => c.label.toLowerCase().includes(query.toLowerCase()))
    : commands;

  async function handleNewNote() {
    try {
      const note = await createNote('Untitled');
      await refreshNotes();
      activeNoteId.set(note.id);
      currentView.set('editor');
    } catch (e) {
      showToast('Failed to create note', 'error');
    }
  }

  function executeCommand(command: any) {
    command.action();
    close();
  }

  function close() {
    commandPaletteOpen.set(false);
    query = '';
    selectedIndex = 0;
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape') {
      close();
    } else if (e.key === 'ArrowDown') {
      e.preventDefault();
      selectedIndex = (selectedIndex + 1) % filteredCommands.length;
    } else if (e.key === 'ArrowUp') {
      e.preventDefault();
      selectedIndex = (selectedIndex - 1 + filteredCommands.length) % filteredCommands.length;
    } else if (e.key === 'Enter') {
      e.preventDefault();
      if (filteredCommands[selectedIndex]) {
        executeCommand(filteredCommands[selectedIndex]);
      }
    }
  }

  onMount(() => {
    window.addEventListener('keydown', handleKeydown);
    return () => window.removeEventListener('keydown', handleKeydown);
  });
</script>

{#if $commandPaletteOpen}
  <div class="palette-overlay" onclick={close} role="presentation">
    <div class="palette-container" onclick={(e) => e.stopPropagation()} role="presentation" bind:this={container}>
      <div class="search-area">
        <Command size={18} class="cmd-icon" />
        <input 
          type="text" 
          placeholder="Type a command or search..." 
          bind:value={query}
          autofocus
        />
        <div class="esc-hint">ESC</div>
      </div>

      <div class="results-area">
        {#each filteredCommands as command, i}
          <button 
            class="command-item" 
            class:selected={i === selectedIndex}
            onclick={() => executeCommand(command)}
          >
            <div class="cmd-left">
              <svelte:component this={command.icon} size={16} />
              <span>{command.label}</span>
            </div>
            {#if command.shortcut}
              <span class="shortcut">{command.shortcut}</span>
            {/if}
          </button>
        {:else}
          <div class="no-results">No commands found for "{query}"</div>
        {/each}
      </div>

      <div class="palette-footer">
        <div class="footer-hint">
          <span><kbd>↑↓</kbd> to navigate</span>
          <span><kbd>Enter</kbd> to select</span>
        </div>
      </div>
    </div>
  </div>
{/if}

<style>
  .palette-overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.4);
    backdrop-filter: blur(4px);
    display: flex;
    justify-content: center;
    padding-top: 15vh;
    z-index: 9999;
    animation: fadeIn 0.15s ease-out;
  }

  @keyframes fadeIn {
    from { opacity: 0; }
    to { opacity: 1; }
  }

  .palette-container {
    width: 600px;
    max-height: 400px;
    background: var(--bg-primary);
    border: 1px solid var(--border-prominent);
    border-radius: 16px;
    box-shadow: 0 24px 64px rgba(0, 0, 0, 0.5);
    display: flex;
    flex-direction: column;
    overflow: hidden;
    animation: slideDown 0.2s cubic-bezier(0.16, 1, 0.3, 1);
  }

  @keyframes slideDown {
    from { transform: translateY(-20px) scale(0.95); }
    to { transform: translateY(0) scale(1); }
  }

  .search-area {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 16px 20px;
    border-bottom: 1px solid var(--border-subtle);
  }

  .cmd-icon { color: var(--green-brand); }

  .search-area input {
    flex: 1;
    background: none;
    border: none;
    outline: none;
    color: var(--text-primary);
    font-size: 16px;
    font-family: var(--font-sans);
  }

  .esc-hint {
    font-size: 10px;
    font-weight: 700;
    color: var(--text-muted);
    background: var(--border-subtle);
    padding: 2px 6px;
    border-radius: 4px;
  }

  .results-area {
    flex: 1;
    overflow-y: auto;
    padding: 8px;
  }

  .command-item {
    width: 100%;
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 10px 12px;
    background: none;
    border: none;
    border-radius: 10px;
    cursor: pointer;
    transition: all 0.15s;
    text-align: left;
  }

  .command-item.selected {
    background: rgba(62, 207, 142, 0.1);
  }

  .cmd-left {
    display: flex;
    align-items: center;
    gap: 12px;
    color: var(--text-secondary);
  }

  .command-item.selected .cmd-left {
    color: var(--green-brand);
  }

  .command-item.selected span {
    color: var(--text-primary);
    font-weight: 500;
  }

  .shortcut {
    font-size: 11px;
    color: var(--text-muted);
    font-family: var(--font-mono);
  }

  .no-results {
    padding: 32px;
    text-align: center;
    color: var(--text-muted);
    font-style: italic;
  }

  .palette-footer {
    padding: 12px 20px;
    background: var(--bg-secondary);
    border-top: 1px solid var(--border-subtle);
  }

  .footer-hint {
    display: flex;
    gap: 20px;
    font-size: 11px;
    color: var(--text-muted);
  }

  kbd {
    background: var(--border-subtle);
    padding: 1px 4px;
    border-radius: 3px;
    font-family: var(--font-mono);
    margin-right: 4px;
  }
</style>
