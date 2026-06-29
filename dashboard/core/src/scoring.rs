use crate::contracts::{CriticScore, GateDecision, ScoreReport};

pub fn aggregate(mut report: ScoreReport) -> ScoreReport {
    let lowest = report
        .critic_scores
        .iter()
        .filter(|score| score.valid)
        .map(|score| score.score)
        .fold(None, |acc, s| Some(acc.map_or(s, |m: f64| m.min(s))));

    report.phase_score = lowest.unwrap_or(0.0);
    report.decision = if report.phase_score >= 9.0 {
        GateDecision::Pass
    } else {
        GateDecision::Fail
    };
    report
}

pub fn score_is_supported(score: &CriticScore) -> bool {
    score.valid && (0.0..=10.0).contains(&score.score)
}
