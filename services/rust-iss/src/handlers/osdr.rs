use axum::{Router, routing::get};
use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::<AppState>::new()
}
