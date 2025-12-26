use axum::{Router, routing::get};
use crate::{AppState, handlers};

pub fn router() -> Router<AppState> {
    Router::<AppState>::new()
        .route("/", get(handlers::iss::fetch_iss))
}
