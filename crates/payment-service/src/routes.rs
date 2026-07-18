use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    handlers::{health::health, payment::create_payment},
    state::AppState,
};

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/payments", post(create_payment))
        .with_state(state)
}
