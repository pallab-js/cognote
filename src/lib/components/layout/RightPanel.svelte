<script lang="ts">
  import { 
    Link, 
    ListTree, 
    Settings, 
    Tag, 
    CheckSquare,
    ChevronDown,
    ChevronRight,
    ExternalLink
  } from 'lucide-svelte';
  import { rightPanelOpen, activeNoteId } from '$lib/stores/app';
  import { onMount } from 'svelte';
  import { getNote, listTasks, type Task } from '$lib/commands';

  let sections = [
    { id: 'backlinks', label: 'Backlinks', icon: Link, expanded: true },
    { id: 'outline', label: 'Outline', icon: ListTree, expanded: true },
    { id: 'properties', label: 'Properties', icon: Settings, expanded: false },
    { id: 'tags', label: 'Tags', icon: Tag, expanded: false },
    { id: 'tasks', label: 'TASKS (mini)', icon: CheckSquare, expanded: true },
  ];

  let miniTasks: Task[] = [];
  
  async function loadMiniTasks() {
    try {
      const allTasks = await listTasks();
      miniTasks = allTasks.filter(t => !t.is_completed).slice(0, 5);
    } catch (e) {
      console.error(e);
    }
  }

  onMount(() => {
    loadMiniTasks();
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
              <div class="empty-state">No backlinks found</div>
            {:else if section.id === 'outline'}
              <div class="outline-list">
                <div class="outline-item h1">Introduction</div>
                <div class="outline-item h2">Core Concepts</div>
                <div class="outline-item h2">Implementation</div>
              </div>
            {:else if section.id === 'properties'}
              <div class="properties-list">
                <div class="prop-row">
                  <span class="prop-key">ID</span>
                  <span class="prop-val mono">{$activeNoteId?.slice(0, 8) || 'None'}</span>
                </div>
                <div class="prop-row">
                  <span class="prop-key">Status</span>
                  <span class="prop-val">Draft</span>
                </div>
              </div>
            {:else if section.id === 'tags'}
              <div class="tags-cloud">
                <span class="tag-pill">#research</span>
                <span class="tag-pill">#ai</span>
                <span class="tag-pill">#draft</span>
              </div>
            {:else if section.id === 'tasks'}
              <div class="mini-tasks">
                {#each miniTasks as task}
                  <div class="mini-task-item">
                    <input type="checkbox" />
                    <span class="task-text">{task.content}</span>
                  </div>
                {:else}
                  <div class="empty-state">No pending tasks</div>
                {/each}
                <button class="add-task-inline">+ Add Task</button>
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
  }

  .outline-item:hover { color: var(--green-brand); }
  .outline-item.h1 { font-weight: 600; color: var(--text-primary); }
  .outline-item.h2 { padding-left: 12px; font-size: 12px; }

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

  .tags-cloud {
    display: flex;
    flex-wrap: wrap;
    gap: 6px;
  }

  .tag-pill {
    background: var(--border-subtle);
    padding: 2px 10px;
    border-radius: 4px;
    font-size: 11px;
    color: var(--green-brand);
    border: 1px solid var(--green-border);
    cursor: pointer;
    transition: all 0.2s;
  }

  .tag-pill:hover {
    background: rgba(62, 207, 142, 0.1);
    transform: translateY(-1px);
  }

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
</style>
