<script lang="ts">
  import { onMount } from 'svelte';
  import { getMindmapData } from '$lib/commands';
  import { activeNoteId } from '$lib/stores/app';

  let svgEl: SVGElement;
  let loading = false;
  let error = '';

  $: if ($activeNoteId) renderMindMap($activeNoteId);

  async function renderMindMap(noteId: string) {
    if (!svgEl) return;
    loading = true;
    error = '';
    try {
      const [d3h, data] = await Promise.all([
        import('d3-hierarchy'),
        getMindmapData(noteId),
      ]);

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
        path.setAttribute('stroke', '#2e2e2e');
        path.setAttribute('stroke-width', '1');
        svgEl.appendChild(path);
      });

      // Nodes
      root.each((node: any) => {
        const g = document.createElementNS(ns, 'g');
        g.setAttribute('transform', `translate(${node.y},${node.x - nodeHeight / 2})`);

        const rect = document.createElementNS(ns, 'rect');
        rect.setAttribute('width', String(nodeWidth));
        rect.setAttribute('height', String(nodeHeight));
        rect.setAttribute('rx', '6');
        rect.setAttribute('fill', '#171717');
        rect.setAttribute('stroke', node.depth === 0 ? '#3ecf8e' : '#2e2e2e');
        rect.setAttribute('stroke-width', node.depth === 0 ? '1.5' : '1');
        g.appendChild(rect);

        const text = document.createElementNS(ns, 'text');
        text.setAttribute('x', String(nodeWidth / 2));
        text.setAttribute('y', String(nodeHeight / 2 + 4));
        text.setAttribute('text-anchor', 'middle');
        text.setAttribute('fill', '#fafafa');
        text.setAttribute('font-size', '12');
        text.setAttribute('font-family', 'Circular, Helvetica Neue, Arial, sans-serif');
        const label = node.data.label.length > 16 ? node.data.label.slice(0, 15) + '…' : node.data.label;
        text.textContent = label;
        g.appendChild(text);

        svgEl.appendChild(g);
      });
    } catch (e) {
      error = 'Failed to load mind map';
      console.error(e);
    } finally {
      loading = false;
    }
  }
</script>

<div class="mindmap-wrap">
  <div class="mm-header">
    <span class="mm-title">Mind Map</span>
    {#if $activeNoteId}
      <span class="mm-hint">Headings from current note</span>
    {/if}
  </div>
  {#if !$activeNoteId}
    <div class="empty">Select a note to see its mind map</div>
  {:else if loading}
    <div class="empty">Loading...</div>
  {:else if error}
    <div class="empty">{error}</div>
  {:else}
    <div class="svg-wrap">
      <svg bind:this={svgEl} class="mindmap-svg"></svg>
    </div>
  {/if}
</div>

<style>
  .mindmap-wrap { display: flex; flex-direction: column; height: 100%; overflow: hidden; }
  .mm-header {
    display: flex; align-items: center; gap: 8px;
    padding: 12px 16px; border-bottom: 1px solid var(--border-subtle); flex-shrink: 0;
  }
  .mm-title { font-size: 11px; font-family: var(--font-mono); text-transform: uppercase; letter-spacing: 1.2px; color: var(--text-muted); }
  .mm-hint { font-size: 11px; color: var(--text-muted); }
  .svg-wrap { flex: 1; overflow: auto; padding: 16px; display: flex; align-items: flex-start; justify-content: center; }
  .mindmap-svg { display: block; }
  .empty { flex: 1; display: flex; align-items: center; justify-content: center; color: var(--text-muted); font-size: 13px; }
</style>
