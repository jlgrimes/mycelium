use anyhow::{anyhow, bail, Context, Result};
use async_trait::async_trait;
use mycelium_core::ReasoningProvider;
use mycelium_types::ProblemResponse;
use reqwest::{header::RETRY_AFTER, Client, Response, StatusCode};
use serde::{Deserialize, Serialize};
use std::time::Duration;

pub mod config {
    use std::time::Duration;

    #[derive(Clone, Debug)]
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
}

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
    cfg: config::OpenClawConfig,
    auth_header: Option<AuthHeader>,
}

#[derive(Clone)]
struct AuthHeader {
    name: String,
    value: String,
}

impl OpenClawProvider {
    pub fn new(cfg: config::OpenClawConfig) -> Result<Self> {
        if cfg.base_url.trim().is_empty() {
            bail!("openclaw base URL is empty");
        }

        let client = Client::builder()
            .timeout(cfg.request_timeout)
            .connect_timeout(cfg.connect_timeout)
            .build()
            .context("failed to construct reqwest client")?;

        Ok(Self {
            client,
            auth_header: resolve_auth_header(&cfg),
            cfg,
        })
    }

    pub fn from_env() -> Self {
        Self::new(config::OpenClawConfig::from_env())
            .expect("OpenClawConfig::from_env should yield a valid provider")
    }

    async fn send_with_retry(&self, body: &ChatRequest) -> Result<ChatResponse> {
        let total_attempts = self.cfg.max_retries + 1;
        let mut last_error: Option<anyhow::Error> = None;

        for attempt in 0..total_attempts {
            let mut req = self.client.post(&self.cfg.base_url).json(body);
            if let Some(auth) = &self.auth_header {
                req = req.header(&auth.name, &auth.value);
            }

            match req.send().await {
                Ok(resp) => {
                    let status = resp.status();
                    let retry_after = parse_retry_after(&resp);
                    let text = resp.text().await.unwrap_or_default();

                    if status.is_success() {
                        return serde_json::from_str::<ChatResponse>(&text)
                            .with_context(|| format!("invalid chat response body: {text}"));
                    }

                    let err = http_error_message(status, attempt + 1, total_attempts, &text, retry_after);

                    if is_retryable_status(status) && attempt < self.cfg.max_retries {
                        last_error = Some(err);
                        let delay = retry_after.unwrap_or_else(|| self.retry_delay(attempt));
                        tokio::time::sleep(delay).await;
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

        Err(last_error.unwrap_or_else(|| anyhow!("openclaw retries exhausted")))
    }

    fn retry_delay(&self, attempt: u32) -> Duration {
        let factor = 2_u32.saturating_pow(attempt.min(10));
        let millis = self
            .cfg
            .retry_base_delay
            .as_millis()
            .saturating_mul(factor as u128)
            .min(self.cfg.retry_max_delay.as_millis()) as u64;
        Duration::from_millis(millis)
    }
}

fn resolve_auth_header(cfg: &config::OpenClawConfig) -> Option<AuthHeader> {
    if let Ok(raw) = std::env::var("OPENCLAW_AUTH_HEADER") {
        if let Some(parsed) = parse_auth_header(&raw) {
            return Some(parsed);
        }
    }

    cfg.token
        .as_ref()
        .map(|token| token.trim())
        .filter(|token| !token.is_empty())
        .map(|token| AuthHeader {
            name: "Authorization".to_string(),
            value: format!("Bearer {token}"),
        })
}

fn parse_auth_header(raw: &str) -> Option<AuthHeader> {
    let raw = raw.trim();
    if raw.is_empty() {
        return None;
    }

    if let Some((name, value)) = raw.split_once(':') {
        let name = name.trim();
        let value = value.trim();
        if name.is_empty() || value.is_empty() {
            return None;
        }
        return Some(AuthHeader {
            name: name.to_string(),
            value: value.to_string(),
        });
    }

    Some(AuthHeader {
        name: "Authorization".to_string(),
        value: raw.to_string(),
    })
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

fn parse_retry_after(resp: &Response) -> Option<Duration> {
    let value = resp.headers().get(RETRY_AFTER)?.to_str().ok()?.trim();
    let seconds = value.parse::<u64>().ok()?;
    Some(Duration::from_secs(seconds))
}

fn http_error_message(
    status: StatusCode,
    attempt: u32,
    total_attempts: u32,
    body: &str,
    retry_after: Option<Duration>,
) -> anyhow::Error {
    let mut details = format!(
        "openclaw HTTP {} on attempt {}/{}: {}",
        status, attempt, total_attempts, body
    );

    if matches!(status, StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN) {
        details.push_str(
            " | auth failed: verify OPENCLAW_TOKEN or OPENCLAW_AUTH_HEADER and endpoint permissions",
        );
    }

    if status == StatusCode::TOO_MANY_REQUESTS {
        if let Some(delay) = retry_after {
            details.push_str(&format!(
                " | rate limited: honoring Retry-After={}s",
                delay.as_secs()
            ));
        } else {
            details.push_str(" | rate limited: no Retry-After header provided");
        }
    }

    anyhow!(details)
}

fn is_retryable_transport_error(err: &reqwest::Error) -> bool {
    err.is_timeout() || err.is_connect() || err.is_request()
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
            temperature: self.cfg.temperature,
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
            .ok_or_else(|| anyhow!("empty response: no choices in chat response"))?;

        parse_problem_response(&content)
    }
}

fn parse_problem_response(raw: &str) -> Result<ProblemResponse> {
    let parsed = if let Ok(parsed) = serde_json::from_str::<ProblemResponse>(raw) {
        parsed
    } else {
        let start = raw
            .find('{')
            .ok_or_else(|| anyhow!("JSON extraction failed: no JSON object in response"))?;
        let end = raw
            .rfind('}')
            .ok_or_else(|| anyhow!("JSON extraction failed: no JSON object in response"))?;
        if end <= start {
            return Err(anyhow!(
                "JSON extraction failed: malformed JSON object bounds"
            ));
        }
        let slice = &raw[start..=end];
        serde_json::from_str(slice).with_context(|| {
            format!("JSON extraction failed: could not parse extracted object: {slice}")
        })?
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

    #[test]
    fn auth_error_has_helpful_hint() {
        let err = http_error_message(StatusCode::UNAUTHORIZED, 1, 3, "bad token", None);
        let msg = err.to_string();
        assert!(msg.contains("auth failed"));
        assert!(msg.contains("OPENCLAW_TOKEN"));
    }

    #[test]
    fn parses_explicit_auth_header() {
        let parsed = parse_auth_header("X-API-Key: secret").expect("should parse");
        assert_eq!(parsed.name, "X-API-Key");
        assert_eq!(parsed.value, "secret");
    }

    #[test]
    fn parses_raw_auth_header_as_authorization() {
        let parsed = parse_auth_header("Bearer abc").expect("should parse");
        assert_eq!(parsed.name, "Authorization");
        assert_eq!(parsed.value, "Bearer abc");
    }
}
