use axum::{Json, response::IntoResponse};
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiErrorBody {
    pub ok: bool,
    pub error: ApiErrorData,
}

#[derive(Serialize)]
pub struct ApiErrorData {
    pub code: String,
    pub message: String,
    pub trace_id: String,
}

#[derive(Debug)]
pub enum ApiError {
    Upstream(String),
    Db(String),
    Internal(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let body = ApiErrorBody {
            ok: false,
            error: ApiErrorData {
                code: "UPSTREAM_ERROR".into(),
                message: format!("{:?}", self),
                trace_id: uuid::Uuid::new_v4().to_string(),
            },
        };

        Json(body).into_response()
    }
}
