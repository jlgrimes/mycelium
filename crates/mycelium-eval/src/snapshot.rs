use crate::{BenchmarkSuite, ScoreReport};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionabilitySnapshot {
    pub generated_at_unix: u64,
    pub suite: String,
    pub baseline: ScoreReport,
    pub staged: ScoreReport,
}

impl ActionabilitySnapshot {
    pub fn new(suite: BenchmarkSuite, baseline: ScoreReport, staged: ScoreReport) -> Self {
        let generated_at_unix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Self {
            generated_at_unix,
            suite: suite.to_string(),
            baseline,
            staged,
        }
    }

    pub fn suite_label(&self) -> &str {
        &self.suite
    }
}

pub fn write_snapshot(snapshot: &ActionabilitySnapshot, dir: &Path) -> Result<PathBuf> {
    fs::create_dir_all(dir)?;
    let filename = format!("{}-{}.json", snapshot.suite, snapshot.generated_at_unix);
    let path = dir.join(filename);
    let payload = serde_json::to_string_pretty(snapshot)?;
    fs::write(&path, payload)?;
    Ok(path)
}

pub fn load_snapshot(path: &Path) -> Result<ActionabilitySnapshot> {
    let raw = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&raw)?)
}

pub struct ActionabilityDelta {
    pub baseline_score_delta: f64,
    pub baseline_actionability_delta: f64,
    pub baseline_verification_delta: f64,
    pub staged_score_delta: f64,
    pub staged_actionability_delta: f64,
    pub staged_verification_delta: f64,
}

pub fn delta(current: &ActionabilitySnapshot, previous: &ActionabilitySnapshot) -> ActionabilityDelta {
    ActionabilityDelta {
        baseline_score_delta: current.baseline.mean_score - previous.baseline.mean_score,
        baseline_actionability_delta: current.baseline.mean_actionability
            - previous.baseline.mean_actionability,
        baseline_verification_delta: current.baseline.verification_rate
            - previous.baseline.verification_rate,
        staged_score_delta: current.staged.mean_score - previous.staged.mean_score,
        staged_actionability_delta: current.staged.mean_actionability
            - previous.staged.mean_actionability,
        staged_verification_delta: current.staged.verification_rate
            - previous.staged.verification_rate,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scoring::EvalResult;

    fn report(score: f64, actionability: f64, verify: f64) -> ScoreReport {
        ScoreReport {
            results: vec![EvalResult {
                case_id: "case".into(),
                mode: "baseline".into(),
                field_scores: vec![],
                overall: score,
                actionability_score: actionability.round() as u8,
                verification_presence: verify > 0.0,
                error: None,
            }],
            mean_score: score,
            mean_actionability: actionability,
            verification_rate: verify,
            cases_passed: 1,
            cases_total: 1,
        }
    }

    #[test]
    fn snapshot_roundtrip() {
        let snapshot = ActionabilitySnapshot::new(
            BenchmarkSuite::Seed,
            report(0.4, 3.0, 0.5),
            report(0.6, 4.0, 0.7),
        );
        let payload = serde_json::to_string(&snapshot).expect("serialize");
        let decoded: ActionabilitySnapshot = serde_json::from_str(&payload).expect("decode");
        assert_eq!(decoded.suite, "seed");
        assert!(decoded.baseline.mean_score > 0.0);
    }

    #[test]
    fn delta_computes_expected_values() {
        let previous = ActionabilitySnapshot::new(
            BenchmarkSuite::Seed,
            report(0.4, 3.0, 0.5),
            report(0.6, 4.0, 0.7),
        );
        let current = ActionabilitySnapshot::new(
            BenchmarkSuite::Seed,
            report(0.5, 3.5, 0.6),
            report(0.7, 4.2, 0.8),
        );
        let diff = delta(&current, &previous);
        assert!((diff.baseline_score_delta - 0.1).abs() < 0.001);
        assert!((diff.staged_actionability_delta - 0.2).abs() < 0.001);
    }
}
