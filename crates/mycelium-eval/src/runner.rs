use crate::benchmark::{BenchmarkCase, DEBUGGING_V1_CASES, SEED_CASES};
use crate::scoring::{EvalResult, ScoreReport, Scorer};
use mycelium_core::ReasoningProvider;
use mycelium_engine::Engine;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunMode {
    Baseline,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BenchmarkSuite {
    #[default]
    Seed,
    DebuggingV1,
}

impl std::fmt::Display for BenchmarkSuite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BenchmarkSuite::Seed => write!(f, "seed"),
            BenchmarkSuite::DebuggingV1 => write!(f, "debugging-v1"),
        }
    }
}

#[derive(Default)]
pub struct EvalConfig {
    pub filter: Option<Vec<String>>,
    pub suite: BenchmarkSuite,
}

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
        let all_cases: &[BenchmarkCase] = match config.suite {
            BenchmarkSuite::Seed => SEED_CASES,
            BenchmarkSuite::DebuggingV1 => DEBUGGING_V1_CASES,
        };

        match &config.filter {
            None => all_cases.iter().collect(),
            Some(ids) => all_cases
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
    async fn runner_processes_all_debugging_cases() {
        let provider = Arc::new(StubProvider);
        let runner = EvalRunner::new(provider, RunMode::Baseline);
        let report = runner
            .run(&EvalConfig {
                suite: BenchmarkSuite::DebuggingV1,
                filter: None,
            })
            .await;
        assert_eq!(report.cases_total, 10);
    }

    #[tokio::test]
    async fn runner_respects_filter() {
        let provider = Arc::new(StubProvider);
        let runner = EvalRunner::new(provider, RunMode::Baseline);
        let config = EvalConfig {
            filter: Some(vec!["trumpet-practice".into(), "reduce-tech-debt".into()]),
            suite: BenchmarkSuite::Seed,
        };
        let report = runner.run(&config).await;
        assert_eq!(report.cases_total, 2);
    }
}
