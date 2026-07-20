use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    handlers::{health::health, order::create_order},
    state::AppState,
};

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/orders", post(create_order))
        .with_state(state)
}
