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

impl Default for OpenClawConfig {
    fn default() -> Self {
        Self {
            base_url: "http://127.0.0.1:18789/v1/chat/completions".to_string(),
            token: None,
            model: "sonnet".to_string(),
            temperature: 0.2,
            request_timeout: Duration::from_millis(30_000),
            connect_timeout: Duration::from_millis(5_000),
            max_retries: 2,
            retry_base_delay: Duration::from_millis(250),
            retry_max_delay: Duration::from_secs(5),
        }
    }
}

impl OpenClawConfig {
    pub fn from_env() -> Self {
        let mut cfg = Self::default();

        if let Ok(base_url) = std::env::var("OPENCLAW_BASE_URL") {
            cfg.base_url = base_url;
        }

        if let Ok(token) = std::env::var("OPENCLAW_TOKEN") {
            let token = token.trim().to_string();
            if !token.is_empty() {
                cfg.token = Some(token);
            }
        }

        if let Ok(model) = std::env::var("MYCELIUM_MODEL") {
            cfg.model = model;
        }

        if let Ok(v) = std::env::var("OPENCLAW_TEMPERATURE") {
            if let Ok(parsed) = v.parse::<f32>() {
                cfg.temperature = parsed;
            }
        }

        if let Ok(v) = std::env::var("OPENCLAW_TIMEOUT_MS") {
            if let Ok(parsed) = v.parse::<u64>() {
                cfg.request_timeout = Duration::from_millis(parsed);
            }
        }

        if let Ok(v) = std::env::var("OPENCLAW_CONNECT_TIMEOUT_MS") {
            if let Ok(parsed) = v.parse::<u64>() {
                cfg.connect_timeout = Duration::from_millis(parsed);
            }
        }

        if let Ok(v) = std::env::var("OPENCLAW_MAX_RETRIES") {
            if let Ok(parsed) = v.parse::<u32>() {
                cfg.max_retries = parsed;
            }
        }

        if let Ok(v) = std::env::var("OPENCLAW_RETRY_BASE_MS") {
            if let Ok(parsed) = v.parse::<u64>() {
                cfg.retry_base_delay = Duration::from_millis(parsed);
            }
        }

        if let Ok(v) = std::env::var("OPENCLAW_RETRY_MAX_MS") {
            if let Ok(parsed) = v.parse::<u64>() {
                cfg.retry_max_delay = Duration::from_millis(parsed);
            }
        }

        cfg
    }
}
