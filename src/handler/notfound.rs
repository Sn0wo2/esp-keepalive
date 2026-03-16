use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;
use tracing::info;

pub async fn init() -> impl IntoResponse {
    info!("Not found handler called");
    (StatusCode::NOT_FOUND, Json(json!({
        "success": false,
        "msg": "not found"
    })))
}
