//! Phase B — the full MAARS orchestration loop.
//!
//! This is the canonical loop (D1 §1): it runs entirely in the backend and emits a
//! single `AgentEvent` stream that the frontend mirrors. It implements all 11 meta-steps
//! (D3 §3), all 17 phases via `phases.config` (filtered by preset), the three loop modes,
//! and every error-recovery state (D3 §5). Streamed output is buffered and committed
//! transactionally (D3 §6); a compact checkpoint at each step boundary makes a run
//! resumable after a crash.

use crate::contracts::{AgentEvent, GateDecision, ScoreReport, StepId, StructuralMarker};
use crate::db;
use crate::fs_service::{ensure_within_root, sanitize_relative_path};
use crate::llm::{CompletionOptions, LLMClient};
use crate::phases::{phases_for, PhaseDef, Preset};
use crate::scoring::{aggregate, parse_score};
use async_trait::async_trait;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

pub const MAX_ITERATIONS: i64 = 5;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LoopMode {
    /// No pauses; phase gates auto-approve.
    FullAuto,
    /// Pause at scoring and at each phase gate.
    SemiAuto,
    /// Pause at every step boundary.
    Supervised,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PausePoint {
    AfterBuilder,
    AfterCritique,
    AfterScoring,
    PhaseGate,
}

impl LoopMode {
    fn pauses_at(self, point: PausePoint) -> bool {
        match self {
            LoopMode::FullAuto => false,
            LoopMode::SemiAuto => matches!(point, PausePoint::AfterScoring | PausePoint::PhaseGate),
            LoopMode::Supervised => true,
        }
    }
}

/// The human (or automated stand-in) the engine consults at pause points and for the
/// two decisions a machine must never make alone: the manual-score fallback and the
/// binding arbitration ruling.
#[derive(Debug, Clone)]
pub enum Directive {
    Approve,
    /// Approve and inject a note into the next builder prompt (pause/resume note).
    ApproveWithNote(String),
    Reject,
    Halt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArbitrationOutcome {
    Resolved,
    UpheldFail,
}

pub struct DecisionContext<'a> {
    pub run_id: &'a str,
    pub phase_id: &'a str,
    pub iteration: i64,
    pub last_score: Option<&'a ScoreReport>,
}

#[async_trait]
pub trait Director: Send + Sync {
    async fn decide(&self, _point: PausePoint, _ctx: &DecisionContext<'_>) -> Directive {
        Directive::Approve
    }
    /// R-001: the human-supplied score when all 3 JSON paths fail twice. `None` halts
    /// the run for a human rather than deadlocking.
    async fn manual_score(&self, _ctx: &DecisionContext<'_>) -> Option<ScoreReport> {
        None
    }
    /// R-003: arbitration is single-shot; this is its binding ruling.
    async fn arbitrate(&self, _ctx: &DecisionContext<'_>) -> ArbitrationOutcome {
        ArbitrationOutcome::Resolved
    }
}

/// Default automated director: approves everything, resolves arbitration, no manual score.
pub struct AutoDirector;

#[async_trait]
impl Director for AutoDirector {}

#[derive(Debug, Clone)]
pub struct RunSpec {
    pub run_id: String,
    pub idea: String,
    pub preset: Preset,
    pub mode: LoopMode,
    /// Test hook: abort right after committing this `(phase_index, step_id, iteration)`
    /// to exercise crash-recovery. `None` in production.
    pub crash_after: Option<(usize, StepId, i64)>,
}

#[derive(Debug, Clone)]
pub struct RunResult {
    pub run_id: String,
    pub events: Vec<AgentEvent>,
    /// True when the run reached the final phase gate.
    pub completed: bool,
    /// True when the run stopped at a human-actionable halt (NeedsHuman).
    pub halted: bool,
    /// True when the run was aborted by the `crash_after` hook.
    pub crashed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Checkpoint {
    phase_index: usize,
    step_id: StepId,
    iteration: i64,
    capsule_ref: Option<String>,
}

enum Flow {
    Advance,
    Halt,
    Crash,
}

pub async fn run_project<C: LLMClient, D: Director>(
    client: &C,
    director: &D,
    spec: RunSpec,
    output_root: &Path,
) -> anyhow::Result<RunResult> {
    std::fs::create_dir_all(output_root)?;
    let conn = db::init_db(&output_root.join("maars.db"))?;
    let phases = phases_for(spec.preset);

    let (start_index, start_iteration, capsule_ref) =
        match db::load_checkpoint(&conn, &spec.run_id)? {
            Some(payload) => {
                let cp: Checkpoint = serde_json::from_str(&payload)?;
                (cp.phase_index, cp.iteration, cp.capsule_ref)
            }
            None => (0, 0, None),
        };

    let mut engine = Engine {
        client,
        director,
        conn,
        output_root: output_root.to_path_buf(),
        run_id: spec.run_id.clone(),
        mode: spec.mode,
        crash_after: spec.crash_after,
        events: Vec::new(),
        capsule_ref,
        pending_note: None,
    };

    let mut completed = false;
    let mut halted = false;
    let mut crashed = false;

    let mut phase_index = start_index;
    let mut iteration = start_iteration;
    while phase_index < phases.len() {
        let phase = phases[phase_index].clone();
        match engine.run_phase(phase_index, &phase, iteration).await? {
            Flow::Advance => {
                phase_index += 1;
                iteration = 0;
                engine.save_checkpoint(phase_index, 1, 0)?;
                if phase_index == phases.len() {
                    completed = true;
                }
            }
            Flow::Halt => {
                halted = true;
                break;
            }
            Flow::Crash => {
                crashed = true;
                break;
            }
        }
    }

    Ok(RunResult {
        run_id: spec.run_id,
        events: engine.events,
        completed,
        halted,
        crashed,
    })
}

struct Engine<'a, C: LLMClient, D: Director> {
    client: &'a C,
    director: &'a D,
    conn: Connection,
    output_root: PathBuf,
    run_id: String,
    mode: LoopMode,
    crash_after: Option<(usize, StepId, i64)>,
    events: Vec<AgentEvent>,
    capsule_ref: Option<String>,
    pending_note: Option<String>,
}

impl<C: LLMClient, D: Director> Engine<'_, C, D> {
    fn emit(&mut self, event: AgentEvent) {
        if let Ok(payload) = serde_json::to_string(&event) {
            let _ = db::insert_event(&self.conn, &self.run_id, &payload);
        }
        self.events.push(event);
    }

    fn commit(&mut self, artifact_ref: &str, body: &str, event: AgentEvent) -> anyhow::Result<()> {
        let payload = serde_json::to_string(&event)?;
        db::commit_step(&mut self.conn, &self.run_id, artifact_ref, body, &payload)?;
        self.events.push(event);
        Ok(())
    }

    fn save_checkpoint(
        &self,
        phase_index: usize,
        step_id: StepId,
        iteration: i64,
    ) -> anyhow::Result<()> {
        let cp = Checkpoint {
            phase_index,
            step_id,
            iteration,
            capsule_ref: self.capsule_ref.clone(),
        };
        db::save_checkpoint(&self.conn, &self.run_id, &serde_json::to_string(&cp)?)
    }

    fn opts(&self, json_mode: bool) -> CompletionOptions {
        CompletionOptions {
            model: "phase-b".into(),
            json_mode,
            max_tokens: None,
        }
    }

    fn artifact_ref(&self, phase_id: &str, iteration: i64, step: StepId) -> String {
        format!("{}:p{}:i{}:s{}", self.run_id, phase_id, iteration, step)
    }

    /// True if the crash hook fires for this boundary (and records the checkpoint first).
    fn should_crash(&self, phase_index: usize, step: StepId, iteration: i64) -> bool {
        self.crash_after == Some((phase_index, step, iteration))
    }

    /// Stream a step with one automatic retry (R-002: partial output is never committed,
    /// so a mid-stream failure simply re-runs the step from the last committed boundary).
    async fn stream_step(
        &mut self,
        phase_id: &str,
        step: StepId,
        iteration: i64,
        marker: Option<StructuralMarker>,
        prompt: &str,
    ) -> anyhow::Result<Option<String>> {
        self.emit(AgentEvent::StepStart {
            run_id: self.run_id.clone(),
            phase_id: phase_id.to_string(),
            step_id: step,
            iteration,
        });
        if let Some(marker) = marker.clone() {
            self.emit(AgentEvent::Marker {
                run_id: self.run_id.clone(),
                step_id: step,
                marker,
            });
        }

        let mut last_err: Option<anyhow::Error> = None;
        for attempt in 0..2 {
            match self.client.stream(prompt, self.opts(false)).await {
                Ok(chunks) => {
                    let mut buf = String::new();
                    for (seq, chunk) in chunks.into_iter().enumerate() {
                        self.emit(AgentEvent::Token {
                            run_id: self.run_id.clone(),
                            step_id: step,
                            chunk: chunk.clone(),
                            seq: seq as u64,
                        });
                        buf.push_str(&chunk);
                    }
                    let aref = self.artifact_ref(phase_id, iteration, step);
                    self.commit(
                        &aref,
                        &buf,
                        AgentEvent::StepComplete {
                            run_id: self.run_id.clone(),
                            step_id: step,
                            artifact_ref: aref.clone(),
                        },
                    )?;
                    return Ok(Some(buf));
                }
                Err(e) => {
                    self.emit(AgentEvent::Error {
                        run_id: self.run_id.clone(),
                        step_id: Some(step),
                        error: format!("stream error (attempt {}): {e}", attempt + 1),
                        recoverable: true,
                    });
                    last_err = Some(e);
                }
            }
        }
        // Both attempts failed: hand off to a human rather than crash (D3 §5).
        self.emit(AgentEvent::NeedsHuman {
            run_id: self.run_id.clone(),
            reason: format!(
                "step {step} stream failed after retry: {}",
                last_err.map(|e| e.to_string()).unwrap_or_default()
            ),
            payload: serde_json::json!({ "step": step }),
        });
        Ok(None)
    }

    /// Run one phase's loop. Returns how the overall run should proceed.
    async fn run_phase(
        &mut self,
        phase_index: usize,
        phase: &PhaseDef,
        start_iteration: i64,
    ) -> anyhow::Result<Flow> {
        let is_phase_zero = phase.id == "0";
        let mut iteration = start_iteration;
        let mut last_builder;

        // Steps 2 & 3 (Continuity, Research) run once per phase; Phase 0 has no
        // predecessor so they are skipped (D3 Phase 0 exception).
        if !is_phase_zero {
            self.save_checkpoint(phase_index, 2, iteration)?;
            if self
                .stream_step(
                    &phase.id,
                    2,
                    iteration,
                    None,
                    &continuity_prompt(phase, self.capsule_ref.as_deref()),
                )
                .await?
                .is_none()
            {
                return Ok(Flow::Halt);
            }
            if self.should_crash(phase_index, 2, iteration) {
                return Ok(Flow::Crash);
            }

            self.save_checkpoint(phase_index, 3, iteration)?;
            // Research can degrade rather than block (R-004); the mock always succeeds.
            if self
                .stream_step(&phase.id, 3, iteration, None, &research_prompt(phase))
                .await?
                .is_none()
            {
                return Ok(Flow::Halt);
            }
            if self.should_crash(phase_index, 3, iteration) {
                return Ok(Flow::Crash);
            }
        }

        // Builder -> Critique -> Scoring -> gate, looping through remediation up to the limit.
        loop {
            // Step 4: Builder.
            self.save_checkpoint(phase_index, 4, iteration)?;
            let note = self.pending_note.take();
            let builder_prompt = builder_prompt(phase, iteration, note.as_deref());
            last_builder = match self
                .stream_step(
                    &phase.id,
                    4,
                    iteration,
                    Some(StructuralMarker::BuilderPersona {
                        name: "Lead".into(),
                    }),
                    &builder_prompt,
                )
                .await?
            {
                Some(b) => b,
                None => return Ok(Flow::Halt),
            };
            if self.should_crash(phase_index, 4, iteration) {
                return Ok(Flow::Crash);
            }
            if let Some(flow) = self
                .pause(PausePoint::AfterBuilder, &phase.id, iteration, None)
                .await
            {
                return Ok(flow);
            }

            // Step 5: Critique + Devil's Advocate.
            self.save_checkpoint(phase_index, 5, iteration)?;
            if self
                .stream_step(
                    &phase.id,
                    5,
                    iteration,
                    Some(StructuralMarker::CritiqueSection {
                        critic: "Panel".into(),
                    }),
                    &critique_prompt(phase),
                )
                .await?
                .is_none()
            {
                return Ok(Flow::Halt);
            }
            if self.should_crash(phase_index, 5, iteration) {
                return Ok(Flow::Crash);
            }
            if let Some(flow) = self
                .pause(PausePoint::AfterCritique, &phase.id, iteration, None)
                .await
            {
                return Ok(flow);
            }

            // Step 6: Scoring (with 3-path parse + manual fallback recovery).
            let report = match self.run_scoring(&phase.id, iteration).await? {
                Some(r) => r,
                None => return Ok(Flow::Halt),
            };
            if self.should_crash(phase_index, 6, iteration) {
                return Ok(Flow::Crash);
            }
            if let Some(flow) = self
                .pause(
                    PausePoint::AfterScoring,
                    &phase.id,
                    iteration,
                    Some(&report),
                )
                .await
            {
                return Ok(flow);
            }

            // Step 7: Gate.
            match report.decision {
                GateDecision::Pass => break,
                GateDecision::Fail if iteration + 1 < MAX_ITERATIONS => {
                    self.save_checkpoint(phase_index, 8, iteration)?;
                    if self
                        .stream_step(
                            &phase.id,
                            8,
                            iteration,
                            None,
                            &remediation_prompt(phase, &report),
                        )
                        .await?
                        .is_none()
                    {
                        return Ok(Flow::Halt);
                    }
                    iteration += 1;
                    continue;
                }
                // Iteration limit hit, or the aggregator demanded arbitration.
                GateDecision::Fail | GateDecision::Arbitration => {
                    self.save_checkpoint(phase_index, 9, iteration)?;
                    self.emit(AgentEvent::StepStart {
                        run_id: self.run_id.clone(),
                        phase_id: phase.id.clone(),
                        step_id: 9,
                        iteration,
                    });
                    let ctx = DecisionContext {
                        run_id: &self.run_id,
                        phase_id: &phase.id,
                        iteration,
                        last_score: Some(&report),
                    };
                    match self.director.arbitrate(&ctx).await {
                        ArbitrationOutcome::Resolved => break, // proceed to Living Document
                        ArbitrationOutcome::UpheldFail => {
                            self.emit(AgentEvent::NeedsHuman {
                                run_id: self.run_id.clone(),
                                reason: format!("phase {} arbitration upheld FAIL", phase.id),
                                payload: serde_json::json!({ "phase": phase.id }),
                            });
                            return Ok(Flow::Halt);
                        }
                    }
                }
            }
        }

        // Step 10: Living Document write (real file, sanitised + contained). R-006: a
        // rejected path does not advance the gate.
        self.save_checkpoint(phase_index, 10, iteration)?;
        let rel_path = match self.write_living_doc(&phase.id, &last_builder) {
            Ok(p) => p,
            Err(e) => {
                self.emit(AgentEvent::Error {
                    run_id: self.run_id.clone(),
                    step_id: Some(10),
                    error: format!("file write failed: {e}"),
                    recoverable: false,
                });
                self.emit(AgentEvent::NeedsHuman {
                    run_id: self.run_id.clone(),
                    reason: format!("living-document write rejected for phase {}", phase.id),
                    payload: serde_json::json!({ "phase": phase.id }),
                });
                return Ok(Flow::Halt);
            }
        };
        self.emit(AgentEvent::FileWritten {
            run_id: self.run_id.clone(),
            path: rel_path,
            phase_id: phase.id.clone(),
        });
        if self.should_crash(phase_index, 10, iteration) {
            return Ok(Flow::Crash);
        }

        // Step 11: Snapshot write -> becomes the next phase's Continuity input.
        self.save_checkpoint(phase_index, 11, iteration)?;
        let snapshot_ref = self.write_snapshot(&phase.id, iteration)?;
        self.capsule_ref = Some(snapshot_ref.clone());
        self.emit(AgentEvent::PhasePassed {
            run_id: self.run_id.clone(),
            phase_id: phase.id.clone(),
            snapshot_ref,
        });
        if self.should_crash(phase_index, 11, iteration) {
            return Ok(Flow::Crash);
        }

        // Phase gate: full-auto auto-approves; otherwise the director decides.
        if let Some(flow) = self
            .pause(PausePoint::PhaseGate, &phase.id, iteration, None)
            .await
        {
            return Ok(flow);
        }
        Ok(Flow::Advance)
    }

    async fn run_scoring(
        &mut self,
        phase_id: &str,
        iteration: i64,
    ) -> anyhow::Result<Option<ScoreReport>> {
        self.emit(AgentEvent::StepStart {
            run_id: self.run_id.clone(),
            phase_id: phase_id.to_string(),
            step_id: 6,
            iteration,
        });
        self.emit(AgentEvent::Marker {
            run_id: self.run_id.clone(),
            step_id: 6,
            marker: StructuralMarker::ScoreCardStart,
        });

        // Re-ask up to twice on a parse failure, then fall back to a human score.
        for attempt in 0..3 {
            let raw = self
                .client
                .complete(&scoring_prompt(phase_id), self.opts(true))
                .await?;
            match parse_score(&raw) {
                Ok(report) => return Ok(Some(self.finalize_score(phase_id, iteration, report)?)),
                Err(_) => {
                    self.emit(AgentEvent::Error {
                        run_id: self.run_id.clone(),
                        step_id: Some(6),
                        error: format!("score JSON parse failed (attempt {})", attempt + 1),
                        recoverable: true,
                    });
                }
            }
        }

        let ctx = DecisionContext {
            run_id: &self.run_id,
            phase_id,
            iteration,
            last_score: None,
        };
        if let Some(report) = self.director.manual_score(&ctx).await {
            return Ok(Some(self.finalize_score(phase_id, iteration, report)?));
        }

        // R-001: never deadlock — surface a manual-score request to a human.
        self.emit(AgentEvent::NeedsHuman {
            run_id: self.run_id.clone(),
            reason: format!("phase {phase_id} scoring could not be parsed; manual score required"),
            payload: serde_json::json!({ "phase": phase_id, "step": 6 }),
        });
        Ok(None)
    }

    fn finalize_score(
        &mut self,
        phase_id: &str,
        iteration: i64,
        report: ScoreReport,
    ) -> anyhow::Result<ScoreReport> {
        let report = aggregate(report);
        let aref = self.artifact_ref(phase_id, iteration, 6);
        let body = serde_json::to_string(&report)?;
        self.commit(
            &aref,
            &body,
            AgentEvent::Score {
                run_id: self.run_id.clone(),
                phase_id: phase_id.to_string(),
                report: report.clone(),
            },
        )?;
        Ok(report)
    }

    fn write_living_doc(&self, phase_id: &str, body: &str) -> anyhow::Result<String> {
        let docs_dir = self.output_root.join("docs");
        std::fs::create_dir_all(&docs_dir)?;
        let filename = sanitize_relative_path(&format!("phase-{phase_id}.md"));
        let target = docs_dir.join(&filename);
        let target = ensure_within_root(&self.output_root, &target)?;
        std::fs::write(&target, body)?;
        Ok(format!("docs/{filename}"))
    }

    fn write_snapshot(&mut self, phase_id: &str, iteration: i64) -> anyhow::Result<String> {
        let snap_dir = self.output_root.join("snapshots");
        std::fs::create_dir_all(&snap_dir)?;
        let filename = sanitize_relative_path(&format!("phase-{phase_id}.json"));
        let target = snap_dir.join(&filename);
        let target = ensure_within_root(&self.output_root, &target)?;
        let capsule = serde_json::json!({
            "phase": phase_id,
            "iteration": iteration,
            "deliverableRef": self.artifact_ref(phase_id, iteration, 4),
            "scoreRef": self.artifact_ref(phase_id, iteration, 6),
        });
        std::fs::write(&target, serde_json::to_string_pretty(&capsule)?)?;
        Ok(format!("snapshots/{filename}"))
    }

    /// Consult the director if the mode pauses at this point. Returns `Some(flow)` when
    /// the run must stop (reject/halt), `None` to continue.
    async fn pause(
        &mut self,
        point: PausePoint,
        phase_id: &str,
        iteration: i64,
        last_score: Option<&ScoreReport>,
    ) -> Option<Flow> {
        if !self.mode.pauses_at(point) {
            return None;
        }
        let directive = {
            let ctx = DecisionContext {
                run_id: &self.run_id,
                phase_id,
                iteration,
                last_score,
            };
            self.director.decide(point, &ctx).await
        };
        match directive {
            Directive::Approve => None,
            Directive::ApproveWithNote(note) => {
                self.pending_note = Some(note);
                None
            }
            Directive::Reject | Directive::Halt => {
                self.emit(AgentEvent::NeedsHuman {
                    run_id: self.run_id.clone(),
                    reason: format!("run halted by director at {point:?} in phase {phase_id}"),
                    payload: serde_json::json!({ "phase": phase_id }),
                });
                Some(Flow::Halt)
            }
        }
    }
}

fn continuity_prompt(phase: &PhaseDef, capsule_ref: Option<&str>) -> String {
    format!(
        "Continuity Agent: compress the prior phase into a handoff for phase {} ({}). Prior snapshot: {}.",
        phase.id,
        phase.title,
        capsule_ref.unwrap_or("none")
    )
}

fn research_prompt(phase: &PhaseDef) -> String {
    format!(
        "Research Agent: audit tech freshness for phase {} ({}).",
        phase.id, phase.title
    )
}

fn builder_prompt(phase: &PhaseDef, iteration: i64, note: Option<&str>) -> String {
    let mut p = format!(
        "Builder Panel: produce the deliverables for phase {} ({}), iteration {}.",
        phase.id, phase.title, iteration
    );
    if let Some(note) = note {
        p.push_str(&format!(" Reviewer note to address: {note}"));
    }
    p
}

fn critique_prompt(phase: &PhaseDef) -> String {
    format!(
        "Critique Panel + Devil's Advocate: find >=3 substantive risks in phase {} ({}).",
        phase.id, phase.title
    )
}

fn scoring_prompt(phase_id: &str) -> String {
    // Must contain "Score" so the mock client returns a ScoreReport JSON.
    format!("Scoring Aggregator: Score phase {phase_id} and return the ScoreReport JSON.")
}

fn remediation_prompt(phase: &PhaseDef, report: &ScoreReport) -> String {
    format!(
        "Remediation Agent: turn findings into tickets for phase {} (bottleneck: {}).",
        phase.id,
        report.bottleneck_critic.as_deref().unwrap_or("n/a")
    )
}
