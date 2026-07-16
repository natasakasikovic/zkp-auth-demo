use axum::{
    Json,
    extract::{Path, State},
};
use common::models::InventoryItemResponse;

use crate::{error::ServiceError, state::AppState};

pub async fn get_inventory_item(
    State(state): State<AppState>,
    Path(product_id): Path<String>,
) -> Result<Json<InventoryItemResponse>, ServiceError> {
    let inventory = state
        .inventory
        .lock()
        .map_err(|_| ServiceError::internal("inventory state is unavailable"))?;
    let available_quantity = inventory
        .get(&product_id)
        .map(|item| item.available_quantity)
        .ok_or_else(|| ServiceError::not_found(format!("product '{product_id}' does not exist")))?;

    Ok(Json(InventoryItemResponse {
        product_id,
        available_quantity,
    }))
}
