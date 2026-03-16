use crate::db;
use crate::response::AnyhowErrorExt;
use anyhow::anyhow;
use axum::extract::{Query, State};
use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;
use serde_json::json;

#[derive(Debug, Deserialize)]
pub struct GetDeviceQuery {
    pub device_id: String,
}

pub(crate) async fn handle(
    State(db): State<db::Database>,
    Query(query): Query<GetDeviceQuery>,
) -> impl IntoResponse {
    let device = db.get_device(&query.device_id).await;

    match device {
        Ok(Some(device)) => {
            Json(json!({
                "success": true,
                "data": device
            })).into_response()
        }
        Ok(None) => {
            anyhow!("invalid device_id").into_response()
        }
        Err(e) => {
            anyhow!("Database error: {}", e).into_response()
        }
    }
}
