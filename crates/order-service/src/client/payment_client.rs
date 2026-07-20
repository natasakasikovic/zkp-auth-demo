use common::models::{CreateOrderRequest, CreatePaymentRequest, CreatePaymentResponse};
use uuid::Uuid;

use crate::{client::parse_internal_response, error::ServiceError, state::AppState};

pub async fn create_payment(
    state: &AppState,
    request: &CreateOrderRequest,
    order_id: Uuid,
    total_amount: u64,
) -> Result<CreatePaymentResponse, ServiceError> {
    let response = state
        .http_client
        .post(format!("{}/payments", state.payment_url))
        .json(&CreatePaymentRequest {
            order_id,
            amount: total_amount,
            payment_method: request.payment_method.clone(),
        })
        .send()
        .await
        .map_err(|error| ServiceError::bad_gateway(format!("payment service error: {error}")))?;

    parse_internal_response(response, "payment service").await
}
