pub mod payment_client;
pub mod warehouse_client;

use common::models::ErrorResponse;
use common::zkp_auth::{ORDER_SERVICE_ID, encode_auth_proof};
use rand_core::OsRng;
use serde::Serialize;
use zkp_auth_schnorr::{AuthContext, create_auth_proof, current_unix_timestamp};

use crate::{error::ServiceError, state::AppState};

pub fn build_zkp_auth_header<T: Serialize>(
    state: &AppState,
    audience: &str,
    method: &str,
    path: &str,
    request_body: &T,
) -> Result<(String, Vec<u8>), ServiceError> {
    let body = serde_json::to_vec(request_body)
        .map_err(|error| ServiceError::internal(format!("failed to serialize request: {error}")))?;
    let mut rng = OsRng;

    let context = AuthContext::new(
        ORDER_SERVICE_ID,
        audience,
        method,
        path,
        &body,
        &mut rng,
        current_unix_timestamp(),
    );

    let proof = create_auth_proof(&mut rng, &state.service_secret_key, context);
    let header_value = encode_auth_proof(&proof)
        .map_err(|error| ServiceError::internal(format!("failed to encode ZKP proof: {error}")))?;

    Ok((header_value, body))
}

pub async fn parse_internal_response<T>(
    response: reqwest::Response,
    service_name: &'static str,
) -> Result<T, ServiceError>
where
    T: serde::de::DeserializeOwned,
{
    let status = response.status();
    if !status.is_success() {
        let error = response
            .json::<ErrorResponse>()
            .await
            .map(|body| body.error)
            .unwrap_or_else(|_| format!("{service_name} returned {status}"));
        return Err(ServiceError::bad_gateway(error));
    }

    response.json::<T>().await.map_err(|error| {
        ServiceError::bad_gateway(format!("invalid {service_name} response: {error}"))
    })
}
