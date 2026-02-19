use serde::Serialize;

/// A single benchmark case: an input problem + expected signals in the output.
#[derive(Debug, Clone, Serialize)]
pub struct BenchmarkCase {
    pub id: &'static str,
    pub input: &'static str,
    /// Keywords that should appear in `abstract_shape`.
    pub expect_abstract: &'static [&'static str],
    /// Minimum number of cross-domain matches expected.
    pub expect_min_matches: usize,
    /// Keywords that should appear somewhere in `mapping` or `synthesis`.
    pub expect_keywords: &'static [&'static str],
}

/// 20 seed benchmark cases spanning diverse problem domains.
pub const SEED_CASES: &[BenchmarkCase] = &[
    // 1 — Skill acquisition
    BenchmarkCase {
        id: "trumpet-practice",
        input: "How do I practice trumpet more effectively?",
        expect_abstract: &["repetition", "skill", "feedback"],
        expect_min_matches: 2,
        expect_keywords: &["practice", "technique"],
    },
    // 2 — Software engineering
    BenchmarkCase {
        id: "reduce-tech-debt",
        input: "How can a startup reduce tech debt while shipping fast?",
        expect_abstract: &["trade-off", "accumulation", "maintenance"],
        expect_min_matches: 2,
        expect_keywords: &["refactor", "incremental"],
    },
    // 3 — Biology
    BenchmarkCase {
        id: "immune-response",
        input: "How does the immune system learn to fight new pathogens?",
        expect_abstract: &["adaptation", "recognition", "memory"],
        expect_min_matches: 2,
        expect_keywords: &["pattern", "response"],
    },
    // 4 — Education
    BenchmarkCase {
        id: "teach-math",
        input: "What is the best way to teach algebra to struggling students?",
        expect_abstract: &["scaffold", "abstraction", "progression"],
        expect_min_matches: 2,
        expect_keywords: &["concrete", "step"],
    },
    // 5 — Urban planning
    BenchmarkCase {
        id: "traffic-flow",
        input: "How can a city reduce traffic congestion without building new roads?",
        expect_abstract: &["flow", "bottleneck", "capacity"],
        expect_min_matches: 2,
        expect_keywords: &["route", "demand"],
    },
    // 6 — Cooking
    BenchmarkCase {
        id: "flavor-balance",
        input: "How do chefs balance flavors in a complex dish?",
        expect_abstract: &["balance", "contrast", "harmony"],
        expect_min_matches: 2,
        expect_keywords: &["taste", "adjust"],
    },
    // 7 — Team management
    BenchmarkCase {
        id: "team-conflict",
        input: "How should a manager resolve conflict between two senior engineers?",
        expect_abstract: &["conflict", "resolution", "alignment"],
        expect_min_matches: 2,
        expect_keywords: &["communicate", "perspective"],
    },
    // 8 — Ecology
    BenchmarkCase {
        id: "forest-recovery",
        input: "How does a forest ecosystem recover after a wildfire?",
        expect_abstract: &["succession", "recovery", "resilience"],
        expect_min_matches: 2,
        expect_keywords: &["pioneer", "regenerat"],
    },
    // 9 — Finance
    BenchmarkCase {
        id: "portfolio-risk",
        input: "How should a retail investor diversify a small portfolio?",
        expect_abstract: &["diversif", "risk", "allocation"],
        expect_min_matches: 2,
        expect_keywords: &["asset", "correlat"],
    },
    // 10 — Writing
    BenchmarkCase {
        id: "writers-block",
        input: "How can a novelist overcome persistent writer's block?",
        expect_abstract: &["block", "creativ", "constraint"],
        expect_min_matches: 2,
        expect_keywords: &["routine", "prompt"],
    },
    // 11 — Supply chain
    BenchmarkCase {
        id: "supply-chain",
        input: "How can a manufacturer reduce supply chain disruptions?",
        expect_abstract: &["dependency", "redundancy", "flow"],
        expect_min_matches: 2,
        expect_keywords: &["supplier", "buffer"],
    },
    // 12 — Machine learning
    BenchmarkCase {
        id: "model-overfit",
        input: "How do you prevent overfitting in a deep learning model?",
        expect_abstract: &["generali", "complexity", "fit"],
        expect_min_matches: 2,
        expect_keywords: &["regulariz", "data"],
    },
    // 13 — Architecture
    BenchmarkCase {
        id: "building-energy",
        input: "How can an old building be retrofitted for energy efficiency?",
        expect_abstract: &["efficiency", "insulation", "system"],
        expect_min_matches: 2,
        expect_keywords: &["heat", "upgrade"],
    },
    // 14 — Healthcare
    BenchmarkCase {
        id: "patient-adherence",
        input: "How can doctors improve patient medication adherence?",
        expect_abstract: &["compliance", "behavior", "reminder"],
        expect_min_matches: 2,
        expect_keywords: &["habit", "simplif"],
    },
    // 15 — Game design
    BenchmarkCase {
        id: "game-difficulty",
        input: "How should a game designer tune difficulty curves?",
        expect_abstract: &["progression", "challenge", "feedback"],
        expect_min_matches: 2,
        expect_keywords: &["player", "adapt"],
    },
    // 16 — Agriculture
    BenchmarkCase {
        id: "soil-health",
        input: "How can a farmer restore depleted soil without synthetic fertilizers?",
        expect_abstract: &["nutrient", "cycle", "regenerat"],
        expect_min_matches: 2,
        expect_keywords: &["compost", "crop"],
    },
    // 17 — Negotiation
    BenchmarkCase {
        id: "salary-negotiation",
        input: "What strategies work for negotiating a higher salary?",
        expect_abstract: &["leverage", "value", "anchor"],
        expect_min_matches: 2,
        expect_keywords: &["offer", "market"],
    },
    // 18 — Physics / engineering
    BenchmarkCase {
        id: "bridge-load",
        input: "How do engineers distribute load across a suspension bridge?",
        expect_abstract: &["distribut", "tension", "equilibrium"],
        expect_min_matches: 2,
        expect_keywords: &["cable", "force"],
    },
    // 19 — Psychology
    BenchmarkCase {
        id: "habit-formation",
        input: "What is the most reliable way to build a new daily habit?",
        expect_abstract: &["cue", "routine", "reward"],
        expect_min_matches: 2,
        expect_keywords: &["trigger", "consist"],
    },
    // 20 — Open-source community
    BenchmarkCase {
        id: "oss-contributors",
        input: "How can an open-source project attract and retain contributors?",
        expect_abstract: &["incentive", "communit", "onboard"],
        expect_min_matches: 2,
        expect_keywords: &["document", "mentor"],
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seed_cases_has_20_entries() {
        assert_eq!(SEED_CASES.len(), 20);
    }

    #[test]
    fn all_ids_unique() {
        let mut ids: Vec<&str> = SEED_CASES.iter().map(|c| c.id).collect();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), SEED_CASES.len(), "duplicate benchmark IDs found");
    }

    #[test]
    fn all_cases_have_expectations() {
        for case in SEED_CASES {
            assert!(!case.expect_abstract.is_empty(), "{}: empty expect_abstract", case.id);
            assert!(case.expect_min_matches >= 1, "{}: expect_min_matches < 1", case.id);
            assert!(!case.expect_keywords.is_empty(), "{}: empty expect_keywords", case.id);
        }
    }
}
