use axum::{Json, extract::State};
use common::models::{ReserveStockRequest, ReserveStockResponse};
use uuid::Uuid;

use crate::{domain::inventory::InventoryReservationError, error::ServiceError, state::AppState};

pub async fn reserve_stock(
    State(state): State<AppState>,
    Json(request): Json<ReserveStockRequest>,
) -> Result<Json<ReserveStockResponse>, ServiceError> {
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
