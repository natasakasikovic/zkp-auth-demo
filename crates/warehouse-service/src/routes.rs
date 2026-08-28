use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    handlers::{health::health, reservation::reserve_stock},
    state::AppState,
};

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/reservations", post(reserve_stock))
        .with_state(state)
}
