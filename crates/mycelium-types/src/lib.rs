use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemRequest {
    pub input: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemResponse {
    pub abstract_shape: String,
    pub cross_domain_matches: Vec<String>,
    pub mapping: String,
    pub synthesis: String,
}

impl ProblemResponse {
    pub fn normalized(mut self) -> Self {
        self.abstract_shape = self.abstract_shape.trim().to_string();
        self.mapping = self.mapping.trim().to_string();
        self.synthesis = self.synthesis.trim().to_string();

        let mut matches = Vec::new();
        for item in self.cross_domain_matches {
            let item = item.trim().to_string();
            if !item.is_empty() && !matches.contains(&item) {
                matches.push(item);
            }
        }
        self.cross_domain_matches = matches;

        self
    }

    pub fn quality_issues(&self) -> Vec<&'static str> {
        let mut issues = Vec::new();
        if self.abstract_shape.trim().is_empty() {
            issues.push("abstract_shape is empty");
        }
        if self.cross_domain_matches.len() < 3 {
            issues.push("cross_domain_matches must contain at least 3 items");
        }
        if self.mapping.trim().is_empty() {
            issues.push("mapping is empty");
        }
        if self.synthesis.trim().is_empty() {
            issues.push("synthesis is empty");
        }
        issues
    }

    pub fn quality_score(&self) -> u8 {
        let mut score = 0_u8;
        if !self.abstract_shape.trim().is_empty() {
            score += 25;
        }
        if self.cross_domain_matches.len() >= 3 {
            score += 25;
        }
        if !self.mapping.trim().is_empty() {
            score += 25;
        }
        if !self.synthesis.trim().is_empty() {
            score += 25;
        }
        score
    }

    pub fn validate_quality(&self) -> Result<(), String> {
        let issues = self.quality_issues();
        if issues.is_empty() {
            Ok(())
        } else {
            Err(issues.join("; "))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ProblemResponse;

    #[test]
    fn normalize_trims_and_dedupes_matches() {
        let response = ProblemResponse {
            abstract_shape: " loop ".to_string(),
            cross_domain_matches: vec![
                " one ".to_string(),
                "".to_string(),
                "one".to_string(),
                " two".to_string(),
            ],
            mapping: " map ".to_string(),
            synthesis: " synth ".to_string(),
        };

        let normalized = response.normalized();
        assert_eq!(normalized.abstract_shape, "loop");
        assert_eq!(normalized.cross_domain_matches, vec!["one", "two"]);
        assert_eq!(normalized.mapping, "map");
        assert_eq!(normalized.synthesis, "synth");
    }

    #[test]
    fn quality_checks_require_three_matches() {
        let response = ProblemResponse {
            abstract_shape: "shape".to_string(),
            cross_domain_matches: vec!["one".to_string(), "two".to_string()],
            mapping: "map".to_string(),
            synthesis: "syn".to_string(),
        };

        assert!(response.validate_quality().is_err());
        assert_eq!(response.quality_score(), 75);
    }
}
