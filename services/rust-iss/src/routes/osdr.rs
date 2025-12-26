use axum::{Router, routing::get};
use crate::{handlers, AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(handlers::osdr::index))
}
