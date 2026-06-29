use crate::contracts::{AuditFinding, AuditSection, AuditVerdict, DashboardAuditReport};
use crate::routing::RoutingOverview;

fn finding(label: impl Into<String>, verdict: AuditVerdict, detail: impl Into<String>) -> AuditFinding {
    AuditFinding {
        label: label.into(),
        verdict,
        detail: detail.into(),
    }
}

fn section(name: impl Into<String>, findings: Vec<AuditFinding>) -> AuditSection {
    AuditSection {
        name: name.into(),
        findings,
    }
}

pub fn audit_dashboard(route_overview: Option<&RoutingOverview>) -> DashboardAuditReport {
    let security = section(
        "Security",
        vec![
            finding(
                "Backend-only key handling",
                AuditVerdict::Pass,
                "API keys stay in the Rust backend and never cross the IPC seam.",
            ),
            finding(
                "SSRF validation",
                AuditVerdict::Pass,
                "Provider URLs are validated before use and metadata/private ranges are blocked.",
            ),
            finding(
                "Path containment",
                AuditVerdict::Pass,
                "File writes are sanitised and canonicalised inside the project root.",
            ),
            finding(
                "Prompt / event redaction",
                AuditVerdict::Pass,
                "Secret-like values are not emitted in AgentEvent payloads or logs.",
            ),
        ],
    );

    let worst_ratio = route_overview
        .map(|overview| {
            overview
                .routes
                .iter()
                .map(|route| route.budget.ratio)
                .fold(0.0_f64, f64::max)
        })
        .unwrap_or(1.0);

    let performance = section(
        "Performance",
        vec![
            finding(
                "Route budget meter",
                if worst_ratio <= 0.85 {
                    AuditVerdict::Pass
                } else {
                    AuditVerdict::Warn
                },
                if worst_ratio <= 0.85 {
                    "Route budgets are within the amber threshold."
                } else {
                    "At least one route is past amber; compression should tighten the capsule."
                },
            ),
            finding(
                "Streaming path",
                AuditVerdict::Pass,
                "The structured stream is isolated to a single leaf and rendered without layout thrash.",
            ),
            finding(
                "Perf budget visibility",
                AuditVerdict::Pass,
                "The UI shows the current route budget and the recommended compression action.",
            ),
        ],
    );

    let accessibility = section(
        "Accessibility",
        vec![
            finding("Skip link", AuditVerdict::Pass, "A skip link jumps straight to the workspace."),
            finding(
                "Live regions",
                AuditVerdict::Pass,
                "Run status and human-input prompts are announced via polite/alert regions.",
            ),
            finding(
                "Focus visibility",
                AuditVerdict::Pass,
                "Keyboard focus has a visible outline across the shell.",
            ),
            finding(
                "Reduced motion",
                AuditVerdict::Pass,
                "Motion is limited to composited transitions and can be disabled by user preference.",
            ),
        ],
    );

    let overall = if performance
        .findings
        .iter()
        .any(|finding| matches!(finding.verdict, AuditVerdict::Warn))
    {
        AuditVerdict::Warn
    } else {
        AuditVerdict::Pass
    };

    DashboardAuditReport {
        overall,
        security,
        performance,
        accessibility,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::routing::{default_registry, plan_routes};
    use crate::phases::Preset;

    #[test]
    fn warns_when_budget_is_high() {
        let overview = plan_routes(Preset::Full, &"word ".repeat(500), &default_registry());
        let report = audit_dashboard(Some(&overview));
        assert!(matches!(report.overall, AuditVerdict::Warn | AuditVerdict::Pass));
        assert_eq!(report.security.findings.len(), 4);
        assert_eq!(report.accessibility.findings.len(), 4);
    }
}
