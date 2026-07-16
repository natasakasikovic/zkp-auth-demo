use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    handlers::{health::health, inventory::get_inventory_item, reservation::reserve_stock},
    state::AppState,
};

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/inventory/:product_id", get(get_inventory_item))
        .route("/reservations", post(reserve_stock))
        .with_state(state)
}
