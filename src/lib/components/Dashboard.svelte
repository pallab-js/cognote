<!-- ANCHOR: DASHBOARD_READY -->
<script lang="ts">
  import { onMount } from 'svelte';
  import { getDailyStats } from '$lib/commands';
  import type { DailyStats } from '$lib/commands';
  import { Flame, FileText, Calendar, TrendingUp } from 'lucide-svelte';

  let stats: DailyStats | null = null;
  let loading = true;

  onMount(async () => {
    try {
      stats = await getDailyStats();
    } catch (e) {
      console.error('Failed to load stats:', e);
    } finally {
      loading = false;
    }
  });

  // Build last 30 days activity grid
  $: activityDays = buildActivityGrid(stats?.recent_days ?? []);

  function buildActivityGrid(days: { date: string; count: number }[]) {
    const map = new Map(days.map(d => [d.date, d.count]));
    const result = [];
    const today = new Date();
    for (let i = 29; i >= 0; i--) {
      const d = new Date(today);
      d.setDate(d.getDate() - i);
      const key = d.toISOString().slice(0, 10);
      result.push({ date: key, count: map.get(key) ?? 0 });
    }
    return result;
  }

  function barHeight(count: number, max: number): number {
    if (max === 0) return 0;
    return Math.max(2, Math.round((count / max) * 48));
  }

  $: maxCount = Math.max(...(stats?.recent_days.map(d => d.count) ?? [0]), 1);
</script>

<div class="dashboard">
  <div class="dash-header">
    <span class="dash-title">Dashboard</span>
  </div>

  {#if loading}
    <div class="loading">Loading stats...</div>
  {:else if stats}
    <!-- Stat cards -->
    <div class="stat-cards">
      <div class="stat-card">
        <div class="stat-icon"><FileText size={18} color="var(--green-brand)"/></div>
        <div class="stat-body">
          <span class="stat-value">{stats.total_notes}</span>
          <span class="stat-label">Total Notes</span>
        </div>
      </div>
      <div class="stat-card">
        <div class="stat-icon"><Calendar size={18} color="var(--green-brand)"/></div>
        <div class="stat-body">
          <span class="stat-value">{stats.today_count}</span>
          <span class="stat-label">Today</span>
        </div>
      </div>
      <div class="stat-card">
        <div class="stat-icon"><Flame size={18} color="var(--green-brand)"/></div>
        <div class="stat-body">
          <span class="stat-value">{stats.streak}</span>
          <span class="stat-label">Day Streak</span>
        </div>
      </div>
    </div>

    <!-- Activity chart -->
    <div class="section">
      <div class="section-header">
        <TrendingUp size={13} color="var(--text-muted)"/>
        <span class="section-title">Activity — Last 30 Days</span>
      </div>
      <div class="bar-chart">
        {#each activityDays as day}
          <div class="bar-col" title="{day.date}: {day.count} notes">
            <div
              class="bar"
              style="height: {barHeight(day.count, maxCount)}px; background: {day.count > 0 ? 'var(--green-brand)' : 'var(--border-standard)'}; opacity: {day.count > 0 ? 0.4 + (day.count / maxCount) * 0.6 : 1}"
            ></div>
          </div>
        {/each}
      </div>
      <div class="chart-labels">
        <span>30 days ago</span>
        <span>Today</span>
      </div>
    </div>

    <!-- Recent days table -->
    {#if stats.recent_days.length > 0}
      <div class="section">
        <div class="section-header">
          <span class="section-title">Recent Activity</span>
        </div>
        <div class="activity-list">
          {#each stats.recent_days.slice(0, 7) as day}
            <div class="activity-row">
              <span class="activity-date">{day.date}</span>
              <div class="activity-bar-wrap">
                <div class="activity-bar" style="width: {Math.round((day.count / maxCount) * 100)}%"></div>
              </div>
              <span class="activity-count">{day.count}</span>
            </div>
          {/each}
        </div>
      </div>
    {/if}
  {/if}
</div>

<style>
  .dashboard { display: flex; flex-direction: column; height: 100%; overflow-y: auto; padding: 24px; gap: 24px; }
  .dash-header { flex-shrink: 0; }
  .dash-title { font-size: 11px; font-family: var(--font-mono); text-transform: uppercase; letter-spacing: 1.2px; color: var(--text-muted); }

  .stat-cards { display: grid; grid-template-columns: repeat(3, 1fr); gap: 12px; }
  .stat-card {
    background: var(--bg-button); border: 1px solid var(--border-standard);
    border-radius: 8px; padding: 16px; display: flex; align-items: center; gap: 12px;
  }
  .stat-icon { flex-shrink: 0; }
  .stat-body { display: flex; flex-direction: column; gap: 2px; }
  .stat-value { font-size: 28px; font-weight: 400; color: var(--text-primary); line-height: 1; }
  .stat-label { font-size: 11px; color: var(--text-muted); font-family: var(--font-mono); text-transform: uppercase; letter-spacing: 0.8px; }

  .section { background: var(--bg-button); border: 1px solid var(--border-standard); border-radius: 8px; padding: 16px; }
  .section-header { display: flex; align-items: center; gap: 6px; margin-bottom: 12px; }
  .section-title { font-size: 11px; font-family: var(--font-mono); text-transform: uppercase; letter-spacing: 1.2px; color: var(--text-muted); }

  .bar-chart { display: flex; align-items: flex-end; gap: 3px; height: 56px; }
  .bar-col { flex: 1; display: flex; align-items: flex-end; }
  .bar { width: 100%; border-radius: 2px 2px 0 0; transition: height 0.2s; min-height: 2px; }
  .chart-labels { display: flex; justify-content: space-between; margin-top: 4px; font-size: 10px; color: var(--text-muted); }

  .activity-list { display: flex; flex-direction: column; gap: 6px; }
  .activity-row { display: flex; align-items: center; gap: 12px; }
  .activity-date { font-size: 11px; color: var(--text-muted); font-family: var(--font-mono); width: 90px; flex-shrink: 0; }
  .activity-bar-wrap { flex: 1; height: 6px; background: var(--border-subtle); border-radius: 3px; overflow: hidden; }
  .activity-bar { height: 100%; background: var(--green-brand); border-radius: 3px; opacity: 0.7; }
  .activity-count { font-size: 11px; color: var(--text-secondary); width: 24px; text-align: right; flex-shrink: 0; }

  .loading { display: flex; align-items: center; justify-content: center; flex: 1; color: var(--text-muted); font-size: 13px; }
</style>
