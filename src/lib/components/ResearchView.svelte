<script lang="ts">
  import { researchSource } from '$lib/stores/app';
  import NoteEditor from './NoteEditor.svelte';
  import { Globe, FileText, ChevronLeft, ChevronRight, RotateCw, ExternalLink } from 'lucide-svelte';

  let sourceUrl = $researchSource;
  
  function updateSource() {
    researchSource.set(sourceUrl);
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Enter') {
      updateSource();
    }
  }
</script>

<div class="research-container">
  <div class="research-sidebar">
    <div class="source-bar">
      <div class="url-input-wrap">
        <Globe size={14} class="icon" />
        <input 
          type="text" 
          bind:value={sourceUrl} 
          onkeydown={handleKeydown} 
          placeholder="Enter URL or file path..."
        />
        <button class="icon-btn" onclick={updateSource} title="Reload">
          <RotateCw size={14} />
        </button>
      </div>
    </div>
    
    <div class="source-viewer">
      <iframe 
        src={$researchSource} 
        title="Research Source"
        frameborder="0"
        allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture"
        allowfullscreen
      ></iframe>
      
      <div class="viewer-overlay">
        <div class="overlay-msg">
          <ExternalLink size={24} />
          <p>Browsing Mode Active</p>
          <a href={$researchSource} target="_blank" rel="noopener noreferrer">Open in Browser</a>
        </div>
      </div>
    </div>
  </div>

  <div class="divider"></div>

  <div class="editor-side">
    <div class="editor-header">
      <FileText size={14} />
      <span>RESEARCH NOTES</span>
    </div>
    <div class="editor-container">
      <NoteEditor />
    </div>
  </div>
</div>

<style>
  .research-container {
    display: flex;
    height: 100%;
    overflow: hidden;
    background: var(--bg-primary);
  }

  .research-sidebar {
    flex: 1;
    display: flex;
    flex-direction: column;
    min-width: 300px;
    background: var(--bg-secondary);
  }

  .source-bar {
    padding: 12px 16px;
    background: var(--bg-primary);
    border-bottom: 1px solid var(--border-subtle);
  }

  .url-input-wrap {
    display: flex;
    align-items: center;
    gap: 10px;
    background: var(--bg-secondary);
    border: 1px solid var(--border-standard);
    border-radius: 8px;
    padding: 0 10px;
    height: 32px;
  }

  .url-input-wrap input {
    flex: 1;
    background: transparent;
    border: none;
    outline: none;
    color: var(--text-primary);
    font-size: 12px;
    font-family: var(--font-sans);
  }

  .icon { color: var(--text-muted); }

  .source-viewer {
    flex: 1;
    position: relative;
    background: #fff;
  }

  .source-viewer iframe {
    width: 100%;
    height: 100%;
    border: none;
  }

  .viewer-overlay {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(15, 15, 15, 0.6);
    backdrop-filter: blur(4px);
    display: flex;
    align-items: center;
    justify-content: center;
    opacity: 0;
    pointer-events: none;
    transition: opacity 0.3s;
  }

  .source-viewer:hover .viewer-overlay {
    opacity: 1;
    pointer-events: auto;
  }

  .overlay-msg {
    text-align: center;
    color: #fff;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 12px;
  }

  .overlay-msg a {
    color: var(--green-brand);
    font-size: 13px;
    font-weight: 600;
    text-decoration: none;
    padding: 6px 16px;
    background: rgba(62, 207, 142, 0.1);
    border: 1px solid var(--green-border);
    border-radius: 6px;
  }

  .divider {
    width: 1px;
    background: var(--border-subtle);
    cursor: col-resize;
  }

  .editor-side {
    width: 45%;
    display: flex;
    flex-direction: column;
    background: var(--bg-primary);
  }

  .editor-header {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 12px 24px;
    border-bottom: 1px solid var(--border-subtle);
    color: var(--text-muted);
    font-size: 11px;
    font-weight: 700;
    letter-spacing: 0.05em;
  }

  .editor-container {
    flex: 1;
    overflow: hidden;
  }

  .icon-btn {
    background: none;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 4px;
    border-radius: 4px;
  }

  .icon-btn:hover {
    color: var(--text-primary);
    background: var(--border-subtle);
  }
</style>
