use crate::{AbstractOutput, EntityMapping, MapOutput, SearchOutput};

#[cfg(test)]
use crate::CrossDomainMatch;
use serde::{Deserialize, Serialize};

/// Advanced precision metrics for entity mappings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MappingPrecisionMetrics {
    pub semantic_similarity: f32,
    pub structural_alignment: f32,
    pub domain_compatibility: f32,
    pub conceptual_distance: f32,
    pub transfer_viability: f32,
    pub overall_precision: f32,
}

/// Enhanced entity mapping with precision analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrecisionEntityMapping {
    pub source: String,
    pub target: String,
    pub relation: String,
    pub confidence: f32,
    pub metrics: MappingPrecisionMetrics,
    pub justification: String,
}

/// Precision-enhanced map output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrecisionMapOutput {
    pub mappings: Vec<PrecisionEntityMapping>,
    pub overall_precision_score: f32,
    pub precision_grade: PrecisionGrade,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PrecisionGrade {
    Excellent, // 0.9+
    Good,      // 0.7-0.89
    Fair,      // 0.5-0.69
    Poor,      // <0.5
}

impl PrecisionGrade {
    pub fn from_score(score: f32) -> Self {
        if score >= 0.9 {
            Self::Excellent
        } else if score >= 0.7 {
            Self::Good
        } else if score >= 0.5 {
            Self::Fair
        } else {
            Self::Poor
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Excellent => "excellent",
            Self::Good => "good", 
            Self::Fair => "fair",
            Self::Poor => "poor",
        }
    }
}

pub struct MappingPrecisionAnalyzer;

impl MappingPrecisionAnalyzer {
    /// Analyze and enhance basic mappings with precision metrics
    pub fn enhance_mappings(
        abstract_out: &AbstractOutput,
        search_out: &SearchOutput,
        basic_map: &MapOutput,
    ) -> PrecisionMapOutput {
        let enhanced_mappings: Vec<PrecisionEntityMapping> = basic_map
            .mappings
            .iter()
            .map(|mapping| Self::analyze_mapping_precision(abstract_out, search_out, mapping))
            .collect();

        let overall_precision_score = if enhanced_mappings.is_empty() {
            0.0
        } else {
            enhanced_mappings.iter().map(|m| m.metrics.overall_precision).sum::<f32>() 
                / enhanced_mappings.len() as f32
        };

        PrecisionMapOutput {
            mappings: enhanced_mappings,
            overall_precision_score,
            precision_grade: PrecisionGrade::from_score(overall_precision_score),
        }
    }

    fn analyze_mapping_precision(
        abstract_out: &AbstractOutput,
        search_out: &SearchOutput,
        mapping: &EntityMapping,
    ) -> PrecisionEntityMapping {
        let semantic_similarity = Self::calculate_semantic_similarity(&mapping.source, &mapping.target);
        let structural_alignment = Self::calculate_structural_alignment(abstract_out, mapping);
        let domain_compatibility = Self::calculate_domain_compatibility(search_out, mapping);
        let conceptual_distance = Self::calculate_conceptual_distance(&mapping.source, &mapping.target);
        let transfer_viability = Self::calculate_transfer_viability(mapping, &semantic_similarity, &structural_alignment);

        let overall_precision = (
            semantic_similarity * 0.3 +
            structural_alignment * 0.25 +
            domain_compatibility * 0.2 +
            (1.0 - conceptual_distance) * 0.15 +
            transfer_viability * 0.1
        ).clamp(0.0, 1.0);

        let justification = Self::generate_justification(
            mapping,
            semantic_similarity,
            structural_alignment,
            domain_compatibility,
            conceptual_distance,
            transfer_viability,
        );

        PrecisionEntityMapping {
            source: mapping.source.clone(),
            target: mapping.target.clone(),
            relation: mapping.relation.clone(),
            confidence: overall_precision,
            metrics: MappingPrecisionMetrics {
                semantic_similarity,
                structural_alignment,
                domain_compatibility,
                conceptual_distance,
                transfer_viability,
                overall_precision,
            },
            justification,
        }
    }

    fn calculate_semantic_similarity(source: &str, target: &str) -> f32 {
        let source_lower = source.to_lowercase();
        let target_lower = target.to_lowercase();
        
        // Simple similarity heuristics (in real implementation, might use embeddings)
        if source_lower == target_lower {
            return 1.0;
        }
        
        // Check for common semantic patterns
        let semantic_clusters = [
            &["loop", "cycle", "iteration", "repetition", "recurring"] as &[&str],
            &["process", "method", "approach", "technique", "strategy"],
            &["system", "framework", "structure", "architecture", "organization"],
            &["pattern", "template", "model", "schema", "blueprint"],
            &["feedback", "response", "reaction", "adjustment", "correction"],
            &["optimization", "improvement", "enhancement", "refinement", "tuning"],
        ];

        for cluster in &semantic_clusters {
            let source_in_cluster = cluster.iter().any(|&word| source_lower.contains(word));
            let target_in_cluster = cluster.iter().any(|&word| target_lower.contains(word));
            
            if source_in_cluster && target_in_cluster {
                return 0.8;
            }
        }

        // Check for shared keywords
        let source_words: Vec<&str> = source_lower.split_whitespace().collect();
        let target_words: Vec<&str> = target_lower.split_whitespace().collect();
        
        let shared_words = source_words.iter()
            .filter(|word| target_words.contains(word))
            .count();
            
        let total_unique_words = source_words.len() + target_words.len() - shared_words;
        
        if total_unique_words > 0 {
            (shared_words as f32 / total_unique_words as f32).min(0.7)
        } else {
            0.1
        }
    }

    fn calculate_structural_alignment(abstract_out: &AbstractOutput, mapping: &EntityMapping) -> f32 {
        let abstract_lower = abstract_out.abstract_shape.to_lowercase();
        let domain_lower = abstract_out.domain.to_lowercase();
        
        let source_relevance = if abstract_lower.contains(&mapping.source.to_lowercase()) 
            || domain_lower.contains(&mapping.source.to_lowercase()) {
            0.9
        } else {
            // Check for conceptual alignment
            if (abstract_lower.contains("pattern") && mapping.source.to_lowercase().contains("pattern")) ||
               (abstract_lower.contains("system") && mapping.source.to_lowercase().contains("system")) {
                0.7
            } else {
                0.3
            }
        };

        // Relation quality affects structural alignment
        let relation_quality = match mapping.relation.to_lowercase().as_str() {
            "maps_to" | "corresponds_to" | "analogous_to" => 0.9,
            "similar_to" | "relates_to" | "connects_to" => 0.7,
            "triggers" | "implements" | "enables" => 0.8,
            _ => 0.5,
        };

        (source_relevance + relation_quality) / 2.0
    }

    fn calculate_domain_compatibility(search_out: &SearchOutput, mapping: &EntityMapping) -> f32 {
        // Check if source/target concepts appear in the cross-domain matches
        let relevant_domains = search_out.matches.iter()
            .filter(|m| {
                let desc_lower = m.description.to_lowercase();
                desc_lower.contains(&mapping.source.to_lowercase()) ||
                desc_lower.contains(&mapping.target.to_lowercase())
            })
            .count();

        if relevant_domains > 0 {
            (relevant_domains as f32 / search_out.matches.len() as f32).min(0.9) + 0.1
        } else {
            // Check for domain diversity (good cross-domain coverage)
            let unique_domains: std::collections::HashSet<&str> = search_out.matches
                .iter()
                .map(|m| m.domain.as_str())
                .collect();
            
            if unique_domains.len() >= 3 {
                0.6 // Good diversity even without direct matches
            } else {
                0.4
            }
        }
    }

    fn calculate_conceptual_distance(source: &str, target: &str) -> f32 {
        // Distance heuristics (lower is better, range 0-1)
        let source_lower = source.to_lowercase();
        let target_lower = target.to_lowercase();

        // Very close concepts
        if source_lower.contains(&target_lower) || target_lower.contains(&source_lower) {
            return 0.1;
        }

        // Abstract vs concrete mismatch increases distance
        let abstract_terms = ["pattern", "system", "framework", "approach", "method"];
        let concrete_terms = ["tool", "device", "machine", "product", "object"];
        
        let source_abstract = abstract_terms.iter().any(|&term| source_lower.contains(term));
        let target_abstract = abstract_terms.iter().any(|&term| target_lower.contains(term));
        let source_concrete = concrete_terms.iter().any(|&term| source_lower.contains(term));
        let target_concrete = concrete_terms.iter().any(|&term| target_lower.contains(term));

        if (source_abstract && target_concrete) || (source_concrete && target_abstract) {
            0.7 // High conceptual distance
        } else if source_abstract && target_abstract {
            0.3 // Low distance, both abstract
        } else if source_concrete && target_concrete {
            0.4 // Medium distance, both concrete but potentially different domains
        } else {
            0.5 // Default medium distance
        }
    }

    fn calculate_transfer_viability(
        mapping: &EntityMapping,
        semantic_sim: &f32,
        structural_alignment: &f32,
    ) -> f32 {
        // Base viability on relation type and existing confidence
        let relation_viability = match mapping.relation.to_lowercase().as_str() {
            "implements" | "enables" | "creates" => 0.9, // High actionability
            "maps_to" | "corresponds_to" | "analogous_to" => 0.8, // Good structural transfer
            "similar_to" | "relates_to" => 0.6, // Medium transfer potential
            "triggers" | "causes" => 0.7, // Good causal transfer
            _ => 0.5,
        };

        // Combine with semantic and structural factors
        let base_score = (*semantic_sim + *structural_alignment + relation_viability) / 3.0;
        
        // Bonus for high-confidence mappings
        if let Some(original_conf) = mapping.confidence {
            (base_score + original_conf) / 2.0
        } else {
            base_score
        }
    }

    fn generate_justification(
        mapping: &EntityMapping,
        semantic_sim: f32,
        structural_align: f32,
        domain_compat: f32,
        conceptual_dist: f32,
        transfer_viab: f32,
    ) -> String {
        let mut reasons = Vec::new();

        if semantic_sim > 0.7 {
            reasons.push("high semantic similarity");
        } else if semantic_sim > 0.4 {
            reasons.push("moderate semantic overlap");
        }

        if structural_align > 0.7 {
            reasons.push("strong structural alignment");
        } else if structural_align > 0.4 {
            reasons.push("adequate structural fit");
        }

        if domain_compat > 0.7 {
            reasons.push("good cross-domain support");
        }

        if conceptual_dist < 0.3 {
            reasons.push("close conceptual proximity");
        }

        if transfer_viab > 0.7 {
            reasons.push("high transfer viability");
        }

        let relation_desc = match mapping.relation.to_lowercase().as_str() {
            "implements" => "implements relationship with clear actionability",
            "maps_to" => "direct mapping relationship",
            "analogous_to" => "analogical relationship",
            "triggers" => "causal trigger relationship",
            _ => &format!("{} relationship", mapping.relation),
        };

        let base = format!("{} -> {} via {}", mapping.source, mapping.target, relation_desc);
        
        if reasons.is_empty() {
            format!("{} (limited precision indicators)", base)
        } else {
            format!("{}: {}", base, reasons.join(", "))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_abstract_output() -> AbstractOutput {
        AbstractOutput {
            domain: "music".into(),
            abstract_shape: "iterative practice pattern with feedback loops".into(),
        }
    }

    fn sample_search_output() -> SearchOutput {
        SearchOutput {
            matches: vec![
                CrossDomainMatch {
                    domain: "athletics".into(),
                    description: "interval training patterns".into(),
                },
                CrossDomainMatch {
                    domain: "programming".into(),
                    description: "code review cycles".into(),
                },
                CrossDomainMatch {
                    domain: "manufacturing".into(),
                    description: "quality control loops".into(),
                },
            ],
        }
    }

    fn sample_basic_mapping() -> MapOutput {
        MapOutput {
            mappings: vec![
                EntityMapping {
                    source: "practice pattern".into(),
                    target: "training interval".into(),
                    relation: "maps_to".into(),
                    confidence: Some(0.8),
                },
                EntityMapping {
                    source: "feedback loop".into(),
                    target: "review cycle".into(),
                    relation: "analogous_to".into(),
                    confidence: Some(0.7),
                },
            ],
        }
    }

    #[test]
    fn precision_analysis_produces_enhanced_mappings() {
        let abstract_out = sample_abstract_output();
        let search_out = sample_search_output();
        let basic_map = sample_basic_mapping();

        let enhanced = MappingPrecisionAnalyzer::enhance_mappings(
            &abstract_out, 
            &search_out, 
            &basic_map
        );

        assert_eq!(enhanced.mappings.len(), 2);
        assert!(enhanced.overall_precision_score > 0.0);
        assert!(enhanced.overall_precision_score <= 1.0);

        // Check that enhanced mappings have metrics
        for mapping in &enhanced.mappings {
            assert!(mapping.confidence > 0.0);
            assert!(mapping.confidence <= 1.0);
            assert!(!mapping.justification.is_empty());
            assert!(mapping.metrics.overall_precision > 0.0);
        }
    }

    #[test]
    fn semantic_similarity_detects_related_terms() {
        let sim1 = MappingPrecisionAnalyzer::calculate_semantic_similarity(
            "practice loop", 
            "training cycle"
        );
        let sim2 = MappingPrecisionAnalyzer::calculate_semantic_similarity(
            "elephant", 
            "bicycle"
        );

        assert!(sim1 > sim2);
        assert!(sim1 > 0.5);
        assert!(sim2 < 0.3);
    }

    #[test]
    fn precision_grade_categorizes_scores_correctly() {
        assert!(matches!(PrecisionGrade::from_score(0.95), PrecisionGrade::Excellent));
        assert!(matches!(PrecisionGrade::from_score(0.75), PrecisionGrade::Good));
        assert!(matches!(PrecisionGrade::from_score(0.55), PrecisionGrade::Fair));
        assert!(matches!(PrecisionGrade::from_score(0.25), PrecisionGrade::Poor));
    }

    #[test]
    fn empty_mappings_produce_zero_precision() {
        let empty_map = MapOutput { mappings: vec![] };
        let enhanced = MappingPrecisionAnalyzer::enhance_mappings(
            &sample_abstract_output(),
            &sample_search_output(),
            &empty_map,
        );

        assert_eq!(enhanced.overall_precision_score, 0.0);
        assert!(matches!(enhanced.precision_grade, PrecisionGrade::Poor));
    }
}