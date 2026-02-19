use anyhow::Result;
use mycelium_core::{ReasoningProvider, StagedProvider};
use mycelium_types::{ProblemResponse, StageTrace};
use std::sync::Arc;

enum ProviderKind {
    SinglePass(Arc<dyn ReasoningProvider>),
    Staged(Arc<dyn StagedProvider>),
}

pub struct Engine {
    kind: ProviderKind,
}

impl Engine {
    pub fn new(provider: Arc<dyn ReasoningProvider>) -> Self {
        Self {
            kind: ProviderKind::SinglePass(provider),
        }
    }

    pub fn staged(provider: Arc<dyn StagedProvider>) -> Self {
        Self {
            kind: ProviderKind::Staged(provider),
        }
    }

    pub async fn run(&self, input: &str) -> Result<ProblemResponse> {
        match &self.kind {
            ProviderKind::SinglePass(p) => p.solve(input).await,
            ProviderKind::Staged(p) => {
                let trace = self.run_staged(p, input).await?;
                Ok(trace.into_response())
            }
        }
    }

    pub async fn run_traced(&self, input: &str) -> Result<StageTrace> {
        match &self.kind {
            ProviderKind::SinglePass(_) => {
                anyhow::bail!("run_traced requires a staged provider")
            }
            ProviderKind::Staged(p) => self.run_staged(p, input).await,
        }
    }

    async fn run_staged(
        &self,
        provider: &Arc<dyn StagedProvider>,
        input: &str,
    ) -> Result<StageTrace> {
        let abstract_out = provider.abstract_problem(input).await?;
        let search_out = provider.search(&abstract_out).await?;
        let map_out = provider.map(&abstract_out, &search_out).await?;
        let synthesize_out = provider
            .synthesize(input, &abstract_out, &search_out, &map_out)
            .await?;

        Ok(StageTrace {
            input: input.to_string(),
            abstract_out,
            search_out,
            map_out,
            synthesize_out,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use mycelium_types::*;
    use std::sync::atomic::{AtomicU8, Ordering};

    struct MockStagedProvider {
        call_order: AtomicU8,
    }

    impl MockStagedProvider {
        fn new() -> Self {
            Self {
                call_order: AtomicU8::new(0),
            }
        }
    }

    #[async_trait]
    impl StagedProvider for MockStagedProvider {
        async fn abstract_problem(&self, input: &str) -> Result<AbstractOutput> {
            assert_eq!(self.call_order.fetch_add(1, Ordering::SeqCst), 0);
            Ok(AbstractOutput {
                domain: "test".into(),
                abstract_shape: format!("abstracted: {input}"),
            })
        }

        async fn search(&self, abs: &AbstractOutput) -> Result<SearchOutput> {
            assert_eq!(self.call_order.fetch_add(1, Ordering::SeqCst), 1);
            assert!(abs.abstract_shape.starts_with("abstracted:"));
            Ok(SearchOutput {
                matches: vec![
                    CrossDomainMatch {
                        domain: "d1".into(),
                        description: "match1".into(),
                    },
                    CrossDomainMatch {
                        domain: "d2".into(),
                        description: "match2".into(),
                    },
                    CrossDomainMatch {
                        domain: "d3".into(),
                        description: "match3".into(),
                    },
                ],
            })
        }

        async fn map(&self, abs: &AbstractOutput, search: &SearchOutput) -> Result<MapOutput> {
            assert_eq!(self.call_order.fetch_add(1, Ordering::SeqCst), 2);
            assert_eq!(abs.domain, "test");
            assert_eq!(search.matches.len(), 3);
            Ok(MapOutput {
                mappings: vec![EntityMapping {
                    source: "src".into(),
                    target: "tgt".into(),
                    relation: "maps_to".into(),
                }],
            })
        }

        async fn synthesize(
            &self,
            input: &str,
            _abs: &AbstractOutput,
            _search: &SearchOutput,
            map: &MapOutput,
        ) -> Result<SynthesizeOutput> {
            assert_eq!(self.call_order.fetch_add(1, Ordering::SeqCst), 3);
            assert!(!input.is_empty());
            assert_eq!(map.mappings.len(), 1);
            Ok(SynthesizeOutput {
                synthesis: "final synthesis".into(),
            })
        }
    }

    #[tokio::test]
    async fn staged_pipeline_runs_all_stages_in_order() {
        let provider = Arc::new(MockStagedProvider::new());
        let engine = Engine::staged(provider);
        let trace = engine.run_traced("test input").await.unwrap();

        assert_eq!(trace.input, "test input");
        assert_eq!(trace.abstract_out.domain, "test");
        assert_eq!(trace.abstract_out.abstract_shape, "abstracted: test input");
        assert_eq!(trace.search_out.matches.len(), 3);
        assert_eq!(trace.map_out.mappings.len(), 1);
        assert_eq!(trace.synthesize_out.synthesis, "final synthesis");
    }

    #[tokio::test]
    async fn staged_pipeline_produces_valid_response() {
        let provider = Arc::new(MockStagedProvider::new());
        let engine = Engine::staged(provider);
        let resp = engine.run("test input").await.unwrap();

        assert_eq!(resp.abstract_shape, "abstracted: test input");
        assert_eq!(resp.cross_domain_matches.len(), 3);
        assert_eq!(resp.cross_domain_matches[0], "d1: match1");
        assert!(resp.mapping.contains("src -> tgt"));
        assert_eq!(resp.synthesis, "final synthesis");
    }

    #[tokio::test]
    async fn run_traced_rejects_single_pass() {
        let provider: Arc<dyn ReasoningProvider> = Arc::new(MockSinglePassProvider);
        let engine = Engine::new(provider);
        let err = engine.run_traced("test").await.unwrap_err();
        assert!(err.to_string().contains("staged provider"));
    }

    #[tokio::test]
    async fn single_pass_still_works() {
        let provider: Arc<dyn ReasoningProvider> = Arc::new(MockSinglePassProvider);
        let engine = Engine::new(provider);
        let resp = engine.run("hello").await.unwrap();
        assert_eq!(resp.synthesis, "single-pass");
    }

    struct MockSinglePassProvider;

    #[async_trait]
    impl ReasoningProvider for MockSinglePassProvider {
        async fn solve(&self, _input: &str) -> Result<ProblemResponse> {
            Ok(ProblemResponse {
                abstract_shape: "shape".into(),
                cross_domain_matches: vec!["m1".into()],
                mapping: "map".into(),
                synthesis: "single-pass".into(),
            })
        }
    }

    struct FailingAbstractProvider;

    #[async_trait]
    impl StagedProvider for FailingAbstractProvider {
        async fn abstract_problem(&self, _input: &str) -> Result<AbstractOutput> {
            anyhow::bail!("abstract failed")
        }
        async fn search(&self, _: &AbstractOutput) -> Result<SearchOutput> {
            unreachable!()
        }
        async fn map(&self, _: &AbstractOutput, _: &SearchOutput) -> Result<MapOutput> {
            unreachable!()
        }
        async fn synthesize(
            &self,
            _: &str,
            _: &AbstractOutput,
            _: &SearchOutput,
            _: &MapOutput,
        ) -> Result<SynthesizeOutput> {
            unreachable!()
        }
    }

    #[tokio::test]
    async fn stage_failure_propagates() {
        let engine = Engine::staged(Arc::new(FailingAbstractProvider));
        let err = engine.run("test").await.unwrap_err();
        assert!(err.to_string().contains("abstract failed"));
    }

    #[tokio::test]
    async fn stage_trace_into_response_formatting() {
        let trace = StageTrace {
            input: "test".into(),
            abstract_out: AbstractOutput {
                domain: "d".into(),
                abstract_shape: "shape".into(),
            },
            search_out: SearchOutput {
                matches: vec![
                    CrossDomainMatch {
                        domain: "A".into(),
                        description: "desc_a".into(),
                    },
                    CrossDomainMatch {
                        domain: "B".into(),
                        description: "desc_b".into(),
                    },
                ],
            },
            map_out: MapOutput {
                mappings: vec![
                    EntityMapping {
                        source: "s1".into(),
                        target: "t1".into(),
                        relation: "r1".into(),
                    },
                    EntityMapping {
                        source: "s2".into(),
                        target: "t2".into(),
                        relation: "r2".into(),
                    },
                ],
            },
            synthesize_out: SynthesizeOutput {
                synthesis: "syn".into(),
            },
        };
        let resp = trace.into_response();
        assert_eq!(resp.abstract_shape, "shape");
        assert_eq!(resp.cross_domain_matches, vec!["A: desc_a", "B: desc_b"]);
        assert_eq!(resp.mapping, "s1 -> t1 (r1); s2 -> t2 (r2)");
        assert_eq!(resp.synthesis, "syn");
    }
}
