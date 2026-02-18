use async_trait::async_trait;
use isomorph_types::ProblemResponse;

#[async_trait]
pub trait ReasoningProvider: Send + Sync {
    async fn solve_with_isomorphism(&self, input: &str) -> anyhow::Result<ProblemResponse>;
}
