# Cognote

A privacy-first, offline desktop notebook with a connected knowledge graph.

## Features

- **Rich-text notes** — Bold, italic, headings, code blocks, lists (TipTap editor)
- **Wiki-links** — Type `[[` to link notes; backlinks tracked automatically
- **Knowledge graph** — Interactive Cytoscape graph of all note connections
- **Mind maps** — Auto-generated tree from note headings (d3-hierarchy)
- **File organiser** — Drag-and-drop import, image/PDF preview, open externally
- **Full-text search** — SQLite FTS5 with highlighted snippets (⌘K)
- **Progress dashboard** — Daily note count, streak, 30-day activity chart
- **Notebooks & tags** — Hierarchical folders + tag filtering
- **Dark/light theme** — Supabase-inspired design system, persisted
- **100% offline** — No cloud, no telemetry, no accounts. Data in local SQLite.

## Keyboard Shortcuts

| Shortcut | Action |
|---|---|
| `⌘N` | New note |
| `⌘K` | Focus search |
| `⌘G` | Knowledge graph view |
| `⌘M` | Mind map view |

## Building from Source

### Prerequisites

- [Rust](https://rustup.rs/) 1.70+
- [Node.js](https://nodejs.org/) 18+
- macOS: Xcode Command Line Tools (`xcode-select --install`)

### Run in development

```bash
git clone <repo-url>
cd cognote
npm install
npm run tauri dev
```

### Build distributable

```bash
npm run tauri build
```

Output: `src-tauri/target/release/bundle/`
- macOS: `.dmg` in `macos/`
- Linux: `.AppImage` and `.deb` in `appimage/` and `deb/`
- Windows: `.msi` in `msi/`

## Data Storage

Cognote stores all data locally:

| File | Location |
|---|---|
| Database | `~/Library/Application Support/com.cognote.app/cognote.db` (macOS) |
| Files vault | Same directory as the database |

To back up your data, copy the entire `com.cognote.app` directory.

## Tech Stack

| Layer | Technology |
|---|---|
| Backend | Rust + Tauri v2 |
| Frontend | Svelte 5 + SvelteKit |
| Database | SQLite (rusqlite, bundled) + FTS5 |
| Editor | TipTap 2 |
| Graph | Cytoscape.js + cytoscape-dagre |
| Mind map | d3-hierarchy |
| Styling | Tailwind CSS + CSS custom properties |

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). All contributions welcome.

## License

[MIT](LICENSE)
