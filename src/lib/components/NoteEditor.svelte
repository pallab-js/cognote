<!-- ANCHOR: EDITOR_WIKI_LINKS_ACTIVE -->
<script lang="ts">
  import { activeNoteId } from '$lib/stores/app';
</script>

<div class="placeholder">
  {#if $activeNoteId}
    <span>Editor — Phase 4 (note: {$activeNoteId})</span>
  {:else}
    <span>Select or create a note</span>
  {/if}
</div>

<style>
  .placeholder { display: flex; align-items: center; justify-content: center; height: 100%; color: var(--text-muted); font-size: 14px; }
</style>
