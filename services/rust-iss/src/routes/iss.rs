use axum::{Router, routing::get};

pub fn router() -> Router {
    Router::new().route("/trend", get(|| async { "ISS trend" }))
}
