/// Categorized errors for the OpenClaw adapter.
#[derive(Debug, thiserror::Error)]
pub enum OpenClawError {
    #[error("request timed out after {0:?}")]
    Timeout(std::time::Duration),

    #[error("HTTP {status}: {body}")]
    Http { status: u16, body: String },

    #[error("rate limited (HTTP 429), retry after {retry_after_secs:?}s")]
    RateLimited { retry_after_secs: Option<u64> },

    #[error("network error: {0}")]
    Network(#[source] reqwest::Error),

    #[error("empty response: no choices returned")]
    EmptyResponse,

    #[error("JSON extraction failed: {reason}")]
    JsonExtraction { reason: String, raw: String },

    #[error("JSON validation failed: {reason}")]
    JsonValidation { reason: String, raw: String },

    #[error("retries exhausted after {attempts} attempts: {last_error}")]
    RetriesExhausted {
        attempts: u32,
        last_error: Box<OpenClawError>,
    },
}

impl OpenClawError {
    /// Whether this error is transient and worth retrying.
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            Self::Timeout(_) | Self::RateLimited { .. } | Self::Network(_)
        ) || matches!(self, Self::Http { status, .. } if *status >= 500)
    }

    /// Display-friendly summary for logging without dumping full raw bodies.
    pub fn summary(&self) -> String {
        match self {
            Self::JsonExtraction { reason, raw } | Self::JsonValidation { reason, raw } => {
                let truncated = if raw.len() > 200 { &raw[..200] } else { raw };
                format!("{reason} (raw: {truncated}...)")
            }
            other => format!("{other}"),
        }
    }
}

/// Convenience alias.
pub type Result<T> = std::result::Result<T, OpenClawError>;
