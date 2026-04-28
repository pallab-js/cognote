<script lang="ts">
  import { onMount } from 'svelte';
  import { 
    CheckSquare, 
    Table, 
    Files, 
    Network, 
    Calendar,
    X,
    MoreHorizontal
  } from 'lucide-svelte';
  import { bottomPanelOpen, bottomView, activeNoteId, currentView } from '$lib/stores/app';
  import { listTasks, type Task } from '$lib/commands';
  import TaskView from '$lib/components/TaskView.svelte';
  import FileBrowser from '$lib/components/FileBrowser.svelte';
  import KnowledgeGraphView from '$lib/components/KnowledgeGraphView.svelte';

  const tabs = [
    { id: 'tasks', label: 'Tasks', icon: CheckSquare },
    { id: 'table', label: 'Table', icon: Table },
    { id: 'files', label: 'Files', icon: Files },
    { id: 'graph', label: 'Graph', icon: Network },
    { id: 'calendar', label: 'Calendar', icon: Calendar },
  ];

  let tasks: Task[] = [];
  let loadingTasks = false;

  async function loadTasks() {
    loadingTasks = true;
    try {
      tasks = await listTasks();
    } catch (e) {
      console.error(e);
    } finally {
      loadingTasks = false;
    }
  }

  onMount(() => {
    loadTasks();
  });

  function closePanel() {
    bottomPanelOpen.set(false);
  }

  function goToNote(noteId: string | null) {
    if (noteId) {
      activeNoteId.set(noteId);
      currentView.set('editor');
    }
  }
</script>

<div class="bottom-panel" class:collapsed={!$bottomPanelOpen}>
  <div class="panel-header">
    <div class="tabs">
      {#each tabs as tab}
        <button 
          class="tab" 
          class:active={$bottomView === tab.id}
          onclick={() => bottomView.set(tab.id as any)}
        >
          <svelte:component this={tab.icon} size={14} />
          <span>{tab.label}</span>
        </button>
      {/each}
    </div>
    <div class="actions">
      <button class="icon-btn" onclick={closePanel}>
        <X size={16} />
      </button>
    </div>
  </div>

  <div class="panel-content">
    {#if $bottomView === 'tasks'}
      <TaskView />
    {:else if $bottomView === 'table'}
      <div class="table-view">
        <table class="data-table">
          <thead>
            <tr>
              <th><CheckSquare size={12}/></th>
              <th>Task Description</th>
              <th>Status</th>
              <th>Due Date</th>
              <th>Project/Note</th>
              <th></th>
            </tr>
          </thead>
          <tbody>
            {#if loadingTasks}
              <tr><td colspan="6" class="status-msg">Loading tasks...</td></tr>
            {:else if tasks.length === 0}
              <tr><td colspan="6" class="status-msg">No tasks found.</td></tr>
            {:else}
              {#each tasks as task}
                <tr class:done={task.is_completed}>
                  <td class="check-col">
                    <div class="check-dot" class:active={task.is_completed}></div>
                  </td>
                  <td class="content-col">{task.content}</td>
                  <td>
                    <span class="badge" class:done={task.is_completed}>
                      {task.is_completed ? 'Completed' : 'To Do'}
                    </span>
                  </td>
                  <td class="date-col">{task.due_date || 'No date'}</td>
                  <td class="link-col">
                    {#if task.note_id}
                      <button class="link-btn" onclick={() => goToNote(task.note_id)}>Linked Note</button>
                    {:else}
                      <span class="text-muted">None</span>
                    {/if}
                  </td>
                  <td class="action-col">
                    <button class="icon-btn-small"><MoreHorizontal size={14}/></button>
                  </td>
                </tr>
              {/each}
            {/if}
          </tbody>
        </table>
      </div>
    {:else if $bottomView === 'files'}
      <FileBrowser />
    {:else if $bottomView === 'graph'}
      <KnowledgeGraphView />
    {:else if $bottomView === 'calendar'}
      <div class="calendar-placeholder">
        <div class="empty-state">Calendar view coming soon...</div>
      </div>
    {/if}
  </div>
</div>

<style>
  .bottom-panel {
    height: 350px;
    background: var(--bg-primary);
    border-top: 1px solid var(--border-subtle);
    display: flex;
    flex-direction: column;
    transition: height 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    z-index: 50;
  }

  .bottom-panel.collapsed {
    height: 0;
    border-top: none;
    overflow: hidden;
  }

  .panel-header {
    height: 44px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 20px;
    background: var(--bg-secondary);
    border-bottom: 1px solid var(--border-subtle);
  }

  .tabs {
    display: flex;
    gap: 8px;
    height: 100%;
  }

  .tab {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 0 16px;
    background: none;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    font-size: 13px;
    font-weight: 500;
    border-bottom: 2px solid transparent;
    transition: all 0.2s;
  }

  .tab:hover {
    color: var(--text-secondary);
    background: var(--border-subtle);
  }

  .tab.active {
    color: var(--green-brand);
    border-bottom-color: var(--green-brand);
    background: rgba(62, 207, 142, 0.05);
  }

  .actions {
    display: flex;
    align-items: center;
  }

  .icon-btn {
    background: none;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    padding: 6px;
    border-radius: 6px;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.2s;
  }

  .icon-btn:hover {
    color: var(--text-primary);
    background: var(--border-subtle);
  }

  .panel-content {
    flex: 1;
    overflow-y: auto;
    padding: 20px;
    background: var(--bg-primary);
  }

  /* Table View Styles */
  .table-view {
    width: 100%;
    overflow-x: auto;
  }

  .data-table {
    width: 100%;
    border-collapse: separate;
    border-spacing: 0;
    font-size: 13px;
  }

  .data-table th {
    text-align: left;
    padding: 12px 16px;
    color: var(--text-muted);
    font-weight: 600;
    font-size: 11px;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    border-bottom: 1px solid var(--border-subtle);
    background: var(--bg-primary);
    position: sticky;
    top: 0;
  }

  .data-table tr:hover td {
    background: var(--bg-secondary);
  }

  .data-table td {
    padding: 14px 16px;
    border-bottom: 1px solid var(--border-subtle);
    color: var(--text-secondary);
    transition: background 0.2s;
  }

  .data-table tr.done td {
    color: var(--text-muted);
  }

  .check-col {
    width: 40px;
  }

  .check-dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    border: 2px solid var(--border-prominent);
  }

  .check-dot.active {
    background: var(--green-brand);
    border-color: var(--green-brand);
    box-shadow: 0 0 8px var(--green-brand);
  }

  .content-col {
    font-weight: 500;
    color: var(--text-primary);
  }

  tr.done .content-col {
    text-decoration: line-through;
    opacity: 0.6;
  }

  .badge {
    padding: 4px 10px;
    border-radius: 6px;
    font-size: 11px;
    font-weight: 600;
    background: rgba(255, 255, 255, 0.05);
    color: var(--text-secondary);
    border: 1px solid var(--border-subtle);
  }

  .badge.done {
    background: rgba(62, 207, 142, 0.1);
    color: var(--green-brand);
    border-color: var(--green-border);
  }

  .link-btn {
    background: none;
    border: none;
    color: var(--green-link);
    cursor: pointer;
    font-size: 12px;
    text-decoration: underline;
    text-decoration-color: transparent;
    transition: all 0.2s;
  }

  .link-btn:hover {
    text-decoration-color: var(--green-link);
  }

  .icon-btn-small {
    background: none;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    padding: 4px;
    border-radius: 4px;
  }

  .icon-btn-small:hover {
    background: var(--border-subtle);
    color: var(--text-primary);
  }

  .status-msg {
    text-align: center;
    padding: 40px;
    color: var(--text-muted);
    font-style: italic;
  }

  .text-muted { color: var(--text-muted); }

  .calendar-placeholder {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100%;
    color: var(--text-muted);
    font-style: italic;
  }
</style>
