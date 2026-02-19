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
    let debug_prompt = format!(
        "You are solving a software debugging problem. Use Loop Escape Protocol: detect loop risk, pivot to an isomorphic frame, map back to code, and include explicit verification steps.\n\nReturn synthesis with sections:\n- Pivot rationale\n- Mapping confidence\n- Fix steps\n- Verification steps\n- Fallback pivot\n\nProblem:\n{}",
        req.input
    );

    let Json(resp) = run_with_input(&state, debug_prompt, "solve_debug").await?;
    Ok(Json(enforce_debug_contract(resp)))
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

fn enforce_debug_contract(mut resp: ProblemResponse) -> ProblemResponse {
    let synthesis_lower = resp.synthesis.to_lowercase();

    if !synthesis_lower.contains("pivot") {
        resp.synthesis = format!("Pivot rationale:\n- Shift to an isomorphic frame that avoids repeating the failed hypothesis.\n\n{}", resp.synthesis);
    }

    if !synthesis_lower.contains("mapping confidence") {
        let confidence = derive_mapping_confidence(&resp);
        resp.synthesis
            .push_str(&format!("\n\nMapping confidence:\n- {confidence}"));
    }

    if !synthesis_lower.contains("verification")
        && !synthesis_lower.contains("assert")
        && !synthesis_lower.contains("test")
    {
        resp.synthesis.push_str(
            "\n\nVerification steps:\n- Add/Run a focused test that reproduces the original failure.\n- Confirm one explicit pass condition and one explicit fail condition.",
        );
    }

    if !synthesis_lower.contains("fallback") {
        resp.synthesis.push_str(
            "\n\nFallback pivot:\n- If verification fails, pivot to the next closest isomorphic frame and avoid retrying the same failed fix pattern.",
        );
    }

    resp
}

fn derive_mapping_confidence(resp: &ProblemResponse) -> &'static str {
    let has_3_matches = resp.cross_domain_matches.len() >= 3;
    let has_mapping = !resp.mapping.trim().is_empty();

    match (has_3_matches, has_mapping) {
        (true, true) => "high — mapping has explicit structure and enough analog evidence.",
        (true, false) | (false, true) => {
            "medium — partly grounded, but one supporting signal is weak."
        }
        (false, false) => {
            "low — sparse analog evidence and mapping detail; verify before applying."
        }
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
    fn adds_mapping_confidence_section_when_missing() {
        let out = enforce_debug_contract(base_response());
        assert!(out.synthesis.to_lowercase().contains("mapping confidence"));
    }

    #[test]
    fn does_not_duplicate_mapping_confidence() {
        let mut resp = base_response();
        resp.synthesis.push_str("\n\nMapping confidence:\n- high");
        let out = enforce_debug_contract(resp);
        assert_eq!(
            out.synthesis
                .to_lowercase()
                .matches("mapping confidence")
                .count(),
            1
        );
    }

    #[test]
    fn confidence_low_when_missing_signals() {
        let mut resp = base_response();
        resp.cross_domain_matches.clear();
        resp.mapping.clear();
        assert!(derive_mapping_confidence(&resp).starts_with("low"));
    }
}
