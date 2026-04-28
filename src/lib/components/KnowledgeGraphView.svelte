<!-- ANCHOR: GRAPH_RENDERED -->
<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { getKnowledgeGraph } from '$lib/commands';
  import { activeNoteId, currentView } from '$lib/stores/app';
  import { 
    RefreshCw, 
    ZoomIn, 
    ZoomOut, 
    Maximize, 
    Filter, 
    Target, 
    Settings2 
  } from 'lucide-svelte';

  let container: HTMLElement;
  let cy: any = null;
  let loading = true;
  let nodeCount = 0;
  let edgeCount = 0;
  let unsubActive: (() => void) | null = null;
  let focusMode = false;
  let layoutName = 'dagre';

  onMount(async () => {
    const [cytoscape, dagre, cytoscapeDagre] = await Promise.all([
      import('cytoscape'),
      import('dagre'),
      import('cytoscape-dagre'),
    ]);
    cytoscape.default.use(cytoscapeDagre.default);
    await loadGraph(cytoscape.default);
  });

  onDestroy(() => {
    if (unsubActive) unsubActive();
    cy?.destroy();
  });

  async function loadGraph(cytoscape: any) {
    loading = true;
    try {
      const data = await getKnowledgeGraph();
      nodeCount = data.nodes.length;
      edgeCount = data.edges.length;

      cy?.destroy();
      cy = cytoscape({
        container,
        textureOnViewport: nodeCount > 100,
        hideEdgesOnViewport: nodeCount > 100,
        hideLabelsOnViewport: nodeCount > 200,
        minZoom: 0.1,
        maxZoom: 4,
        elements: [
          ...data.nodes.map(n => ({ data: { id: n.id, label: n.label } })),
          ...data.edges.map(e => ({ data: { source: e.source, target: e.target } })),
        ],
        style: [
          {
            selector: 'node',
            style: {
              'background-color': '#1f1f1f',
              'border-width': 1.5,
              'border-color': 'var(--border-prominent)',
              'color': 'var(--text-secondary)',
              'font-size': '10px',
              'font-family': 'var(--font-sans)',
              'label': 'data(label)',
              'text-valign': 'bottom',
              'text-margin-y': '6px',
              'width': '12px',
              'height': '12px',
              'shape': 'ellipse',
              'transition-property': 'background-color, border-color, width, height',
              'transition-duration': '0.2s',
            },
          },
          {
            selector: 'node:selected',
            style: {
              'border-color': 'var(--green-brand)',
              'border-width': 2,
              'background-color': 'var(--green-brand)',
              'width': '16px',
              'height': '16px',
              'color': 'var(--text-primary)',
              'font-weight': 'bold',
            },
          },
          {
            selector: 'node.active',
            style: {
              'border-color': 'var(--green-brand)',
              'border-width': 3,
              'background-color': 'var(--green-brand)',
              'width': '18px',
              'height': '18px',
              'color': 'var(--text-primary)',
              'font-weight': '600',
              'overlay-opacity': 0.1,
              'overlay-color': 'var(--green-brand)',
              'overlay-padding': '8px',
            },
          },
          {
            selector: 'node.neighbor',
            style: {
              'border-color': 'var(--text-primary)',
              'background-color': 'var(--border-prominent)',
              'opacity': 1,
            },
          },
          {
            selector: 'node.dimmed',
            style: {
              'opacity': 0.2,
            },
          },
          {
            selector: 'edge',
            style: {
              'width': 1,
              'line-color': 'var(--border-subtle)',
              'curve-style': 'bezier',
              'opacity': 0.4,
              'transition-property': 'line-color, opacity, width',
              'transition-duration': '0.2s',
            },
          },
          {
            selector: 'edge.active',
            style: {
              'line-color': 'var(--green-brand)',
              'width': 2,
              'opacity': 0.8,
            },
          },
          {
            selector: 'edge.dimmed',
            style: {
              'opacity': 0.05,
            },
          },
        ],
        layout: {
          name: layoutName === 'dagre' ? (data.nodes.length > 200 ? 'concentric' : 'dagre') : 'cose',
          rankDir: 'LR',
          nodeSep: 60,
          rankSep: 100,
          padding: 50,
          animate: true,
          animationDuration: 500,
        },
        wheelSensitivity: 0.2,
      });

      cy.on('tap', 'node', (evt: any) => {
        const id = evt.target.id();
        activeNoteId.set(id);
        if (focusMode) {
          applyFocus(id);
        }
      });

      cy.on('dbltap', 'node', (evt: any) => {
        activeNoteId.set(evt.target.id());
        currentView.set('editor');
      });

      if (unsubActive) unsubActive();
      unsubActive = activeNoteId.subscribe(id => {
        if (!cy) return;
        cy.nodes().removeClass('active');
        if (id) {
          const node = cy.getElementById(id);
          if (node.length) {
            node.addClass('active');
            if (focusMode) applyFocus(id);
          }
        }
      });

    } catch (e) {
      console.error('Graph load failed:', e);
    } finally {
      loading = false;
    }
  }

  function applyFocus(nodeId: string) {
    if (!cy) return;
    const node = cy.getElementById(nodeId);
    const neighbors = node.neighborhood();
    
    cy.elements().addClass('dimmed').removeClass('neighbor active-edge');
    node.removeClass('dimmed').addClass('active');
    neighbors.removeClass('dimmed').addClass('neighbor');
    node.connectedEdges().removeClass('dimmed').addClass('active-edge');
  }

  function clearFocus() {
    if (!cy) return;
    cy.elements().removeClass('dimmed neighbor active-edge');
  }

  function toggleFocusMode() {
    focusMode = !focusMode;
    if (focusMode && $activeNoteId) {
      applyFocus($activeNoteId);
    } else {
      clearFocus();
    }
  }

  function zoomIn() { cy?.zoom(cy.zoom() * 1.2); }
  function zoomOut() { cy?.zoom(cy.zoom() * 0.8); }
  function fit() { cy?.fit(undefined, 50); }

  async function toggleLayout() {
    layoutName = layoutName === 'dagre' ? 'cose' : 'dagre';
    const cytoscape = (await import('cytoscape')).default;
    await loadGraph(cytoscape);
  }

  async function refresh() {
    const cytoscape = (await import('cytoscape')).default;
    await loadGraph(cytoscape);
  }
</script>

<div class="graph-wrap">
  <div class="graph-header">
    <div class="left">
      <span class="graph-title">Knowledge Graph</span>
      <span class="graph-stats">{nodeCount} nodes · {edgeCount} links</span>
    </div>
    
    <div class="center-controls">
      <div class="control-group">
        <button class="ctrl-btn" onclick={zoomIn} title="Zoom In"><ZoomIn size={14}/></button>
        <button class="ctrl-btn" onclick={zoomOut} title="Zoom Out"><ZoomOut size={14}/></button>
        <button class="ctrl-btn" onclick={fit} title="Fit Content"><Maximize size={14}/></button>
      </div>

      <div class="divider"></div>

      <div class="control-group">
        <button class="ctrl-btn" class:active={focusMode} onclick={toggleFocusMode} title="Focus Mode">
          <Target size={14}/>
        </button>
        <button class="ctrl-btn" onclick={() => {}} title="Filter by Tag">
          <Filter size={14}/>
        </button>
        <button class="ctrl-btn" onclick={toggleLayout} title="Toggle Layout (Dagre/Force)">
          <Settings2 size={14}/>
        </button>
      </div>
    </div>

    <div class="right">
      <button class="icon-btn" title="Refresh Graph" onclick={refresh}><RefreshCw size={14}/></button>
    </div>
  </div>

  <div class="graph-body">
    {#if loading}
      <div class="loading">
        <div class="spinner"></div>
        <span>Rendering Knowledge...</span>
      </div>
    {/if}
    <div class="cy-container" bind:this={container}></div>
    {#if !loading && nodeCount === 0}
      <div class="empty">
        <p>No connections found.</p>
        <p class="hint">Use <code>[[</code> to link notes together.</p>
      </div>
    {/if}
  </div>

  <div class="graph-footer">
    <div class="legend">
      <span class="dot active"></span> <span>Active</span>
      <span class="dot neighbor"></span> <span>Related</span>
      <span class="dot standard"></span> <span>Note</span>
    </div>
    <div class="hint">Double-click a node to edit</div>
  </div>
</div>

<style>
  .graph-wrap {
    display: flex;
    flex-direction: column;
    height: 100%;
    overflow: hidden;
    background: var(--bg-primary);
  }

  .graph-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 12px 20px;
    border-bottom: 1px solid var(--border-subtle);
    background: var(--bg-secondary);
    z-index: 10;
  }

  .graph-title {
    font-size: 12px;
    font-weight: 600;
    color: var(--text-primary);
    margin-right: 12px;
  }

  .graph-stats {
    font-size: 11px;
    color: var(--text-muted);
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

  .divider {
    width: 1px;
    height: 16px;
    background: var(--border-subtle);
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

  .ctrl-btn.active {
    color: var(--green-brand);
    background: rgba(62, 207, 142, 0.1);
  }

  .icon-btn {
    background: none;
    border: none;
    cursor: pointer;
    color: var(--text-muted);
    padding: 8px;
    border-radius: 8px;
    transition: all 0.2s;
  }

  .icon-btn:hover {
    color: var(--text-primary);
    background: var(--border-subtle);
  }

  .graph-body {
    flex: 1;
    position: relative;
    overflow: hidden;
  }

  .cy-container {
    width: 100%;
    height: 100%;
  }

  .loading {
    position: absolute;
    inset: 0;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 16px;
    background: var(--bg-primary);
    z-index: 5;
    color: var(--text-muted);
    font-size: 13px;
  }

  .spinner {
    width: 24px;
    height: 24px;
    border: 2px solid var(--border-standard);
    border-top-color: var(--green-brand);
    border-radius: 50%;
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  .empty {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    text-align: center;
    color: var(--text-muted);
  }

  .empty code { color: var(--green-brand); }

  .graph-footer {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 8px 20px;
    background: var(--bg-secondary);
    border-top: 1px solid var(--border-subtle);
  }

  .legend {
    display: flex;
    align-items: center;
    gap: 12px;
    font-size: 10px;
    color: var(--text-muted);
  }

  .dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
  }

  .dot.active { background: var(--green-brand); }
  .dot.neighbor { background: var(--text-primary); }
  .dot.standard { background: var(--border-prominent); }

  .hint {
    font-size: 10px;
    color: var(--text-muted);
    font-style: italic;
  }
</style>
