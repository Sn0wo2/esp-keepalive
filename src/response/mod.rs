use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;
use tracing::error;

fn error_response(err: anyhow::Error) -> Response {
    error!("{:?}", err);

    let err_string = err.to_string().to_lowercase();
    let status = if err_string.contains("not found") {
        StatusCode::NOT_FOUND
    } else if err_string.contains("invalid") || err_string.contains("required") {
        StatusCode::BAD_REQUEST
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    };

    (
        status,
        Json(json!({
            "success": false,
            "msg": err.to_string(),
        }))
    ).into_response()
}

pub fn success_response<T: serde::Serialize>(data: T) -> Response {
    (
        StatusCode::OK,
        Json(json!({ "success": true, "data": data }))
    ).into_response()
}

impl<T> IntoResponseExt for anyhow::Result<T>
where
    T: serde::Serialize,
{
    fn into_response(self) -> Response {
        match self {
            Ok(data) => success_response(data),
            Err(err) => error_response(err),
        }
    }
}

pub trait IntoResponseExt {
    fn into_response(self) -> Response;
}

impl IntoResponseExt for anyhow::Error {
    fn into_response(self) -> Response {
        error_response(self)
    }
}