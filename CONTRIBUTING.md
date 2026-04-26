# Contributing to Cognote

## Getting Started

1. Fork the repo and clone it
2. Follow the [build instructions](README.md#building-from-source)
3. Create a branch: `git checkout -b feat/your-feature`

## Project Structure

```
cognote/
├── src/                    # Svelte frontend
│   ├── lib/
│   │   ├── commands.ts     # Typed Tauri IPC wrappers
│   │   ├── stores/         # Svelte stores
│   │   └── components/     # UI components
│   └── routes/             # SvelteKit pages
└── src-tauri/
    └── src/
        ├── db.rs           # SQLite data layer + models
        ├── commands.rs     # Tauri command handlers
        └── lib.rs          # App entry point
```

## Guidelines

- **No cloud/telemetry** — Cognote is strictly offline. Don't add network calls.
- **Rust tests** — Add `#[cfg(test)]` tests for any new DB functions.
- **Design system** — Follow the CSS token system in `src/app.css`. No inline colors.
- **Commits** — One logical change per commit, descriptive message.

## Running Tests

```bash
cd src-tauri && cargo test --lib
```

## Submitting a PR

- Keep PRs focused and small
- Describe what changed and why
- All tests must pass
