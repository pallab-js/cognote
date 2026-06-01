<script lang="ts">
  import { 
    Link, 
    ListTree, 
    Settings, 
    CheckSquare,
    ChevronDown,
    ChevronRight,
    ExternalLink
  } from 'lucide-svelte';
  import { onMount } from 'svelte';
  import { rightPanelOpen, activeNoteId } from '$lib/stores/app';
  import { getNote, listTasks, getBacklinks, getMindmapData, updateTask, createTask, type Task, type NoteLink } from '$lib/commands';

  let sections = [
    { id: 'backlinks', label: 'Backlinks', icon: Link, expanded: true },
    { id: 'outline', label: 'Outline', icon: ListTree, expanded: true },
    { id: 'properties', label: 'Properties', icon: Settings, expanded: false },
    { id: 'tasks', label: 'TASKS (mini)', icon: CheckSquare, expanded: true },
  ];

  interface OutlineItem {
    id: string;
    label: string;
    level: number;
  }

  let miniTasks: Task[] = [];
  let backlinks: NoteLink[] = [];
  let activeNote: any = null;
  let outline: OutlineItem[] = [];
  
  let showInlineAdd = false;
  let inlineContent = '';

  async function loadMiniTasks() {
    try {
      const allTasks = await listTasks();
      miniTasks = allTasks.filter(t => !t.is_completed).slice(0, 5);
    } catch (e) {
      console.error(e);
    }
  }

  async function toggleMiniTask(task: Task) {
    try {
      await updateTask(task.id, undefined, true);
      await loadMiniTasks();
      window.dispatchEvent(new CustomEvent('tasks-updated'));
    } catch (e) {
      console.error('Failed to update task:', e);
    }
  }

  async function addInlineTask() {
    const content = inlineContent.trim();
    if (!content) {
      showInlineAdd = false;
      return;
    }
    try {
      await createTask(content, $activeNoteId ?? undefined);
      inlineContent = '';
      showInlineAdd = false;
      await loadMiniTasks();
      window.dispatchEvent(new CustomEvent('tasks-updated'));
    } catch (e) {
      console.error('Failed to create task:', e);
    }
  }

  async function loadBacklinks(noteId: string) {
    try {
      backlinks = await getBacklinks(noteId);
    } catch (e) {
      console.error('Failed to load backlinks:', e);
      backlinks = [];
    }
  }

  async function loadActiveNote(noteId: string) {
    try {
      activeNote = await getNote(noteId);
    } catch (e) {
      console.error('Failed to load active note:', e);
      activeNote = null;
    }
  }

  async function loadOutline(noteId: string) {
    try {
      const data = await getMindmapData(noteId);
      const items: OutlineItem[] = [];
      function traverse(node: any, level: number) {
        if (level > 0) {
          items.push({ id: node.id, label: node.label, level });
        }
        for (const child of node.children || []) {
          traverse(child, level + 1);
        }
      }
      traverse(data, 0);
      outline = items;
    } catch (e) {
      console.error('Failed to load outline:', e);
      outline = [];
    }
  }

  function formatDate(dStr: string) {
    if (!dStr) return '';
    return new Date(dStr).toLocaleDateString(undefined, { 
      month: 'short', 
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit'
    });
  }

  $: if ($activeNoteId) {
    loadBacklinks($activeNoteId);
    loadActiveNote($activeNoteId);
    loadOutline($activeNoteId);
  } else {
    backlinks = [];
    activeNote = null;
    outline = [];
  }

  onMount(() => {
    loadMiniTasks();
    window.addEventListener('tasks-updated', loadMiniTasks);
    return () => {
      window.removeEventListener('tasks-updated', loadMiniTasks);
    };
  });

  function toggleSection(id: string) {
    sections = sections.map(s => s.id === id ? { ...s, expanded: !s.expanded } : s);
  }
</script>

<aside class="right-panel" class:collapsed={!$rightPanelOpen}>
  <div class="panel-content">
    {#each sections as section}
      <div class="section" class:expanded={section.expanded}>
        <button class="section-header" onclick={() => toggleSection(section.id)}>
          {#if section.expanded}<ChevronDown size={14} />{:else}<ChevronRight size={14} />{/if}
          <svelte:component this={section.icon} size={14} />
          <span>{section.label}</span>
        </button>

        {#if section.expanded}
          <div class="section-body">
            {#if section.id === 'backlinks'}
              <div class="backlinks-list">
                {#each backlinks as link}
                  <button class="backlink-item" onclick={() => activeNoteId.set(link.source_note_id)}>
                    <span class="backlink-title">{link.source_title || 'Untitled'}</span>
                    {#if link.context}
                      <span class="backlink-context">{link.context}</span>
                    {/if}
                  </button>
                {:else}
                  <div class="empty-state">No backlinks found</div>
                {/each}
              </div>
            {:else if section.id === 'outline'}
              <div class="outline-list">
                {#each outline as item}
                  <div class="outline-item h{item.level}">{item.label}</div>
                {:else}
                  <div class="empty-state">No headings in this note</div>
                {/each}
              </div>
            {:else if section.id === 'properties'}
              {#if activeNote}
                <div class="properties-list">
                  <div class="prop-row">
                    <span class="prop-key">ID</span>
                    <span class="prop-val mono">{$activeNoteId?.slice(0, 8) || 'None'}</span>
                  </div>
                  <div class="prop-row">
                    <span class="prop-key">Created</span>
                    <span class="prop-val">{formatDate(activeNote.created_at)}</span>
                  </div>
                  <div class="prop-row">
                    <span class="prop-key">Updated</span>
                    <span class="prop-val">{formatDate(activeNote.updated_at)}</span>
                  </div>
                  <div class="prop-row">
                    <span class="prop-key">Pinned</span>
                    <span class="prop-val">{activeNote.is_pinned ? 'Yes' : 'No'}</span>
                  </div>
                </div>
              {:else}
                <div class="empty-state">No note selected</div>
              {/if}
            {:else if section.id === 'tasks'}
              <div class="mini-tasks">
                {#each miniTasks as task}
                  <div class="mini-task-item">
                    <input 
                      type="checkbox" 
                      checked={task.is_completed} 
                      onchange={() => toggleMiniTask(task)} 
                    />
                    <span class="task-text">{task.content}</span>
                  </div>
                {:else}
                  <div class="empty-state">No pending tasks</div>
                {/each}
                
                {#if showInlineAdd}
                  <div class="inline-add-container">
                    <input 
                      type="text" 
                      class="inline-add-input"
                      placeholder="New task... (Enter)"
                      bind:value={inlineContent}
                      onkeydown={e => e.key === 'Enter' && addInlineTask()}
                      onblur={addInlineTask}
                      autofocus
                    />
                  </div>
                {:else}
                  <button class="add-task-inline" onclick={() => showInlineAdd = true}>+ Add Task</button>
                {/if}
              </div>
            {/if}
          </div>
        {/if}
      </div>
    {/each}
  </div>
</aside>

<style>
  .right-panel {
    width: 280px;
    height: 100%;
    background: var(--bg-secondary);
    border-left: 1px solid var(--border-subtle);
    display: flex;
    flex-direction: column;
    transition: width 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    overflow-x: hidden;
  }

  .right-panel.collapsed {
    width: 0;
    border-left: none;
  }

  .panel-content {
    display: flex;
    flex-direction: column;
    height: 100%;
    overflow-y: auto;
  }

  .section {
    border-bottom: 1px solid var(--border-subtle);
  }

  .section-header {
    width: 100%;
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 14px 16px;
    background: none;
    border: none;
    color: var(--text-secondary);
    cursor: pointer;
    font-size: 11px;
    font-weight: 700;
    letter-spacing: 0.05em;
    text-transform: uppercase;
    transition: all 0.2s;
  }

  .section-header:hover {
    background: var(--border-subtle);
    color: var(--text-primary);
  }

  .section-body {
    padding: 0 16px 16px 34px;
    font-size: 13px;
    color: var(--text-secondary);
  }

  .empty-state {
    color: var(--text-muted);
    font-style: italic;
    font-size: 12px;
    padding: 4px 0;
  }

  .outline-list {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .outline-item {
    cursor: pointer;
    transition: color 0.2s;
    font-size: 12px;
    padding: 2px 0;
  }

  .outline-item:hover { color: var(--green-brand); }
  .outline-item.h1 { font-weight: 500; color: var(--text-primary); padding-left: 0; }
  .outline-item.h2 { padding-left: 8px; color: var(--text-secondary); }
  .outline-item.h3 { padding-left: 16px; color: var(--text-muted); }
  .outline-item.h4 { padding-left: 24px; color: var(--text-muted); }
  .outline-item.h5 { padding-left: 32px; color: var(--text-muted); }
  .outline-item.h6 { padding-left: 40px; color: var(--text-muted); }

  .properties-list {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .prop-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .prop-key {
    color: var(--text-muted);
    font-size: 11px;
  }

  .prop-val {
    color: var(--text-secondary);
    font-size: 12px;
  }

  .prop-val.mono { font-family: var(--font-mono); }

  .mini-tasks {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .mini-task-item {
    display: flex;
    align-items: flex-start;
    gap: 10px;
    cursor: pointer;
  }

  .mini-task-item input {
    margin-top: 3px;
    accent-color: var(--green-brand);
  }

  .task-text {
    font-size: 12px;
    line-height: 1.4;
    color: var(--text-secondary);
  }

  .add-task-inline {
    background: none;
    border: none;
    color: var(--text-muted);
    font-size: 11px;
    cursor: pointer;
    padding: 4px 0;
    text-align: left;
    transition: color 0.2s;
  }

  .add-task-inline:hover {
    color: var(--green-brand);
  }

  .backlinks-list {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .backlink-item {
    width: 100%;
    text-align: left;
    background: none;
    border: 1px solid transparent;
    border-radius: 6px;
    padding: 6px 8px;
    color: var(--text-secondary);
    cursor: pointer;
    display: flex;
    flex-direction: column;
    gap: 2px;
    transition: all 0.2s;
  }

  .backlink-item:hover {
    background: var(--border-subtle);
    border-color: var(--border-standard);
    color: var(--text-primary);
  }

  .backlink-title {
    font-size: 12px;
    font-weight: 500;
    color: var(--text-primary);
  }

  .backlink-context {
    font-size: 11px;
    color: var(--text-muted);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .inline-add-container {
    margin-top: 4px;
    width: 100%;
  }

  .inline-add-input {
    width: 100%;
    background: var(--bg-primary);
    border: 1px solid var(--border-standard);
    border-radius: 6px;
    padding: 6px 8px;
    color: var(--text-primary);
    font-size: 12px;
    outline: none;
    transition: border-color 0.15s ease;
  }

  .inline-add-input:focus {
    border-color: var(--green-brand);
  }
</style>
