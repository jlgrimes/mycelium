use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use mycelium_core::ReasoningProvider;
use mycelium_types::ProblemResponse;
use reqwest::Client;
use serde::{Deserialize, Serialize};

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
    base_url: String,
    token: Option<String>,
    model: String,
}

impl OpenClawProvider {
    pub fn from_env() -> Self {
        Self {
            client: Client::new(),
            base_url: std::env::var("OPENCLAW_BASE_URL")
                .unwrap_or_else(|_| "http://127.0.0.1:18789/v1/chat/completions".to_string()),
            token: std::env::var("OPENCLAW_TOKEN").ok(),
            model: std::env::var("MYCELIUM_MODEL").unwrap_or_else(|_| "sonnet".to_string()),
        }
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

#[async_trait]
impl ReasoningProvider for OpenClawProvider {
    async fn solve(&self, input: &str) -> Result<ProblemResponse> {
        let body = ChatRequest {
            model: self.model.clone(),
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

        let mut req = self.client.post(&self.base_url).json(&body);
        if let Some(token) = &self.token {
            req = req.bearer_auth(token);
        }

        let resp = req.send().await.context("openclaw request failed")?;
        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            return Err(anyhow!("openclaw error {}: {}", status, text));
        }

        let payload: ChatResponse = resp.json().await.context("invalid chat response")?;
        let content = payload
            .choices
            .first()
            .map(|c| c.message.content.clone())
            .ok_or_else(|| anyhow!("no choices in chat response"))?;

        parse_problem_response(&content)
    }
}

fn parse_problem_response(raw: &str) -> Result<ProblemResponse> {
    if let Ok(parsed) = serde_json::from_str::<ProblemResponse>(raw) {
        return Ok(parsed);
    }

    // Fallback: try to extract first JSON object from markdown fences or extra text.
    let start = raw.find('{').ok_or_else(|| anyhow!("no JSON object in response"))?;
    let end = raw.rfind('}').ok_or_else(|| anyhow!("no JSON object in response"))?;
    if end <= start {
        return Err(anyhow!("malformed JSON object bounds"));
    }
    let slice = &raw[start..=end];
    let parsed: ProblemResponse = serde_json::from_str(slice)
        .with_context(|| format!("failed parsing extracted JSON: {slice}"))?;
    Ok(parsed)
}
