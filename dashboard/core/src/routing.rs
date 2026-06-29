use crate::phases::Preset;
use crate::providers::{ModelCapabilities, ProviderKind, ProviderRef, ProviderRegistry};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RouteRole {
    Continuity,
    Research,
    Builder,
    Critique,
    Scoring,
    Arbitration,
    Snapshot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ContextBand {
    Healthy,
    Amber,
    Red,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContextBudget {
    pub used_tokens: usize,
    pub limit_tokens: usize,
    pub ratio: f64,
    pub band: ContextBand,
    pub recommendation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoutePlan {
    pub role: RouteRole,
    pub provider_id: String,
    pub provider_kind: ProviderKind,
    pub model: String,
    pub reason: String,
    pub requirements: Vec<String>,
    pub capabilities: ModelCapabilities,
    pub budget: ContextBudget,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoutingOverview {
    pub preset: Preset,
    pub routes: Vec<RoutePlan>,
}

pub fn default_registry() -> ProviderRegistry {
    let mut registry = ProviderRegistry::new();
    for kind in ProviderKind::ALL {
        let id = kind.id().to_string();
        registry.upsert(ProviderRef {
            id: id.clone(),
            kind,
            label: kind.display_name().to_string(),
            base_url: kind.default_base_url().to_string(),
            model: match kind {
                ProviderKind::OpenAI => "gpt-4o-mini".into(),
                ProviderKind::Anthropic => "claude-3-5-sonnet-latest".into(),
                ProviderKind::Gemini => "gemini-1.5-pro".into(),
                ProviderKind::Custom => "custom-openai-compatible".into(),
                ProviderKind::Ollama => "llama3.1".into(),
                ProviderKind::LlamaCpp => "mistral".into(),
                ProviderKind::Mlx => "qwen2.5".into(),
            },
            capabilities: kind.default_capabilities(),
        });
    }
    registry
}

pub fn budget_for(prompt: &str, limit_tokens: usize) -> ContextBudget {
    let used_tokens = prompt.split_whitespace().count();
    let ratio = if limit_tokens == 0 {
        1.0
    } else {
        used_tokens as f64 / limit_tokens as f64
    };
    let (band, recommendation) = if ratio < 0.70 {
        (ContextBand::Healthy, "none".to_string())
    } else if ratio < 0.85 {
        (ContextBand::Amber, "drop verbose research prose".to_string())
    } else if ratio < 0.95 {
        (
            ContextBand::Red,
            "tighten capsule and trim advisory history".to_string(),
        )
    } else {
        (
            ContextBand::Critical,
            "Context Compression Active: keep only the minimum lossless set".to_string(),
        )
    };
    ContextBudget {
        used_tokens,
        limit_tokens,
        ratio,
        band,
        recommendation,
    }
}

fn strongest(registry: &ProviderRegistry) -> Option<ProviderRef> {
    registry
        .list()
        .iter()
        .cloned()
        .max_by_key(|p| {
            (
                p.capabilities.est_context_window,
                p.capabilities.json_mode,
                p.capabilities.function_calling,
                p.capabilities.streaming,
            )
        })
}

fn smallest(registry: &ProviderRegistry) -> Option<ProviderRef> {
    registry
        .list()
        .iter()
        .cloned()
        .min_by_key(|p| {
            (
                p.capabilities.est_context_window,
                p.capabilities.json_mode as u8,
                p.capabilities.function_calling as u8,
            )
        })
}

fn first_with<F>(registry: &ProviderRegistry, pred: F) -> Option<ProviderRef>
where
    F: Fn(&ProviderRef) -> bool,
{
    registry.list().iter().cloned().find(pred)
}

pub fn plan_routes(preset: Preset, prompt: &str, registry: &ProviderRegistry) -> RoutingOverview {
    let mut routes = Vec::new();
    let strong = strongest(registry).expect("provider registry is empty");
    let tiny = smallest(registry).unwrap_or_else(|| strong.clone());

    for role in [
        RouteRole::Continuity,
        RouteRole::Research,
        RouteRole::Builder,
        RouteRole::Critique,
        RouteRole::Scoring,
        RouteRole::Arbitration,
        RouteRole::Snapshot,
    ] {
        let provider = match role {
            RouteRole::Research => first_with(registry, |p| p.capabilities.function_calling)
                .unwrap_or_else(|| strong.clone()),
            RouteRole::Scoring => first_with(registry, |p| p.capabilities.json_mode)
                .unwrap_or_else(|| strong.clone()),
            RouteRole::Continuity | RouteRole::Snapshot => tiny.clone(),
            _ => strong.clone(),
        };

        let limit = match role {
            RouteRole::Continuity | RouteRole::Snapshot => {
                provider.capabilities.est_context_window / 2
            }
            RouteRole::Research => provider.capabilities.est_context_window / 3,
            RouteRole::Scoring => provider.capabilities.est_context_window / 4,
            _ => provider.capabilities.est_context_window,
        } as usize;

        routes.push(RoutePlan {
            role,
            provider_id: provider.id.clone(),
            provider_kind: provider.kind,
            model: provider.model.clone(),
            reason: match role {
                RouteRole::Research => "function-calling required".into(),
                RouteRole::Scoring => "json mode required".into(),
                RouteRole::Continuity | RouteRole::Snapshot => {
                    "efficient compression step".into()
                }
                RouteRole::Builder => "strongest configured model".into(),
                RouteRole::Critique => "strongest configured model".into(),
                RouteRole::Arbitration => "strongest configured model".into(),
            },
            requirements: match role {
                RouteRole::Research => vec!["functionCalling".into()],
                RouteRole::Scoring => vec!["jsonMode".into()],
                RouteRole::Continuity | RouteRole::Snapshot => vec!["efficient".into()],
                _ => vec!["mostCapable".into()],
            },
            capabilities: provider.capabilities,
            budget: budget_for(prompt, limit),
        });
    }

    if matches!(preset, Preset::Full) {
        for route in &mut routes {
            route.budget.limit_tokens = route.budget.limit_tokens.saturating_mul(2);
        }
    }

    RoutingOverview { preset, routes }
}

pub fn validate_provider_url(base_url: &str) -> anyhow::Result<()> {
    let url = url::Url::parse(base_url)?;
    if url.scheme() != "http" && url.scheme() != "https" {
        anyhow::bail!("only http/https provider URLs are allowed");
    }
    let host = url
        .host_str()
        .ok_or_else(|| anyhow::anyhow!("missing host"))?;
    let is_loopback =
        matches!(host, "127.0.0.1" | "localhost" | "::1") || host.ends_with(".localhost");
    if !is_loopback {
        if host == "169.254.169.254" || host.starts_with("169.254.") {
            anyhow::bail!("metadata-link-local host is blocked");
        }
        if host.starts_with("10.")
            || host.starts_with("192.168.")
            || host.starts_with("172.16.")
            || host.starts_with("172.17.")
            || host.starts_with("172.18.")
            || host.starts_with("172.19.")
            || host.starts_with("172.2")
        {
            anyhow::bail!("private network host requires explicit confirmation");
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plan_covers_all_roles() {
        let registry = default_registry();
        let overview = plan_routes(Preset::Standard, "Build a dashboard with routing", &registry);
        assert_eq!(overview.routes.len(), 7);
        assert!(overview.routes.iter().any(|r| r.role == RouteRole::Research && r.requirements.contains(&"functionCalling".to_string())));
        assert!(overview.routes.iter().any(|r| r.role == RouteRole::Scoring && r.requirements.contains(&"jsonMode".to_string())));
    }

    #[test]
    fn budget_bands_trigger() {
        let healthy = budget_for("short prompt", 100);
        assert_eq!(healthy.band, ContextBand::Healthy);
        let red = budget_for(&"word ".repeat(90), 100);
        assert!(matches!(red.band, ContextBand::Red | ContextBand::Critical));
    }

    #[test]
    fn ssrf_validation_blocks_bad_hosts() {
        assert!(validate_provider_url("https://api.openai.com").is_ok());
        assert!(validate_provider_url("http://127.0.0.1:11434").is_ok());
        assert!(validate_provider_url("https://169.254.169.254").is_err());
    }
}
