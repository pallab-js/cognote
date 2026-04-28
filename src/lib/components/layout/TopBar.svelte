<script lang="ts">
  import { Search, Plus, Bell, User, Sun, Moon } from 'lucide-svelte';
  import { searchQuery, appConfig, showToast } from '$lib/stores/app';
  import { updateAppConfig, createNote } from '$lib/commands';
  import { activeNoteId, activeNotebookId } from '$lib/stores/app';
  import { currentView } from '$lib/stores/app';
  import { refreshNotes } from '$lib/stores/notes';

  let searchInput: HTMLInputElement;

  async function toggleTheme() {
    const newTheme = $appConfig.theme === 'dark' ? 'light' : 'dark';
    appConfig.update(c => ({ ...c, theme: newTheme }));
    document.body.classList.toggle('light', newTheme === 'light');
    try {
      await updateAppConfig(newTheme);
    } catch (err) {
      showToast('Failed to save theme preference', 'error');
    }
  }

  async function handleQuickAdd() {
    try {
      const note = await createNote('Untitled', $activeNotebookId ?? undefined);
      await refreshNotes($activeNotebookId ?? undefined);
      activeNoteId.set(note.id);
      currentView.set('editor');
    } catch (err) {
      showToast('Failed to create note', 'error');
    }
  }
</script>

<header class="top-bar">
  <div class="left-section">
    <div class="brand">
      <div class="logo-orb"></div>
      <span class="brand-name">Cognote</span>
    </div>
  </div>

  <div class="center-section">
    <div class="search-container">
      <Search size={14} class="search-icon" />
      <input
        type="text"
        placeholder="Search everything... (⌘K)"
        bind:value={$searchQuery}
        bind:this={searchInput}
      />
      <div class="shortcut-hint">⌘K</div>
    </div>
  </div>

  <div class="right-section">
    <button class="action-btn plus" title="Quick Add" onclick={handleQuickAdd}>
      <Plus size={18} />
    </button>
    
    <button class="icon-btn" title="Notifications">
      <Bell size={18} />
      <span class="badge"></span>
    </button>

    <button class="icon-btn" title="Toggle Theme" onclick={toggleTheme}>
      {#if $appConfig.theme === 'dark'}<Sun size={18} />{:else}<Moon size={18} />{/if}
    </button>

    <div class="profile-btn">
      <User size={18} />
    </div>
  </div>
</header>

<style>
  .top-bar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    height: 56px;
    padding: 0 20px;
    background: var(--bg-primary);
    border-bottom: 1px solid var(--border-subtle);
    backdrop-filter: blur(10px);
    z-index: 100;
  }

  .left-section {
    display: flex;
    align-items: center;
    width: 240px;
  }

  .brand {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .logo-orb {
    width: 12px;
    height: 12px;
    background: var(--green-brand);
    border-radius: 50%;
    box-shadow: 0 0 10px var(--green-brand);
    animation: pulse 2s infinite;
  }

  @keyframes pulse {
    0% { transform: scale(1); opacity: 1; }
    50% { transform: scale(1.2); opacity: 0.7; }
    100% { transform: scale(1); opacity: 1; }
  }

  .brand-name {
    font-size: 16px;
    font-weight: 600;
    letter-spacing: -0.02em;
    background: linear-gradient(135deg, var(--text-primary) 0%, var(--text-secondary) 100%);
    -webkit-background-clip: text;
    background-clip: text;
    -webkit-text-fill-color: transparent;
  }

  .center-section {
    flex: 1;
    display: flex;
    justify-content: center;
    max-width: 600px;
  }

  .search-container {
    position: relative;
    width: 100%;
    display: flex;
    align-items: center;
    background: var(--bg-secondary);
    border: 1px solid var(--border-standard);
    border-radius: 10px;
    padding: 0 12px;
    height: 36px;
    transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
  }

  .search-container:focus-within {
    border-color: var(--green-brand);
    box-shadow: 0 0 0 2px rgba(62, 207, 142, 0.1);
    background: var(--bg-primary);
  }

  :global(.search-icon) {
    color: var(--text-muted);
    margin-right: 10px;
  }

  .search-container input {
    flex: 1;
    background: transparent;
    border: none;
    outline: none;
    color: var(--text-primary);
    font-size: 13px;
  }

  .shortcut-hint {
    font-size: 10px;
    font-weight: 600;
    color: var(--text-muted);
    background: var(--border-subtle);
    padding: 2px 6px;
    border-radius: 4px;
    margin-left: 8px;
  }

  .right-section {
    display: flex;
    align-items: center;
    gap: 12px;
    width: 240px;
    justify-content: flex-end;
  }

  .icon-btn {
    position: relative;
    background: none;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    padding: 8px;
    border-radius: 8px;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.2s;
  }

  .icon-btn:hover {
    color: var(--text-primary);
    background: var(--border-subtle);
  }

  .action-btn.plus {
    background: var(--green-brand);
    color: #111;
    border-radius: 50%;
    width: 32px;
    height: 32px;
    padding: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    box-shadow: 0 4px 12px rgba(62, 207, 142, 0.3);
  }

  .action-btn.plus:hover {
    transform: scale(1.05);
    box-shadow: 0 6px 16px rgba(62, 207, 142, 0.4);
  }

  .badge {
    position: absolute;
    top: 6px;
    right: 6px;
    width: 6px;
    height: 6px;
    background: #ff4d4f;
    border-radius: 50%;
    border: 2px solid var(--bg-primary);
  }

  .profile-btn {
    width: 32px;
    height: 32px;
    background: var(--border-subtle);
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--text-secondary);
    cursor: pointer;
    transition: all 0.2s;
    border: 1px solid var(--border-standard);
  }

  .profile-btn:hover {
    border-color: var(--text-muted);
    color: var(--text-primary);
  }
</style>
