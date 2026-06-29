# MAARS Dashboard

Phase A walking skeleton for the MAARS Dashboard spec — a Tauri v2 desktop app that
runs the reduced MAARS Phase 0 loop end-to-end and writes a real artifact to disk.

## Architecture

The canonical orchestration loop lives in the Rust backend; the frontend is a thin
mirror driven by a single `AgentEvent` stream.

- `core/` — tauri-free orchestration crate (the loop, LLM clients, scoring, SQLite,
  fs service). All logic and the critical-path integration test live here, so CI can
  verify them without the desktop GUI libraries.
- `src-tauri/` — the Tauri v2 shell and IPC commands.
- `src/` — React + TypeScript tri-panel UI (intake express path + structured renderer).

## Phase 0 loop (reduced)

Builder → Scoring → gate (`phase_score >= 9`) → Living-Document file write. Every event
is persisted to SQLite, and on a passing gate the Phase 0 document is written under the
app's local-data directory (path sanitised and contained within the project root).

## LLM clients

- `MockLLMClient` — offline, used by tests and as the default.
- `OpenAICompatibleClient` — any OpenAI-compatible Chat Completions endpoint. Enabled at
  runtime when `MAARS_OPENAI_API_KEY` is set (optionally `MAARS_OPENAI_BASE_URL` /
  `MAARS_OPENAI_MODEL`). The key is read in the backend and never crosses the IPC bridge.

## Scripts

- `pnpm dev`
- `pnpm build`
- `pnpm typecheck`
- `pnpm lint`
- `cargo test -p maars_core`
