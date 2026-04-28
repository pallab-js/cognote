<script lang="ts">
  import { onMount } from 'svelte';
  import { listTasks, updateTask, deleteTask, createTask, type Task } from '$lib/commands';
  import { CheckSquare, Square, Trash2, Calendar } from 'lucide-svelte';
  import { activeNoteId, currentView } from '$lib/stores/app';

  let tasks: Task[] = [];
  let loading = true;
  let showCompleted = false;

  let newTaskContent = '';

  export async function refreshTasks() {
    loading = true;
    try {
      const fetchedTasks = await listTasks(undefined, showCompleted ? undefined : false);
      tasks = fetchedTasks;
    } catch (e) {
      console.error('Failed to load tasks', e);
    } finally {
      loading = false;
    }
  }

  onMount(() => {
    refreshTasks();
  });

  async function addTask() {
    const content = newTaskContent.trim();
    if (!content) return;
    try {
      const task = await createTask(content);
      newTaskContent = '';
      if (showCompleted || !task.is_completed) {
        tasks = [task, ...tasks];
      }
    } catch (e) {
      console.error('Failed to create task', e);
    }
  }

  async function toggleTask(task: Task) {
    const newStatus = !task.is_completed;
    tasks = tasks.map(t => t.id === task.id ? { ...t, is_completed: newStatus } : t);
    try {
      await updateTask(task.id, undefined, newStatus);
      if (!showCompleted && newStatus) {
        tasks = tasks.filter(t => t.id !== task.id);
      }
    } catch (e) {
      console.error('Failed to update task', e);
      tasks = tasks.map(t => t.id === task.id ? { ...t, is_completed: !newStatus } : t);
    }
  }

  async function removeTask(id: string) {
    tasks = tasks.filter(t => t.id !== id);
    try {
      await deleteTask(id);
    } catch (e) {
      console.error('Failed to delete task', e);
      refreshTasks();
    }
  }

  function goToNote(noteId: string | null) {
    if (noteId) {
      activeNoteId.set(noteId);
      currentView.set('editor');
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Enter') {
      e.preventDefault();
      addTask();
    }
  }
</script>

<div class="task-view">
  <div class="header">
    <span class="title">Tasks</span>
    <button class="filter-btn" class:active={showCompleted} onclick={() => { showCompleted = !showCompleted; refreshTasks(); }}>
      {showCompleted ? 'Hide Done' : 'Show All'}
    </button>
  </div>
  
  <div class="input-area">
    <input 
      type="text" 
      class="task-input" 
      placeholder="Add a new task... (Enter)" 
      bind:value={newTaskContent}
      onkeydown={handleKeydown}
    />
  </div>

  {#if loading}
    <div class="msg">Loading tasks...</div>
  {:else if tasks.length === 0}
    <div class="msg">No tasks found.</div>
  {:else}
    <div class="task-list">
      {#each tasks as task (task.id)}
        <div class="task-item" class:completed={task.is_completed}>
          <button class="check-btn" onclick={() => toggleTask(task)}>
            {#if task.is_completed}
              <CheckSquare size={16} color="var(--green-brand)" />
            {:else}
              <Square size={16} color="var(--text-muted)" />
            {/if}
          </button>
          
          <div class="task-content">
            <div class="task-text">{task.content}</div>
            <div class="task-meta">
              {#if task.due_date}
                <span class="meta-item due-date"><Calendar size={10}/> {task.due_date}</span>
              {/if}
              {#if task.note_id}
                <button class="meta-item link" onclick={() => goToNote(task.note_id)}>Go to Note</button>
              {/if}
            </div>
          </div>

          <button class="delete-btn" onclick={() => removeTask(task.id)}>
            <Trash2 size={14} />
          </button>
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .task-view { display: flex; flex-direction: column; height: 100%; overflow: hidden; background: var(--bg-primary); }
  .header { display: flex; justify-content: space-between; align-items: center; padding: 12px 16px; border-bottom: 1px solid var(--border-subtle); }
  .title { font-family: var(--font-mono); font-size: 11px; text-transform: uppercase; letter-spacing: 1.2px; color: var(--text-muted); font-weight: 400; }
  .filter-btn { background: none; border: none; font-size: 11px; color: var(--text-muted); cursor: pointer; transition: color 0.15s; }
  .filter-btn:hover { color: var(--text-secondary); }
  .filter-btn.active { color: var(--green-brand); }
  
  .input-area { padding: 16px; border-bottom: 1px solid var(--border-subtle); flex-shrink: 0; }
  .task-input { 
    width: 100%; background: var(--bg-primary); border: 1px solid var(--border-standard); 
    border-radius: 6px; padding: 10px 14px; color: var(--text-primary); font-size: 14px; font-family: var(--font-sans);
    outline: none; transition: border-color 0.15s;
  }
  .task-input:focus { border-color: var(--border-prominent); }
  .task-input::placeholder { color: var(--text-muted); }

  .msg { padding: 32px 16px; text-align: center; font-size: 13px; color: var(--text-muted); }
  
  .task-list { flex: 1; overflow-y: auto; display: flex; flex-direction: column; gap: 8px; padding: 16px; }
  
  .task-item {
    display: flex; gap: 12px; align-items: flex-start;
    padding: 12px; background: var(--bg-primary);
    border: 1px solid var(--border-standard); border-radius: 8px;
    transition: all 0.15s ease;
  }
  .task-item:hover { border-color: var(--border-prominent); background: var(--bg-secondary); }
  .task-item.completed { border-color: var(--border-subtle); opacity: 0.7; }
  .task-item.completed .task-text { text-decoration: line-through; color: var(--text-muted); }
  
  .check-btn { background: none; border: none; cursor: pointer; padding: 2px 0 0; display: flex; align-items: center; transition: transform 0.1s; }
  .check-btn:active { transform: scale(0.95); }
  
  .task-content { flex: 1; display: flex; flex-direction: column; gap: 6px; min-width: 0; }
  .task-text { font-size: 14px; color: var(--text-primary); word-break: break-word; line-height: 1.5; font-weight: 400; }
  
  .task-meta { display: flex; gap: 12px; flex-wrap: wrap; }
  .meta-item { display: flex; align-items: center; gap: 6px; font-size: 12px; color: var(--text-muted); }
  .meta-item.due-date { color: #f76b15; } /* Orange6 equivalent */
  button.meta-item.link { background: none; border: none; cursor: pointer; color: var(--green-link); padding: 0; text-decoration: none; border-bottom: 1px solid transparent; }
  button.meta-item.link:hover { border-bottom-color: var(--green-link); }
  
  .delete-btn { background: none; border: 1px solid transparent; cursor: pointer; color: var(--text-heavy); padding: 6px; border-radius: 6px; transition: all 0.15s; }
  .delete-btn:hover { color: #ff6b6b; background: var(--border-subtle); border-color: rgba(255, 107, 107, 0.2); }
</style>
