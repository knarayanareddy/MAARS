//! The 17 MAARS phases as data (ADR: data-driven phases — adding one is a config
//! change, not new control flow). The engine iterates whatever `phases_for` returns.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Preset {
    /// Phases 0, 1, 4 — a weekend-sized slice.
    Lite,
    /// The main spine 0..10 (no `.5` interstitials).
    Standard,
    /// Every phase, including all interstitials.
    Full,
}

impl Preset {
    pub fn parse(s: &str) -> Preset {
        match s.to_ascii_lowercase().as_str() {
            "lite" => Preset::Lite,
            "full" => Preset::Full,
            _ => Preset::Standard,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PhaseDef {
    pub id: String,
    pub title: String,
    /// `.5` interstitial phases run only under the Full preset.
    pub interstitial: bool,
    /// Whether this phase is part of the Lite preset.
    pub in_lite: bool,
}

fn all_phases() -> Vec<PhaseDef> {
    let raw: &[(&str, &str, bool, bool)] = &[
        ("0", "Ideation & Inception", false, true),
        ("1", "Requirements & Analysis", false, true),
        ("1.5", "API Contract Design", true, false),
        ("2", "System Architecture", false, false),
        ("2.5", "Database Design", true, false),
        ("3", "UX / UI Design", false, false),
        ("4", "Implementation Planning", false, true),
        ("5", "Security & Threat Modelling", false, false),
        ("5.5", "Penetration Test Planning", true, false),
        ("6", "Implementation", false, false),
        ("6.5", "Migration & Cutover", true, false),
        ("7", "Testing & QA", false, false),
        ("7.5", "Performance & Load", true, false),
        ("8", "Deployment & Release", false, false),
        ("8.5", "Observability & SRE", true, false),
        ("9", "Operations & Maintenance", false, false),
        ("10", "Retrospective & Evolution", false, false),
    ];
    raw.iter()
        .map(|(id, title, interstitial, in_lite)| PhaseDef {
            id: (*id).to_string(),
            title: (*title).to_string(),
            interstitial: *interstitial,
            in_lite: *in_lite,
        })
        .collect()
}

/// The ordered phase list for a preset.
pub fn phases_for(preset: Preset) -> Vec<PhaseDef> {
    all_phases()
        .into_iter()
        .filter(|p| match preset {
            Preset::Full => true,
            Preset::Standard => !p.interstitial,
            Preset::Lite => p.in_lite,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preset_counts() {
        assert_eq!(phases_for(Preset::Full).len(), 17);
        assert_eq!(phases_for(Preset::Standard).len(), 11); // 0..10, no .5
        assert_eq!(phases_for(Preset::Lite).len(), 3); // 0, 1, 4
    }

    #[test]
    fn standard_runs_zero_through_ten() {
        let ids: Vec<String> = phases_for(Preset::Standard)
            .into_iter()
            .map(|p| p.id)
            .collect();
        assert_eq!(ids.first().unwrap(), "0");
        assert_eq!(ids.last().unwrap(), "10");
    }
}
