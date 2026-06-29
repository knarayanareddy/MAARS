use maars_core::{
    llm::MockLLMClient,
    orchestrator::run_phase0,
    ModelSelection,
    Phase0Context,
    Phase0Seed,
};
use rusqlite::Connection;

#[tokio::test]
async fn phase0_runs_end_to_end_with_mock_client() {
    let client = MockLLMClient::default();
    let out = tempfile::tempdir().expect("tempdir");
    let seed = Phase0Seed {
        idea: "Build the MAARS dashboard".into(),
        context: Phase0Context {
            platform: vec!["desktop".into()],
            scale: "prototype".into(),
            compliance: vec![],
            constraints: None,
        },
        preset: "Lite".into(),
        model: ModelSelection {
            provider_id: "mock".into(),
            model: "mock".into(),
        },
    };

    let result = run_phase0(&client, seed, out.path()).await.expect("phase0");
    assert!(!result.events.is_empty());

    let score = result
        .events
        .iter()
        .find_map(|e| match e {
            maars_core::AgentEvent::Score { report, .. } => Some(report.phase_score),
            _ => None,
        })
        .expect("score");
    assert_eq!(score, 9.0);

    // The gate passed, so the Living Document must be a real file on disk.
    let doc = out.path().join("docs").join("phase-0.md");
    assert!(doc.exists(), "phase-0 living document was not written");

    // Every emitted event must have been persisted to SQLite.
    let conn = Connection::open(out.path().join("maars.db")).expect("open db");
    let persisted: i64 = conn
        .query_row("select count(*) from events where run_id = ?1", [&result.run_id], |r| r.get(0))
        .expect("count events");
    assert_eq!(persisted as usize, result.events.len());
}
