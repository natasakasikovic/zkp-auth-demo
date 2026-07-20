pub mod payment_client;
pub mod warehouse_client;

use common::models::ErrorResponse;

use crate::error::ServiceError;

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
