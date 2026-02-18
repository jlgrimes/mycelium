use async_trait::async_trait;
use mycelium_types::ProblemResponse;

#[async_trait]
pub trait ReasoningProvider: Send + Sync {
    async fn solve(&self, input: &str) -> anyhow::Result<ProblemResponse>;
}
