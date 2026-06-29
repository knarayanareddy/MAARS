//! Rust mirror of `src/lib/types/contracts.ts`. Keep the two in sync.

use serde::{Deserialize, Serialize};

pub type StepId = u8;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Severity {
    Critical,
    Major,
    Minor,
    Advisory,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GateDecision {
    Pass,
    Fail,
    Arbitration,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FindingStatus {
    Open,
    Resolved,
    AcceptedRisk,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
    pub id: String,
    pub severity: Severity,
    pub what: String,
    pub why: String,
    pub how_to_fix: String,
    pub status: FindingStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CriticScore {
    pub critic_id: String,
    pub score: f64,
    pub valid: bool,
    pub invalidation_reason: Option<String>,
    pub key_condition: String,
    #[serde(default)]
    pub findings: Vec<Finding>,
}

/// The gate's only input (R7).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreReport {
    pub phase: i64,
    pub iteration: i64,
    pub phase_score: f64,
    pub decision: GateDecision,
    pub bottleneck_critic: Option<String>,
    pub next_action: String,
    pub critic_scores: Vec<CriticScore>,
}

impl ScoreReport {
    /// Phase score = lowest VALID critic score (no averaging). Unsupported scores
    /// (no findings backing a sub-9 score is allowed; a >=9 score with open CRITICAL
    /// findings is invalidated by the aggregator). This recomputes from valid scores.
    pub fn lowest_valid_score(&self) -> Option<f64> {
        let mut lowest: Option<f64> = None;
        for score in self.critic_scores.iter().filter(|c| c.valid).map(|c| c.score) {
            lowest = Some(lowest.map_or(score, |current| current.min(score)));
        }
        lowest
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum StructuralMarker {
    BuilderPersona { name: String },
    CritiqueSection { critic: String },
    ScoreCardStart,
    RemediationTicket { id: String },
    PhaseExit,
}

/// The single backend -> frontend seam (ADR-005).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum AgentEvent {
    StepStart { run_id: String, phase_id: String, step_id: StepId, iteration: i64 },
    Token { run_id: String, step_id: StepId, chunk: String, seq: u64 },
    Marker { run_id: String, step_id: StepId, marker: StructuralMarker },
    StepComplete { run_id: String, step_id: StepId, artifact_ref: String },
    Score { run_id: String, phase_id: String, report: ScoreReport },
    FileWritten { run_id: String, path: String, phase_id: String },
    PhasePassed { run_id: String, phase_id: String, snapshot_ref: String },
    NeedsHuman { run_id: String, reason: String, payload: serde_json::Value },
    Error { run_id: String, step_id: Option<StepId>, error: String, recoverable: bool },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Phase0Context {
    pub platform: Vec<String>,
    pub scale: String,
    pub compliance: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraints: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelSelection {
    pub provider_id: String,
    pub model: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Phase0Seed {
    pub idea: String,
    pub context: Phase0Context,
    pub preset: String,
    /// Express path: one model for every step. Keys never travel in this struct.
    pub model: ModelSelection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectMeta {
    pub id: String,
    pub name: String,
    pub preset: String,
    pub current_phase: String,
    pub last_active: String,
}
