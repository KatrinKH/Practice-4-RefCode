use axum::{Json, extract::State};
use crate::{AppState, error::ApiError};

pub async fn index(
    State(_state): State<AppState>,
) -> Result<Json<serde_json::Value>, ApiError> {
    Ok(Json(serde_json::json!({
        "ok": true
    })))
}
