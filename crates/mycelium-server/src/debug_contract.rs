use mycelium_types::ProblemResponse;
use serde::{Deserialize, Serialize};

/// Validation result for debug contract compliance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractValidation {
    pub valid: bool,
    pub issues: Vec<String>,
    pub confidence: String,
}

/// Debug contract validator for ensuring response compliance.
pub struct DebugContractValidator;

impl DebugContractValidator {
    pub fn validate(resp: &ProblemResponse) -> ContractValidation {
        let mut issues = Vec::new();
        
        // Check ABSTRACT prefix
        if !resp.abstract_shape.starts_with("ABSTRACT:") {
            issues.push("abstract_shape must start with 'ABSTRACT:'".to_string());
        }
        
        // Check SEARCH prefixes
        if resp.cross_domain_matches.len() < 3 {
            issues.push("cross_domain_matches must contain at least 3 items".to_string());
        }
        
        let search_prefix_violations = resp.cross_domain_matches.iter()
            .enumerate()
            .filter(|(_, m)| !m.starts_with("SEARCH:"))
            .count();
            
        if search_prefix_violations > 0 {
            issues.push(format!(
                "{} cross_domain_matches missing 'SEARCH:' prefix", 
                search_prefix_violations
            ));
        }
        
        // Check MAP prefix
        if !resp.mapping.starts_with("MAP:") {
            issues.push("mapping must start with 'MAP:'".to_string());
        }
        
        // Check SYNTHESIZE prefix and verification content
        if !resp.synthesis.starts_with("SYNTHESIZE:") {
            issues.push("synthesis must start with 'SYNTHESIZE:'".to_string());
        }
        
        let synthesis_lower = resp.synthesis.to_lowercase();
        if !synthesis_lower.contains("verification") {
            issues.push("synthesis must contain verification steps".to_string());
        }
        
        // Check for required synthesis sections
        let required_sections = ["pivot", "fix", "verification", "fallback"];
        for section in required_sections {
            if !synthesis_lower.contains(section) {
                issues.push(format!("synthesis missing required section: {}", section));
            }
        }
        
        // Check mapping confidence
        if !resp.mapping.contains("confidence:") {
            issues.push("mapping must include confidence assessment".to_string());
        }
        
        let confidence = derive_mapping_confidence(resp);
        let valid = issues.is_empty();
        
        ContractValidation {
            valid,
            issues,
            confidence: confidence.to_string(),
        }
    }
    
    /// Enforce debug contract on a response, making it compliant.
    pub fn enforce(resp: ProblemResponse, concise: bool) -> ProblemResponse {
        let confidence = derive_mapping_confidence(&resp);

        let synthesize = if concise {
            format!(
                "SYNTHESIZE:\n- Pivot: shift to a non-repeating isomorphic frame.\n- Fix: {}\n- Verification: run a focused reproducer test with explicit pass/fail checks.\n- Fallback: pivot to the next frame if checks fail.",
                first_line(&resp.synthesis)
            )
        } else {
            format_detailed_synthesize(&resp.synthesis)
        };

        ProblemResponse {
            abstract_shape: format!(
                "ABSTRACT:\n- {}",
                non_empty_or(&resp.abstract_shape, "Debug loop with uncertain root cause")
            ),
            cross_domain_matches: ensure_search_matches(&resp.cross_domain_matches, concise),
            mapping: format!(
                "MAP:\n- {}\n- Mapping confidence: {confidence}",
                non_empty_or(
                    &resp.mapping,
                    "Map repeating failure symptom -> instrumentation point -> isolating test"
                )
            ),
            synthesis: synthesize,
        }
    }
}

fn ensure_search_matches(matches: &[String], concise: bool) -> Vec<String> {
    let mut result: Vec<String> = matches
        .iter()
        .take(if concise { 3 } else { 5 })
        .map(|m| {
            if m.starts_with("SEARCH:") {
                m.clone()
            } else {
                format!("SEARCH: {m}")
            }
        })
        .collect();

    // Fill in defaults if we don't have enough
    while result.len() < 3 {
        match result.len() {
            0 => result.push("SEARCH: Compiler pass ordering as a loop-breaking analog".to_string()),
            1 => result.push("SEARCH: Incident response triage as a hypothesis isolation analog".to_string()),
            2 => result.push("SEARCH: Medical differential diagnosis as a verification analog".to_string()),
            _ => break,
        }
    }
    
    if concise {
        result.truncate(3);
    }
    
    result
}

fn format_detailed_synthesize(raw: &str) -> String {
    format!(
        "SYNTHESIZE:\nPivot rationale:\n- Shift to an isomorphic frame that avoids repeating the failed hypothesis.\n\nFix steps:\n- {}\n\nVerification steps:\n- Add/Run a focused test that reproduces the original failure.\n- Confirm one explicit pass condition and one explicit fail condition.\n\nFallback pivot:\n- If verification fails, pivot to the next closest isomorphic frame and avoid retrying the same failed fix pattern.",
        first_line(raw)
    )
}

fn non_empty_or<'a>(value: &'a str, fallback: &'a str) -> &'a str {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        fallback
    } else {
        trimmed
    }
}

fn first_line(value: &str) -> &str {
    value
        .lines()
        .find(|line| !line.trim().is_empty())
        .unwrap_or("Create one isolated hypothesis and verify it before broad changes")
}

fn derive_mapping_confidence(resp: &ProblemResponse) -> &'static str {
    let has_3_matches = resp.cross_domain_matches.len() >= 3;
    let has_mapping = !resp.mapping.trim().is_empty();

    match (has_3_matches, has_mapping) {
        (true, true) => "high",
        (true, false) | (false, true) => "medium",
        (false, false) => "low",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base_response() -> ProblemResponse {
        ProblemResponse {
            abstract_shape: "ABSTRACT:\n- shape".to_string(),
            cross_domain_matches: vec![
                "SEARCH: a".to_string(), 
                "SEARCH: b".to_string(), 
                "SEARCH: c".to_string()
            ],
            mapping: "MAP:\n- x -> y\n- Mapping confidence: high".to_string(),
            synthesis: "SYNTHESIZE:\nPivot rationale:\n- test\n\nFix steps:\n- Do the thing\n\nVerification steps:\n- test it\n\nFallback pivot:\n- try something else".to_string(),
        }
    }

    fn non_compliant_response() -> ProblemResponse {
        ProblemResponse {
            abstract_shape: "shape".to_string(),
            cross_domain_matches: vec!["a".to_string()],
            mapping: "x -> y".to_string(),
            synthesis: "Do the thing".to_string(),
        }
    }

    #[test]
    fn validation_passes_for_compliant_response() {
        let validation = DebugContractValidator::validate(&base_response());
        assert!(validation.valid);
        assert!(validation.issues.is_empty());
        assert_eq!(validation.confidence, "high");
    }

    #[test]
    fn validation_fails_for_non_compliant_response() {
        let validation = DebugContractValidator::validate(&non_compliant_response());
        assert!(!validation.valid);
        assert!(!validation.issues.is_empty());
        
        // Check for specific issues
        let issues_text = validation.issues.join(" ");
        assert!(issues_text.contains("ABSTRACT:"));
        assert!(issues_text.contains("at least 3 items"));
        assert!(issues_text.contains("SEARCH:"));
        assert!(issues_text.contains("MAP:"));
        assert!(issues_text.contains("SYNTHESIZE:"));
    }

    #[test]
    fn enforce_makes_response_compliant() {
        let enforced = DebugContractValidator::enforce(non_compliant_response(), false);
        let validation = DebugContractValidator::validate(&enforced);
        assert!(validation.valid);
    }

    #[test]
    fn concise_mode_limits_matches() {
        let mut resp = base_response();
        resp.cross_domain_matches = vec![
            "a".to_string(), 
            "b".to_string(), 
            "c".to_string(), 
            "d".to_string(), 
            "e".to_string()
        ];
        let enforced = DebugContractValidator::enforce(resp, true);
        assert_eq!(enforced.cross_domain_matches.len(), 3);
    }

    #[test]
    fn ensure_search_matches_adds_prefixes() {
        let matches = vec!["plain text".to_string()];
        let result = ensure_search_matches(&matches, false);
        assert!(result[0].starts_with("SEARCH:"));
        assert_eq!(result.len(), 3); // Should fill to minimum 3
    }
}