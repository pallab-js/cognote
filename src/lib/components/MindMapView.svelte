<script lang="ts">
  import { onMount } from 'svelte';
  import { getMindmapData } from '$lib/commands';
  import { activeNoteId } from '$lib/stores/app';
  import { ZoomIn, ZoomOut, Maximize } from 'lucide-svelte';

  let svgEl: SVGElement;
  let viewportG: SVGGElement;
  let loading = false;
  let error = '';
  let renderId = 0;

  let zoom = 1;
  let panX = 0;
  let panY = 0;
  let isPanning = false;
  let startX = 0;
  let startY = 0;

  function handleMouseDown(e: MouseEvent) {
    if (loading || error || !$activeNoteId) return;
    isPanning = true;
    startX = e.clientX - panX;
    startY = e.clientY - panY;
    document.body.classList.add('panning');
  }

  function handleMouseMove(e: MouseEvent) {
    if (!isPanning) return;
    panX = e.clientX - startX;
    panY = e.clientY - startY;
  }

  function handleMouseUp() {
    isPanning = false;
    document.body.classList.remove('panning');
  }

  function handleWheel(e: WheelEvent) {
    if (loading || error || !$activeNoteId) return;
    e.preventDefault();
    const zoomIntensity = 0.05;
    const nextZoom = e.deltaY < 0 ? zoom + zoomIntensity : zoom - zoomIntensity;
    zoom = Math.max(0.3, Math.min(3, nextZoom));
  }

  function resetZoom() {
    zoom = 1;
    panX = 0;
    panY = 0;
  }

  $: if (viewportG) {
    viewportG.setAttribute('transform', `translate(${panX}, ${panY}) scale(${zoom})`);
  }

  $: {
    if ($activeNoteId) {
      clearTimeout(debounceTimer);
      debounceTimer = setTimeout(() => renderMindMap($activeNoteId), 100);
    }
  }

  let debounceTimer: ReturnType<typeof setTimeout>;

  async function renderMindMap(noteId: string) {
    if (!svgEl) return;
    const currentId = ++renderId;
    loading = true;
    error = '';
    try {
      const [d3h, data] = await Promise.all([
        import('d3-hierarchy'),
        getMindmapData(noteId),
      ]);

      if (currentId !== renderId) return; // Prevent race conditions

      const root = d3h.hierarchy(data);
      const nodeWidth = 140, nodeHeight = 36, hGap = 60, vGap = 12;
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      const treeLayout = (d3h.tree as any)().nodeSize([nodeHeight + vGap, nodeWidth + hGap]);
      treeLayout(root);

      // Compute bounds
      let minX = Infinity, maxX = -Infinity, minY = Infinity, maxY = -Infinity;
      root.each((n: any) => {
        minX = Math.min(minX, n.x); maxX = Math.max(maxX, n.x);
        minY = Math.min(minY, n.y); maxY = Math.max(maxY, n.y);
      });
      const pad = 40;
      const w = maxY - minY + nodeWidth + pad * 2;
      const h = maxX - minX + nodeHeight + pad * 2;

      // Build SVG
      const ns = 'http://www.w3.org/2000/svg';
      while (svgEl.firstChild) svgEl.removeChild(svgEl.firstChild);
      
      // Create viewport group
      viewportG = document.createElementNS(ns, 'g') as SVGGElement;
      viewportG.setAttribute('class', 'viewport');
      viewportG.setAttribute('transform', `translate(${panX}, ${panY}) scale(${zoom})`);
      svgEl.appendChild(viewportG);

      svgEl.setAttribute('viewBox', `${minY - pad} ${minX - pad} ${w} ${h}`);
      svgEl.setAttribute('width', String(w));
      svgEl.setAttribute('height', String(h));

      // Links
      root.links().forEach((link: any) => {
        const path = document.createElementNS(ns, 'path');
        const sx = link.source.y + nodeWidth / 2, sy = link.source.x;
        const tx = link.target.y, ty = link.target.x;
        const mx = (sx + tx) / 2;
        path.setAttribute('d', `M${sx},${sy} C${mx},${sy} ${mx},${ty} ${tx},${ty}`);
        path.setAttribute('fill', 'none');
        path.setAttribute('stroke', 'var(--border-prominent)');
        path.setAttribute('stroke-width', '1');
        viewportG.appendChild(path);
      });

      // Nodes
      root.each((node: any) => {
        const g = document.createElementNS(ns, 'g');
        g.setAttribute('transform', `translate(${node.y},${node.x - nodeHeight / 2})`);

        const rect = document.createElementNS(ns, 'rect');
        rect.setAttribute('width', String(nodeWidth));
        rect.setAttribute('height', String(nodeHeight));
        rect.setAttribute('rx', '6');
        rect.setAttribute('fill', 'var(--bg-primary)');
        rect.setAttribute('stroke', node.depth === 0 ? 'var(--green-brand)' : 'var(--border-standard)');
        rect.setAttribute('stroke-width', node.depth === 0 ? '1.5' : '1');
        g.appendChild(rect);

        const text = document.createElementNS(ns, 'text');
        text.setAttribute('x', String(nodeWidth / 2));
        text.setAttribute('y', String(nodeHeight / 2 + 4));
        text.setAttribute('text-anchor', 'middle');
        text.setAttribute('fill', 'var(--text-primary)');
        text.setAttribute('font-size', '12');
        text.setAttribute('font-family', 'var(--font-sans)');
        const label = node.data.label.length > 16 ? node.data.label.slice(0, 15) + '…' : node.data.label;
        text.textContent = label;
        g.appendChild(text);

        viewportG.appendChild(g);
      });
    } catch (e) {
      error = 'Failed to load mind map';
      console.error(e);
    } finally {
      if (currentId === renderId) loading = false;
    }
  }
</script>

<div class="mindmap-wrap">
  <div class="mm-header">
    <div class="left">
      <span class="mm-title">Mind Map</span>
      {#if $activeNoteId}
        <span class="mm-hint">Headings from current note</span>
      {/if}
    </div>
    
    {#if $activeNoteId && !loading && !error}
      <div class="center-controls">
        <div class="control-group">
          <button class="ctrl-btn" onclick={() => zoom = Math.min(3, zoom + 0.1)} title="Zoom In"><ZoomIn size={14}/></button>
          <button class="ctrl-btn" onclick={() => zoom = Math.max(0.3, zoom - 0.1)} title="Zoom Out"><ZoomOut size={14}/></button>
          <button class="ctrl-btn" onclick={resetZoom} title="Reset Zoom"><Maximize size={14}/></button>
        </div>
      </div>
    {/if}
  </div>
  
  {#if !$activeNoteId}
    <div class="empty">Select a note to see its mind map</div>
  {:else if loading}
    <div class="empty">Loading...</div>
  {:else if error}
    <div class="empty">{error}</div>
  {:else}
    <div 
      class="svg-wrap"
      onmousedown={handleMouseDown}
      onmousemove={handleMouseMove}
      onmouseup={handleMouseUp}
      onmouseleave={handleMouseUp}
      onwheel={handleWheel}
      role="presentation"
    >
      <svg bind:this={svgEl} class="mindmap-svg"></svg>
    </div>
  {/if}
</div>

<style>
  .mindmap-wrap { display: flex; flex-direction: column; height: 100%; overflow: hidden; background: var(--bg-primary); }
  
  .mm-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 12px 20px;
    border-bottom: 1px solid var(--border-subtle);
    background: var(--bg-secondary);
    z-index: 10;
    flex-shrink: 0;
  }

  .left {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .center-controls {
    display: flex;
    align-items: center;
    gap: 12px;
    background: var(--bg-primary);
    padding: 4px 12px;
    border-radius: 10px;
    border: 1px solid var(--border-standard);
  }

  .control-group {
    display: flex;
    align-items: center;
    gap: 4px;
  }

  .ctrl-btn {
    background: none;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    padding: 6px;
    border-radius: 6px;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.2s;
  }

  .ctrl-btn:hover {
    color: var(--text-primary);
    background: var(--border-subtle);
  }

  .mm-title { font-size: 11px; font-family: var(--font-mono); text-transform: uppercase; letter-spacing: 1.2px; color: var(--text-muted); }
  .mm-hint { font-size: 11px; color: var(--text-muted); }
  
  .svg-wrap {
    flex: 1;
    overflow: hidden;
    padding: 16px;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: grab;
    user-select: none;
    position: relative;
    background: var(--bg-primary);
  }

  .svg-wrap:active {
    cursor: grabbing;
  }

  :global(body.panning) {
    cursor: grabbing !important;
    user-select: none;
  }

  .mindmap-svg {
    display: block;
    width: 100%;
    height: 100%;
  }

  .empty { flex: 1; display: flex; align-items: center; justify-content: center; color: var(--text-muted); font-size: 13px; }
</style>
