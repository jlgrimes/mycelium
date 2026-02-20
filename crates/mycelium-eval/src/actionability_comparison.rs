use crate::actionability_heuristics::{ActionabilityAnalysis, CalibratedActionabilityScorer};
use crate::benchmark::BenchmarkCase;
use mycelium_types::ProblemResponse;
use serde::{Deserialize, Serialize};

/// Comparison between legacy and calibrated actionability scoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionabilityComparison {
    pub case_id: String,
    pub legacy_score: u8,
    pub calibrated_score: u8,
    pub calibrated_analysis: ActionabilityAnalysis,
    pub improvement_percentage: f32,
    pub score_delta: i8,
}

/// Report comparing legacy vs calibrated scoring across multiple cases
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionabilityComparisonReport {
    pub comparisons: Vec<ActionabilityComparison>,
    pub legacy_mean: f32,
    pub calibrated_mean: f32,
    pub overall_improvement_percentage: f32,
    pub cases_improved: usize,
    pub cases_total: usize,
    pub improvement_distribution: ImprovementDistribution,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImprovementDistribution {
    pub significant_improvement: usize, // >30% improvement
    pub moderate_improvement: usize,    // 10-30% improvement
    pub minor_improvement: usize,       // 1-10% improvement
    pub no_change: usize,               // 0% change
    pub regression: usize,              // negative change
}

pub struct ActionabilityComparisonRunner;

impl ActionabilityComparisonRunner {
    /// Compare legacy vs calibrated scoring for a single case
    pub fn compare_case(
        case: &BenchmarkCase,
        response: &ProblemResponse,
    ) -> ActionabilityComparison {
        let legacy_score = Self::legacy_actionability_score(response);
        let calibrated_analysis = CalibratedActionabilityScorer::analyze(response);
        let calibrated_score =
            ((calibrated_analysis.overall_score as f32 / 2.0).round() as u8).min(5); // Scale to 0-5

        let improvement_percentage = if legacy_score > 0 {
            ((calibrated_score as f32 - legacy_score as f32) / legacy_score as f32) * 100.0
        } else if calibrated_score > 0 {
            100.0 // From 0 to something is 100% improvement
        } else {
            0.0
        };

        let score_delta = calibrated_score as i8 - legacy_score as i8;

        ActionabilityComparison {
            case_id: case.id.to_string(),
            legacy_score,
            calibrated_score,
            calibrated_analysis,
            improvement_percentage,
            score_delta,
        }
    }

    /// Generate a report comparing legacy vs calibrated scoring across multiple cases and responses
    pub fn generate_report(
        cases_and_responses: Vec<(&BenchmarkCase, &ProblemResponse)>,
    ) -> ActionabilityComparisonReport {
        let comparisons: Vec<ActionabilityComparison> = cases_and_responses
            .iter()
            .map(|(case, response)| Self::compare_case(case, response))
            .collect();

        let legacy_mean = if !comparisons.is_empty() {
            comparisons
                .iter()
                .map(|c| c.legacy_score as f32)
                .sum::<f32>()
                / comparisons.len() as f32
        } else {
            0.0
        };

        let calibrated_mean = if !comparisons.is_empty() {
            comparisons
                .iter()
                .map(|c| c.calibrated_score as f32)
                .sum::<f32>()
                / comparisons.len() as f32
        } else {
            0.0
        };

        let overall_improvement_percentage = if legacy_mean > 0.0 {
            ((calibrated_mean - legacy_mean) / legacy_mean) * 100.0
        } else if calibrated_mean > 0.0 {
            100.0
        } else {
            0.0
        };

        let cases_improved = comparisons.iter().filter(|c| c.score_delta > 0).count();
        let cases_total = comparisons.len();

        let improvement_distribution = Self::calculate_improvement_distribution(&comparisons);

        ActionabilityComparisonReport {
            comparisons,
            legacy_mean,
            calibrated_mean,
            overall_improvement_percentage,
            cases_improved,
            cases_total,
            improvement_distribution,
        }
    }

    fn calculate_improvement_distribution(
        comparisons: &[ActionabilityComparison],
    ) -> ImprovementDistribution {
        let mut significant_improvement = 0;
        let mut moderate_improvement = 0;
        let mut minor_improvement = 0;
        let mut no_change = 0;
        let mut regression = 0;

        for comparison in comparisons {
            let improvement = comparison.improvement_percentage;
            if improvement > 30.0 {
                significant_improvement += 1;
            } else if improvement > 10.0 {
                moderate_improvement += 1;
            } else if improvement > 0.0 {
                minor_improvement += 1;
            } else if improvement == 0.0 {
                no_change += 1;
            } else {
                regression += 1;
            }
        }

        ImprovementDistribution {
            significant_improvement,
            moderate_improvement,
            minor_improvement,
            no_change,
            regression,
        }
    }

    /// Legacy scoring implementation for comparison baseline
    fn legacy_actionability_score(response: &ProblemResponse) -> u8 {
        let synth = response.synthesis.to_lowercase();
        let map = response.mapping.to_lowercase();

        let mut score = 0_u8;
        if !response.abstract_shape.trim().is_empty() {
            score += 1;
        }
        if response.cross_domain_matches.len() >= 3 {
            score += 1;
        }
        if ["step", "first", "then", "run", "check", "measure"]
            .iter()
            .any(|kw| synth.contains(kw))
        {
            score += 1;
        }
        if ["verify", "test", "assert", "pass", "fail"]
            .iter()
            .any(|kw| synth.contains(kw))
        {
            score += 1;
        }
        if ["map", "confidence", "source", "target", "->"]
            .iter()
            .any(|kw| map.contains(kw) || synth.contains(kw))
        {
            score += 1;
        }
        score.min(5)
    }

    /// Helper to create sample high-quality responses for testing
    pub fn create_high_actionability_response() -> ProblemResponse {
        ProblemResponse {
            abstract_shape: "Systematic skill acquisition framework with measurement loops and feedback optimization".to_string(),
            cross_domain_matches: vec![
                "Athletics: periodized training with performance tracking".to_string(),
                "Manufacturing: kaizen continuous improvement cycles".to_string(),
                "Software: test-driven development with CI/CD pipelines".to_string(),
                "Music: deliberate practice with metronome and recording analysis".to_string(),
            ],
            mapping: "Map skill acquisition phases -> systematic training blocks with confidence tracking (high confidence: 0.92) -> measurable performance outcomes".to_string(),
            synthesis: "1. First, establish baseline measurements using objective assessment tools. 2. Then implement structured practice sessions with specific, measurable goals. 3. Apply continuous verification through regular testing and performance tracking. 4. Use feedback loops to adjust training intensity and focus areas based on measured results. 5. Finally, validate improvement through comparative analysis against initial baseline metrics.".to_string(),
        }
    }

    /// Helper to create sample low-quality responses for testing
    pub fn create_low_actionability_response() -> ProblemResponse {
        ProblemResponse {
            abstract_shape: "".to_string(),
            cross_domain_matches: vec!["something".to_string()],
            mapping: "".to_string(),
            synthesis: "Just try harder and see what happens.".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::benchmark::SEED_CASES;

    #[test]
    fn comparison_shows_improvement_for_high_quality_response() {
        let case = &SEED_CASES[0];
        let response = ActionabilityComparisonRunner::create_high_actionability_response();

        let comparison = ActionabilityComparisonRunner::compare_case(case, &response);

        assert!(comparison.calibrated_score >= comparison.legacy_score);
        assert!(comparison.improvement_percentage >= 0.0);
        assert!(comparison.calibrated_analysis.overall_score > 0);
    }

    #[test]
    fn comparison_handles_low_quality_response() {
        let case = &SEED_CASES[0];
        let response = ActionabilityComparisonRunner::create_low_actionability_response();

        let comparison = ActionabilityComparisonRunner::compare_case(case, &response);

        // Even low quality responses might get some improvement from calibrated scoring
        assert!(comparison.calibrated_analysis.overall_score <= 10); // Score should be within expected range
        assert!(!comparison
            .calibrated_analysis
            .improvement_suggestions
            .is_empty());
    }

    #[test]
    fn report_generation_works_with_multiple_cases() {
        let high_response = ActionabilityComparisonRunner::create_high_actionability_response();
        let low_response = ActionabilityComparisonRunner::create_low_actionability_response();

        let cases_and_responses = vec![
            (&SEED_CASES[0], &high_response),
            (&SEED_CASES[1], &low_response),
        ];

        let report = ActionabilityComparisonRunner::generate_report(cases_and_responses);

        assert_eq!(report.cases_total, 2);
        assert!(report.calibrated_mean >= report.legacy_mean);
        assert!(!report.comparisons.is_empty());
    }

    #[test]
    fn improvement_distribution_categorizes_correctly() {
        let high_response = ActionabilityComparisonRunner::create_high_actionability_response();
        let low_response = ActionabilityComparisonRunner::create_low_actionability_response();

        let cases_and_responses = vec![
            (&SEED_CASES[0], &high_response),
            (&SEED_CASES[1], &low_response),
        ];

        let report = ActionabilityComparisonRunner::generate_report(cases_and_responses);
        let dist = &report.improvement_distribution;

        // Should have some distribution of improvements
        let total_categorized = dist.significant_improvement
            + dist.moderate_improvement
            + dist.minor_improvement
            + dist.no_change
            + dist.regression;
        assert_eq!(total_categorized, report.cases_total);
    }

    #[test]
    fn legacy_scoring_matches_original_implementation() {
        let response = ProblemResponse {
            abstract_shape: "test shape".to_string(),
            cross_domain_matches: vec!["a".to_string(), "b".to_string(), "c".to_string()],
            mapping: "source -> target".to_string(),
            synthesis: "First, run a test and verify the result".to_string(),
        };

        let legacy_score = ActionabilityComparisonRunner::legacy_actionability_score(&response);
        assert_eq!(legacy_score, 5); // Should get full score for this well-formed response
    }
}
