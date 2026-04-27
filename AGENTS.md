# AGENTS.md - Cognote Development Guide

## Tech Stack
- **Backend**: Rust + Tauri v2
- **Frontend**: Svelte 5 + SvelteKit + Tailwind CSS
- **Database**: SQLite (rusqlite, bundled) + FTS5
- **Editor**: TipTap 2
- **Graph**: Cytoscape.js + d3-hierarchy

## Developer Commands

```bash
# Development
npm run tauri dev          # Run app in dev mode
npm run tauri build        # Build distributable

# Frontend only
npm run dev                # SvelteKit dev server
npm run build              # Build frontend
npm run check              # TypeScript + Svelte check

# Rust
cd src-tauri && cargo check    # Typecheck Rust
```

## Key Files & Entry Points
- `src-tauri/src/lib.rs` - Tauri app entry, command registration
- `src-tauri/src/db.rs` - Database schema, queries, FTS5
- `src-tauri/src/commands.rs` - Tauri commands (file ops, notes, search)
- `src/lib/components/` - Svelte components (NoteEditor, KnowledgeGraph, etc.)
- `src/routes/+page.svelte` - Main app shell
- `src/lib/commands.ts` - Frontend Tauri invoke wrappers

## Database
- Location: `~/Library/Application Support/com.cognote.app/cognote.db` (macOS)
- Migrations in `db.rs:migrate()` - indexes added at line ~216
- FTS5 virtual table for full-text search

## Important Conventions
- **Wiki-links**: Type `[[` in editor to link notes (Mention extension)
- **Keyboard shortcuts**: `⌘N` new note, `⌘K` search, `⌘G` graph, `⌘M` mindmap
- **Theme**: Dark/light via CSS variables in `app.css`, persisted in `app_config` table

## Recent Fixes Applied
1. SQL injection fix in FTS5 search (`db.rs:533-575`)
2. Input validation: title max 500 chars, file size 100MB limit
3. Path traversal protection in file operations
4. Memory leak fix in NoteEditor popup cleanup
5. Autosave race condition fix with save queue
6. Database indexes for performance
7. Toast notification system for error feedback
8. Bulk delete: `deleteNotes(ids)` command available