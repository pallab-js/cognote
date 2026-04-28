<script lang="ts">
  import TopBar from './TopBar.svelte';
  import LeftSidebar from './LeftSidebar.svelte';
  import RightPanel from './RightPanel.svelte';
  import BottomPanel from './BottomPanel.svelte';
  import StatusBar from '../StatusBar.svelte';
  import { leftSidebarOpen, rightPanelOpen, bottomPanelOpen } from '$lib/stores/app';
  import { PanelLeftClose, PanelLeftOpen, PanelRightClose, PanelRightOpen, PanelBottom } from 'lucide-svelte';

  let { children } = $props();

  let sidebarWidth = $state(260);
  let isResizing = $state(false);

  function startResizing(e: MouseEvent) {
    isResizing = true;
    document.body.classList.add('resizing');
    window.addEventListener('mousemove', handleMouseMove);
    window.addEventListener('mouseup', stopResizing);
  }

  function handleMouseMove(e: MouseEvent) {
    if (!isResizing) return;
    const newWidth = e.clientX;
    if (newWidth > 180 && newWidth < 500) {
      sidebarWidth = newWidth;
    }
  }

  function stopResizing() {
    isResizing = false;
    document.body.classList.remove('resizing');
    window.removeEventListener('mousemove', handleMouseMove);
    window.removeEventListener('mouseup', stopResizing);
  }

  function toggleLeft() { leftSidebarOpen.update(v => !v); }
  function toggleRight() { rightPanelOpen.update(v => !v); }
  function toggleBottom() { bottomPanelOpen.update(v => !v); }
</script>

<div class="layout-container">
  <TopBar />
  
  <div class="middle-section">
    <div class="sidebar-wrapper" style="width: {$leftSidebarOpen ? sidebarWidth : 0}px">
      <LeftSidebar />
      {#if $leftSidebarOpen}
        <div class="resizer" onmousedown={startResizing} role="presentation"></div>
      {/if}
    </div>
    
    <main class="main-content">
      <div class="content-area">
        {@render children()}
      </div>

      <div class="layout-controls">
        <button class="control-btn" onclick={toggleLeft} title="Toggle Sidebar">
          {#if $leftSidebarOpen}<PanelLeftClose size={16}/>{:else}<PanelLeftOpen size={16}/>{/if}
        </button>
        <button class="control-btn" onclick={toggleBottom} title="Toggle Bottom Panel">
          <PanelBottom size={16} />
        </button>
        <button class="control-btn" onclick={toggleRight} title="Toggle Right Panel">
          {#if $rightPanelOpen}<PanelRightClose size={16}/>{:else}<PanelRightOpen size={16}/>{/if}
        </button>
      </div>

      <BottomPanel />
    </main>

    <div class="right-panel-wrapper" style="width: {$rightPanelOpen ? 300 : 0}px">
      <RightPanel />
    </div>
  </div>

  <StatusBar />
</div>

<style>
  .layout-container {
    display: flex;
    flex-direction: column;
    height: 100vh;
    overflow: hidden;
    background: var(--bg-primary);
  }

  .middle-section {
    flex: 1;
    display: flex;
    overflow: hidden;
    position: relative;
  }

  .sidebar-wrapper, .right-panel-wrapper {
    overflow: hidden;
    transition: width 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    position: relative;
    flex-shrink: 0;
  }

  :global(body.resizing) .sidebar-wrapper {
    transition: none; /* Disable transition while dragging */
  }

  :global(body.resizing) {
    cursor: col-resize !important;
    user-select: none;
  }

  .resizer {
    position: absolute;
    right: -3px;
    top: 0;
    bottom: 0;
    width: 6px;
    cursor: col-resize;
    z-index: 100;
    transition: background 0.2s;
  }

  .resizer:hover {
    background: var(--green-brand);
    opacity: 0.5;
  }

  .main-content {
    flex: 1;
    display: flex;
    flex-direction: column;
    position: relative;
    overflow: hidden;
    background: var(--bg-primary);
    border-left: 1px solid var(--border-subtle);
    border-right: 1px solid var(--border-subtle);
  }

  .content-area {
    flex: 1;
    overflow: auto;
    position: relative;
    display: flex;
    flex-direction: column;
  }

  .layout-controls {
    position: absolute;
    bottom: 20px;
    left: 50%;
    transform: translateX(-50%);
    display: flex;
    gap: 8px;
    padding: 6px;
    background: var(--surface-glass);
    backdrop-filter: blur(8px);
    border: 1px solid var(--border-subtle);
    border-radius: 12px;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.4);
    z-index: 1000;
    transition: opacity 0.3s, transform 0.3s;
  }

  .layout-controls:hover {
    transform: translateX(-50%) translateY(-2px);
  }

  .control-btn {
    background: none;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    padding: 8px;
    border-radius: 8px;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.2s;
  }

  .control-btn:hover {
    color: var(--text-primary);
    background: var(--border-subtle);
  }
</style>
