// src/main.rs
// Application entry point

use axum::{Json, Router, http::StatusCode, response::IntoResponse, routing::get};
use serde_json::json;
use std::net::SocketAddr;
use tower_http::LatencyUnit;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::Level;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod db;
mod error;
mod handlers;
mod models;
mod routes;

#[cfg(test)]
mod tests;

use db::pool::create_pool;
use db::tasks::TaskRepository;
use handlers::tasks::AppState;
use routes::tasks::task_routes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing for structured logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "task_api=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load environment variables from .env file
    dotenvy::dotenv().ok();

    // Get database URL from environment
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Create database connection pool
    tracing::info!("Connecting to database...");
    let pool = create_pool(&database_url).await?;
    tracing::info!("Database connection established");

    // Run migrations
    tracing::info!("Running database migrations...");
    sqlx::migrate!("./migrations").run(&pool).await?;
    tracing::info!("Migrations complete");

    // Create application state
    let state = AppState {
        task_repo: TaskRepository::new(pool),
    };

    let app = create_app(state);

    // Start the server
    let port: u16 = match std::env::var("PORT") {
        Ok(val) if !val.trim().is_empty() => val.parse().expect("PORT must be a valid port number"),
        _ => 3000,
    };
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("Starting server on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

/// Create the application router with all routes and middlewares
pub fn create_app(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/health", get(health_check))
        .merge(task_routes())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(
                    DefaultMakeSpan::new()
                        .level(Level::INFO)
                        .include_headers(true),
                )
                .on_response(
                    DefaultOnResponse::new()
                        .level(Level::INFO)
                        .latency_unit(LatencyUnit::Millis),
                ),
        )
        .layer(cors)
        .with_state(state)
}

/// Health check endpoint to verify service availability
pub async fn health_check() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(json!({
            "status": "ok",
            "timestamp": chrono::Utc::now().to_rfc3339()
        })),
    )
}

