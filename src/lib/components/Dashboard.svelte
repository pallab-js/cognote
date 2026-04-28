<!-- ANCHOR: DASHBOARD_READY -->
<script lang="ts">
  import { onMount } from 'svelte';
  import { 
    getDailyStats, 
    listTasks, 
    getNotebookTree, 
    getKnowledgeGraph,
    listNotes,
    type DailyStats,
    type Note,
    type Task
  } from '$lib/commands';
  import { 
    FileText, 
    CheckSquare, 
    Folder, 
    Network, 
    Clock, 
    Star,
    ArrowUpRight,
    Calendar
  } from 'lucide-svelte';
  import { activeNoteId, currentView } from '$lib/stores/app';

  let stats: DailyStats | null = null;
  let tasksDueToday: Task[] = [];
  let totalProjects = 0;
  let totalLinks = 0;
  let recentNotes: Note[] = [];
  let pinnedNotes: Note[] = [];
  let loading = true;

  onMount(async () => {
    try {
      const [s, tasks, notebooks, graph, notes] = await Promise.all([
        getDailyStats(),
        listTasks(undefined, false),
        getNotebookTree(),
        getKnowledgeGraph(),
        listNotes()
      ]);
      
      stats = s;
      tasksDueToday = tasks.filter(t => {
        if (!t.due_date) return false;
        const today = new Date().toISOString().split('T')[0];
        return t.due_date === today;
      });
      totalProjects = notebooks.length;
      totalLinks = graph.edges.length;
      recentNotes = [...notes].sort((a, b) => 
        new Date(b.updated_at).getTime() - new Date(a.updated_at).getTime()
      ).slice(0, 5);
      pinnedNotes = notes.filter(n => n.is_pinned).slice(0, 5);

    } catch (e) {
      console.error('Failed to load dashboard data:', e);
    } finally {
      loading = false;
    }
  });

  function openNote(id: string) {
    activeNoteId.set(id);
    currentView.set('editor');
  }

  function formatDate(d: string) {
    return new Date(d).toLocaleDateString(undefined, { month: 'short', day: 'numeric' });
  }
</script>

<div class="dashboard-container">
  <header class="dash-header">
    <div class="title-group">
      <h1>Dashboard</h1>
      <p class="subtitle">Welcome back. Here's your knowledge at a glance.</p>
    </div>
    <div class="date-chip">
      <Calendar size={14} />
      <span>{new Date().toLocaleDateString(undefined, { weekday: 'long', month: 'long', day: 'numeric' })}</span>
    </div>
  </header>

  {#if loading}
    <div class="loading-state">
      <div class="shimmer-grid">
        {#each Array(6) as _}
          <div class="shimmer-card"></div>
        {/each}
      </div>
    </div>
  {:else}
    <div class="dash-grid">
      <!-- 📘 Notes Card -->
      <div class="card stat-card blue">
        <div class="card-header">
          <div class="icon-box"><FileText size={20} /></div>
          <button class="arrow-btn" onclick={() => currentView.set('editor')}><ArrowUpRight size={14}/></button>
        </div>
        <div class="card-body">
          <span class="value">{stats?.total_notes || 0}</span>
          <span class="label">Total Notes</span>
        </div>
        <div class="card-footer">
          <span class="trend">+{stats?.today_count || 0} today</span>
        </div>
      </div>

      <!-- ✅ Tasks Card -->
      <div class="card stat-card green">
        <div class="card-header">
          <div class="icon-box"><CheckSquare size={20} /></div>
          <button class="arrow-btn" onclick={() => currentView.set('tasks')}><ArrowUpRight size={14}/></button>
        </div>
        <div class="card-body">
          <span class="value">{tasksDueToday.length}</span>
          <span class="label">Tasks Due Today</span>
        </div>
        <div class="card-footer">
          <span class="trend">{stats?.streak || 0} day streak</span>
        </div>
      </div>

      <!-- 📂 Projects Card -->
      <div class="card stat-card purple">
        <div class="card-header">
          <div class="icon-box"><Folder size={20} /></div>
          <button class="arrow-btn" onclick={() => {}}><ArrowUpRight size={14}/></button>
        </div>
        <div class="card-body">
          <span class="value">{totalProjects}</span>
          <span class="label">Active Projects</span>
        </div>
        <div class="card-footer">
          <span class="trend">Notebooks & Folders</span>
        </div>
      </div>

      <!-- 🔗 Graph Card -->
      <div class="card stat-card orange">
        <div class="card-header">
          <div class="icon-box"><Network size={20} /></div>
          <button class="arrow-btn" onclick={() => currentView.set('graph')}><ArrowUpRight size={14}/></button>
        </div>
        <div class="card-body">
          <span class="value">{totalLinks}</span>
          <span class="label">Knowledge Links</span>
        </div>
        <div class="card-footer">
          <span class="trend">Connected ideas</span>
        </div>
      </div>

      <!-- 🕒 Recent Card -->
      <div class="card list-card large">
        <div class="card-header">
          <div class="title-with-icon">
            <Clock size={16} />
            <span>Recent Edits</span>
          </div>
        </div>
        <div class="list-body">
          {#each recentNotes as note}
            <button class="list-item" onclick={() => openNote(note.id)}>
              <span class="item-title">{note.title || 'Untitled'}</span>
              <span class="item-meta">{formatDate(note.updated_at)}</span>
            </button>
          {:else}
            <div class="empty-list">No recent edits</div>
          {/each}
        </div>
      </div>

      <!-- ⭐ Pinned Card -->
      <div class="card list-card large">
        <div class="card-header">
          <div class="title-with-icon">
            <Star size={16} />
            <span>Pinned Notes</span>
          </div>
        </div>
        <div class="list-body">
          {#each pinnedNotes as note}
            <button class="list-item" onclick={() => openNote(note.id)}>
              <span class="item-title">{note.title || 'Untitled'}</span>
              <span class="item-meta">Pinned</span>
            </button>
          {:else}
            <div class="empty-list">No pinned notes</div>
          {/each}
        </div>
      </div>
    </div>
  {/if}
</div>

<style>
  .dashboard-container {
    padding: 40px;
    height: 100%;
    overflow-y: auto;
    background: var(--bg-primary);
  }

  .dash-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    margin-bottom: 40px;
  }

  .title-group h1 {
    font-size: 32px;
    font-weight: 600;
    margin-bottom: 8px;
    letter-spacing: -0.02em;
  }

  .subtitle {
    color: var(--text-muted);
    font-size: 15px;
  }

  .date-chip {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 8px 16px;
    background: var(--bg-secondary);
    border: 1px solid var(--border-subtle);
    border-radius: 100px;
    font-size: 13px;
    color: var(--text-secondary);
  }

  .dash-grid {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 24px;
  }

  .card {
    background: var(--bg-secondary);
    border: 1px solid var(--border-subtle);
    border-radius: 20px;
    padding: 24px;
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  }

  .card:hover {
    border-color: var(--border-prominent);
    transform: translateY(-4px);
    box-shadow: 0 12px 32px rgba(0, 0, 0, 0.2);
  }

  .stat-card {
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    min-height: 180px;
  }

  .card-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    margin-bottom: 20px;
  }

  .icon-box {
    width: 44px;
    height: 44px;
    border-radius: 12px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  /* Color Themes */
  .blue .icon-box { background: rgba(59, 130, 246, 0.1); color: #3b82f6; }
  .green .icon-box { background: rgba(62, 207, 142, 0.1); color: var(--green-brand); }
  .purple .icon-box { background: rgba(139, 92, 246, 0.1); color: #8b5cf6; }
  .orange .icon-box { background: rgba(245, 158, 11, 0.1); color: #f59e0b; }

  .arrow-btn {
    background: none;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    padding: 4px;
    transition: color 0.2s;
  }

  .arrow-btn:hover { color: var(--text-primary); }

  .card-body {
    display: flex;
    flex-direction: column;
  }

  .stat-card .value {
    font-size: 36px;
    font-weight: 600;
    color: var(--text-primary);
    line-height: 1.2;
  }

  .stat-card .label {
    font-size: 13px;
    color: var(--text-muted);
    font-weight: 500;
  }

  .card-footer {
    margin-top: 16px;
    padding-top: 16px;
    border-top: 1px solid var(--border-subtle);
  }

  .trend {
    font-size: 11px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--text-muted);
  }

  .large {
    grid-column: span 2;
    min-height: 300px;
  }

  .title-with-icon {
    display: flex;
    align-items: center;
    gap: 10px;
    font-size: 15px;
    font-weight: 600;
    color: var(--text-primary);
  }

  .list-body {
    display: flex;
    flex-direction: column;
    gap: 8px;
    margin-top: 16px;
  }

  .list-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 12px 16px;
    background: var(--bg-primary);
    border: 1px solid var(--border-subtle);
    border-radius: 12px;
    cursor: pointer;
    transition: all 0.2s;
    text-align: left;
  }

  .list-item:hover {
    border-color: var(--green-border);
    background: rgba(62, 207, 142, 0.05);
  }

  .item-title {
    font-size: 14px;
    font-weight: 500;
    color: var(--text-secondary);
  }

  .item-meta {
    font-size: 11px;
    color: var(--text-muted);
  }

  .empty-list {
    padding: 40px;
    text-align: center;
    color: var(--text-muted);
    font-style: italic;
    font-size: 13px;
  }

  .loading-state {
    flex: 1;
  }

  .shimmer-grid {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 24px;
  }

  .shimmer-card {
    height: 180px;
    background: var(--bg-secondary);
    border-radius: 20px;
    position: relative;
    overflow: hidden;
  }

  .shimmer-card::after {
    content: '';
    position: absolute;
    inset: 0;
    background: linear-gradient(90deg, transparent, rgba(255,255,255,0.03), transparent);
    animation: shimmer 1.5s infinite;
  }

  @keyframes shimmer {
    0% { transform: translateX(-100%); }
    100% { transform: translateX(100%); }
  }
</style>
