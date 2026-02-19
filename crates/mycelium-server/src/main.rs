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
    state.engine.run(&req.input).await.map(Json).map_err(|err| {
        tracing::error!("solve failed: {err:#}");
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: format!("solve failed: {err}"),
            }),
        )
    })
}
