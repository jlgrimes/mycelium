use openclaw_adapter::config::OpenClawConfig;
use openclaw_adapter::OpenClawProvider;
use mycelium_core::ReasoningProvider;
use std::time::Duration;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn chat_response_body(content: &str) -> serde_json::Value {
    serde_json::json!({
        "choices": [{
            "message": {
                "role": "assistant",
                "content": content
            }
        }]
    })
}

fn valid_problem_json() -> String {
    serde_json::json!({
        "abstract_shape": "A recursive optimization problem",
        "cross_domain_matches": ["Genetic algorithms", "Neural pruning", "Supply chain optimization"],
        "mapping": "Map local optima to pruning decisions",
        "synthesis": "Apply iterative refinement with crossover strategies"
    })
    .to_string()
}

fn config_for(server: &MockServer) -> OpenClawConfig {
    OpenClawConfig {
        base_url: format!("{}/v1/chat/completions", server.uri()),
        token: Some("test-token".into()),
        model: "test-model".into(),
        temperature: 0.1,
        request_timeout: Duration::from_secs(5),
        connect_timeout: Duration::from_secs(2),
        max_retries: 2,
        retry_base_delay: Duration::from_millis(10),
        retry_max_delay: Duration::from_millis(100),
    }
}

#[tokio::test]
async fn happy_path_returns_parsed_response() {
    let server = MockServer::start().await;
    let body = chat_response_body(&valid_problem_json());

    Mock::given(method("POST"))
        .and(path("/v1/chat/completions"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&body))
        .expect(1)
        .mount(&server)
        .await;

    let provider = OpenClawProvider::new(config_for(&server)).unwrap();
    let resp = provider.solve("test input").await.unwrap();

    assert_eq!(resp.abstract_shape, "A recursive optimization problem");
    assert_eq!(resp.cross_domain_matches.len(), 3);
}

#[tokio::test]
async fn extracts_json_from_markdown_fenced_response() {
    let server = MockServer::start().await;
    let content = format!("Here's the result:\n```json\n{}\n```", valid_problem_json());
    let body = chat_response_body(&content);

    Mock::given(method("POST"))
        .and(path("/v1/chat/completions"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&body))
        .expect(1)
        .mount(&server)
        .await;

    let provider = OpenClawProvider::new(config_for(&server)).unwrap();
    let resp = provider.solve("test").await.unwrap();
    assert!(!resp.synthesis.is_empty());
}

#[tokio::test]
async fn retries_on_500_then_succeeds() {
    let server = MockServer::start().await;
    let body = chat_response_body(&valid_problem_json());

    // First request fails with 500
    Mock::given(method("POST"))
        .and(path("/v1/chat/completions"))
        .respond_with(ResponseTemplate::new(500).set_body_string("internal error"))
        .up_to_n_times(1)
        .expect(1)
        .mount(&server)
        .await;

    // Second request succeeds
    Mock::given(method("POST"))
        .and(path("/v1/chat/completions"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&body))
        .expect(1)
        .mount(&server)
        .await;

    let provider = OpenClawProvider::new(config_for(&server)).unwrap();
    let resp = provider.solve("test").await.unwrap();
    assert_eq!(resp.cross_domain_matches.len(), 3);
}

#[tokio::test]
async fn retries_exhausted_returns_error() {
    let server = MockServer::start().await;

    // All requests fail with 500
    Mock::given(method("POST"))
        .and(path("/v1/chat/completions"))
        .respond_with(ResponseTemplate::new(500).set_body_string("down"))
        .expect(3) // 1 initial + 2 retries
        .mount(&server)
        .await;

    let provider = OpenClawProvider::new(config_for(&server)).unwrap();
    let err = provider.solve("test").await.unwrap_err();
    let msg = format!("{err}");
    assert!(msg.contains("500") || msg.contains("retries exhausted"), "got: {msg}");
}

#[tokio::test]
async fn no_retry_on_400() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/chat/completions"))
        .respond_with(ResponseTemplate::new(400).set_body_string("bad request"))
        .expect(1) // should only be called once (no retries for 4xx)
        .mount(&server)
        .await;

    let provider = OpenClawProvider::new(config_for(&server)).unwrap();
    let err = provider.solve("test").await.unwrap_err();
    let msg = format!("{err}");
    assert!(msg.contains("400"), "got: {msg}");
}

#[tokio::test]
async fn rate_limit_429_is_retried() {
    let server = MockServer::start().await;
    let body = chat_response_body(&valid_problem_json());

    Mock::given(method("POST"))
        .and(path("/v1/chat/completions"))
        .respond_with(ResponseTemplate::new(429).set_body_string("rate limited"))
        .up_to_n_times(1)
        .expect(1)
        .mount(&server)
        .await;

    Mock::given(method("POST"))
        .and(path("/v1/chat/completions"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&body))
        .expect(1)
        .mount(&server)
        .await;

    let provider = OpenClawProvider::new(config_for(&server)).unwrap();
    let resp = provider.solve("test").await.unwrap();
    assert_eq!(resp.cross_domain_matches.len(), 3);
}

#[tokio::test]
async fn empty_choices_returns_error() {
    let server = MockServer::start().await;
    let body = serde_json::json!({ "choices": [] });

    Mock::given(method("POST"))
        .and(path("/v1/chat/completions"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&body))
        .expect(1)
        .mount(&server)
        .await;

    let provider = OpenClawProvider::new(config_for(&server)).unwrap();
    let err = provider.solve("test").await.unwrap_err();
    let msg = format!("{err}");
    assert!(msg.contains("empty response") || msg.contains("no choices"), "got: {msg}");
}

#[tokio::test]
async fn invalid_json_content_returns_extraction_error() {
    let server = MockServer::start().await;
    let body = chat_response_body("This is not JSON at all, just plain text.");

    Mock::given(method("POST"))
        .and(path("/v1/chat/completions"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&body))
        .expect(1)
        .mount(&server)
        .await;

    let provider = OpenClawProvider::new(config_for(&server)).unwrap();
    let err = provider.solve("test").await.unwrap_err();
    let msg = format!("{err}");
    assert!(msg.contains("JSON extraction"), "got: {msg}");
}

#[tokio::test]
async fn sends_bearer_token_header() {
    let server = MockServer::start().await;
    let body = chat_response_body(&valid_problem_json());

    Mock::given(method("POST"))
        .and(path("/v1/chat/completions"))
        .and(wiremock::matchers::header("Authorization", "Bearer test-token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&body))
        .expect(1)
        .mount(&server)
        .await;

    let provider = OpenClawProvider::new(config_for(&server)).unwrap();
    provider.solve("test").await.unwrap();
}
