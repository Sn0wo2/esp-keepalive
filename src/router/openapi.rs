use utoipa::{OpenApi, ToSchema, IntoParams};
use serde::Serialize;
use axum::{routing::get, Router, Json, response::Html};

#[derive(ToSchema)]
pub struct Device {
    pub device_id: String,
    pub battery: Option<f32>,
    pub rssi: Option<i32>,
    pub last_seen: String, 
}

#[derive(ToSchema, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub msg: Option<String>,
}

#[derive(ToSchema)]
pub struct PostDeviceRequest {
    pub device_id: String,
    pub battery: Option<f32>,
    pub rssi: Option<i32>,
}

#[derive(IntoParams, serde::Deserialize)]
pub struct GetDeviceQuery {
    pub device_id: String,
}

#[derive(OpenApi)]
#[openapi(
    paths(device_get, device_post),
    components(schemas(Device, PostDeviceRequest, ApiResponse<Device>)),
    tags((name = "device", description = "设备管理接口"))
)]
pub struct ApiDoc;

#[utoipa::path(
    get, path = "/v1/device", params(GetDeviceQuery),
    responses((status = 200, description = "获取详情", body = ApiResponse<Device>)),
    tag = "device"
)]
fn device_get() {}

#[utoipa::path(
    post, path = "/v1/device", request_body = PostDeviceRequest,
    responses((status = 200, description = "更新设备", body = ApiResponse<Device>)),
    tag = "device"
)]
fn device_post() {}

pub fn register(router: Router<crate::db::Database>) -> Router<crate::db::Database> {
    router
        .route("/api-docs/openapi.json", get(|| async { Json(ApiDoc::openapi()) }))
        .route("/swagger-ui", get(|| async {
            Html(format!(r#"
                <!DOCTYPE html>
                <html lang="en">
                <head>
                    <meta charset="utf-8" />
                    <meta name="viewport" content="width=device-width, initial-scale=1" />
                    <title>Swagger UI</title>
                    <link rel="stylesheet" href="https://unpkg.com/swagger-ui-dist@5/swagger-ui.css" />
                </head>
                <body>
                    <div id="swagger-ui"></div>
                    <script src="https://unpkg.com/swagger-ui-dist@5/swagger-ui-bundle.js"></script>
                    <script>
                        window.onload = () => {{
                            window.ui = SwaggerUIBundle({{
                                url: '/api-docs/openapi.json',
                                dom_id: '#swagger-ui',
                            }});
                        }};
                    </script>
                </body>
                </html>
            "#))
        }))
}
