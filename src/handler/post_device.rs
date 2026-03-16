use crate::db;
use crate::response::IntoResponseExt;
use anyhow::anyhow;
use axum::extract::rejection::JsonRejection;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PostDeviceRequest {
    pub device_id: String,
    pub battery: Option<f32>,
    pub rssi: Option<i32>,
}

pub(crate) async fn handle(
    State(db): State<db::Database>,
    payload: Result<Json<PostDeviceRequest>, JsonRejection>,
) -> impl IntoResponse {
    let Json(payload) = match payload {
        Ok(payload) => payload,
        Err(err) => {
            return anyhow::anyhow!("Invalid JSON payload: {}", err).into_response();
        }
    };

    let result = db.upsert_device(
        &payload.device_id,
        payload.battery,
        payload.rssi,
    ).await.map_err(|e| anyhow!("Database error: {}", e));

    result.into_response()
}
