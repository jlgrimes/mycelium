mod debug_contract;

use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use debug_contract::DebugContractValidator;
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
        .route("/debug/validate", post(validate_debug_contract))
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

async fn validate_debug_contract(
    Json(resp): Json<ProblemResponse>,
) -> Json<debug_contract::ContractValidation> {
    Json(DebugContractValidator::validate(&resp))
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
    let enforced = DebugContractValidator::enforce(resp, concise);
    
    // Validate the enforced response
    let validation = DebugContractValidator::validate(&enforced);
    if !validation.valid {
        tracing::warn!("Debug contract validation failed: {:?}", validation.issues);
    }
    
    Ok(Json(enforced))
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

// Debug contract functions moved to debug_contract module

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
        let out = DebugContractValidator::enforce(base_response(), false);
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
        let out = DebugContractValidator::enforce(resp, true);
        assert_eq!(out.cross_domain_matches.len(), 3);
    }

    #[test]
    fn validator_middleware_validates_responses() {
        let resp = base_response();
        let validation = DebugContractValidator::validate(&resp);
        assert!(!validation.valid);
        assert!(!validation.issues.is_empty());
    }
}
