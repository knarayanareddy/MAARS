use crate::contracts::{CriticScore, GateDecision, ScoreReport};

/// 3-path JSON extraction for the Scoring Aggregator output (R7 / D4). The loop must
/// never deadlock on a malformed score: callers re-ask on `Err`, then fall back to a
/// human-supplied score. Paths, in order:
///   1. The whole response parses as a ScoreReport.
///   2. A ```json fenced block parses.
///   3. The first balanced `{...}` span parses.
pub fn parse_score(raw: &str) -> anyhow::Result<ScoreReport> {
    if let Ok(report) = serde_json::from_str::<ScoreReport>(raw.trim()) {
        return Ok(report);
    }
    if let Some(block) = fenced_json_block(raw) {
        if let Ok(report) = serde_json::from_str::<ScoreReport>(&block) {
            return Ok(report);
        }
    }
    if let Some(span) = first_balanced_object(raw) {
        if let Ok(report) = serde_json::from_str::<ScoreReport>(&span) {
            return Ok(report);
        }
    }
    anyhow::bail!("score JSON could not be extracted by any path")
}

fn fenced_json_block(raw: &str) -> Option<String> {
    let start = raw.find("```")?;
    let after = &raw[start + 3..];
    // Skip an optional language tag (e.g. "json\n").
    let body_start = after.find('\n').map(|i| i + 1).unwrap_or(0);
    let body = &after[body_start..];
    let end = body.find("```")?;
    Some(body[..end].trim().to_string())
}

fn first_balanced_object(raw: &str) -> Option<String> {
    let bytes = raw.as_bytes();
    let start = raw.find('{')?;
    let mut depth = 0usize;
    let mut in_string = false;
    let mut escaped = false;
    for i in start..bytes.len() {
        let c = bytes[i] as char;
        if in_string {
            if escaped {
                escaped = false;
            } else if c == '\\' {
                escaped = true;
            } else if c == '"' {
                in_string = false;
            }
            continue;
        }
        match c {
            '"' => in_string = true,
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return Some(raw[start..=i].to_string());
                }
            }
            _ => {}
        }
    }
    None
}

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
