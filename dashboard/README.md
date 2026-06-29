# MAARS Dashboard

A Tauri v2 desktop app that runs the MAARS adversarial-refinement loop. Phase A landed
a reduced Phase 0 walking skeleton; **Phase B** lands the full orchestration loop.

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

## Phase B — full orchestration loop (`core/src/engine.rs`)

The engine runs the complete MAARS loop, driven from the backend and emitted as one
`AgentEvent` stream:

- **All 11 meta-steps** per phase: Continuity → Research → Builder → Critique → Scoring →
  Gate → Remediation (⟳ up to 5×) → Arbitration → Living Document → Snapshot. Phase 0 has
  no predecessor, so Continuity/Research are skipped.
- **All 17 phases as data** (`core/src/phases.rs`), filtered by preset: `Lite` (0,1,4),
  `Standard` (0..10), `Full` (every `.5` interstitial too). Adding a phase is a data edit.
- **Three loop modes** — `full-auto` (no pauses), `semi-auto` (pause at scoring + each
  phase gate), `supervised` (pause at every step). Pauses are resolved by a `Director`.
- **Error recovery** (every failure has a defined path; the loop never deadlocks):
  - *Scoring parse* — 3-path JSON extraction (`scoring::parse_score`), re-ask twice, then
    a human manual-score fallback; `NeedsHuman` if none (R-001).
  - *Transactional commit* — streamed output is buffered and committed to SQLite with its
    terminating event in one transaction; a crash before commit just re-runs the step.
    A per-step checkpoint makes a run resumable (R-002).
  - *Arbitration* — single-shot after the iteration limit; binding ruling resolves or
    halts (R-003).
  - *Research blocker / file-write rejection* surface `NeedsHuman` instead of crashing.

Verified by `core/tests/engine.rs`: a full Standard run 0→10, crash-then-resume,
manual-score recovery, no-deadlock halt, transient-stream-error retry, arbitration halt,
and a supervised reject.

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
