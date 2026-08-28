use common::models::{CreateOrderRequest, ReserveStockRequest, ReserveStockResponse};
use uuid::Uuid;

use common::zkp_auth::ZKP_AUTH_HEADER;

use crate::{
    client::{build_zkp_auth_header, parse_internal_response},
    error::ServiceError,
    state::AppState,
};

pub async fn reserve_stock(
    state: &AppState,
    request: &CreateOrderRequest,
    order_id: Uuid,
) -> Result<ReserveStockResponse, ServiceError> {
    let path = "/reservations";
    let request_body = ReserveStockRequest {
        order_id,
        product_id: request.product_id.clone(),
        quantity: request.quantity,
    };
    let (auth_header, body) =
        build_zkp_auth_header(state, "warehouse-service", "POST", path, &request_body)?;

    let response = state
        .http_client
        .post(format!("{}{}", state.warehouse_url, path))
        .header(ZKP_AUTH_HEADER, auth_header)
        .header("content-type", "application/json")
        .body(body)
        .send()
        .await
        .map_err(|error| ServiceError::bad_gateway(format!("warehouse service error: {error}")))?;

    parse_internal_response(response, "warehouse service").await
}
