//! Phase C — provider registry, capability auto-tagging, and per-provider request
//! shaping. Adding a provider touches only this module and the routing table; the
//! orchestration engine (D3) is untouched (D4 §2). The API key lives only inside the
//! request builder and never appears on a `ProviderRef` (D5 SURFACE 1).

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ProviderKind {
    OpenAI,
    Anthropic,
    Gemini,
    /// Any other OpenAI-compatible HTTP endpoint.
    Custom,
    Ollama,
    LlamaCpp,
    Mlx,
}

/// Which HTTP request family a provider speaks. Keeps the request builder small.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApiFamily {
    /// `POST /v1/chat/completions`, `Authorization: Bearer`.
    OpenAICompatible,
    /// `POST /v1/messages`, `x-api-key` + `anthropic-version`.
    Anthropic,
    /// `POST /v1beta/models/{model}:generateContent?key=`.
    Gemini,
}

impl ProviderKind {
    pub const ALL: [ProviderKind; 7] = [
        ProviderKind::OpenAI,
        ProviderKind::Anthropic,
        ProviderKind::Gemini,
        ProviderKind::Custom,
        ProviderKind::Ollama,
        ProviderKind::LlamaCpp,
        ProviderKind::Mlx,
    ];

    pub fn id(self) -> &'static str {
        match self {
            ProviderKind::OpenAI => "openai",
            ProviderKind::Anthropic => "anthropic",
            ProviderKind::Gemini => "gemini",
            ProviderKind::Custom => "custom",
            ProviderKind::Ollama => "ollama",
            ProviderKind::LlamaCpp => "llamacpp",
            ProviderKind::Mlx => "mlx",
        }
    }

    pub fn parse(s: &str) -> Option<ProviderKind> {
        ProviderKind::ALL.into_iter().find(|k| k.id() == s)
    }

    pub fn display_name(self) -> &'static str {
        match self {
            ProviderKind::OpenAI => "OpenAI",
            ProviderKind::Anthropic => "Anthropic",
            ProviderKind::Gemini => "Google Gemini",
            ProviderKind::Custom => "Custom (OpenAI-compatible)",
            ProviderKind::Ollama => "Ollama",
            ProviderKind::LlamaCpp => "llama.cpp",
            ProviderKind::Mlx => "MLX",
        }
    }

    pub fn default_base_url(self) -> &'static str {
        match self {
            ProviderKind::OpenAI => "https://api.openai.com",
            ProviderKind::Anthropic => "https://api.anthropic.com",
            ProviderKind::Gemini => "https://generativelanguage.googleapis.com",
            ProviderKind::Custom => "http://127.0.0.1:8080",
            ProviderKind::Ollama => "http://127.0.0.1:11434",
            ProviderKind::LlamaCpp => "http://127.0.0.1:8080",
            ProviderKind::Mlx => "http://127.0.0.1:8080",
        }
    }

    pub fn is_local(self) -> bool {
        matches!(self, ProviderKind::Ollama | ProviderKind::LlamaCpp | ProviderKind::Mlx)
    }

    pub fn api_family(self) -> ApiFamily {
        match self {
            ProviderKind::Anthropic => ApiFamily::Anthropic,
            ProviderKind::Gemini => ApiFamily::Gemini,
            _ => ApiFamily::OpenAICompatible,
        }
    }

    /// The host a built-in provider's requests are pinned to. A key is only attached
    /// to a request whose host matches (D5 SURFACE 1, control 6). Local/custom
    /// providers have no pinned host (validated by the SSRF filter instead).
    pub fn pinned_host(self) -> Option<&'static str> {
        match self {
            ProviderKind::OpenAI => Some("api.openai.com"),
            ProviderKind::Anthropic => Some("api.anthropic.com"),
            ProviderKind::Gemini => Some("generativelanguage.googleapis.com"),
            _ => None,
        }
    }

    /// Known-good starting capabilities, refined by a live probe where possible.
    pub fn default_capabilities(self) -> ModelCapabilities {
        match self {
            ProviderKind::OpenAI => ModelCapabilities {
                json_mode: true,
                function_calling: true,
                streaming: true,
                est_context_window: 128_000,
            },
            ProviderKind::Anthropic => ModelCapabilities {
                json_mode: false,
                function_calling: true,
                streaming: true,
                est_context_window: 200_000,
            },
            ProviderKind::Gemini => ModelCapabilities {
                json_mode: true,
                function_calling: true,
                streaming: true,
                est_context_window: 1_000_000,
            },
            ProviderKind::Custom => ModelCapabilities {
                json_mode: false,
                function_calling: false,
                streaming: true,
                est_context_window: 8_192,
            },
            ProviderKind::Ollama | ProviderKind::LlamaCpp | ProviderKind::Mlx => {
                ModelCapabilities {
                    json_mode: false,
                    function_calling: false,
                    streaming: true,
                    est_context_window: 8_192,
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelCapabilities {
    pub json_mode: bool,
    pub function_calling: bool,
    pub streaming: bool,
    pub est_context_window: u32,
}

/// A configured provider as the renderer sees it: an opaque id, never a key.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderRef {
    pub id: String,
    pub kind: ProviderKind,
    pub label: String,
    pub base_url: String,
    pub model: String,
    pub capabilities: ModelCapabilities,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ProviderRegistry {
    providers: Vec<ProviderRef>,
}

impl ProviderRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn upsert(&mut self, provider: ProviderRef) {
        if let Some(existing) = self.providers.iter_mut().find(|p| p.id == provider.id) {
            *existing = provider;
        } else {
            self.providers.push(provider);
        }
    }

    pub fn remove(&mut self, id: &str) {
        self.providers.retain(|p| p.id != id);
    }

    pub fn get(&self, id: &str) -> Option<&ProviderRef> {
        self.providers.iter().find(|p| p.id == id)
    }

    pub fn list(&self) -> &[ProviderRef] {
        &self.providers
    }
}

/// Refine starting capabilities from a live probe response. For Ollama/llama.cpp the
/// `/api/tags` (or `/v1/models`) response only confirms the server is reachable and
/// well-shaped; capability flags stay conservative for untrusted local servers.
pub fn refine_from_probe(kind: ProviderKind, probe: Option<&serde_json::Value>) -> ModelCapabilities {
    let mut caps = kind.default_capabilities();
    if kind.is_local() {
        // A reachable local server confirms streaming; json/FC remain off unless proven.
        caps.streaming = probe.is_some();
    }
    caps
}

/// Parse an Ollama `/api/tags` response into model names, verifying the shape before
/// trusting it (D5 SURFACE 2, control 4).
pub fn parse_ollama_tags(body: &serde_json::Value) -> anyhow::Result<Vec<String>> {
    let models = body
        .get("models")
        .and_then(|m| m.as_array())
        .ok_or_else(|| anyhow::anyhow!("unexpected /api/tags shape: missing 'models' array"))?;
    let mut names = Vec::new();
    for m in models {
        let name = m
            .get("name")
            .and_then(|n| n.as_str())
            .ok_or_else(|| anyhow::anyhow!("unexpected /api/tags shape: model without 'name'"))?;
        names.push(name.to_string());
    }
    Ok(names)
}

/// Validate a model name for the Model Pull Assistant and return the fixed-shape
/// argument vector `["pull", <model>]`. No shell is invoked with arbitrary input
/// (D5 §Tauri IPC least-privilege): the name must match a strict allowlist.
pub fn ollama_pull_args(model: &str) -> anyhow::Result<Vec<String>> {
    let model = model.trim();
    if model.is_empty() || model.len() > 128 {
        anyhow::bail!("invalid model name length");
    }
    let ok = model
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-' | ':' | '/'));
    if !ok {
        anyhow::bail!("model name contains disallowed characters");
    }
    if model.contains("..") {
        anyhow::bail!("model name must not contain '..'");
    }
    Ok(vec!["pull".to_string(), model.to_string()])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seven_providers_are_registered() {
        assert_eq!(ProviderKind::ALL.len(), 7);
        for kind in ProviderKind::ALL {
            assert_eq!(ProviderKind::parse(kind.id()), Some(kind));
            assert!(!kind.default_base_url().is_empty());
        }
    }

    #[test]
    fn cloud_providers_are_host_pinned() {
        assert_eq!(ProviderKind::OpenAI.pinned_host(), Some("api.openai.com"));
        assert_eq!(ProviderKind::Anthropic.pinned_host(), Some("api.anthropic.com"));
        assert!(ProviderKind::Ollama.pinned_host().is_none());
    }

    #[test]
    fn ollama_tags_parsed_and_shape_checked() {
        let good = serde_json::json!({ "models": [{ "name": "mistral:7b" }, { "name": "llama3" }] });
        assert_eq!(parse_ollama_tags(&good).unwrap(), vec!["mistral:7b", "llama3"]);
        let bad = serde_json::json!({ "tags": [] });
        assert!(parse_ollama_tags(&bad).is_err());
    }

    #[test]
    fn pull_args_reject_injection() {
        assert_eq!(ollama_pull_args("mistral:7b").unwrap(), vec!["pull", "mistral:7b"]);
        assert!(ollama_pull_args("mistral; rm -rf /").is_err());
        assert!(ollama_pull_args("../../etc/passwd").is_err());
        assert!(ollama_pull_args("").is_err());
    }
}
