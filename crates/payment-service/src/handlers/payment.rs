use axum::{Json, extract::State, http::HeaderMap};
use common::{
    models::{CreatePaymentRequest, CreatePaymentResponse, PaymentStatus},
    zkp_auth::{ZKP_AUTH_HEADER, decode_auth_proof},
};
use uuid::Uuid;
use zkp_auth_schnorr::{AuthVerificationError, current_unix_timestamp, verify_auth_proof};

use crate::{domain::payment::validate_payment_request, error::ServiceError, state::AppState};

pub async fn create_payment(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<CreatePaymentRequest>,
) -> Result<Json<CreatePaymentResponse>, ServiceError> {
    verify_order_service_auth(&state, &headers, &request)?;
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

fn verify_order_service_auth(
    state: &AppState,
    headers: &HeaderMap,
    request: &CreatePaymentRequest,
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
        "payment-service",
        "POST",
        "/payments",
        &body,
        current_unix_timestamp(),
        &mut replay_protector,
    )
    .map_err(map_auth_error)
}

fn map_auth_error(error: AuthVerificationError) -> ServiceError {
    ServiceError::unauthorized(format!("ZKP authentication failed: {error}"))
}
