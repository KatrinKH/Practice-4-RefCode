use axum::{extract::State, Json};
use crate::{AppState, error::ApiError};

pub async fn fetch_iss(
    State(_state): State<AppState>,
) -> Result<Json<serde_json::Value>, ApiError> {
    Ok(Json(serde_json::json!({
        "ok": true
    })))
}
