use anyhow::Result;
use async_trait::async_trait;
use mycelium_core::StagedProvider;
use mycelium_types::{
    mapping_precision::MappingPrecisionAnalyzer, AbstractOutput, CrossDomainMatch, EntityMapping,
    MapOutput, SearchOutput, SynthesizeOutput,
};

/// Enhanced provider with precision-focused map stage
pub struct PrecisionEnhancedProvider;

#[async_trait]
impl StagedProvider for PrecisionEnhancedProvider {
    async fn abstract_problem(&self, input: &str) -> Result<AbstractOutput> {
        // Enhanced abstraction with pattern recognition
        let domain = Self::identify_domain(input);
        let abstract_shape = Self::extract_abstract_pattern(input);

        Ok(AbstractOutput {
            domain,
            abstract_shape,
        })
    }

    async fn search(&self, abstraction: &AbstractOutput) -> Result<SearchOutput> {
        // Enhanced search with domain-specific matching
        let matches =
            Self::find_cross_domain_matches(&abstraction.abstract_shape, &abstraction.domain);

        Ok(SearchOutput { matches })
    }

    async fn map(&self, abstraction: &AbstractOutput, search: &SearchOutput) -> Result<MapOutput> {
        // Generate base mappings using improved heuristics
        let base_mappings = Self::generate_enhanced_mappings(abstraction, search);

        // Use precision analyzer to enhance and validate mappings
        let precision_output = MappingPrecisionAnalyzer::enhance_mappings(
            abstraction,
            search,
            &MapOutput {
                mappings: base_mappings,
            },
        );

        // Convert back to basic MapOutput but with improved confidence scores
        let enhanced_mappings = precision_output
            .mappings
            .into_iter()
            .map(|pm| EntityMapping {
                source: pm.source,
                target: pm.target,
                relation: pm.relation,
                confidence: Some(pm.confidence),
            })
            .collect();

        Ok(MapOutput {
            mappings: enhanced_mappings,
        })
    }

    async fn synthesize(
        &self,
        input: &str,
        abstraction: &AbstractOutput,
        search: &SearchOutput,
        map: &MapOutput,
    ) -> Result<SynthesizeOutput> {
        let synthesis = Self::generate_precision_aware_synthesis(input, abstraction, search, map);
        Ok(SynthesizeOutput { synthesis })
    }
}

impl PrecisionEnhancedProvider {
    fn identify_domain(input: &str) -> String {
        let input_lower = input.to_lowercase();

        // Enhanced domain detection
        if input_lower.contains("music")
            || input_lower.contains("practice")
            || input_lower.contains("instrument")
        {
            "music".into()
        } else if input_lower.contains("code")
            || input_lower.contains("programming")
            || input_lower.contains("software")
        {
            "programming".into()
        } else if input_lower.contains("team")
            || input_lower.contains("management")
            || input_lower.contains("organization")
        {
            "management".into()
        } else if input_lower.contains("system") || input_lower.contains("architecture") {
            "systems".into()
        } else if input_lower.contains("learn")
            || input_lower.contains("skill")
            || input_lower.contains("train")
        {
            "learning".into()
        } else {
            "general".into()
        }
    }

    fn extract_abstract_pattern(input: &str) -> String {
        let input_lower = input.to_lowercase();

        // Pattern recognition for common abstract structures
        if input_lower.contains("improve")
            || input_lower.contains("better")
            || input_lower.contains("enhance")
        {
            if input_lower.contains("practice") || input_lower.contains("skill") {
                "Iterative skill development with optimization loops".into()
            } else if input_lower.contains("system") || input_lower.contains("process") {
                "System optimization through iterative refinement".into()
            } else {
                "Improvement process through structured iteration".into()
            }
        } else if input_lower.contains("reduce")
            || input_lower.contains("minimize")
            || input_lower.contains("decrease")
        {
            "Constraint satisfaction through systematic reduction".into()
        } else if input_lower.contains("balance") || input_lower.contains("equilibrium") {
            "Multi-objective optimization with dynamic balance".into()
        } else if input_lower.contains("coordinate")
            || input_lower.contains("sync")
            || input_lower.contains("align")
        {
            "Coordination mechanism with feedback alignment".into()
        } else if input_lower.contains("adapt")
            || input_lower.contains("respond")
            || input_lower.contains("adjust")
        {
            "Adaptive system with environmental response loops".into()
        } else {
            format!("Structured problem-solving pattern applied to: {}", input)
        }
    }

    fn find_cross_domain_matches(abstract_shape: &str, domain: &str) -> Vec<CrossDomainMatch> {
        let shape_lower = abstract_shape.to_lowercase();
        let mut matches = Vec::new();

        // Enhanced cross-domain matching based on abstract patterns
        if shape_lower.contains("iterative") || shape_lower.contains("loop") {
            matches.extend(vec![
                CrossDomainMatch {
                    domain: "athletics".into(),
                    description: "Progressive training with recovery cycles".into(),
                },
                CrossDomainMatch {
                    domain: "manufacturing".into(),
                    description: "Continuous improvement (kaizen) cycles".into(),
                },
                CrossDomainMatch {
                    domain: "biology".into(),
                    description: "Homeostatic feedback regulation".into(),
                },
                CrossDomainMatch {
                    domain: "economics".into(),
                    description: "Market correction mechanisms".into(),
                },
            ]);
        }

        if shape_lower.contains("optimization") || shape_lower.contains("improvement") {
            matches.extend(vec![
                CrossDomainMatch {
                    domain: "algorithms".into(),
                    description: "Gradient descent optimization".into(),
                },
                CrossDomainMatch {
                    domain: "evolution".into(),
                    description: "Natural selection pressure mechanisms".into(),
                },
                CrossDomainMatch {
                    domain: "engineering".into(),
                    description: "Design iteration and prototyping".into(),
                },
            ]);
        }

        if shape_lower.contains("balance") || shape_lower.contains("equilibrium") {
            matches.extend(vec![
                CrossDomainMatch {
                    domain: "physics".into(),
                    description: "Dynamic equilibrium systems".into(),
                },
                CrossDomainMatch {
                    domain: "ecology".into(),
                    description: "Predator-prey population balance".into(),
                },
                CrossDomainMatch {
                    domain: "chemistry".into(),
                    description: "Chemical equilibrium reactions".into(),
                },
            ]);
        }

        if shape_lower.contains("coordination") || shape_lower.contains("sync") {
            matches.extend(vec![
                CrossDomainMatch {
                    domain: "orchestration".into(),
                    description: "Musical ensemble synchronization".into(),
                },
                CrossDomainMatch {
                    domain: "aviation".into(),
                    description: "Flight formation coordination".into(),
                },
                CrossDomainMatch {
                    domain: "distributed_systems".into(),
                    description: "Consensus protocol mechanisms".into(),
                },
            ]);
        }

        if shape_lower.contains("adaptive") || shape_lower.contains("response") {
            matches.extend(vec![
                CrossDomainMatch {
                    domain: "immunology".into(),
                    description: "Immune system adaptive response".into(),
                },
                CrossDomainMatch {
                    domain: "psychology".into(),
                    description: "Behavioral adaptation patterns".into(),
                },
                CrossDomainMatch {
                    domain: "robotics".into(),
                    description: "Sensor-based adaptive control".into(),
                },
            ]);
        }

        // Filter out matches from the same domain to ensure cross-domain diversity
        let filtered_matches: Vec<CrossDomainMatch> =
            matches.into_iter().filter(|m| m.domain != domain).collect();

        // Ensure minimum of 3 matches, add defaults if needed
        if filtered_matches.len() < 3 {
            let mut result = filtered_matches;
            let defaults = vec![
                CrossDomainMatch {
                    domain: "systems_theory".into(),
                    description: "General systems feedback loops".into(),
                },
                CrossDomainMatch {
                    domain: "cybernetics".into(),
                    description: "Control and communication patterns".into(),
                },
                CrossDomainMatch {
                    domain: "complexity_science".into(),
                    description: "Emergence and self-organization".into(),
                },
            ];

            for default in defaults {
                if result.len() >= 3 {
                    break;
                }
                if !result.iter().any(|m| m.domain == default.domain) {
                    result.push(default);
                }
            }
            result
        } else {
            // Take top matches but ensure diversity
            Self::select_diverse_matches(filtered_matches, 5)
        }
    }

    fn select_diverse_matches(
        matches: Vec<CrossDomainMatch>,
        max_count: usize,
    ) -> Vec<CrossDomainMatch> {
        // Select diverse matches preferring different domains
        let mut selected = Vec::new();
        let mut used_domains = std::collections::HashSet::new();

        // First pass: select one from each unique domain
        for m in &matches {
            if !used_domains.contains(&m.domain) && selected.len() < max_count {
                selected.push(m.clone());
                used_domains.insert(&m.domain);
            }
        }

        // Second pass: fill remaining slots with best remaining matches
        for m in &matches {
            if selected.len() >= max_count {
                break;
            }
            if !selected
                .iter()
                .any(|s| s.domain == m.domain && s.description == m.description)
            {
                selected.push(m.clone());
            }
        }

        selected
    }

    fn generate_enhanced_mappings(
        abstraction: &AbstractOutput,
        search: &SearchOutput,
    ) -> Vec<EntityMapping> {
        let mut mappings = Vec::new();
        let abstract_lower = abstraction.abstract_shape.to_lowercase();

        // Extract key concepts from abstraction
        let key_concepts = Self::extract_key_concepts(&abstract_lower);

        // Create precise mappings based on pattern analysis
        for concept in &key_concepts {
            for search_match in &search.matches {
                let match_lower = search_match.description.to_lowercase();

                // Enhanced mapping logic with better precision
                if Self::concepts_align(concept, &match_lower) {
                    let target_concept = Self::extract_target_concept(&match_lower, concept);
                    let relation = Self::determine_relation_type(concept, &target_concept);

                    mappings.push(EntityMapping {
                        source: concept.clone(),
                        target: target_concept,
                        relation,
                        confidence: None, // Will be calculated by precision analyzer
                    });
                }
            }
        }

        // Ensure minimum mappings with fallback generation
        if mappings.len() < 2 {
            mappings.extend(Self::generate_fallback_mappings(&abstract_lower, search));
        }

        mappings
    }

    fn extract_key_concepts(abstract_text: &str) -> Vec<String> {
        let mut concepts = Vec::new();

        // Enhanced concept extraction
        if abstract_text.contains("iterative") || abstract_text.contains("loop") {
            concepts.push("iterative process".into());
        }
        if abstract_text.contains("optimization") || abstract_text.contains("improvement") {
            concepts.push("optimization mechanism".into());
        }
        if abstract_text.contains("feedback") {
            concepts.push("feedback loop".into());
        }
        if abstract_text.contains("balance") || abstract_text.contains("equilibrium") {
            concepts.push("balance mechanism".into());
        }
        if abstract_text.contains("coordination") || abstract_text.contains("sync") {
            concepts.push("coordination pattern".into());
        }
        if abstract_text.contains("adaptive") || abstract_text.contains("response") {
            concepts.push("adaptive response".into());
        }
        if abstract_text.contains("skill") || abstract_text.contains("learning") {
            concepts.push("skill development".into());
        }
        if abstract_text.contains("system") {
            concepts.push("system component".into());
        }

        // Fallback concept extraction
        if concepts.is_empty() {
            concepts.push("core process".into());
            concepts.push("structural pattern".into());
        }

        concepts
    }

    fn concepts_align(source_concept: &str, target_description: &str) -> bool {
        let source_lower = source_concept.to_lowercase();
        let target_lower = target_description.to_lowercase();

        // Enhanced concept alignment detection
        if source_lower.contains("iterative") || source_lower.contains("loop") {
            return target_lower.contains("cycle")
                || target_lower.contains("repeat")
                || target_lower.contains("iteration")
                || target_lower.contains("loop");
        }

        if source_lower.contains("optimization") || source_lower.contains("improvement") {
            return target_lower.contains("improve")
                || target_lower.contains("optimize")
                || target_lower.contains("enhance")
                || target_lower.contains("refine");
        }

        if source_lower.contains("feedback") {
            return target_lower.contains("feedback")
                || target_lower.contains("response")
                || target_lower.contains("adjust")
                || target_lower.contains("correct");
        }

        if source_lower.contains("balance") {
            return target_lower.contains("balance")
                || target_lower.contains("equilibrium")
                || target_lower.contains("stability")
                || target_lower.contains("steady");
        }

        if source_lower.contains("coordination") {
            return target_lower.contains("coordinate")
                || target_lower.contains("sync")
                || target_lower.contains("align")
                || target_lower.contains("together");
        }

        if source_lower.contains("adaptive") {
            return target_lower.contains("adapt")
                || target_lower.contains("respond")
                || target_lower.contains("adjust")
                || target_lower.contains("flexible");
        }

        // Generic alignment check
        let source_words: std::collections::HashSet<&str> =
            source_lower.split_whitespace().collect();
        let target_words: std::collections::HashSet<&str> =
            target_lower.split_whitespace().collect();

        let overlap = source_words.intersection(&target_words).count();
        overlap > 0
    }

    fn extract_target_concept(target_description: &str, source_concept: &str) -> String {
        let desc_lower = target_description.to_lowercase();

        // Enhanced target concept extraction
        if desc_lower.contains("training") {
            if source_concept.contains("iterative") {
                "training iteration".into()
            } else if source_concept.contains("feedback") {
                "training feedback".into()
            } else {
                "training process".into()
            }
        } else if desc_lower.contains("optimization") || desc_lower.contains("improvement") {
            "optimization step".into()
        } else if desc_lower.contains("feedback") || desc_lower.contains("regulation") {
            "regulatory mechanism".into()
        } else if desc_lower.contains("balance") || desc_lower.contains("equilibrium") {
            "balance point".into()
        } else if desc_lower.contains("synchron") || desc_lower.contains("coordinat") {
            "synchronization method".into()
        } else if desc_lower.contains("adaptive") || desc_lower.contains("response") {
            "adaptive strategy".into()
        } else {
            // Extract the key noun phrase from description
            let words: Vec<&str> = target_description.split_whitespace().collect();
            if words.len() >= 2 {
                format!("{} {}", words[words.len() - 2], words[words.len() - 1])
            } else {
                target_description.to_string()
            }
        }
    }

    fn determine_relation_type(source: &str, target: &str) -> String {
        let source_lower = source.to_lowercase();
        let target_lower = target.to_lowercase();

        // Enhanced relation determination
        if source_lower.contains("process") && target_lower.contains("method") {
            "implements".into()
        } else if source_lower.contains("mechanism") && target_lower.contains("mechanism") {
            "analogous_to".into()
        } else if source_lower.contains("loop") && target_lower.contains("cycle") {
            "corresponds_to".into()
        } else if source_lower.contains("pattern") || target_lower.contains("pattern") {
            "maps_to".into()
        } else if source_lower.contains("adaptive") || target_lower.contains("adaptive") {
            "enables".into()
        } else {
            "relates_to".into()
        }
    }

    fn generate_fallback_mappings(
        _abstract_text: &str,
        search: &SearchOutput,
    ) -> Vec<EntityMapping> {
        // Generate minimum viable mappings when concept extraction yields few results
        let mut fallbacks = Vec::new();

        if let Some(first_match) = search.matches.first() {
            fallbacks.push(EntityMapping {
                source: "core pattern".into(),
                target: first_match
                    .description
                    .split_whitespace()
                    .last()
                    .unwrap_or("element")
                    .into(),
                relation: "maps_to".into(),
                confidence: None,
            });
        }

        if search.matches.len() > 1 {
            fallbacks.push(EntityMapping {
                source: "structural element".into(),
                target: search.matches[1]
                    .description
                    .split_whitespace()
                    .last()
                    .unwrap_or("component")
                    .into(),
                relation: "analogous_to".into(),
                confidence: None,
            });
        }

        fallbacks
    }

    fn generate_precision_aware_synthesis(
        _input: &str,
        abstraction: &AbstractOutput,
        search: &SearchOutput,
        map: &MapOutput,
    ) -> String {
        let domain_context = format!("In the {} domain", abstraction.domain);

        let pattern_description = if abstraction.abstract_shape.len() > 50 {
            format!("Applying the pattern: {}.", abstraction.abstract_shape)
        } else {
            format!(
                "Using {}: {}.",
                abstraction.domain, abstraction.abstract_shape
            )
        };

        let cross_domain_insights = if search.matches.len() >= 3 {
            let domains: Vec<String> = search.matches.iter().map(|m| m.domain.clone()).collect();
            format!(
                "Drawing insights from {} domains ({})",
                domains.len(),
                domains.join(", ")
            )
        } else {
            "Using available cross-domain analogies".into()
        };

        let mapping_strength = if !map.mappings.is_empty() {
            let avg_confidence: f32 = map
                .mappings
                .iter()
                .filter_map(|m| m.confidence)
                .sum::<f32>()
                / map.mappings.len() as f32;

            if avg_confidence > 0.8 {
                "with high-precision mappings"
            } else if avg_confidence > 0.6 {
                "with good mapping precision"
            } else {
                "with moderate mapping confidence"
            }
        } else {
            "with basic structural alignment"
        };

        let actionable_steps = Self::generate_actionable_steps(map);

        format!(
            "{}. {} {} {}.\\n\\nImplementation approach:\\n{}",
            domain_context,
            pattern_description,
            cross_domain_insights,
            mapping_strength,
            actionable_steps
        )
    }

    fn generate_actionable_steps(map: &MapOutput) -> String {
        let mut steps = Vec::new();

        for (i, mapping) in map.mappings.iter().enumerate() {
            let confidence_desc = mapping
                .confidence
                .map(|c| {
                    if c > 0.8 {
                        "high confidence"
                    } else if c > 0.6 {
                        "medium confidence"
                    } else {
                        "lower confidence"
                    }
                })
                .unwrap_or("confidence TBD");

            steps.push(format!(
                "{}. {} → {} ({}): Apply {} relationship with {}",
                i + 1,
                mapping.source,
                mapping.target,
                mapping.relation,
                mapping.relation,
                confidence_desc
            ));
        }

        if steps.is_empty() {
            "1. Identify key structural elements\\n2. Map to analogous patterns\\n3. Adapt implementation details".into()
        } else {
            steps.join("\\n")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mycelium_core::StagedProvider;

    #[tokio::test]
    async fn precision_provider_produces_enhanced_mappings() {
        let provider = PrecisionEnhancedProvider;
        let input = "How do I improve my trumpet practice efficiency?";

        let abstract_out = provider.abstract_problem(input).await.unwrap();
        assert_eq!(abstract_out.domain, "music");
        assert!(abstract_out.abstract_shape.contains("skill development"));

        let search_out = provider.search(&abstract_out).await.unwrap();
        assert!(search_out.matches.len() >= 3);

        let map_out = provider.map(&abstract_out, &search_out).await.unwrap();
        assert!(!map_out.mappings.is_empty());

        // Check that mappings have confidence scores from precision analyzer
        for mapping in &map_out.mappings {
            assert!(mapping.confidence.is_some());
            assert!(mapping.confidence.unwrap() > 0.0);
            assert!(mapping.confidence.unwrap() <= 1.0);
        }
    }

    #[tokio::test]
    async fn precision_provider_handles_system_optimization() {
        let provider = PrecisionEnhancedProvider;
        let input = "How to improve distributed system performance?";

        let abstract_out = provider.abstract_problem(input).await.unwrap();
        assert_eq!(abstract_out.domain, "systems");

        let search_out = provider.search(&abstract_out).await.unwrap();
        let map_out = provider.map(&abstract_out, &search_out).await.unwrap();

        // Should have meaningful mappings with good precision
        assert!(!map_out.mappings.is_empty());
        let has_high_confidence = map_out
            .mappings
            .iter()
            .any(|m| m.confidence.unwrap_or(0.0) > 0.6);
        assert!(has_high_confidence);
    }

    #[tokio::test]
    async fn domain_identification_works_correctly() {
        assert_eq!(
            PrecisionEnhancedProvider::identify_domain("practice piano"),
            "music"
        );
        assert_eq!(
            PrecisionEnhancedProvider::identify_domain("debug code"),
            "programming"
        );
        assert_eq!(
            PrecisionEnhancedProvider::identify_domain("manage team"),
            "management"
        );
        assert_eq!(
            PrecisionEnhancedProvider::identify_domain("system architecture"),
            "systems"
        );
        assert_eq!(
            PrecisionEnhancedProvider::identify_domain("random topic"),
            "general"
        );
    }

    #[tokio::test]
    async fn cross_domain_matches_are_diverse() {
        let abstract_shape = "Iterative skill development with optimization loops";
        let domain = "music";

        let matches = PrecisionEnhancedProvider::find_cross_domain_matches(abstract_shape, domain);

        assert!(matches.len() >= 3);

        // Check domain diversity - no matches should be from the source domain
        let domains: std::collections::HashSet<&str> =
            matches.iter().map(|m| m.domain.as_str()).collect();
        assert!(!domains.contains("music"));
        assert!(domains.len() >= 2); // At least 2 different target domains
    }
}
