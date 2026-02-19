use crate::actionability_heuristics::CalibratedActionabilityScorer;
use crate::benchmark::BenchmarkCase;
use mycelium_types::ProblemResponse;
use serde::{Deserialize, Serialize};

/// Score for an individual response field (0.0–1.0).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldScore {
    pub field: String,
    pub score: f64,
    pub matched: Vec<String>,
    pub missed: Vec<String>,
}

/// Full evaluation result for one benchmark case.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvalResult {
    pub case_id: String,
    pub mode: String,
    pub field_scores: Vec<FieldScore>,
    pub overall: f64,
    pub actionability_score: u8,
    pub verification_presence: bool,
    pub error: Option<String>,
}

/// Aggregate report comparing two run modes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreReport {
    pub results: Vec<EvalResult>,
    pub mean_score: f64,
    pub mean_actionability: f64,
    pub verification_rate: f64,
    pub cases_passed: usize,
    pub cases_total: usize,
}

pub struct Scorer;

impl Scorer {
    pub fn score(case: &BenchmarkCase, response: &ProblemResponse) -> EvalResult {
        let abstract_score = Self::keyword_score(
            "abstract_shape",
            &response.abstract_shape,
            case.expect_abstract,
        );
        let matches_score =
            Self::matches_score(&response.cross_domain_matches, case.expect_min_matches);
        let keyword_score = Self::combined_keyword_score(
            &response.mapping,
            &response.synthesis,
            case.expect_keywords,
        );

        let overall = (abstract_score.score + matches_score.score + keyword_score.score) / 3.0;
        let verification_presence = has_verification_signal(&response.synthesis);
        let actionability_score = CalibratedActionabilityScorer::legacy_score(response);

        EvalResult {
            case_id: case.id.to_string(),
            mode: String::new(),
            field_scores: vec![abstract_score, matches_score, keyword_score],
            overall,
            actionability_score,
            verification_presence,
            error: None,
        }
    }

    pub fn error_result(case: &BenchmarkCase, mode: &str, err: &str) -> EvalResult {
        EvalResult {
            case_id: case.id.to_string(),
            mode: mode.to_string(),
            field_scores: vec![],
            overall: 0.0,
            actionability_score: 0,
            verification_presence: false,
            error: Some(err.to_string()),
        }
    }

    pub fn report(results: Vec<EvalResult>) -> ScoreReport {
        let cases_total = results.len();
        let pass_threshold = 0.3;
        let cases_passed = results
            .iter()
            .filter(|r| r.overall >= pass_threshold)
            .count();

        let mean_score = if cases_total > 0 {
            results.iter().map(|r| r.overall).sum::<f64>() / cases_total as f64
        } else {
            0.0
        };

        let mean_actionability = if cases_total > 0 {
            results
                .iter()
                .map(|r| r.actionability_score as f64)
                .sum::<f64>()
                / cases_total as f64
        } else {
            0.0
        };

        let verification_rate = if cases_total > 0 {
            results.iter().filter(|r| r.verification_presence).count() as f64 / cases_total as f64
        } else {
            0.0
        };

        ScoreReport {
            results,
            mean_score,
            mean_actionability,
            verification_rate,
            cases_passed,
            cases_total,
        }
    }

    fn keyword_score(field: &str, text: &str, keywords: &[&str]) -> FieldScore {
        let lower = text.to_lowercase();
        let mut matched = vec![];
        let mut missed = vec![];
        for &kw in keywords {
            if lower.contains(&kw.to_lowercase()) {
                matched.push(kw.to_string());
            } else {
                missed.push(kw.to_string());
            }
        }
        let score = if keywords.is_empty() {
            1.0
        } else {
            matched.len() as f64 / keywords.len() as f64
        };
        FieldScore {
            field: field.to_string(),
            score,
            matched,
            missed,
        }
    }

    fn matches_score(matches: &[String], min_expected: usize) -> FieldScore {
        let count = matches.len();
        let score = if min_expected == 0 {
            1.0
        } else {
            (count as f64 / min_expected as f64).min(1.0)
        };
        FieldScore {
            field: "cross_domain_matches".to_string(),
            score,
            matched: vec![format!("{count} matches (need >= {min_expected})")],
            missed: if count < min_expected {
                vec![format!("only {count}, expected >= {min_expected}")]
            } else {
                vec![]
            },
        }
    }

    fn combined_keyword_score(mapping: &str, synthesis: &str, keywords: &[&str]) -> FieldScore {
        let combined = format!("{} {}", mapping.to_lowercase(), synthesis.to_lowercase());
        let mut matched = vec![];
        let mut missed = vec![];
        for &kw in keywords {
            if combined.contains(&kw.to_lowercase()) {
                matched.push(kw.to_string());
            } else {
                missed.push(kw.to_string());
            }
        }
        let score = if keywords.is_empty() {
            1.0
        } else {
            matched.len() as f64 / keywords.len() as f64
        };
        FieldScore {
            field: "mapping+synthesis".to_string(),
            score,
            matched,
            missed,
        }
    }
}

fn has_verification_signal(synthesis: &str) -> bool {
    let lower = synthesis.to_lowercase();
    [
        "verify",
        "verification",
        "assert",
        "test",
        "reproduce",
        "pass condition",
    ]
    .iter()
    .any(|kw| lower.contains(kw))
}

// Legacy actionability scoring function removed - use CalibratedActionabilityScorer::legacy_score instead

#[cfg(test)]
mod tests {
    use super::*;
    use crate::benchmark::SEED_CASES;

    fn make_response(
        abstract_shape: &str,
        matches: Vec<&str>,
        mapping: &str,
        synthesis: &str,
    ) -> ProblemResponse {
        ProblemResponse {
            abstract_shape: abstract_shape.to_string(),
            cross_domain_matches: matches.into_iter().map(String::from).collect(),
            mapping: mapping.to_string(),
            synthesis: synthesis.to_string(),
        }
    }

    #[test]
    fn perfect_score_for_matching_response() {
        let case = &SEED_CASES[0];
        let resp = make_response(
            "Repetition-based skill acquisition with feedback loops",
            vec![
                "Music practice chunking",
                "Athletic interval training",
                "Compiler passes",
            ],
            "Map practice sessions to technique drills",
            "Use spaced repetition technique; then run a test recording and verify improvements",
        );
        let result = Scorer::score(case, &resp);
        assert!(result.overall > 0.8);
        assert!(result.verification_presence);
        assert!(result.actionability_score >= 4);
    }

    #[test]
    fn zero_score_for_empty_response() {
        let case = &SEED_CASES[0];
        let resp = make_response("", vec![], "", "");
        let result = Scorer::score(case, &resp);
        assert!(result.overall < 0.1);
        assert_eq!(result.actionability_score, 0);
        assert!(!result.verification_presence);
    }

    #[test]
    fn error_result_has_zero_score() {
        let case = &SEED_CASES[0];
        let result = Scorer::error_result(case, "baseline", "provider timeout");
        assert_eq!(result.overall, 0.0);
        assert_eq!(result.actionability_score, 0);
    }

    #[test]
    fn report_computes_aggregate_metrics() {
        let results = vec![
            EvalResult {
                case_id: "a".into(),
                mode: "test".into(),
                field_scores: vec![],
                overall: 0.8,
                actionability_score: 5,
                verification_presence: true,
                error: None,
            },
            EvalResult {
                case_id: "b".into(),
                mode: "test".into(),
                field_scores: vec![],
                overall: 0.4,
                actionability_score: 1,
                verification_presence: false,
                error: None,
            },
        ];
        let report = Scorer::report(results);
        assert!((report.mean_score - 0.6).abs() < 0.001);
        assert!((report.mean_actionability - 3.0).abs() < 0.001);
        assert!((report.verification_rate - 0.5).abs() < 0.001);
    }
}
