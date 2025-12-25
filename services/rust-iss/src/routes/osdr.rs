use axum::{Router, routing::get};

pub fn router() -> Router {
    Router::new().route("/status", get(|| async { "OSDR ok" }))
}
