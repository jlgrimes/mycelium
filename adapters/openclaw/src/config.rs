use std::time::Duration;

/// Configuration for the OpenClaw adapter.
#[derive(Debug, Clone)]
pub struct OpenClawConfig {
    pub base_url: String,
    pub token: Option<String>,
    pub model: String,
    pub temperature: f32,
    pub request_timeout: Duration,
    pub connect_timeout: Duration,
    pub max_retries: u32,
    pub retry_base_delay: Duration,
    pub retry_max_delay: Duration,
}

impl OpenClawConfig {
    /// Build config from environment variables with sensible defaults.
    pub fn from_env() -> Self {
        let parse_duration_ms = |var: &str, default_ms: u64| -> Duration {
            std::env::var(var)
                .ok()
                .and_then(|v| v.parse::<u64>().ok())
                .map(Duration::from_millis)
                .unwrap_or(Duration::from_millis(default_ms))
        };

        Self {
            base_url: std::env::var("OPENCLAW_BASE_URL")
                .unwrap_or_else(|_| "http://127.0.0.1:18789/v1/chat/completions".into()),
            token: std::env::var("OPENCLAW_TOKEN").ok(),
            model: std::env::var("MYCELIUM_MODEL").unwrap_or_else(|_| "sonnet".into()),
            temperature: std::env::var("MYCELIUM_TEMPERATURE")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(0.2),
            request_timeout: parse_duration_ms("OPENCLAW_TIMEOUT_MS", 60_000),
            connect_timeout: parse_duration_ms("OPENCLAW_CONNECT_TIMEOUT_MS", 10_000),
            max_retries: std::env::var("OPENCLAW_MAX_RETRIES")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(3),
            retry_base_delay: parse_duration_ms("OPENCLAW_RETRY_BASE_MS", 500),
            retry_max_delay: parse_duration_ms("OPENCLAW_RETRY_MAX_MS", 15_000),
        }
    }
}

impl Default for OpenClawConfig {
    fn default() -> Self {
        Self {
            base_url: "http://127.0.0.1:18789/v1/chat/completions".into(),
            token: None,
            model: "sonnet".into(),
            temperature: 0.2,
            request_timeout: Duration::from_secs(60),
            connect_timeout: Duration::from_secs(10),
            max_retries: 3,
            retry_base_delay: Duration::from_millis(500),
            retry_max_delay: Duration::from_secs(15),
        }
    }
}
