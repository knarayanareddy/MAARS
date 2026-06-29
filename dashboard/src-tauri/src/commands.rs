use maars_core::{
    llm::{MockLLMClient, OpenAICompatibleClient},
    orchestrator::run_phase0 as core_run_phase0,
    ModelSelection, Phase0Context, Phase0Seed,
};
use tauri::Manager;

fn seed_for(idea: String, provider_id: &str, model: &str) -> Phase0Seed {
    Phase0Seed {
        idea,
        context: Phase0Context {
            platform: vec!["desktop".into()],
            scale: "prototype".into(),
            compliance: vec![],
            constraints: None,
        },
        preset: "Lite".into(),
        model: ModelSelection { provider_id: provider_id.into(), model: model.into() },
    }
}

#[tauri::command]
pub async fn run_phase0_command(app: tauri::AppHandle, idea: String) -> Result<serde_json::Value, String> {
    let output_root = app
        .path()
        .app_local_data_dir()
        .map_err(|e| e.to_string())?;

    // Express path: use a real OpenAI-compatible endpoint when a key is configured
    // in the backend environment, otherwise fall back to the offline mock. The key
    // is read here and never crosses the IPC boundary back to the renderer.
    let result = match std::env::var("MAARS_OPENAI_API_KEY") {
        Ok(key) if !key.is_empty() => {
            let base_url = std::env::var("MAARS_OPENAI_BASE_URL")
                .unwrap_or_else(|_| "https://api.openai.com".to_string());
            let model = std::env::var("MAARS_OPENAI_MODEL")
                .unwrap_or_else(|_| "gpt-4o-mini".to_string());
            let client = OpenAICompatibleClient::new(base_url, key);
            let seed = seed_for(idea, "openai", &model);
            core_run_phase0(&client, seed, &output_root).await
        }
        _ => {
            let client = MockLLMClient::default();
            let seed = seed_for(idea, "mock", "mock");
            core_run_phase0(&client, seed, &output_root).await
        }
    }
    .map_err(|e| e.to_string())?;

    Ok(serde_json::json!({ "runId": result.run_id, "events": result.events }))
}

#[tauri::command]
pub fn read_config() -> String {
    "Phase A".to_string()
}
