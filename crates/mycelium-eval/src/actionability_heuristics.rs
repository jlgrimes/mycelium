use mycelium_types::ProblemResponse;
use serde::{Deserialize, Serialize};

/// Detailed actionability analysis with component scores
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionabilityAnalysis {
    pub overall_score: u8,
    pub max_score: u8,
    pub components: ActionabilityComponents,
    pub improvement_suggestions: Vec<String>,
}

/// Individual actionability components with scores and weights
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionabilityComponents {
    pub structure_clarity: ComponentScore,
    pub step_specificity: ComponentScore, 
    pub verification_presence: ComponentScore,
    pub contextual_grounding: ComponentScore,
    pub temporal_ordering: ComponentScore,
    pub outcome_measurability: ComponentScore,
    pub resource_identification: ComponentScore,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentScore {
    pub score: f32,
    pub weight: f32,
    pub max_points: u8,
    pub rationale: String,
}

pub struct CalibratedActionabilityScorer;

impl CalibratedActionabilityScorer {
    /// Enhanced actionability scoring with detailed analysis
    pub fn analyze(response: &ProblemResponse) -> ActionabilityAnalysis {
        let components = ActionabilityComponents {
            structure_clarity: Self::score_structure_clarity(response),
            step_specificity: Self::score_step_specificity(response),
            verification_presence: Self::score_verification_presence(response),
            contextual_grounding: Self::score_contextual_grounding(response),
            temporal_ordering: Self::score_temporal_ordering(response),
            outcome_measurability: Self::score_outcome_measurability(response),
            resource_identification: Self::score_resource_identification(response),
        };

        let weighted_total = components.structure_clarity.score * components.structure_clarity.weight
            + components.step_specificity.score * components.step_specificity.weight
            + components.verification_presence.score * components.verification_presence.weight
            + components.contextual_grounding.score * components.contextual_grounding.weight
            + components.temporal_ordering.score * components.temporal_ordering.weight
            + components.outcome_measurability.score * components.outcome_measurability.weight
            + components.resource_identification.score * components.resource_identification.weight;

        let max_weighted = components.structure_clarity.weight
            + components.step_specificity.weight
            + components.verification_presence.weight
            + components.contextual_grounding.weight
            + components.temporal_ordering.weight
            + components.outcome_measurability.weight
            + components.resource_identification.weight;

        let normalized_score = if max_weighted > 0.0 {
            (weighted_total / max_weighted * 10.0).round() as u8
        } else {
            0
        }.min(10);

        let improvement_suggestions = Self::generate_improvement_suggestions(&components);

        ActionabilityAnalysis {
            overall_score: normalized_score,
            max_score: 10,
            components,
            improvement_suggestions,
        }
    }

    /// Backward-compatible scoring function for legacy integration
    pub fn legacy_score(response: &ProblemResponse) -> u8 {
        let analysis = Self::analyze(response);
        // Scale 0-10 score back to 0-5 for compatibility
        ((analysis.overall_score as f32 / 2.0).round() as u8).min(5)
    }

    fn score_structure_clarity(response: &ProblemResponse) -> ComponentScore {
        let mut score = 0.0_f32;
        let mut rationale_parts: Vec<String> = Vec::new();

        // Check abstract shape quality
        let abstract_shape = response.abstract_shape.trim();
        if !abstract_shape.is_empty() {
            score += 0.3;
            rationale_parts.push("has abstract shape".to_string());
            
            // Bonus for structured abstractions
            if abstract_shape.to_lowercase().contains("pattern") 
                || abstract_shape.to_lowercase().contains("framework")
                || abstract_shape.to_lowercase().contains("system") {
                score += 0.2;
                rationale_parts.push("structured abstraction".to_string());
            }
        } else {
            rationale_parts.push("missing abstract shape".to_string());
        }

        // Check cross-domain matches quality
        if response.cross_domain_matches.len() >= 3 {
            score += 0.3;
            rationale_parts.push("sufficient cross-domain matches".to_string());
            
            // Bonus for diverse domain coverage
            let unique_domains: std::collections::HashSet<String> = response.cross_domain_matches
                .iter()
                .map(|m| {
                    // Extract domain from "domain: description" format
                    m.split(':').next().unwrap_or(m).trim().to_lowercase()
                })
                .collect();
            
            if unique_domains.len() >= 3 {
                score += 0.2;
                rationale_parts.push("diverse domains".to_string());
            }
        } else {
            rationale_parts.push("insufficient cross-domain matches".to_string());
        }

        ComponentScore {
            score: score.min(1.0),
            weight: 2.0, // High importance for structure
            max_points: 1,
            rationale: rationale_parts.join(", "),
        }
    }

    fn score_step_specificity(response: &ProblemResponse) -> ComponentScore {
        let synthesis = response.synthesis.to_lowercase();
        let mapping = response.mapping.to_lowercase();
        let combined = format!("{} {}", synthesis, mapping);
        
        let mut score = 0.0_f32;
        let mut rationale_parts: Vec<String> = Vec::new();

        // Check for action words
        let action_words = ["step", "first", "then", "next", "run", "execute", "perform", "apply", "implement", "create"];
        let action_count = action_words.iter().filter(|&&word| combined.contains(word)).count();
        
        if action_count > 0 {
            score += (action_count as f32 * 0.15).min(0.5);
            rationale_parts.push(format!("{} action words", action_count));
        }

        // Check for imperative language
        let imperative_patterns = ["do", "use", "try", "start", "begin", "focus", "practice", "set"];
        let imperative_count = imperative_patterns.iter().filter(|&&word| combined.contains(word)).count();
        
        if imperative_count > 0 {
            score += (imperative_count as f32 * 0.1).min(0.4);
            rationale_parts.push(format!("{} imperative terms", imperative_count));
        }

        // Check for numbered or sequenced steps
        let has_sequencing = combined.contains("1.") || combined.contains("2.") 
            || combined.contains("first") || combined.contains("second")
            || combined.contains("then") || combined.contains("next")
            || combined.contains("finally");
        
        if has_sequencing {
            score += 0.3;
            rationale_parts.push("sequential structure".to_string());
        }

        if rationale_parts.is_empty() {
            rationale_parts.push("limited specific actions".to_string());
        }

        ComponentScore {
            score: score.min(1.0),
            weight: 1.8,
            max_points: 1,
            rationale: rationale_parts.join(", "),
        }
    }

    fn score_verification_presence(response: &ProblemResponse) -> ComponentScore {
        let synthesis = response.synthesis.to_lowercase();
        let mapping = response.mapping.to_lowercase();
        let combined = format!("{} {}", synthesis, mapping);
        
        let mut score = 0.0_f32;
        let mut rationale_parts: Vec<String> = Vec::new();

        // Enhanced verification terms
        let verification_terms = [
            ("verify", 0.4), ("test", 0.35), ("check", 0.25), ("validate", 0.35),
            ("assert", 0.25), ("confirm", 0.25), ("measure", 0.3), ("assess", 0.25),
            ("evaluate", 0.2), ("review", 0.15), ("monitor", 0.2), ("track", 0.2),
            ("pass condition", 0.4), ("success criteria", 0.4), ("benchmark", 0.25)
        ];

        for &(term, value) in &verification_terms {
            if combined.contains(term) {
                score += value;
                rationale_parts.push(term.to_string());
            }
        }

        // Bonus for explicit success/failure conditions
        if combined.contains("pass") || combined.contains("fail") || combined.contains("success") {
            score += 0.2;
            rationale_parts.push("success/failure criteria".to_string());
        }

        if rationale_parts.is_empty() {
            rationale_parts.push("no verification signals".to_string());
        }

        ComponentScore {
            score: score.min(1.0),
            weight: 1.5,
            max_points: 1,
            rationale: rationale_parts.join(", "),
        }
    }

    fn score_contextual_grounding(response: &ProblemResponse) -> ComponentScore {
        let mut score = 0.0_f32;
        let mut rationale_parts: Vec<String> = Vec::new();

        // Check mapping quality
        let mapping = response.mapping.trim();
        if !mapping.is_empty() {
            score += 0.5; // More generous base score
            rationale_parts.push("has mapping".to_string());
            
            // Bonus for explicit relationships
            if mapping.contains("->") || mapping.contains("maps to") || mapping.contains("corresponds") || mapping.contains("map ") || mapping.contains(" to ") {
                score += 0.4; // More generous bonus
                rationale_parts.push("explicit relationships".to_string());
            }
            
            // Bonus for confidence indicators
            if mapping.contains("confidence") || mapping.contains("high") || mapping.contains("medium") {
                score += 0.2;
                rationale_parts.push("confidence indicators".to_string());
            }
        }

        // Check for contextual connections in synthesis
        let synthesis = response.synthesis.to_lowercase();
        let context_indicators = ["because", "since", "due to", "given", "considering", "based on"];
        let context_count = context_indicators.iter().filter(|&&word| synthesis.contains(word)).count();
        
        if context_count > 0 {
            score += (context_count as f32 * 0.1).min(0.3);
            rationale_parts.push(format!("{} context indicators", context_count));
        }

        if rationale_parts.is_empty() {
            rationale_parts.push("limited contextual grounding".to_string());
        }

        ComponentScore {
            score: score.min(1.0),
            weight: 1.2,
            max_points: 1,
            rationale: rationale_parts.join(", "),
        }
    }

    fn score_temporal_ordering(response: &ProblemResponse) -> ComponentScore {
        let synthesis = response.synthesis.to_lowercase();
        let mut score = 0.0_f32;
        let mut rationale_parts: Vec<String> = Vec::new();

        // Check for temporal sequencing
        let temporal_words = ["first", "then", "next", "after", "before", "while", "during", "finally", "eventually"];
        let temporal_count = temporal_words.iter().filter(|&&word| synthesis.contains(word)).count();
        
        if temporal_count > 0 {
            score += (temporal_count as f32 * 0.4).min(0.8); // More generous for temporal words
            rationale_parts.push(format!("{} temporal markers", temporal_count));
        }

        // Check for explicit ordering (this is very important)
        if synthesis.contains("1.") || synthesis.contains("2.") || synthesis.contains("step") {
            score += 0.5; // More generous for explicit numbering
            rationale_parts.push("explicit ordering".to_string());
        }

        // Check for process flow indicators
        let flow_words = ["workflow", "process", "sequence", "order", "progression"];
        let flow_count = flow_words.iter().filter(|&&word| synthesis.contains(word)).count();
        
        if flow_count > 0 {
            score += (flow_count as f32 * 0.1).min(0.2);
            rationale_parts.push(format!("{} flow indicators", flow_count));
        }

        if rationale_parts.is_empty() {
            rationale_parts.push("no temporal ordering".to_string());
        }

        ComponentScore {
            score: score.min(1.0),
            weight: 1.0,
            max_points: 1,
            rationale: rationale_parts.join(", "),
        }
    }

    fn score_outcome_measurability(response: &ProblemResponse) -> ComponentScore {
        let synthesis = response.synthesis.to_lowercase();
        let mapping = response.mapping.to_lowercase();
        let combined = format!("{} {}", synthesis, mapping);
        
        let mut score = 0.0_f32;
        let mut rationale_parts: Vec<String> = Vec::new();

        // Check for measurable outcomes
        let measurement_terms = ["measure", "metric", "score", "rate", "percentage", "count", "number", "amount", "level", "degree"];
        let measurement_count = measurement_terms.iter().filter(|&&word| combined.contains(word)).count();
        
        if measurement_count > 0 {
            score += (measurement_count as f32 * 0.15).min(0.4);
            rationale_parts.push(format!("{} measurement terms", measurement_count));
        }

        // Check for quantifiable language
        let quantity_words = ["improve", "increase", "reduce", "optimize", "maximize", "minimize", "achieve", "better", "enhancement", "improvement"];
        let quantity_count = quantity_words.iter().filter(|&&word| combined.contains(word)).count();
        
        if quantity_count > 0 {
            score += (quantity_count as f32 * 0.2).min(0.5); // More generous
            rationale_parts.push(format!("{} quantifiable terms", quantity_count));
        }

        // Bonus for explicit success criteria
        if combined.contains("goal") || combined.contains("target") || combined.contains("objective") {
            score += 0.3;
            rationale_parts.push("explicit goals".to_string());
        }

        if rationale_parts.is_empty() {
            rationale_parts.push("limited measurability".to_string());
        }

        ComponentScore {
            score: score.min(1.0),
            weight: 1.1,
            max_points: 1,
            rationale: rationale_parts.join(", "),
        }
    }

    fn score_resource_identification(response: &ProblemResponse) -> ComponentScore {
        let synthesis = response.synthesis.to_lowercase();
        let mapping = response.mapping.to_lowercase();
        let combined = format!("{} {}", synthesis, mapping);
        
        let mut score = 0.0_f32;
        let mut rationale_parts: Vec<String> = Vec::new();

        // Check for resource identification
        let resource_terms = ["tool", "resource", "material", "equipment", "platform", "system", "framework", "method", "technique", "recording", "device", "software", "app"];
        let resource_count = resource_terms.iter().filter(|&&word| combined.contains(word)).count();
        
        if resource_count > 0 {
            score += (resource_count as f32 * 0.1).min(0.4);
            rationale_parts.push(format!("{} resource references", resource_count));
        }

        // Check for skill or capability requirements
        let capability_terms = ["skill", "knowledge", "experience", "expertise", "ability", "competence"];
        let capability_count = capability_terms.iter().filter(|&&word| combined.contains(word)).count();
        
        if capability_count > 0 {
            score += (capability_count as f32 * 0.1).min(0.3);
            rationale_parts.push(format!("{} capability references", capability_count));
        }

        // Bonus for time/effort estimates
        let effort_terms = ["time", "effort", "duration", "hours", "days", "weeks", "practice", "training"];
        let effort_count = effort_terms.iter().filter(|&&word| combined.contains(word)).count();
        
        if effort_count > 0 {
            score += (effort_count as f32 * 0.05).min(0.3);
            rationale_parts.push(format!("{} effort indicators", effort_count));
        }

        if rationale_parts.is_empty() {
            rationale_parts.push("no resource identification".to_string());
        }

        ComponentScore {
            score: score.min(1.0),
            weight: 0.8,
            max_points: 1,
            rationale: rationale_parts.join(", "),
        }
    }

    fn generate_improvement_suggestions(components: &ActionabilityComponents) -> Vec<String> {
        let mut suggestions = Vec::new();

        if components.structure_clarity.score < 0.7 {
            suggestions.push("Improve structure clarity with clearer abstract patterns and diverse cross-domain matches".to_string());
        }

        if components.step_specificity.score < 0.6 {
            suggestions.push("Add more specific action steps with imperative language and sequential ordering".to_string());
        }

        if components.verification_presence.score < 0.5 {
            suggestions.push("Include explicit verification methods, success criteria, and measurement approaches".to_string());
        }

        if components.contextual_grounding.score < 0.6 {
            suggestions.push("Strengthen contextual connections with explicit relationships and confidence indicators".to_string());
        }

        if components.temporal_ordering.score < 0.5 {
            suggestions.push("Add temporal sequencing with clear ordering and process flow indicators".to_string());
        }

        if components.outcome_measurability.score < 0.6 {
            suggestions.push("Include measurable outcomes with quantifiable goals and success metrics".to_string());
        }

        if components.resource_identification.score < 0.5 {
            suggestions.push("Identify required resources, tools, skills, and time/effort estimates".to_string());
        }

        if suggestions.is_empty() {
            suggestions.push("Actionability analysis shows good coverage across all components".to_string());
        }

        suggestions
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mycelium_types::ProblemResponse;

    fn make_response(abstract_shape: &str, matches: Vec<&str>, mapping: &str, synthesis: &str) -> ProblemResponse {
        ProblemResponse {
            abstract_shape: abstract_shape.to_string(),
            cross_domain_matches: matches.into_iter().map(String::from).collect(),
            mapping: mapping.to_string(),
            synthesis: synthesis.to_string(),
        }
    }

    #[test]
    fn calibrated_scorer_produces_detailed_analysis() {
        let response = make_response(
            "Iterative practice pattern with feedback loops",
            vec!["Music: practice chunking", "Athletics: interval training", "Programming: code review cycles"],
            "Map practice sessions -> technique drills (high confidence)",
            "First, establish baseline measurement. Then apply spaced repetition technique with explicit verification steps to test and measure improvement."
        );

        let analysis = CalibratedActionabilityScorer::analyze(&response);
        
        assert!(analysis.overall_score > 6); // Should be better than basic scoring
        assert_eq!(analysis.max_score, 10);
        assert!(!analysis.improvement_suggestions.is_empty());
        
        // Check component scores are reasonable
        assert!(analysis.components.structure_clarity.score > 0.5);
        assert!(analysis.components.step_specificity.score > 0.5);
        assert!(analysis.components.verification_presence.score > 0.5);
    }

    #[test]
    fn legacy_compatibility_maintained() {
        let response = make_response(
            "Test pattern",
            vec!["Domain 1", "Domain 2", "Domain 3"],
            "source -> target",
            "Step 1: do this. Step 2: verify result."
        );

        let legacy_score = CalibratedActionabilityScorer::legacy_score(&response);
        assert!(legacy_score <= 5); // Should be within legacy range
        
        let analysis = CalibratedActionabilityScorer::analyze(&response);
        let expected_legacy = ((analysis.overall_score as f32 / 2.0).round() as u8).min(5);
        assert_eq!(legacy_score, expected_legacy);
    }

    #[test]
    fn empty_response_scores_zero() {
        let response = make_response("", vec![], "", "");
        let analysis = CalibratedActionabilityScorer::analyze(&response);
        
        assert_eq!(analysis.overall_score, 0);
        assert!(analysis.improvement_suggestions.len() > 1); // Should have multiple suggestions
    }

    #[test]
    fn high_actionability_response_scores_well() {
        let response = make_response(
            "Systematic optimization framework with measurement loops",
            vec!["Manufacturing: kaizen cycles", "Software: CI/CD pipelines", "Athletics: training periodization", "Science: experimental method"],
            "Map optimization cycles -> systematic improvement with confidence tracking (high confidence: 0.85)",
            "1. First, establish baseline measurements and success criteria. 2. Then implement systematic optimization cycles using proven techniques. 3. Apply continuous measurement and verification to test improvements. 4. Finally, use feedback loops to refine the approach based on measurable outcomes."
        );

        let analysis = CalibratedActionabilityScorer::analyze(&response);
        
        assert!(analysis.overall_score >= 8); // Should score very highly
        assert!(analysis.components.structure_clarity.score > 0.8);
        assert!(analysis.components.step_specificity.score > 0.8);
        assert!(analysis.components.verification_presence.score > 0.8);
        assert!(analysis.components.temporal_ordering.score > 0.8);
    }

    #[test]
    fn improvement_suggestions_are_contextual() {
        let weak_structure = make_response("", vec!["only one"], "", "do something");
        let analysis = CalibratedActionabilityScorer::analyze(&weak_structure);
        
        let suggestions_text = analysis.improvement_suggestions.join(" ");
        assert!(suggestions_text.contains("structure"));
        assert!(suggestions_text.contains("verification"));
    }
}