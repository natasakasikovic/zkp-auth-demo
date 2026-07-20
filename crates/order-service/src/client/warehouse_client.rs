use common::models::{CreateOrderRequest, ReserveStockRequest, ReserveStockResponse};
use uuid::Uuid;

use crate::{client::parse_internal_response, error::ServiceError, state::AppState};

pub async fn reserve_stock(
    state: &AppState,
    request: &CreateOrderRequest,
    order_id: Uuid,
) -> Result<ReserveStockResponse, ServiceError> {
    let response = state
        .http_client
        .post(format!("{}/reservations", state.warehouse_url))
        .json(&ReserveStockRequest {
            order_id,
            product_id: request.product_id.clone(),
            quantity: request.quantity,
        })
        .send()
        .await
        .map_err(|error| ServiceError::bad_gateway(format!("warehouse service error: {error}")))?;

    parse_internal_response(response, "warehouse service").await
}
