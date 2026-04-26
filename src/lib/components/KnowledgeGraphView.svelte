<!-- ANCHOR: GRAPH_RENDERED -->
<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { getKnowledgeGraph } from '$lib/commands';
  import { activeNoteId } from '$lib/stores/app';
  import { RefreshCw } from 'lucide-svelte';

  let container: HTMLElement;
  let cy: any = null;
  let loading = true;
  let nodeCount = 0;
  let edgeCount = 0;

  onMount(async () => {
    const [cytoscape, dagre, cytoscapeDagre] = await Promise.all([
      import('cytoscape'),
      import('dagre'),
      import('cytoscape-dagre'),
    ]);
    cytoscape.default.use(cytoscapeDagre.default);
    await loadGraph(cytoscape.default);
  });

  onDestroy(() => { cy?.destroy(); });

  async function loadGraph(cytoscape: any) {
    loading = true;
    try {
      const data = await getKnowledgeGraph();
      nodeCount = data.nodes.length;
      edgeCount = data.edges.length;

      cy?.destroy();
      cy = cytoscape({
        container,
        elements: [
          ...data.nodes.map(n => ({ data: { id: n.id, label: n.label } })),
          ...data.edges.map(e => ({ data: { source: e.source, target: e.target } })),
        ],
        style: [
          {
            selector: 'node',
            style: {
              'background-color': '#171717',
              'border-width': 1,
              'border-color': '#2e2e2e',
              'color': '#fafafa',
              'font-size': '12px',
              'font-family': 'Circular, Helvetica Neue, Arial, sans-serif',
              'label': 'data(label)',
              'text-valign': 'center',
              'text-halign': 'center',
              'width': 'label',
              'height': 'label',
              'padding': '8px',
              'shape': 'roundrectangle',
              'text-wrap': 'wrap',
              'text-max-width': '120px',
            },
          },
          {
            selector: 'node:selected',
            style: {
              'border-color': '#3ecf8e',
              'border-width': 2,
              'background-color': '#1a2e24',
            },
          },
          {
            selector: 'node.active',
            style: {
              'border-color': '#3ecf8e',
              'border-width': 2,
              'background-color': '#1a2e24',
            },
          },
          {
            selector: 'edge',
            style: {
              'width': 1,
              'line-color': '#363636',
              'target-arrow-color': '#363636',
              'target-arrow-shape': 'triangle',
              'curve-style': 'bezier',
              'arrow-scale': 0.8,
            },
          },
        ],
        layout: {
          name: data.nodes.length > 0 ? 'dagre' : 'grid',
          rankDir: 'LR',
          nodeSep: 40,
          rankSep: 80,
          padding: 20,
        },
        wheelSensitivity: 0.3,
      });

      // Click node → open note
      cy.on('tap', 'node', (evt: any) => {
        activeNoteId.set(evt.target.id());
      });

      // Highlight active note
      const unsub = activeNoteId.subscribe(id => {
        cy?.nodes().removeClass('active');
        if (id) cy?.getElementById(id).addClass('active');
      });

    } catch (e) {
      console.error('Graph load failed:', e);
    } finally {
      loading = false;
    }
  }

  async function refresh() {
    if (!cy) return;
    const cytoscape = (await import('cytoscape')).default;
    await loadGraph(cytoscape);
  }
</script>

<div class="graph-wrap">
  <div class="graph-header">
    <span class="graph-title">Knowledge Graph</span>
    <span class="graph-stats">{nodeCount} notes · {edgeCount} links</span>
    <button class="icon-btn" title="Refresh" on:click={refresh}><RefreshCw size={13}/></button>
  </div>
  {#if loading}
    <div class="loading">Loading graph...</div>
  {/if}
  <div class="cy-container" bind:this={container}></div>
  {#if !loading && nodeCount === 0}
    <div class="empty">Create notes and link them with <code>[[</code> to see the graph</div>
  {/if}
</div>

<style>
  .graph-wrap { display: flex; flex-direction: column; height: 100%; overflow: hidden; }
  .graph-header {
    display: flex; align-items: center; gap: 8px;
    padding: 12px 16px; border-bottom: 1px solid var(--border-subtle); flex-shrink: 0;
  }
  .graph-title { font-size: 11px; font-family: var(--font-mono); text-transform: uppercase; letter-spacing: 1.2px; color: var(--text-muted); }
  .graph-stats { font-size: 11px; color: var(--text-muted); }
  .icon-btn { background: none; border: none; cursor: pointer; color: var(--text-muted); padding: 4px; border-radius: 4px; display: flex; align-items: center; margin-left: auto; }
  .icon-btn:hover { color: var(--text-primary); }
  .cy-container { flex: 1; background: var(--bg-primary); }
  .loading { position: absolute; top: 50%; left: 50%; transform: translate(-50%, -50%); color: var(--text-muted); font-size: 13px; }
  .empty { position: absolute; top: 50%; left: 50%; transform: translate(-50%, -50%); color: var(--text-muted); font-size: 13px; text-align: center; }
  .empty code { font-family: var(--font-mono); color: var(--green-brand); }
</style>
