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
