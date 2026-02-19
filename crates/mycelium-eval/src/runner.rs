use crate::benchmark::{BenchmarkCase, SEED_CASES};
use crate::scoring::{EvalResult, ScoreReport, Scorer};
use mycelium_core::ReasoningProvider;
use mycelium_engine::Engine;
use std::sync::Arc;

/// Which pipeline mode to evaluate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunMode {
    /// Single-pass: the current Engine.run() behavior.
    Baseline,
    /// Staged pipeline (abstract -> search -> map -> synthesize).
    Staged,
}

impl std::fmt::Display for RunMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RunMode::Baseline => write!(f, "baseline"),
            RunMode::Staged => write!(f, "staged"),
        }
    }
}

/// Configuration for an eval run.
pub struct EvalConfig {
    /// Which cases to run (None = all seed cases).
    pub filter: Option<Vec<String>>,
}

impl Default for EvalConfig {
    fn default() -> Self {
        Self { filter: None }
    }
}

/// Runs benchmark cases against a provider and collects scored results.
pub struct EvalRunner {
    engine: Engine,
    mode: RunMode,
}

impl EvalRunner {
    pub fn new(provider: Arc<dyn ReasoningProvider>, mode: RunMode) -> Self {
        Self {
            engine: Engine::new(provider),
            mode,
        }
    }

    /// Run all selected cases and return a score report.
    pub async fn run(&self, config: &EvalConfig) -> ScoreReport {
        let cases = self.selected_cases(config);
        let mut results = Vec::with_capacity(cases.len());

        for case in &cases {
            let result = self.run_case(case).await;
            results.push(result);
        }

        Scorer::report(results)
    }

    async fn run_case(&self, case: &BenchmarkCase) -> EvalResult {
        match self.engine.run(case.input).await {
            Ok(response) => {
                let mut result = Scorer::score(case, &response);
                result.mode = self.mode.to_string();
                result
            }
            Err(e) => Scorer::error_result(case, &self.mode.to_string(), &e.to_string()),
        }
    }

    fn selected_cases(&self, config: &EvalConfig) -> Vec<&'static BenchmarkCase> {
        match &config.filter {
            None => SEED_CASES.iter().collect(),
            Some(ids) => SEED_CASES
                .iter()
                .filter(|c| ids.iter().any(|id| id == c.id))
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mycelium_providers::StubProvider;

    #[tokio::test]
    async fn runner_processes_all_seed_cases() {
        let provider = Arc::new(StubProvider);
        let runner = EvalRunner::new(provider, RunMode::Baseline);
        let report = runner.run(&EvalConfig::default()).await;
        assert_eq!(report.cases_total, 20);
    }

    #[tokio::test]
    async fn runner_respects_filter() {
        let provider = Arc::new(StubProvider);
        let runner = EvalRunner::new(provider, RunMode::Baseline);
        let config = EvalConfig {
            filter: Some(vec!["trumpet-practice".into(), "reduce-tech-debt".into()]),
        };
        let report = runner.run(&config).await;
        assert_eq!(report.cases_total, 2);
    }

    #[tokio::test]
    async fn runner_sets_mode_label() {
        let provider = Arc::new(StubProvider);
        let runner = EvalRunner::new(provider, RunMode::Staged);
        let config = EvalConfig {
            filter: Some(vec!["trumpet-practice".into()]),
        };
        let report = runner.run(&config).await;
        assert_eq!(report.results[0].mode, "staged");
    }
}
