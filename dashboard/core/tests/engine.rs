use std::sync::atomic::{AtomicBool, Ordering};

use async_trait::async_trait;
use maars_core::contracts::{AgentEvent, GateDecision, ScoreReport};
use maars_core::engine::{
    run_project, ArbitrationOutcome, AutoDirector, DecisionContext, Directive, Director, LoopMode,
    PausePoint, RunSpec,
};
use maars_core::llm::{CompletionOptions, LLMClient, MockLLMClient};
use maars_core::phases::Preset;

// ---------- helpers ----------

fn score(value: f64, decision: GateDecision) -> ScoreReport {
    ScoreReport {
        phase: 0,
        iteration: 0,
        phase_score: value,
        decision,
        bottleneck_critic: Some("critic".into()),
        next_action: "n/a".into(),
        critic_scores: vec![maars_core::contracts::CriticScore {
            critic_id: "c1".into(),
            score: value,
            valid: true,
            invalidation_reason: None,
            key_condition: "k".into(),
            findings: vec![],
        }],
    }
}

fn spec(run_id: &str, preset: Preset, mode: LoopMode) -> RunSpec {
    RunSpec {
        run_id: run_id.into(),
        idea: "test idea".into(),
        preset,
        mode,
        crash_after: None,
    }
}

fn phase_passed_ids(events: &[AgentEvent]) -> Vec<String> {
    events
        .iter()
        .filter_map(|e| match e {
            AgentEvent::PhasePassed { phase_id, .. } => Some(phase_id.clone()),
            _ => None,
        })
        .collect()
}

/// A client whose scoring output is fixed and whose other steps return plain text.
struct ScriptedClient {
    score_raw: String,
}

#[async_trait]
impl LLMClient for ScriptedClient {
    async fn complete(&self, prompt: &str, _o: CompletionOptions) -> anyhow::Result<String> {
        if prompt.contains("Score") {
            Ok(self.score_raw.clone())
        } else {
            Ok("deliverable text".into())
        }
    }
    async fn stream(&self, prompt: &str, o: CompletionOptions) -> anyhow::Result<Vec<String>> {
        Ok(vec![self.complete(prompt, o).await?])
    }
    async fn count_tokens(&self, t: &str) -> anyhow::Result<usize> {
        Ok(t.split_whitespace().count())
    }
}

/// Fails the first `stream` call, then behaves like the mock.
struct FlakyStreamClient {
    failed: AtomicBool,
}

#[async_trait]
impl LLMClient for FlakyStreamClient {
    async fn complete(&self, prompt: &str, o: CompletionOptions) -> anyhow::Result<String> {
        MockLLMClient.complete(prompt, o).await.map(|_| {
            if prompt.contains("Score") {
                r#"{"phase":0,"iteration":1,"phase_score":9,"decision":"PASS","bottleneck_critic":null,"next_action":"advance","critic_scores":[{"critic_id":"m","score":9,"valid":true,"invalidation_reason":null,"key_condition":"k","findings":[]}]}"#.to_string()
            } else {
                "text".to_string()
            }
        })
    }
    async fn stream(&self, prompt: &str, o: CompletionOptions) -> anyhow::Result<Vec<String>> {
        if self.failed.swap(true, Ordering::SeqCst) == false {
            anyhow::bail!("transient stream failure");
        }
        Ok(vec![self.complete(prompt, o).await?])
    }
    async fn count_tokens(&self, t: &str) -> anyhow::Result<usize> {
        Ok(t.split_whitespace().count())
    }
}

struct ManualScoreDirector;
#[async_trait]
impl Director for ManualScoreDirector {
    async fn manual_score(&self, _ctx: &DecisionContext<'_>) -> Option<ScoreReport> {
        Some(score(9.0, GateDecision::Pass))
    }
}

struct HaltingArbiter;
#[async_trait]
impl Director for HaltingArbiter {
    async fn arbitrate(&self, _ctx: &DecisionContext<'_>) -> ArbitrationOutcome {
        ArbitrationOutcome::UpheldFail
    }
}

struct RejectAfterBuilder;
#[async_trait]
impl Director for RejectAfterBuilder {
    async fn decide(&self, point: PausePoint, _ctx: &DecisionContext<'_>) -> Directive {
        if point == PausePoint::AfterBuilder {
            Directive::Reject
        } else {
            Directive::Approve
        }
    }
}

// ---------- tests ----------

#[tokio::test]
async fn standard_preset_runs_phase_0_through_10() {
    let out = tempfile::tempdir().unwrap();
    let result = run_project(
        &MockLLMClient,
        &AutoDirector,
        spec("run-std", Preset::Standard, LoopMode::FullAuto),
        out.path(),
    )
    .await
    .unwrap();

    assert!(result.completed, "standard run should complete");
    assert!(!result.halted);
    let passed = phase_passed_ids(&result.events);
    assert_eq!(passed.len(), 11);
    assert_eq!(passed.first().unwrap(), "0");
    assert_eq!(passed.last().unwrap(), "10");

    // Every phase wrote a real living document and a snapshot.
    assert!(out.path().join("docs/phase-0.md").exists());
    assert!(out.path().join("docs/phase-10.md").exists());
    assert!(out.path().join("snapshots/phase-10.json").exists());
}

#[tokio::test]
async fn crash_then_resume_from_checkpoint() {
    let out = tempfile::tempdir().unwrap();

    // Crash right after committing phase index 1's snapshot (phases 0 and 1 done).
    let mut crash_spec = spec("run-resume", Preset::Standard, LoopMode::FullAuto);
    crash_spec.crash_after = Some((1, 11, 0));
    let first = run_project(&MockLLMClient, &AutoDirector, crash_spec, out.path())
        .await
        .unwrap();
    assert!(
        first.crashed,
        "first run should be aborted by the crash hook"
    );
    assert!(!first.completed);

    // Resume with the same run id + database; it should pick up at the checkpoint.
    let second = run_project(
        &MockLLMClient,
        &AutoDirector,
        spec("run-resume", Preset::Standard, LoopMode::FullAuto),
        out.path(),
    )
    .await
    .unwrap();
    assert!(second.completed, "resumed run should complete");

    // Phase 0 was already done before the crash, so the resumed run does not redo it.
    let resumed_first = phase_passed_ids(&second.events);
    assert_eq!(resumed_first.first().unwrap(), "1");
    assert!(out.path().join("docs/phase-10.md").exists());
}

#[tokio::test]
async fn malformed_score_recovers_via_manual_fallback() {
    let out = tempfile::tempdir().unwrap();
    let client = ScriptedClient {
        score_raw: "this is not json".into(),
    };
    let result = run_project(
        &client,
        &ManualScoreDirector,
        spec("run-manual", Preset::Lite, LoopMode::FullAuto),
        out.path(),
    )
    .await
    .unwrap();

    assert!(
        result.completed,
        "manual fallback should let the run finish"
    );
    // The re-ask attempts surfaced recoverable parse errors before the fallback.
    let parse_errors = result
        .events
        .iter()
        .filter(|e| matches!(e, AgentEvent::Error { recoverable: true, error, .. } if error.contains("parse failed")))
        .count();
    assert!(
        parse_errors >= 3,
        "expected three parse attempts, got {parse_errors}"
    );
}

#[tokio::test]
async fn malformed_score_without_fallback_halts_without_deadlock() {
    let out = tempfile::tempdir().unwrap();
    let client = ScriptedClient {
        score_raw: "still not json".into(),
    };
    let result = run_project(
        &client,
        &AutoDirector, // no manual score available
        spec("run-halt", Preset::Lite, LoopMode::FullAuto),
        out.path(),
    )
    .await
    .unwrap();

    assert!(result.halted, "run must halt for a human, never deadlock");
    assert!(!result.completed);
    assert!(result.events.iter().any(
        |e| matches!(e, AgentEvent::NeedsHuman { reason, .. } if reason.contains("manual score"))
    ));
}

#[tokio::test]
async fn transient_stream_error_is_retried() {
    let out = tempfile::tempdir().unwrap();
    let client = FlakyStreamClient {
        failed: AtomicBool::new(false),
    };
    let result = run_project(
        &client,
        &AutoDirector,
        spec("run-flaky", Preset::Lite, LoopMode::FullAuto),
        out.path(),
    )
    .await
    .unwrap();

    assert!(
        result.completed,
        "a single transient stream error should be retried"
    );
    assert!(result
        .events
        .iter()
        .any(|e| matches!(e, AgentEvent::Error { recoverable: true, error, .. } if error.contains("stream error"))));
}

#[tokio::test]
async fn persistent_fail_escalates_to_arbitration_then_halts() {
    let out = tempfile::tempdir().unwrap();
    let client = ScriptedClient {
        score_raw: serde_json::to_string(&score(6.0, GateDecision::Fail)).unwrap(),
    };
    let result = run_project(
        &client,
        &HaltingArbiter,
        spec("run-arb", Preset::Lite, LoopMode::FullAuto),
        out.path(),
    )
    .await
    .unwrap();

    assert!(result.halted, "upheld-fail arbitration halts the run");
    assert!(!result.completed);
    // Builder (step 4) ran exactly MAX_ITERATIONS times before arbitration fired.
    let builder_runs = result
        .events
        .iter()
        .filter(|e| matches!(e, AgentEvent::StepStart { step_id: 4, .. }))
        .count();
    assert_eq!(builder_runs, 5);
}

#[tokio::test]
async fn supervised_reject_halts_at_builder_gate() {
    let out = tempfile::tempdir().unwrap();
    let result = run_project(
        &MockLLMClient,
        &RejectAfterBuilder,
        spec("run-reject", Preset::Lite, LoopMode::Supervised),
        out.path(),
    )
    .await
    .unwrap();

    assert!(result.halted);
    assert!(!result.completed);
}
