use anyhow::Result;
use async_trait::async_trait;
use mycelium_core::{ReasoningProvider, StagedProvider};
use mycelium_types::{
    AbstractOutput, CrossDomainMatch, EntityMapping, MapOutput, ProblemResponse, SearchOutput,
    SynthesizeOutput,
};

pub struct StubProvider;

#[async_trait]
impl ReasoningProvider for StubProvider {
    async fn solve(&self, input: &str) -> Result<ProblemResponse> {
        Ok(ProblemResponse {
            abstract_shape: format!("Abstract shape for: {input}"),
            cross_domain_matches: vec![
                "Music practice chunking".into(),
                "Compiler optimization passes".into(),
                "Athletic interval training".into(),
            ],
            mapping: "Map recurring local failure modes to reusable correction loops.".into(),
            synthesis: "Use micro-isolation, progressive integration, spaced review, and randomized transfer reps.".into(),
        })
    }
}

#[async_trait]
impl StagedProvider for StubProvider {
    async fn abstract_problem(&self, input: &str) -> Result<AbstractOutput> {
        Ok(AbstractOutput {
            domain: "general".into(),
            abstract_shape: format!("Iterative refinement with feedback loops applied to: {input}"),
        })
    }

    async fn search(&self, _abstraction: &AbstractOutput) -> Result<SearchOutput> {
        Ok(SearchOutput {
            matches: vec![
                CrossDomainMatch { domain: "music".into(), description: "Practice chunking".into() },
                CrossDomainMatch { domain: "compilers".into(), description: "Optimization passes".into() },
                CrossDomainMatch { domain: "athletics".into(), description: "Interval training".into() },
            ],
        })
    }

    async fn map(&self, _abstraction: &AbstractOutput, _search: &SearchOutput) -> Result<MapOutput> {
        Ok(MapOutput {
            mappings: vec![
                EntityMapping { source: "failure mode".into(), target: "correction loop".into(), relation: "triggers".into() },
                EntityMapping { source: "practice chunk".into(), target: "micro-isolation".into(), relation: "implements".into() },
            ],
        })
    }

    async fn synthesize(&self, _input: &str, _abstraction: &AbstractOutput, _search: &SearchOutput, _map: &MapOutput) -> Result<SynthesizeOutput> {
        Ok(SynthesizeOutput {
            synthesis: "Use micro-isolation, progressive integration, spaced review, and randomized transfer reps.".into(),
        })
    }
}
