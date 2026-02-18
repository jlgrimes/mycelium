use axum::{extract::State, routing::post, Json, Router};
use isomorph_engine::Engine;
use isomorph_providers::StubProvider;
use isomorph_types::{ProblemRequest, ProblemResponse};
use std::{net::SocketAddr, sync::Arc};
use tracing::info;

#[derive(Clone)]
struct AppState {
    engine: Arc<Engine<StubProvider>>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().with_env_filter("info").init();

    let state = AppState {
        engine: Arc::new(Engine::new(StubProvider)),
    };

    let app = Router::new()
        .route("/solve", post(solve))
        .with_state(state);

    let addr: SocketAddr = "127.0.0.1:8787".parse()?;
    info!("mycelium server listening on {addr}");
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn solve(
    State(state): State<AppState>,
    Json(req): Json<ProblemRequest>,
) -> Result<Json<ProblemResponse>, axum::http::StatusCode> {
    state
        .engine
        .run(&req.input)
        .await
        .map(Json)
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)
}
