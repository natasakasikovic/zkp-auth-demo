use axum::Json;
use common::models::HealthResponse;

pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        service: "order-service",
        status: "ok",
    })
}
