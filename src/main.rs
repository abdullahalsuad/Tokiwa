use axum::{routing::get, Json, Router};
use serde_json::{json, Value};
use std::net::SocketAddr;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

mod api;
mod config;
mod db;
mod watcher;

// ---------------------------------------------------------------------------
// Health check handler
// ---------------------------------------------------------------------------

async fn health() -> Json<Value> {
    Json(json!({
        "status":  "ok",
        "service": "tokiwa-backend",
        "version": env!("CARGO_PKG_VERSION"),
    }))
}

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

#[tokio::main]
async fn main() {
    // ── Tracing ──────────────────────────────────────────────────────────────
    // Respect RUST_LOG if set; otherwise default to info-level for our crate.
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("tokiwa_backend=info"));

    tracing_subscriber::registry()
        .with(env_filter)
        .with(tracing_subscriber::fmt::layer())
        .init();

    // ── Config ───────────────────────────────────────────────────────────────
    let cfg = config::Config::load();

    tracing::info!(
        host = %cfg.server.host,
        port = cfg.server.port,
        db   = %cfg.database.path,
        "Tokiwa backend starting"
    );

    // ── CORS ─────────────────────────────────────────────────────────────────
    // Permissive — this server is localhost-only, single-user, no auth.
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // ── Router ───────────────────────────────────────────────────────────────
    let app = Router::new()
        .route("/api/health", get(health))
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    // ── Bind & serve ─────────────────────────────────────────────────────────
    let addr: SocketAddr = format!("{}:{}", cfg.server.host, cfg.server.port)
        .parse()
        .expect("Invalid server address in config");

    tracing::info!(%addr, "Listening — open http://{addr}/api/health to verify");

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind TCP listener");

    axum::serve(listener, app)
        .await
        .expect("Server error");
}
