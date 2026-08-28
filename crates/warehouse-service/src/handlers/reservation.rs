use axum::{Json, extract::State, http::HeaderMap};
use common::{
    models::{ReserveStockRequest, ReserveStockResponse},
    zkp_auth::{ZKP_AUTH_HEADER, decode_auth_proof},
};
use uuid::Uuid;
use zkp_auth_schnorr::{AuthVerificationError, current_unix_timestamp, verify_auth_proof};

use crate::{domain::inventory::InventoryReservationError, error::ServiceError, state::AppState};

pub async fn reserve_stock(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<ReserveStockRequest>,
) -> Result<Json<ReserveStockResponse>, ServiceError> {
    verify_order_service_auth(&state, &headers, &request)?;

    let mut inventory = state
        .inventory
        .lock()
        .map_err(|_| ServiceError::internal("inventory state is unavailable"))?;

    let item = inventory.get_mut(&request.product_id).ok_or_else(|| {
        ServiceError::not_found(format!("product '{}' does not exist", request.product_id))
    })?;

    item.reserve(request.quantity)
        .map_err(|error| map_reservation_error(error, &request.product_id))?;

    Ok(Json(ReserveStockResponse {
        reservation_id: Uuid::new_v4(),
        product_id: item.product_id.clone(),
        reserved_quantity: request.quantity,
        remaining_quantity: item.available_quantity,
    }))
}

fn verify_order_service_auth(
    state: &AppState,
    headers: &HeaderMap,
    request: &ReserveStockRequest,
) -> Result<(), ServiceError> {
    let header_value = headers
        .get(ZKP_AUTH_HEADER)
        .ok_or_else(|| ServiceError::unauthorized("missing ZKP authentication proof"))?
        .to_str()
        .map_err(|_| ServiceError::unauthorized("invalid ZKP authentication header"))?;
    let proof = decode_auth_proof(header_value)
        .map_err(|_| ServiceError::unauthorized("invalid ZKP authentication proof format"))?;
    let body = serde_json::to_vec(request)
        .map_err(|error| ServiceError::internal(format!("failed to serialize request: {error}")))?;
    let mut replay_protector = state
        .replay_protector
        .lock()
        .map_err(|_| ServiceError::internal("replay protection state is unavailable"))?;

    verify_auth_proof(
        &proof,
        &state.order_service_public_key,
        "warehouse-service",
        "POST",
        "/reservations",
        &body,
        current_unix_timestamp(),
        &mut replay_protector,
    )
    .map_err(map_auth_error)
}

fn map_auth_error(error: AuthVerificationError) -> ServiceError {
    ServiceError::unauthorized(format!("ZKP authentication failed: {error}"))
}

fn map_reservation_error(error: InventoryReservationError, product_id: &str) -> ServiceError {
    match error {
        InventoryReservationError::InvalidQuantity => {
            ServiceError::bad_request("quantity must be greater than zero")
        }
        InventoryReservationError::InsufficientStock {
            requested_quantity,
            available_quantity,
        } => ServiceError::bad_request(format!(
            "not enough stock for '{product_id}': requested {requested_quantity}, available {available_quantity}"
        )),
    }
}
