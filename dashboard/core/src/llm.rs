use async_trait::async_trait;

#[derive(Debug, Clone)]
pub struct CompletionOptions {
    pub model: String,
    pub json_mode: bool,
    pub max_tokens: Option<u32>,
}

#[async_trait]
pub trait LLMClient: Send + Sync {
    async fn complete(&self, prompt: &str, options: CompletionOptions) -> anyhow::Result<String>;
    async fn stream(&self, prompt: &str, options: CompletionOptions) -> anyhow::Result<Vec<String>>;
    async fn count_tokens(&self, text: &str) -> anyhow::Result<usize>;
}

#[derive(Debug, Default, Clone)]
pub struct MockLLMClient;

#[async_trait]
impl LLMClient for MockLLMClient {
    async fn complete(&self, prompt: &str, _options: CompletionOptions) -> anyhow::Result<String> {
        if prompt.contains("Score") {
            Ok(r#"{"phase":0,"iteration":1,"phase_score":9,"decision":"PASS","bottleneck_critic":null,"next_action":"Advance to Phase 1","critic_scores":[{"critic_id":"mock","score":9,"valid":true,"invalidation_reason":null,"key_condition":"mock","findings":[]}]}"#.to_string())
        } else {
            Ok("Builder Card\nPhase 0 skeleton".to_string())
        }
    }

    async fn stream(&self, prompt: &str, options: CompletionOptions) -> anyhow::Result<Vec<String>> {
        Ok(vec![self.complete(prompt, options).await?])
    }

    async fn count_tokens(&self, text: &str) -> anyhow::Result<usize> {
        Ok(text.split_whitespace().count())
    }
}

/// Client for any OpenAI-compatible Chat Completions endpoint (OpenAI, Ollama's
/// OpenAI shim, llama.cpp server, etc.). The API key is held here and is never
/// logged, emitted, or returned in any output (D5 SURFACE 1).
pub struct OpenAICompatibleClient {
    base_url: String,
    api_key: String,
    http: reqwest::Client,
}

impl OpenAICompatibleClient {
    pub fn new(base_url: impl Into<String>, api_key: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into().trim_end_matches('/').to_string(),
            api_key: api_key.into(),
            http: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl LLMClient for OpenAICompatibleClient {
    async fn complete(&self, prompt: &str, options: CompletionOptions) -> anyhow::Result<String> {
        let mut body = serde_json::json!({
            "model": options.model,
            "messages": [{ "role": "user", "content": prompt }],
        });
        if let Some(max) = options.max_tokens {
            body["max_tokens"] = serde_json::json!(max);
        }
        if options.json_mode {
            body["response_format"] = serde_json::json!({ "type": "json_object" });
        }

        let resp = self
            .http
            .post(format!("{}/v1/chat/completions", self.base_url))
            .bearer_auth(&self.api_key)
            .json(&body)
            .send()
            .await?;

        if !resp.status().is_success() {
            // Surface the status only — never the request (which carries the key).
            anyhow::bail!("LLM request failed with status {}", resp.status());
        }

        let json: serde_json::Value = resp.json().await?;
        let content = json["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("malformed completion response"))?;
        Ok(content.to_string())
    }

    async fn stream(&self, prompt: &str, options: CompletionOptions) -> anyhow::Result<Vec<String>> {
        Ok(vec![self.complete(prompt, options).await?])
    }

    async fn count_tokens(&self, text: &str) -> anyhow::Result<usize> {
        // Heuristic fallback (D4 §5); a provider-native counter replaces this in Phase C.
        Ok(text.split_whitespace().count())
    }
}
