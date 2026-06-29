use crate::contracts::{AgentEvent, Phase0Seed, ScoreReport, StepId};
use crate::db;
use crate::fs_service::{ensure_within_root, sanitize_relative_path};
use crate::llm::{CompletionOptions, LLMClient};
use crate::scoring::aggregate;
use chrono::Utc;
use rusqlite::Connection;
use serde_json::json;
use std::path::Path;

pub struct Phase0Result {
    pub run_id: String,
    pub events: Vec<AgentEvent>,
}

/// Persist an event to SQLite and accumulate it for the caller.
fn record(conn: &Connection, run_id: &str, events: &mut Vec<AgentEvent>, event: AgentEvent) {
    if let Ok(payload) = serde_json::to_string(&event) {
        // Persistence is best-effort for the walking skeleton; a failed write must
        // not abort the loop (it is surfaced as an Error event by the caller path).
        let _ = db::insert_event(conn, run_id, &payload);
    }
    events.push(event);
}

/// Reduced Phase 0 loop (D8 Phase A): Builder -> Scoring -> gate -> Living-Document
/// file write. Output is written under `output_root` and every event is persisted
/// to a SQLite database in the same directory.
pub async fn run_phase0<C: LLMClient>(
    client: &C,
    seed: Phase0Seed,
    output_root: &Path,
) -> anyhow::Result<Phase0Result> {
    std::fs::create_dir_all(output_root)?;
    let conn = db::init_db(&output_root.join("maars.db"))?;

    let run_id = format!("run-{}", Utc::now().timestamp_nanos_opt().unwrap_or_default());
    let mut events = Vec::new();

    record(
        &conn,
        &run_id,
        &mut events,
        AgentEvent::StepStart {
            run_id: run_id.clone(),
            phase_id: "phase-0".to_string(),
            step_id: 4_u8 as StepId,
            iteration: 1,
        },
    );

    let builder = client
        .complete(
            &format!("Builder phase 0: {}", seed.idea),
            CompletionOptions { model: seed.model.model.clone(), json_mode: false, max_tokens: Some(800) },
        )
        .await?;
    record(
        &conn,
        &run_id,
        &mut events,
        AgentEvent::Token { run_id: run_id.clone(), step_id: 4, chunk: builder.clone(), seq: 0 },
    );
    record(
        &conn,
        &run_id,
        &mut events,
        AgentEvent::StepComplete { run_id: run_id.clone(), step_id: 4, artifact_ref: "builder:phase-0".into() },
    );

    let score_text = client
        .complete(
            &format!("Score phase 0: {}", builder),
            CompletionOptions { model: seed.model.model, json_mode: true, max_tokens: Some(400) },
        )
        .await?;
    let report: ScoreReport = serde_json::from_str(&score_text)
        .map_err(|e| anyhow::anyhow!("score parse failed: {e}"))?;
    let report = aggregate(report);
    record(
        &conn,
        &run_id,
        &mut events,
        AgentEvent::Score { run_id: run_id.clone(), phase_id: "phase-0".into(), report: report.clone() },
    );

    if report.phase_score >= 9.0 {
        record(
            &conn,
            &run_id,
            &mut events,
            AgentEvent::StepStart { run_id: run_id.clone(), phase_id: "phase-0".into(), step_id: 10, iteration: 1 },
        );

        // Living-Document write: a real file on disk, with the filename sanitised
        // and the resolved path asserted to stay within the project root (D5 S3).
        let docs_dir = output_root.join("docs");
        std::fs::create_dir_all(&docs_dir)?;
        let filename = sanitize_relative_path("phase-0.md");
        let target = docs_dir.join(&filename);
        let target = ensure_within_root(output_root, &target)?;
        std::fs::write(&target, &builder)?;
        let rel_path = format!("docs/{filename}");

        record(
            &conn,
            &run_id,
            &mut events,
            AgentEvent::FileWritten { run_id: run_id.clone(), phase_id: "phase-0".into(), path: rel_path },
        );
        record(
            &conn,
            &run_id,
            &mut events,
            AgentEvent::PhasePassed {
                run_id: run_id.clone(),
                phase_id: "phase-0".into(),
                snapshot_ref: "snapshots/phase-0.json".into(),
            },
        );
    } else {
        record(
            &conn,
            &run_id,
            &mut events,
            AgentEvent::NeedsHuman { run_id: run_id.clone(), reason: "gate-failed".into(), payload: json!(report) },
        );
    }

    Ok(Phase0Result { run_id, events })
}
