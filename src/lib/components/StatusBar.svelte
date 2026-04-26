<script lang="ts">
  import { wordCount, lastSaved } from '$lib/stores/notes';

  $: savedText = $lastSaved
    ? `Last saved: ${formatRelative($lastSaved)}`
    : 'Not saved yet';

  function formatRelative(d: Date): string {
    const diff = (Date.now() - d.getTime()) / 1000;
    if (diff < 5) return 'just now';
    if (diff < 60) return `${Math.floor(diff)}s ago`;
    if (diff < 3600) return `${Math.floor(diff / 60)}m ago`;
    return d.toLocaleTimeString();
  }
</script>

<div class="status-bar">
  <span class="stat">{$wordCount} words</span>
  <span class="sep">·</span>
  <span class="stat">{savedText}</span>
  <span class="spacer"/>
  <span class="local-badge">● Local</span>
</div>

<style>
  .status-bar {
    display: flex; align-items: center; gap: 8px;
    padding: 0 16px; height: 28px;
    border-top: 1px solid var(--border-subtle);
    background: var(--bg-button);
    font-size: 11px; color: var(--text-muted);
    font-family: var(--font-mono); flex-shrink: 0;
  }
  .spacer { flex: 1; }
  .sep { color: var(--border-prominent); }
  .local-badge { color: var(--green-brand); letter-spacing: 0.5px; }
</style>
