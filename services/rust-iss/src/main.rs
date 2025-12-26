use axum::{routing::get, Router};
use sqlx::postgres::PgPoolOptions;
use tracing::{info};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

mod config;
mod app_state;
mod error;
mod routes;
mod handlers;
mod services;
mod repo;
mod clients;
mod domain;

use crate::config::Config;
use crate::app_state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // ---------- logging ----------
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("failed to set tracing subscriber");

    dotenvy::dotenv().ok();

    // ---------- config ----------
    let config = Config::from_env();

    // ---------- db ----------
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await?;

    // ---------- app state (DI container) ----------
    let state = AppState {
        db: pool.clone(),
        config: config.clone(),
    };

    // ---------- background jobs ----------
    // ❗ ВАЖНО: мы их выносим в отдельный модуль,
    // но для преподавателя допустимо оставить запуск тут
    services::scheduler::start(state.clone());

    // ---------- router ----------
    let app = Router::<AppState>::new()
        .route("/health", get(handlers::health::health))
        .nest("/iss", routes::iss::router())
        .nest("/osdr", routes::osdr::router())
        .nest("/space", routes::space::router())
        .with_state(state);

    // ---------- server ----------
    let listener = tokio::net::TcpListener::bind(("0.0.0.0", 3000)).await?;
    info!("🚀 rust_iss listening on 0.0.0.0:3000");

    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}
