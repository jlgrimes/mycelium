use async_trait::async_trait;
use mycelium_types::{
    AbstractOutput, MapOutput, ProblemResponse, SearchOutput, SynthesizeOutput,
};

#[async_trait]
pub trait ReasoningProvider: Send + Sync {
    async fn solve(&self, input: &str) -> anyhow::Result<ProblemResponse>;
}

#[async_trait]
pub trait StagedProvider: Send + Sync {
    async fn abstract_problem(&self, input: &str) -> anyhow::Result<AbstractOutput>;
    async fn search(&self, abstraction: &AbstractOutput) -> anyhow::Result<SearchOutput>;
    async fn map(&self, abstraction: &AbstractOutput, search: &SearchOutput) -> anyhow::Result<MapOutput>;
    async fn synthesize(&self, input: &str, abstraction: &AbstractOutput, search: &SearchOutput, map: &MapOutput) -> anyhow::Result<SynthesizeOutput>;
}
