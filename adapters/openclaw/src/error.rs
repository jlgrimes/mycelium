/// Categorized JSON parse/validation errors for the OpenClaw adapter.
#[derive(Debug, thiserror::Error)]
pub enum OpenClawError {
    #[error("JSON extraction failed: {reason}")]
    JsonExtraction { reason: String, raw: String },

    #[error("JSON validation failed: {reason}")]
    JsonValidation { reason: String, raw: String },
}

impl OpenClawError {
    /// Display-friendly summary for logging without dumping full raw bodies.
    pub fn summary(&self) -> String {
        match self {
            Self::JsonExtraction { reason, raw } | Self::JsonValidation { reason, raw } => {
                let truncated = if raw.len() > 200 { &raw[..200] } else { raw };
                format!("{reason} (raw: {truncated}...)")
            }
        }
    }
}
