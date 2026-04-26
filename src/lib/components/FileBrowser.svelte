<!-- ANCHOR: FILE_ORGANISER_READY -->
<script lang="ts">
  import { onMount } from 'svelte';
  import { Upload, File, Image, FileText, Trash2, ExternalLink, Grid, List } from 'lucide-svelte';
  import { listFiles, importFile, deleteFile, openFileExternal } from '$lib/commands';
  import type { FileInfo } from '$lib/commands';
  import { activeNotebookId } from '$lib/stores/app';
  import { convertFileSrc } from '@tauri-apps/api/core';

  let files: FileInfo[] = [];
  let viewMode: 'grid' | 'list' = 'grid';
  let isDragging = false;
  let loading = false;
  let previewFile: FileInfo | null = null;

  onMount(() => { refresh(); });
  $: $activeNotebookId, refresh();

  async function refresh() {
    try {
      files = await listFiles($activeNotebookId ?? undefined);
    } catch (e) {
      console.error('Failed to list files:', e);
    }
  }

  async function handleDrop(e: DragEvent) {
    e.preventDefault();
    isDragging = false;
    const items = e.dataTransfer?.files;
    if (!items?.length) return;
    loading = true;
    try {
      for (const file of Array.from(items)) {
        await importFile(file.name, $activeNotebookId ?? undefined);
      }
      await refresh();
    } catch (err) {
      console.error('Import failed:', err);
    } finally {
      loading = false;
    }
  }

  async function remove(id: string) {
    if (!confirm('Delete this file?')) return;
    await deleteFile(id);
    if (previewFile?.id === id) previewFile = null;
    await refresh();
  }

  async function openExternal(id: string) {
    try { await openFileExternal(id); } catch (e) { console.error(e); }
  }

  function isImage(f: FileInfo) {
    return f.mime_type?.startsWith('image/') || /\.(png|jpg|jpeg|gif|webp|svg)$/i.test(f.name);
  }

  function isPdf(f: FileInfo) {
    return f.mime_type === 'application/pdf' || f.name.endsWith('.pdf');
  }

  function formatSize(bytes: number | null): string {
    if (!bytes) return '';
    if (bytes < 1024) return `${bytes} B`;
    if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
    return `${(bytes / 1024 / 1024).toFixed(1)} MB`;
  }

  function getIcon(f: FileInfo) {
    if (isImage(f)) return Image;
    if (f.mime_type?.includes('text') || /\.(md|txt|json)$/i.test(f.name)) return FileText;
    return File;
  }
</script>

<div
  class="file-browser"
  on:dragover|preventDefault={() => isDragging = true}
  on:dragleave={() => isDragging = false}
  on:drop={handleDrop}
  role="region"
  aria-label="File browser"
>
  <!-- Header -->
  <div class="fb-header">
    <span class="fb-title">Files</span>
    <span class="fb-count">{files.length}</span>
    <div class="fb-actions">
      <button class="icon-btn" class:active={viewMode === 'grid'} on:click={() => viewMode = 'grid'} title="Grid view"><Grid size={13}/></button>
      <button class="icon-btn" class:active={viewMode === 'list'} on:click={() => viewMode = 'list'} title="List view"><List size={13}/></button>
    </div>
  </div>

  <!-- Drop zone hint -->
  {#if isDragging}
    <div class="drop-overlay">
      <Upload size={32} color="var(--green-brand)"/>
      <span>Drop files to import</span>
    </div>
  {/if}

  <!-- File grid/list -->
  {#if files.length === 0}
    <div class="empty-drop">
      <Upload size={24} color="var(--text-muted)"/>
      <p>Drag & drop files here</p>
      <p class="hint">Images, PDFs, text files</p>
    </div>
  {:else if viewMode === 'grid'}
    <div class="file-grid">
      {#each files as f (f.id)}
        <div
          class="file-card"
          class:selected={previewFile?.id === f.id}
          role="button"
          tabindex="0"
          on:click={() => previewFile = previewFile?.id === f.id ? null : f}
          on:keydown={e => e.key === 'Enter' && (previewFile = f)}
        >
          {#if isImage(f)}
            <div class="thumb-wrap">
              <img src={convertFileSrc(f.path)} alt={f.name} class="thumb" on:error={e => (e.currentTarget as HTMLImageElement).style.display='none'}/>
            </div>
          {:else}
            <div class="file-icon-wrap">
              <svelte:component this={getIcon(f)} size={28} color="var(--text-muted)"/>
            </div>
          {/if}
          <span class="file-name" title={f.name}>{f.name}</span>
          <span class="file-size">{formatSize(f.size)}</span>
          <div class="card-actions">
            <button class="icon-btn" title="Open externally" on:click|stopPropagation={() => openExternal(f.id)}><ExternalLink size={11}/></button>
            <button class="icon-btn danger" title="Delete" on:click|stopPropagation={() => remove(f.id)}><Trash2 size={11}/></button>
          </div>
        </div>
      {/each}
    </div>
  {:else}
    <div class="file-list">
      {#each files as f (f.id)}
        <div
          class="file-row"
          class:selected={previewFile?.id === f.id}
          role="button"
          tabindex="0"
          on:click={() => previewFile = previewFile?.id === f.id ? null : f}
          on:keydown={e => e.key === 'Enter' && (previewFile = f)}
        >
          <svelte:component this={getIcon(f)} size={14} color="var(--text-muted)"/>
          <span class="file-name-row">{f.name}</span>
          <span class="file-size-row">{formatSize(f.size)}</span>
          <button class="icon-btn" title="Open" on:click|stopPropagation={() => openExternal(f.id)}><ExternalLink size={11}/></button>
          <button class="icon-btn danger" title="Delete" on:click|stopPropagation={() => remove(f.id)}><Trash2 size={11}/></button>
        </div>
      {/each}
    </div>
  {/if}

  <!-- Preview panel -->
  {#if previewFile}
    <div class="preview-panel">
      <div class="preview-header">
        <span class="preview-name">{previewFile.name}</span>
        <button class="icon-btn" on:click={() => previewFile = null}>✕</button>
      </div>
      {#if isImage(previewFile)}
        <img src={convertFileSrc(previewFile.path)} alt={previewFile.name} class="preview-img"/>
      {:else if isPdf(previewFile)}
        <iframe src={convertFileSrc(previewFile.path)} title={previewFile.name} class="preview-iframe"></iframe>
      {:else}
        <div class="preview-fallback">
          <svelte:component this={getIcon(previewFile)} size={48} color="var(--text-muted)"/>
          <p>{previewFile.name}</p>
          <button class="open-btn" on:click={() => openExternal(previewFile!.id)}>
            <ExternalLink size={13}/> Open in system viewer
          </button>
        </div>
      {/if}
    </div>
  {/if}
</div>

<style>
  .file-browser { display: flex; flex-direction: column; height: 100%; overflow: hidden; position: relative; }

  .fb-header {
    display: flex; align-items: center; gap: 8px;
    padding: 12px 16px; border-bottom: 1px solid var(--border-subtle); flex-shrink: 0;
  }
  .fb-title { font-size: 11px; font-family: var(--font-mono); text-transform: uppercase; letter-spacing: 1.2px; color: var(--text-muted); }
  .fb-count { font-size: 11px; color: var(--text-muted); background: var(--border-standard); padding: 1px 6px; border-radius: 9999px; }
  .fb-actions { display: flex; gap: 4px; margin-left: auto; }

  .drop-overlay {
    position: absolute; inset: 0; z-index: 10;
    background: rgba(62, 207, 142, 0.05); border: 2px dashed var(--green-brand);
    display: flex; flex-direction: column; align-items: center; justify-content: center;
    gap: 12px; font-size: 14px; color: var(--green-brand); border-radius: 8px;
  }

  .empty-drop {
    flex: 1; display: flex; flex-direction: column; align-items: center; justify-content: center;
    gap: 8px; color: var(--text-muted); border: 2px dashed var(--border-standard);
    margin: 16px; border-radius: 8px;
  }
  .empty-drop p { font-size: 13px; }
  .empty-drop .hint { font-size: 11px; }

  .file-grid {
    flex: 1; overflow-y: auto; padding: 12px;
    display: grid; grid-template-columns: repeat(auto-fill, minmax(100px, 1fr)); gap: 8px;
  }
  .file-card {
    background: var(--bg-button); border: 1px solid var(--border-standard);
    border-radius: 8px; padding: 8px; cursor: pointer;
    display: flex; flex-direction: column; align-items: center; gap: 4px;
    position: relative; transition: border-color 0.1s;
  }
  .file-card:hover { border-color: var(--border-prominent); }
  .file-card.selected { border-color: var(--green-border); }
  .thumb-wrap { width: 64px; height: 64px; overflow: hidden; border-radius: 4px; }
  .thumb { width: 100%; height: 100%; object-fit: cover; }
  .file-icon-wrap { width: 64px; height: 64px; display: flex; align-items: center; justify-content: center; }
  .file-name { font-size: 11px; color: var(--text-secondary); text-align: center; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; width: 100%; }
  .file-size { font-size: 10px; color: var(--text-muted); }
  .card-actions { display: none; position: absolute; top: 4px; right: 4px; gap: 2px; }
  .file-card:hover .card-actions { display: flex; }

  .file-list { flex: 1; overflow-y: auto; }
  .file-row {
    display: flex; align-items: center; gap: 8px;
    padding: 8px 16px; border-bottom: 1px solid var(--border-subtle);
    cursor: pointer; transition: background 0.1s;
  }
  .file-row:hover { background: var(--border-subtle); }
  .file-row.selected { background: var(--border-standard); border-left: 2px solid var(--green-brand); }
  .file-name-row { flex: 1; font-size: 13px; color: var(--text-primary); overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
  .file-size-row { font-size: 11px; color: var(--text-muted); flex-shrink: 0; }

  .icon-btn { background: none; border: none; cursor: pointer; color: var(--text-muted); padding: 3px; border-radius: 4px; display: flex; align-items: center; }
  .icon-btn:hover { color: var(--text-primary); }
  .icon-btn.active { color: var(--green-brand); }
  .icon-btn.danger:hover { color: #ff6b6b; }

  .preview-panel {
    border-top: 1px solid var(--border-subtle); flex-shrink: 0;
    max-height: 300px; display: flex; flex-direction: column; overflow: hidden;
  }
  .preview-header {
    display: flex; align-items: center; justify-content: space-between;
    padding: 8px 12px; border-bottom: 1px solid var(--border-subtle);
    flex-shrink: 0;
  }
  .preview-name { font-size: 12px; color: var(--text-secondary); overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
  .preview-img { max-width: 100%; max-height: 240px; object-fit: contain; margin: auto; display: block; padding: 8px; }
  .preview-iframe { width: 100%; height: 240px; border: none; }
  .preview-fallback { display: flex; flex-direction: column; align-items: center; justify-content: center; gap: 8px; padding: 24px; color: var(--text-muted); }
  .open-btn {
    display: flex; align-items: center; gap: 6px;
    padding: 6px 16px; border-radius: 9999px;
    border: 1px solid var(--border-standard); background: none;
    color: var(--text-secondary); font-size: 12px; cursor: pointer;
  }
  .open-btn:hover { border-color: var(--green-border); color: var(--green-brand); }
</style>
