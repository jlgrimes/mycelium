use anyhow::{anyhow, bail, Context, Result};
use async_trait::async_trait;
use mycelium_core::ReasoningProvider;
use mycelium_types::ProblemResponse;
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use std::time::Duration;

const DEFAULT_BASE_URL: &str = "http://127.0.0.1:18789/v1/chat/completions";
const DEFAULT_MODEL: &str = "sonnet";
const DEFAULT_TIMEOUT_MS: u64 = 30_000;
const DEFAULT_MAX_RETRIES: u32 = 2;
const DEFAULT_RETRY_BASE_MS: u64 = 250;

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
    cfg: OpenClawConfig,
}

#[derive(Clone, Debug)]
struct OpenClawConfig {
    base_url: String,
    auth: Option<AuthHeader>,
    model: String,
    max_retries: u32,
    retry_base_ms: u64,
}

#[derive(Clone, Debug)]
struct AuthHeader {
    name: String,
    value: String,
}

impl OpenClawProvider {
    pub fn from_env() -> Self {
        let timeout_ms = read_env_u64("OPENCLAW_TIMEOUT_MS", DEFAULT_TIMEOUT_MS);
        let client = Client::builder()
            .timeout(Duration::from_millis(timeout_ms))
            .build()
            .expect("reqwest client construction should not fail");

        Self {
            client,
            cfg: OpenClawConfig {
                base_url: std::env::var("OPENCLAW_BASE_URL")
                    .unwrap_or_else(|_| DEFAULT_BASE_URL.to_string()),
                auth: read_auth_from_env(),
                model: std::env::var("MYCELIUM_MODEL")
                    .unwrap_or_else(|_| DEFAULT_MODEL.to_string()),
                max_retries: read_env_u32("OPENCLAW_MAX_RETRIES", DEFAULT_MAX_RETRIES),
                retry_base_ms: read_env_u64("OPENCLAW_RETRY_BASE_MS", DEFAULT_RETRY_BASE_MS),
            },
        }
    }

    async fn send_with_retry(&self, body: &ChatRequest) -> Result<ChatResponse> {
        if self.cfg.base_url.trim().is_empty() {
            bail!("openclaw base URL is empty; set OPENCLAW_BASE_URL");
        }

        let total_attempts = self.cfg.max_retries + 1;
        let mut last_error: Option<anyhow::Error> = None;

        for attempt in 0..total_attempts {
            let request = self.build_request(body);
            match request.send().await {
                Ok(resp) => {
                    let status = resp.status();
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

                    if is_retryable_status(status) && attempt < self.cfg.max_retries {
                        last_error = Some(err);
                        tokio::time::sleep(self.retry_delay(attempt)).await;
                        continue;
                    }

                    return Err(err);
                }
                Err(err) => {
                    let wrapped = anyhow!(
                        "openclaw request failed on attempt {}/{}: {}",
                        attempt + 1,
                        total_attempts,
                        err
                    );

                    if is_retryable_transport_error(&err) && attempt < self.cfg.max_retries {
                        last_error = Some(wrapped);
                        tokio::time::sleep(self.retry_delay(attempt)).await;
                        continue;
                    }

                    return Err(wrapped);
                }
            }
        }

        Err(last_error.unwrap_or_else(|| anyhow!("openclaw request failed without error details")))
    }

    fn build_request(&self, body: &ChatRequest) -> reqwest::RequestBuilder {
        let mut req = self.client.post(&self.cfg.base_url).json(body);
        if let Some(auth) = &self.cfg.auth {
            req = req.header(&auth.name, &auth.value);
        }
        req
    }

    fn retry_delay(&self, attempt: u32) -> Duration {
        // exponential backoff with a small, bounded cap
        let factor = 2_u64.saturating_pow(attempt.min(6));
        let delay_ms = self.cfg.retry_base_ms.saturating_mul(factor).min(5_000);
        Duration::from_millis(delay_ms)
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

fn is_retryable_transport_error(err: &reqwest::Error) -> bool {
    err.is_timeout() || err.is_connect() || err.is_request()
}

fn read_auth_from_env() -> Option<AuthHeader> {
    if let Ok(raw_header) = std::env::var("OPENCLAW_AUTH_HEADER") {
        let raw = raw_header.trim();
        if raw.is_empty() {
            return None;
        }

        let mut parts = raw.splitn(2, ':');
        if let (Some(name), Some(value)) = (parts.next(), parts.next()) {
            return Some(AuthHeader {
                name: name.trim().to_string(),
                value: value.trim().to_string(),
            });
        }

        return Some(AuthHeader {
            name: "Authorization".to_string(),
            value: raw.to_string(),
        });
    }

    std::env::var("OPENCLAW_TOKEN")
        .ok()
        .map(|token| token.trim().to_string())
        .filter(|token| !token.is_empty())
        .map(|token| AuthHeader {
            name: "Authorization".to_string(),
            value: format!("Bearer {token}"),
        })
}

fn read_env_u32(name: &str, default: u32) -> u32 {
    std::env::var(name)
        .ok()
        .and_then(|v| v.trim().parse::<u32>().ok())
        .unwrap_or(default)
}

fn read_env_u64(name: &str, default: u64) -> u64 {
    std::env::var(name)
        .ok()
        .and_then(|v| v.trim().parse::<u64>().ok())
        .unwrap_or(default)
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

#[async_trait]
impl ReasoningProvider for OpenClawProvider {
    async fn solve(&self, input: &str) -> Result<ProblemResponse> {
        let body = ChatRequest {
            model: self.cfg.model.clone(),
            temperature: 0.2,
            messages: vec![
                ChatMessage {
                    role: "system".to_string(),
                    content: SYSTEM_PROMPT.to_string(),
                },
                ChatMessage {
                    role: "user".to_string(),
                    content: input.to_string(),
                },
            ],
        };

        let payload = self.send_with_retry(&body).await?;
        let content = payload
            .choices
            .first()
            .map(|c| c.message.content.clone())
            .ok_or_else(|| anyhow!("no choices in chat response"))?;

        parse_problem_response(&content)
    }
}

fn parse_problem_response(raw: &str) -> Result<ProblemResponse> {
    let parsed = if let Ok(parsed) = serde_json::from_str::<ProblemResponse>(raw) {
        parsed
    } else {
        // Fallback: try to extract first JSON object from markdown fences or extra text.
        let start = raw
            .find('{')
            .ok_or_else(|| anyhow!("no JSON object in response"))?;
        let end = raw
            .rfind('}')
            .ok_or_else(|| anyhow!("no JSON object in response"))?;
        if end <= start {
            return Err(anyhow!("malformed JSON object bounds"));
        }
        let slice = &raw[start..=end];
        serde_json::from_str(slice)
            .with_context(|| format!("failed parsing extracted JSON: {slice}"))?
    };

    let parsed = parsed.normalized();
    parsed.validate_quality().map_err(|issues| {
        anyhow!(
            "response quality check failed (score {}): {issues}",
            parsed.quality_score()
        )
    })?;

    Ok(parsed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_markdown_wrapped_json() {
        let raw = r#"Sure, here you go:
```json
{
  "abstract_shape": "loop",
  "cross_domain_matches": ["a", "b", "c"],
  "mapping": "m",
  "synthesis": "s"
}
```
"#;

        let parsed = parse_problem_response(raw).expect("should parse");
        assert_eq!(parsed.abstract_shape, "loop");
        assert_eq!(parsed.cross_domain_matches.len(), 3);
    }

    #[test]
    fn quality_rejects_too_few_matches() {
        let raw = r#"{
  "abstract_shape": "shape",
  "cross_domain_matches": ["only one", "only two"],
  "mapping": "map",
  "synthesis": "syn"
}"#;

        let err = parse_problem_response(raw).expect_err("should fail quality gate");
        assert!(
            err.to_string()
                .contains("cross_domain_matches must contain at least 3 items"),
            "unexpected error: {err:#}"
        );
    }

    #[test]
    fn retryable_statuses_are_expected() {
        assert!(is_retryable_status(StatusCode::TOO_MANY_REQUESTS));
        assert!(is_retryable_status(StatusCode::INTERNAL_SERVER_ERROR));
        assert!(!is_retryable_status(StatusCode::UNAUTHORIZED));
    }
}
