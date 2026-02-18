use anyhow::Result;
use async_trait::async_trait;
use isomorph_core::ReasoningProvider;
use isomorph_types::ProblemResponse;

pub struct StubProvider;

#[async_trait]
impl ReasoningProvider for StubProvider {
    async fn solve_with_isomorphism(&self, input: &str) -> Result<ProblemResponse> {
        Ok(ProblemResponse {
            abstract_shape: format!("Abstract shape for: {input}"),
            cross_domain_matches: vec![
                "Music practice chunking".to_string(),
                "Compiler optimization passes".to_string(),
                "Athletic interval training".to_string(),
            ],
            mapping: "Map recurring local failure modes to reusable correction loops.".to_string(),
            synthesis: "Use micro-isolation, progressive integration, spaced review, and randomized transfer reps.".to_string(),
        })
    }
}
