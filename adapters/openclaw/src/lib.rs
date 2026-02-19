pub mod config;
pub mod error;
pub mod json;

use anyhow::{anyhow, bail, Context, Result};
use async_trait::async_trait;
use config::OpenClawConfig;
use mycelium_core::ReasoningProvider;
use mycelium_types::ProblemResponse;
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tracing::{debug, warn};

const SYSTEM_PROMPT: &str = r#"You are Mycelium, a cross-domain reasoning engine.
Return ONLY JSON with this exact schema:
{
  "abstract_shape": string,
  "cross_domain_matches": string[],
  "mapping": string,
  "synthesis": string
}

Pipeline:
1) Abstract the user problem into domain-agnostic structure.
2) Find at least 3 cross-domain isomorphic matches.
3) Map entities/processes explicitly.
4) Synthesize concrete advice back in the original domain.
No markdown. No extra keys."#;

#[derive(Clone)]
pub struct OpenClawProvider {
    client: Client,
    config: OpenClawConfig,
}

impl OpenClawProvider {
    pub fn new(config: OpenClawConfig) -> Result<Self> {
        let client = Client::builder()
            .timeout(config.request_timeout)
            .connect_timeout(config.connect_timeout)
            .build()
            .context("failed to build HTTP client")?;
        Ok(Self { client, config })
    }

    pub fn from_env() -> Self {
        Self::new(OpenClawConfig::from_env()).expect("failed to build HTTP client")
    }

    async fn send_with_retry(&self, body: &ChatRequest) -> Result<ChatResponse> {
        if self.config.base_url.trim().is_empty() {
            bail!("openclaw base URL is empty; set OPENCLAW_BASE_URL");
        }

        let total_attempts = self.config.max_retries + 1;
        let mut last_error: Option<anyhow::Error> = None;

        for attempt in 0..total_attempts {
            if attempt > 0 {
                let delay = self.retry_delay(attempt, last_error.as_ref());
                warn!(attempt, delay_ms = delay.as_millis() as u64, "retrying after error");
                tokio::time::sleep(delay).await;
            }

            let mut req = self.client.post(&self.config.base_url).json(body);
            if let Some(token) = &self.config.token {
                req = req.bearer_auth(token);
            }

            match req.send().await {
                Ok(resp) => {
                    let status = resp.status();

                    if status == StatusCode::TOO_MANY_REQUESTS {
                        let retry_after = resp
                            .headers()
                            .get("retry-after")
                            .and_then(|v| v.to_str().ok())
                            .and_then(|v| v.parse::<u64>().ok());
                        let err = anyhow!(
                            "rate limited (429) on attempt {}/{}{}",
                            attempt + 1,
                            total_attempts,
                            retry_after.map(|s| format!(", retry-after: {s}s")).unwrap_or_default()
                        );
                        last_error = Some(err);
                        if attempt < self.config.max_retries {
                            continue;
                        }
                        return Err(last_error.unwrap());
                    }

                    let text = resp.text().await.unwrap_or_default();

                    if status.is_success() {
                        return serde_json::from_str::<ChatResponse>(&text)
                            .with_context(|| format!("invalid chat response body: {text}"));
                    }

                    let err = anyhow!(
                        "openclaw HTTP {} on attempt {}/{}: {}",
                        status,
                        attempt + 1,
                        total_attempts,
                        text
                    );

                    if is_retryable_status(status) && attempt < self.config.max_retries {
                        debug!(attempt, %status, "transient HTTP error, will retry");
                        last_error = Some(err);
                        continue;
                    }

                    return Err(err);
                }
                Err(err) => {
                    let is_timeout = err.is_timeout();
                    let wrapped = if is_timeout {
                        anyhow!("request timed out on attempt {}/{}", attempt + 1, total_attempts)
                    } else {
                        anyhow!("network error on attempt {}/{}: {}", attempt + 1, total_attempts, err)
                    };

                    if (err.is_timeout() || err.is_connect()) && attempt < self.config.max_retries {
                        debug!(attempt, "transient transport error, will retry");
                        last_error = Some(wrapped);
                        continue;
                    }

                    return Err(wrapped);
                }
            }
        }

        Err(last_error.unwrap_or_else(|| anyhow!("retries exhausted after {total_attempts} attempts")))
    }

    fn retry_delay(&self, attempt: u32, _last_error: Option<&anyhow::Error>) -> Duration {
        let factor = 2_u64.saturating_pow(attempt.saturating_sub(1).min(6));
        let delay = self.config.retry_base_delay * factor as u32;
        delay.min(self.config.retry_max_delay)
    }
}

fn is_retryable_status(status: StatusCode) -> bool {
    matches!(
        status,
        StatusCode::REQUEST_TIMEOUT
            | StatusCode::TOO_MANY_REQUESTS
            | StatusCode::BAD_GATEWAY
            | StatusCode::SERVICE_UNAVAILABLE
            | StatusCode::GATEWAY_TIMEOUT
            | StatusCode::INTERNAL_SERVER_ERROR
    )
}

#[async_trait]
impl ReasoningProvider for OpenClawProvider {
    async fn solve(&self, input: &str) -> Result<ProblemResponse> {
        let body = ChatRequest {
            model: self.config.model.clone(),
            temperature: self.config.temperature,
            messages: vec![
                ChatMessage { role: "system".into(), content: SYSTEM_PROMPT.into() },
                ChatMessage { role: "user".into(), content: input.into() },
            ],
        };

        let payload = self.send_with_retry(&body).await?;
        let content = payload
            .choices
            .first()
            .map(|c| c.message.content.clone())
            .ok_or_else(|| anyhow!("no choices in chat response"))?;

        // Use robust JSON extraction with validation
        json::extract_problem_response(&content).map_err(|e| anyhow!("{e}"))
    }
}

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    temperature: f32,
    messages: Vec<ChatMessage>,
}

#[derive(Serialize, Deserialize, Clone)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: ChatMessage,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn retryable_statuses_are_expected() {
        assert!(is_retryable_status(StatusCode::TOO_MANY_REQUESTS));
        assert!(is_retryable_status(StatusCode::INTERNAL_SERVER_ERROR));
        assert!(is_retryable_status(StatusCode::SERVICE_UNAVAILABLE));
        assert!(is_retryable_status(StatusCode::BAD_GATEWAY));
        assert!(is_retryable_status(StatusCode::GATEWAY_TIMEOUT));
        assert!(!is_retryable_status(StatusCode::UNAUTHORIZED));
        assert!(!is_retryable_status(StatusCode::BAD_REQUEST));
    }

    #[test]
    fn retry_delay_exponential_backoff() {
        let config = OpenClawConfig {
            retry_base_delay: Duration::from_millis(100),
            retry_max_delay: Duration::from_secs(5),
            ..Default::default()
        };
        let provider = OpenClawProvider::new(config).unwrap();
        assert_eq!(provider.retry_delay(1, None), Duration::from_millis(100));
        assert_eq!(provider.retry_delay(2, None), Duration::from_millis(200));
        assert_eq!(provider.retry_delay(3, None), Duration::from_millis(400));
    }

    #[test]
    fn retry_delay_respects_max() {
        let config = OpenClawConfig {
            retry_base_delay: Duration::from_secs(1),
            retry_max_delay: Duration::from_secs(2),
            ..Default::default()
        };
        let provider = OpenClawProvider::new(config).unwrap();
        assert_eq!(provider.retry_delay(10, None), Duration::from_secs(2));
    }

    #[test]
    fn config_defaults_are_sane() {
        let config = OpenClawConfig::default();
        assert_eq!(config.max_retries, 3);
        assert_eq!(config.request_timeout, Duration::from_secs(60));
        assert_eq!(config.connect_timeout, Duration::from_secs(10));
        assert_eq!(config.temperature, 0.2);
    }

    #[test]
    fn new_with_config_builds_provider() {
        let config = OpenClawConfig::default();
        let provider = OpenClawProvider::new(config);
        assert!(provider.is_ok());
    }
}
