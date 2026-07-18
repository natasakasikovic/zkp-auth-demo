use axum::{Json, extract::State};
use common::models::{CreatePaymentRequest, CreatePaymentResponse, PaymentStatus};
use uuid::Uuid;

use crate::{domain::payment::validate_payment_request, error::ServiceError, state::AppState};

pub async fn create_payment(
    State(state): State<AppState>,
    Json(request): Json<CreatePaymentRequest>,
) -> Result<Json<CreatePaymentResponse>, ServiceError> {
    validate_payment_request(&request)?;

    // Payment for now is simulated.
    let response = CreatePaymentResponse {
        payment_id: Uuid::new_v4(),
        order_id: request.order_id,
        status: PaymentStatus::Approved,
        amount: request.amount,
    };

    state
        .payments
        .lock()
        .map_err(|_| ServiceError::internal("payment state is unavailable"))?
        .insert(response.payment_id, response.clone());

    Ok(Json(response))
}
