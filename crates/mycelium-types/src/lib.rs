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

// --- Staged pipeline types ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbstractOutput {
    pub domain: String,
    pub abstract_shape: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchOutput {
    pub matches: Vec<CrossDomainMatch>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossDomainMatch {
    pub domain: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapOutput {
    pub mappings: Vec<EntityMapping>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityMapping {
    pub source: String,
    pub target: String,
    pub relation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynthesizeOutput {
    pub synthesis: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StageTrace {
    pub input: String,
    pub abstract_out: AbstractOutput,
    pub search_out: SearchOutput,
    pub map_out: MapOutput,
    pub synthesize_out: SynthesizeOutput,
}

impl StageTrace {
    pub fn into_response(self) -> ProblemResponse {
        ProblemResponse {
            abstract_shape: self.abstract_out.abstract_shape,
            cross_domain_matches: self
                .search_out
                .matches
                .into_iter()
                .map(|m| format!("{}: {}", m.domain, m.description))
                .collect(),
            mapping: self
                .map_out
                .mappings
                .iter()
                .map(|m| format!("{} -> {} ({})", m.source, m.target, m.relation))
                .collect::<Vec<_>>()
                .join("; "),
            synthesis: self.synthesize_out.synthesis,
        }
    }
}
