use anyhow::Result;
use mycelium_core::ReasoningProvider;
use mycelium_types::ProblemResponse;
use std::sync::Arc;

pub struct Engine {
    provider: Arc<dyn ReasoningProvider>,
}

impl Engine {
    pub fn new(provider: Arc<dyn ReasoningProvider>) -> Self {
        Self { provider }
    }

    pub async fn run(&self, input: &str) -> Result<ProblemResponse> {
        self.provider.solve(input).await
    }
}
