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
    BenchmarkCase {
        id: "trumpet-practice",
        input: "How do I practice trumpet more effectively?",
        expect_abstract: &["repetition", "skill", "feedback"],
        expect_min_matches: 2,
        expect_keywords: &["practice", "technique"],
    },
    BenchmarkCase {
        id: "reduce-tech-debt",
        input: "How can a startup reduce tech debt while shipping fast?",
        expect_abstract: &["trade-off", "accumulation", "maintenance"],
        expect_min_matches: 2,
        expect_keywords: &["refactor", "incremental"],
    },
    BenchmarkCase {
        id: "immune-response",
        input: "How does the immune system learn to fight new pathogens?",
        expect_abstract: &["adaptation", "recognition", "memory"],
        expect_min_matches: 2,
        expect_keywords: &["pattern", "response"],
    },
    BenchmarkCase {
        id: "teach-math",
        input: "What is the best way to teach algebra to struggling students?",
        expect_abstract: &["scaffold", "abstraction", "progression"],
        expect_min_matches: 2,
        expect_keywords: &["concrete", "step"],
    },
    BenchmarkCase {
        id: "traffic-flow",
        input: "How can a city reduce traffic congestion without building new roads?",
        expect_abstract: &["flow", "bottleneck", "capacity"],
        expect_min_matches: 2,
        expect_keywords: &["route", "demand"],
    },
    BenchmarkCase {
        id: "flavor-balance",
        input: "How do chefs balance flavors in a complex dish?",
        expect_abstract: &["balance", "contrast", "harmony"],
        expect_min_matches: 2,
        expect_keywords: &["taste", "adjust"],
    },
    BenchmarkCase {
        id: "team-conflict",
        input: "How should a manager resolve conflict between two senior engineers?",
        expect_abstract: &["conflict", "resolution", "alignment"],
        expect_min_matches: 2,
        expect_keywords: &["communicate", "perspective"],
    },
    BenchmarkCase {
        id: "forest-recovery",
        input: "How does a forest ecosystem recover after a wildfire?",
        expect_abstract: &["succession", "recovery", "resilience"],
        expect_min_matches: 2,
        expect_keywords: &["pioneer", "regenerat"],
    },
    BenchmarkCase {
        id: "portfolio-risk",
        input: "How should a retail investor diversify a small portfolio?",
        expect_abstract: &["diversif", "risk", "allocation"],
        expect_min_matches: 2,
        expect_keywords: &["asset", "correlat"],
    },
    BenchmarkCase {
        id: "writers-block",
        input: "How can a novelist overcome persistent writer's block?",
        expect_abstract: &["block", "creativ", "constraint"],
        expect_min_matches: 2,
        expect_keywords: &["routine", "prompt"],
    },
    BenchmarkCase {
        id: "supply-chain",
        input: "How can a manufacturer reduce supply chain disruptions?",
        expect_abstract: &["dependency", "redundancy", "flow"],
        expect_min_matches: 2,
        expect_keywords: &["supplier", "buffer"],
    },
    BenchmarkCase {
        id: "model-overfit",
        input: "How do you prevent overfitting in a deep learning model?",
        expect_abstract: &["generali", "complexity", "fit"],
        expect_min_matches: 2,
        expect_keywords: &["regulariz", "data"],
    },
    BenchmarkCase {
        id: "building-energy",
        input: "How can an old building be retrofitted for energy efficiency?",
        expect_abstract: &["efficiency", "insulation", "system"],
        expect_min_matches: 2,
        expect_keywords: &["heat", "upgrade"],
    },
    BenchmarkCase {
        id: "patient-adherence",
        input: "How can doctors improve patient medication adherence?",
        expect_abstract: &["compliance", "behavior", "reminder"],
        expect_min_matches: 2,
        expect_keywords: &["habit", "simplif"],
    },
    BenchmarkCase {
        id: "game-difficulty",
        input: "How should a game designer tune difficulty curves?",
        expect_abstract: &["progression", "challenge", "feedback"],
        expect_min_matches: 2,
        expect_keywords: &["player", "adapt"],
    },
    BenchmarkCase {
        id: "soil-health",
        input: "How can a farmer restore depleted soil without synthetic fertilizers?",
        expect_abstract: &["nutrient", "cycle", "regenerat"],
        expect_min_matches: 2,
        expect_keywords: &["compost", "crop"],
    },
    BenchmarkCase {
        id: "salary-negotiation",
        input: "What strategies work for negotiating a higher salary?",
        expect_abstract: &["leverage", "value", "anchor"],
        expect_min_matches: 2,
        expect_keywords: &["offer", "market"],
    },
    BenchmarkCase {
        id: "bridge-load",
        input: "How do engineers distribute load across a suspension bridge?",
        expect_abstract: &["distribut", "tension", "equilibrium"],
        expect_min_matches: 2,
        expect_keywords: &["cable", "force"],
    },
    BenchmarkCase {
        id: "habit-formation",
        input: "What is the most reliable way to build a new daily habit?",
        expect_abstract: &["cue", "routine", "reward"],
        expect_min_matches: 2,
        expect_keywords: &["trigger", "consist"],
    },
    BenchmarkCase {
        id: "oss-contributors",
        input: "How can an open-source project attract and retain contributors?",
        expect_abstract: &["incentive", "communit", "onboard"],
        expect_min_matches: 2,
        expect_keywords: &["document", "mentor"],
    },
];

/// 12 isomorphic transfer regression cases for KR3 evaluation.
pub const ISOMORPHIC_TRANSFER_CASES: &[BenchmarkCase] = &[
    BenchmarkCase {
        id: "transfer-music-to-code",
        input: "Apply practice chunking techniques from music to learning programming.",
        expect_abstract: &["segmentation", "repetition", "mastery"],
        expect_min_matches: 3,
        expect_keywords: &["isolate", "drill", "integrate"],
    },
    BenchmarkCase {
        id: "transfer-cooking-to-pm",
        input: "Use mise en place principles to improve project management.",
        expect_abstract: &["preparation", "organization", "workflow"],
        expect_min_matches: 3,
        expect_keywords: &["setup", "dependency", "execute"],
    },
    BenchmarkCase {
        id: "transfer-immune-to-security",
        input: "Apply immune system recognition patterns to network intrusion detection.",
        expect_abstract: &["recognition", "adaptation", "memory"],
        expect_min_matches: 3,
        expect_keywords: &["pattern", "anomaly", "learn"],
    },
    BenchmarkCase {
        id: "transfer-jazz-to-agile",
        input: "Transfer jazz improvisation principles to agile team coordination.",
        expect_abstract: &["improvisation", "structure", "coordination"],
        expect_min_matches: 3,
        expect_keywords: &["framework", "responsive", "rhythm"],
    },
    BenchmarkCase {
        id: "transfer-forest-to-architecture",
        input: "Use forest succession patterns to design software system evolution.",
        expect_abstract: &["succession", "phases", "resilience"],
        expect_min_matches: 3,
        expect_keywords: &["pioneer", "mature", "stable"],
    },
    BenchmarkCase {
        id: "transfer-chess-to-negotiation",
        input: "Apply chess strategic principles to business negotiations.",
        expect_abstract: &["strategy", "position", "exchange"],
        expect_min_matches: 3,
        expect_keywords: &["tempo", "sacrifice", "endgame"],
    },
    BenchmarkCase {
        id: "transfer-garden-to-community",
        input: "Transfer permaculture principles to online community building.",
        expect_abstract: &["ecosystem", "diversity", "sustainability"],
        expect_min_matches: 3,
        expect_keywords: &["guilds", "succession", "observe"],
    },
    BenchmarkCase {
        id: "transfer-athlete-to-startup",
        input: "Use athletic training periodization for startup growth phases.",
        expect_abstract: &["periodization", "adaptation", "recovery"],
        expect_min_matches: 3,
        expect_keywords: &["base", "intensity", "taper"],
    },
    BenchmarkCase {
        id: "transfer-ant-to-distributed",
        input: "Apply ant colony optimization to distributed system load balancing.",
        expect_abstract: &["emergence", "stigmergy", "optimization"],
        expect_min_matches: 3,
        expect_keywords: &["pheromone", "trail", "converge"],
    },
    BenchmarkCase {
        id: "transfer-theater-to-ui",
        input: "Transfer theatrical staging principles to user interface design.",
        expect_abstract: &["staging", "focus", "narrative"],
        expect_min_matches: 3,
        expect_keywords: &["attention", "flow", "scene"],
    },
    BenchmarkCase {
        id: "transfer-ecology-to-microservices",
        input: "Use ecological niche theory to optimize microservice boundaries.",
        expect_abstract: &["specialization", "niche", "coevolution"],
        expect_min_matches: 3,
        expect_keywords: &["resource", "boundary", "interface"],
    },
    BenchmarkCase {
        id: "transfer-memory-to-caching",
        input: "Apply human memory consolidation to database caching strategies.",
        expect_abstract: &["consolidation", "hierarchy", "decay"],
        expect_min_matches: 3,
        expect_keywords: &["rehearsal", "priority", "eviction"],
    },
];

/// 10 debugging-focused cases for wedge v1 evaluation.
pub const DEBUGGING_V1_CASES: &[BenchmarkCase] = &[
    BenchmarkCase {
        id: "debug-null-pointer",
        input: "Service crashes with a null pointer only under high load.",
        expect_abstract: &["race", "state", "timing"],
        expect_min_matches: 3,
        expect_keywords: &["reproduce", "instrument", "verify"],
    },
    BenchmarkCase {
        id: "debug-memory-leak",
        input: "Memory usage climbs steadily after deploying worker autoscaling.",
        expect_abstract: &["resource", "accumulation", "lifecycle"],
        expect_min_matches: 3,
        expect_keywords: &["heap", "profile", "rollback"],
    },
    BenchmarkCase {
        id: "debug-flaky-test",
        input: "CI test fails randomly but passes locally.",
        expect_abstract: &["nondetermin", "environment", "timing"],
        expect_min_matches: 3,
        expect_keywords: &["isolate", "seed", "assert"],
    },
    BenchmarkCase {
        id: "debug-cache-stale",
        input: "Users see stale data despite successful writes.",
        expect_abstract: &["consistency", "cache", "invalidation"],
        expect_min_matches: 3,
        expect_keywords: &["ttl", "invalidate", "verify"],
    },
    BenchmarkCase {
        id: "debug-auth-regression",
        input: "Auth tokens started failing after rotating signing keys.",
        expect_abstract: &["compatibility", "trust", "version"],
        expect_min_matches: 3,
        expect_keywords: &["key", "clock", "fallback"],
    },
    BenchmarkCase {
        id: "debug-db-deadlock",
        input: "Two services intermittently deadlock on shared records.",
        expect_abstract: &["contention", "ordering", "lock"],
        expect_min_matches: 3,
        expect_keywords: &["transaction", "order", "retry"],
    },
    BenchmarkCase {
        id: "debug-queue-backlog",
        input: "Background job queue keeps growing even after scaling workers.",
        expect_abstract: &["throughput", "bottleneck", "backpressure"],
        expect_min_matches: 3,
        expect_keywords: &["rate", "batch", "monitor"],
    },
    BenchmarkCase {
        id: "debug-schema-drift",
        input: "Downstream consumers break after a minor event schema change.",
        expect_abstract: &["contract", "evolution", "compatibility"],
        expect_min_matches: 3,
        expect_keywords: &["version", "transform", "validate"],
    },
    BenchmarkCase {
        id: "debug-feature-flag",
        input: "Feature flag rollout causes inconsistent behavior by region.",
        expect_abstract: &["segmentation", "state", "control"],
        expect_min_matches: 3,
        expect_keywords: &["cohort", "toggle", "observability"],
    },
    BenchmarkCase {
        id: "debug-latency-spike",
        input: "API latency spikes every hour with no obvious traffic surge.",
        expect_abstract: &["periodic", "resource", "saturation"],
        expect_min_matches: 3,
        expect_keywords: &["cron", "profile", "mitigate"],
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_case_shape(cases: &[BenchmarkCase]) {
        for case in cases {
            assert!(
                !case.expect_abstract.is_empty(),
                "{}: empty abstract",
                case.id
            );
            assert!(
                case.expect_min_matches >= 1,
                "{}: expect_min_matches < 1",
                case.id
            );
            assert!(
                !case.expect_keywords.is_empty(),
                "{}: empty keywords",
                case.id
            );
        }
    }

    #[test]
    fn seed_cases_has_20_entries() {
        assert_eq!(SEED_CASES.len(), 20);
    }

    #[test]
    fn debugging_cases_has_10_entries() {
        assert_eq!(DEBUGGING_V1_CASES.len(), 10);
    }

    #[test]
    fn isomorphic_transfer_cases_has_12_entries() {
        assert_eq!(ISOMORPHIC_TRANSFER_CASES.len(), 12);
    }

    #[test]
    fn all_ids_unique() {
        let mut ids: Vec<&str> = SEED_CASES
            .iter()
            .chain(DEBUGGING_V1_CASES.iter())
            .chain(ISOMORPHIC_TRANSFER_CASES.iter())
            .map(|c| c.id)
            .collect();
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(
            ids.len(),
            SEED_CASES.len() + DEBUGGING_V1_CASES.len() + ISOMORPHIC_TRANSFER_CASES.len()
        );
    }

    #[test]
    fn all_cases_have_expectations() {
        assert_case_shape(SEED_CASES);
        assert_case_shape(DEBUGGING_V1_CASES);
        assert_case_shape(ISOMORPHIC_TRANSFER_CASES);
    }
}
