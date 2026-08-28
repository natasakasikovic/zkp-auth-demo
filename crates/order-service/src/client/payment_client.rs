use common::models::{CreateOrderRequest, CreatePaymentRequest, CreatePaymentResponse};
use uuid::Uuid;

use common::zkp_auth::ZKP_AUTH_HEADER;

use crate::{
    client::{build_zkp_auth_header, parse_internal_response},
    error::ServiceError,
    state::AppState,
};

pub async fn create_payment(
    state: &AppState,
    request: &CreateOrderRequest,
    order_id: Uuid,
    total_amount: u64,
) -> Result<CreatePaymentResponse, ServiceError> {
    let path = "/payments";
    let request_body = CreatePaymentRequest {
        order_id,
        amount: total_amount,
        payment_method: request.payment_method.clone(),
    };

    let (auth_header, body) =
        build_zkp_auth_header(state, "payment-service", "POST", path, &request_body)?;

    let response = state
        .http_client
        .post(format!("{}{}", state.payment_url, path))
        .header(ZKP_AUTH_HEADER, auth_header)
        .header("content-type", "application/json")
        .body(body)
        .send()
        .await
        .map_err(|error| ServiceError::bad_gateway(format!("payment service error: {error}")))?;

    parse_internal_response(response, "payment service").await
}
