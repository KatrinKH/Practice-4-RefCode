use axum::{Router, routing::get};

pub fn router() -> Router {
    Router::new().route("/info", get(|| async { "Space info" }))
}
