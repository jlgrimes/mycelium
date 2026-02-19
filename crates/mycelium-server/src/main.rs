use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use mycelium_core::ReasoningProvider;
use mycelium_engine::Engine;
use mycelium_providers::StubProvider;
use mycelium_types::{ProblemRequest, ProblemResponse};
use openclaw_adapter::OpenClawProvider;
use serde::Serialize;
use std::{net::SocketAddr, sync::Arc};
use tracing::info;

#[derive(Clone)]
struct AppState {
    engine: Arc<Engine>,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

fn build_provider() -> Arc<dyn ReasoningProvider> {
    if std::env::var("MYCELIUM_USE_STUB").ok().as_deref() == Some("1") {
        Arc::new(StubProvider)
    } else {
        Arc::new(OpenClawProvider::from_env())
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().with_env_filter("info").init();

    let provider = build_provider();

    let state = AppState {
        engine: Arc::new(Engine::new(provider)),
    };

    let app = Router::new()
        .route("/health", get(health))
        .route("/solve", post(solve))
        .route("/solve/debug", post(solve_debug))
        .route("/solve/debug/concise", post(solve_debug_concise))
        .with_state(state);

    let addr: SocketAddr = std::env::var("MYCELIUM_BIND")
        .unwrap_or_else(|_| "127.0.0.1:8787".to_string())
        .parse()?;
    info!("mycelium server listening on {addr}");
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn health() -> &'static str {
    "ok"
}

async fn solve(
    State(state): State<AppState>,
    Json(req): Json<ProblemRequest>,
) -> Result<Json<ProblemResponse>, (axum::http::StatusCode, Json<ErrorResponse>)> {
    run_with_input(&state, req.input, "solve").await
}

async fn solve_debug(
    State(state): State<AppState>,
    Json(req): Json<ProblemRequest>,
) -> Result<Json<ProblemResponse>, (axum::http::StatusCode, Json<ErrorResponse>)> {
    run_debug_route(&state, req.input, false).await
}

async fn solve_debug_concise(
    State(state): State<AppState>,
    Json(req): Json<ProblemRequest>,
) -> Result<Json<ProblemResponse>, (axum::http::StatusCode, Json<ErrorResponse>)> {
    run_debug_route(&state, req.input, true).await
}

async fn run_debug_route(
    state: &AppState,
    input: String,
    concise: bool,
) -> Result<Json<ProblemResponse>, (axum::http::StatusCode, Json<ErrorResponse>)> {
    let debug_prompt = format!(
        "You are solving a software debugging problem. Use Loop Escape Protocol: detect loop risk, pivot to an isomorphic frame, map back to code, and include explicit verification steps.\n\nReturn synthesis with sections:\n- Pivot rationale\n- Mapping confidence\n- Fix steps\n- Verification steps\n- Fallback pivot\n\nProblem:\n{}",
        input
    );

    let Json(resp) = run_with_input(state, debug_prompt, "solve_debug").await?;
    Ok(Json(enforce_debug_contract(resp, concise)))
}

async fn run_with_input(
    state: &AppState,
    input: String,
    route: &str,
) -> Result<Json<ProblemResponse>, (axum::http::StatusCode, Json<ErrorResponse>)> {
    state.engine.run(&input).await.map(Json).map_err(|err| {
        tracing::error!("{route} failed: {err:#}");
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: format!("{route} failed: {err}"),
            }),
        )
    })
}

fn enforce_debug_contract(mut resp: ProblemResponse, concise: bool) -> ProblemResponse {
    let confidence = derive_mapping_confidence(&resp);

    let synthesize = if concise {
        format!(
            "SYNTHESIZE:\n- Pivot: shift to a non-repeating isomorphic frame.\n- Fix: {}\n- Verification: run a focused reproducer test with explicit pass/fail checks.\n- Fallback: pivot to the next frame if checks fail.",
            first_line(&resp.synthesis)
        )
    } else {
        format_detailed_synthesize(&resp.synthesis)
    };

    resp.abstract_shape = format!(
        "ABSTRACT:\n- {}",
        non_empty_or(&resp.abstract_shape, "Debug loop with uncertain root cause")
    );

    resp.cross_domain_matches = resp
        .cross_domain_matches
        .iter()
        .take(if concise { 3 } else { 5 })
        .map(|m| format!("SEARCH: {m}"))
        .collect();

    if resp.cross_domain_matches.len() < 3 {
        resp.cross_domain_matches
            .push("SEARCH: Compiler pass ordering as a loop-breaking analog".to_string());
        resp.cross_domain_matches
            .push("SEARCH: Incident response triage as a hypothesis isolation analog".to_string());
        resp.cross_domain_matches
            .push("SEARCH: Medical differential diagnosis as a verification analog".to_string());
        resp.cross_domain_matches.truncate(3);
    }

    resp.mapping = format!(
        "MAP:\n- {}\n- Mapping confidence: {confidence}",
        non_empty_or(
            &resp.mapping,
            "Map repeating failure symptom -> instrumentation point -> isolating test"
        )
    );

    resp.synthesis = synthesize;
    resp
}

fn format_detailed_synthesize(raw: &str) -> String {
    format!(
        "SYNTHESIZE:\nPivot rationale:\n- Shift to an isomorphic frame that avoids repeating the failed hypothesis.\n\nFix steps:\n- {}\n\nVerification steps:\n- Add/Run a focused test that reproduces the original failure.\n- Confirm one explicit pass condition and one explicit fail condition.\n\nFallback pivot:\n- If verification fails, pivot to the next closest isomorphic frame and avoid retrying the same failed fix pattern.",
        first_line(raw)
    )
}

fn non_empty_or<'a>(value: &'a str, fallback: &'a str) -> &'a str {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        fallback
    } else {
        trimmed
    }
}

fn first_line(value: &str) -> &str {
    value
        .lines()
        .find(|line| !line.trim().is_empty())
        .unwrap_or("Create one isolated hypothesis and verify it before broad changes")
}

fn derive_mapping_confidence(resp: &ProblemResponse) -> &'static str {
    let has_3_matches = resp.cross_domain_matches.len() >= 3;
    let has_mapping = !resp.mapping.trim().is_empty();

    match (has_3_matches, has_mapping) {
        (true, true) => "high",
        (true, false) | (false, true) => "medium",
        (false, false) => "low",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base_response() -> ProblemResponse {
        ProblemResponse {
            abstract_shape: "shape".to_string(),
            cross_domain_matches: vec!["a".to_string(), "b".to_string(), "c".to_string()],
            mapping: "x -> y".to_string(),
            synthesis: "Fix steps:\n- Do the thing".to_string(),
        }
    }

    #[test]
    fn debug_contract_has_required_stage_headers() {
        let out = enforce_debug_contract(base_response(), false);
        assert!(out.abstract_shape.starts_with("ABSTRACT:"));
        assert!(out
            .cross_domain_matches
            .iter()
            .all(|m| m.starts_with("SEARCH:")));
        assert!(out.mapping.starts_with("MAP:"));
        assert!(out.synthesis.starts_with("SYNTHESIZE:"));
        assert!(out.synthesis.to_lowercase().contains("verification"));
    }

    #[test]
    fn concise_mode_trims_to_three_search_items() {
        let mut resp = base_response();
        resp.cross_domain_matches.push("d".to_string());
        let out = enforce_debug_contract(resp, true);
        assert_eq!(out.cross_domain_matches.len(), 3);
    }

    #[test]
    fn confidence_low_when_missing_signals() {
        let mut resp = base_response();
        resp.cross_domain_matches.clear();
        resp.mapping.clear();
        assert_eq!(derive_mapping_confidence(&resp), "low");
    }
}
