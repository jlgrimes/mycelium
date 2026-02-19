use crate::error::OpenClawError;
use mycelium_types::ProblemResponse;

/// Extract and validate a `ProblemResponse` from raw LLM output.
///
/// Tries, in order:
/// 1. Direct parse of the entire string.
/// 2. Extract the outermost `{...}` block (handles markdown fences, preamble text).
/// 3. Scan for multiple `{...}` blocks and try each.
pub fn extract_problem_response(raw: &str) -> Result<ProblemResponse, OpenClawError> {
    // Fast path: direct parse
    if let Ok(parsed) = serde_json::from_str::<ProblemResponse>(raw) {
        return validate(parsed, raw);
    }

    // Try extracting outermost JSON object
    if let Some(json_str) = extract_outermost_json(raw) {
        if let Ok(parsed) = serde_json::from_str::<ProblemResponse>(json_str) {
            return validate(parsed, raw);
        }
    }

    // Scan for all top-level JSON objects and try each
    for candidate in extract_all_json_objects(raw) {
        if let Ok(parsed) = serde_json::from_str::<ProblemResponse>(candidate) {
            return validate(parsed, raw);
        }
    }

    Err(OpenClawError::JsonExtraction {
        reason: "no valid ProblemResponse JSON found in response".into(),
        raw: raw.to_string(),
    })
}

fn validate(resp: ProblemResponse, raw: &str) -> Result<ProblemResponse, OpenClawError> {
    if resp.abstract_shape.trim().is_empty() {
        return Err(OpenClawError::JsonValidation {
            reason: "abstract_shape is empty".into(),
            raw: raw.to_string(),
        });
    }
    if resp.cross_domain_matches.is_empty() {
        return Err(OpenClawError::JsonValidation {
            reason: "cross_domain_matches is empty".into(),
            raw: raw.to_string(),
        });
    }
    if resp.synthesis.trim().is_empty() {
        return Err(OpenClawError::JsonValidation {
            reason: "synthesis is empty".into(),
            raw: raw.to_string(),
        });
    }
    Ok(resp)
}

/// Find the outermost `{...}` by tracking brace depth, skipping string literals.
fn extract_outermost_json(raw: &str) -> Option<&str> {
    let start = raw.find('{')?;
    let bytes = raw.as_bytes();
    let mut depth = 0i32;
    let mut in_string = false;
    let mut escape_next = false;

    for i in start..bytes.len() {
        let b = bytes[i];
        if escape_next {
            escape_next = false;
            continue;
        }
        if b == b'\\' && in_string {
            escape_next = true;
            continue;
        }
        if b == b'"' {
            in_string = !in_string;
            continue;
        }
        if in_string {
            continue;
        }
        match b {
            b'{' => depth += 1,
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    return Some(&raw[start..=i]);
                }
            }
            _ => {}
        }
    }
    None
}

/// Extract all top-level `{...}` objects from the string.
fn extract_all_json_objects(raw: &str) -> Vec<&str> {
    let mut results = Vec::new();
    let bytes = raw.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        if bytes[i] == b'{' {
            let start = i;
            let mut depth = 0i32;
            let mut in_string = false;
            let mut escape_next = false;

            for j in start..bytes.len() {
                let b = bytes[j];
                if escape_next {
                    escape_next = false;
                    continue;
                }
                if b == b'\\' && in_string {
                    escape_next = true;
                    continue;
                }
                if b == b'"' {
                    in_string = !in_string;
                    continue;
                }
                if in_string {
                    continue;
                }
                match b {
                    b'{' => depth += 1,
                    b'}' => {
                        depth -= 1;
                        if depth == 0 {
                            results.push(&raw[start..=j]);
                            i = j + 1;
                            break;
                        }
                    }
                    _ => {}
                }
                if j == bytes.len() - 1 {
                    // unmatched brace, skip
                    i = j + 1;
                }
            }
        } else {
            i += 1;
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_json() -> String {
        serde_json::json!({
            "abstract_shape": "A recursive optimization problem",
            "cross_domain_matches": [
                "Genetic algorithms",
                "Neural pruning",
                "Supply chain optimization"
            ],
            "mapping": "Map local optima to pruning decisions",
            "synthesis": "Apply iterative refinement with crossover strategies"
        })
        .to_string()
    }

    #[test]
    fn direct_parse() {
        let raw = valid_json();
        let resp = extract_problem_response(&raw).unwrap();
        assert_eq!(resp.cross_domain_matches.len(), 3);
    }

    #[test]
    fn extract_from_markdown_fences() {
        let raw = format!("Here is the result:\n```json\n{}\n```\nDone.", valid_json());
        let resp = extract_problem_response(&raw).unwrap();
        assert_eq!(resp.abstract_shape, "A recursive optimization problem");
    }

    #[test]
    fn extract_from_preamble_text() {
        let raw = format!("Sure! Here's my analysis:\n\n{}", valid_json());
        let resp = extract_problem_response(&raw).unwrap();
        assert!(!resp.synthesis.is_empty());
    }

    #[test]
    fn nested_json_in_strings() {
        // Ensure escaped braces inside strings don't break extraction
        let raw = r#"{"abstract_shape": "shape with {braces}", "cross_domain_matches": ["a", "b", "c"], "mapping": "map", "synthesis": "synth"}"#;
        let resp = extract_problem_response(raw).unwrap();
        assert_eq!(resp.abstract_shape, "shape with {braces}");
    }

    #[test]
    fn rejects_empty_abstract_shape() {
        let raw = r#"{"abstract_shape": "", "cross_domain_matches": ["a"], "mapping": "m", "synthesis": "s"}"#;
        let err = extract_problem_response(raw).unwrap_err();
        assert!(matches!(err, OpenClawError::JsonValidation { .. }));
    }

    #[test]
    fn rejects_empty_matches() {
        let raw = r#"{"abstract_shape": "ok", "cross_domain_matches": [], "mapping": "m", "synthesis": "s"}"#;
        let err = extract_problem_response(raw).unwrap_err();
        assert!(matches!(err, OpenClawError::JsonValidation { .. }));
    }

    #[test]
    fn rejects_empty_synthesis() {
        let raw = r#"{"abstract_shape": "ok", "cross_domain_matches": ["a"], "mapping": "m", "synthesis": "  "}"#;
        let err = extract_problem_response(raw).unwrap_err();
        assert!(matches!(err, OpenClawError::JsonValidation { .. }));
    }

    #[test]
    fn no_json_at_all() {
        let err = extract_problem_response("just plain text with no json").unwrap_err();
        assert!(matches!(err, OpenClawError::JsonExtraction { .. }));
    }

    #[test]
    fn multiple_json_objects_picks_valid() {
        let noise = r#"{"irrelevant": true}"#;
        let raw = format!("prefix {} then {}", noise, valid_json());
        let resp = extract_problem_response(&raw).unwrap();
        assert_eq!(resp.cross_domain_matches.len(), 3);
    }

    #[test]
    fn extract_outermost_handles_nested() {
        let input = r#"blah {"a": {"b": 1}, "c": 2} more"#;
        let result = extract_outermost_json(input).unwrap();
        assert_eq!(result, r#"{"a": {"b": 1}, "c": 2}"#);
    }
}
