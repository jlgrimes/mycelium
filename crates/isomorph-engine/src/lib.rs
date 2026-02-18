use anyhow::Result;
use isomorph_core::ReasoningProvider;
use isomorph_types::ProblemResponse;

pub struct Engine<P: ReasoningProvider> {
    provider: P,
}

impl<P: ReasoningProvider> Engine<P> {
    pub fn new(provider: P) -> Self {
        Self { provider }
    }

    pub async fn run(&self, input: &str) -> Result<ProblemResponse> {
        self.provider.solve_with_isomorphism(input).await
    }
}
