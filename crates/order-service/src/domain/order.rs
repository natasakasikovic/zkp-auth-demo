use std::collections::HashMap;

use common::models::CreateOrderRequest;

use crate::error::ServiceError;

pub fn validate_order_request(request: &CreateOrderRequest) -> Result<(), ServiceError> {
    if request.customer_id.trim().is_empty() {
        return Err(ServiceError::bad_request("customer_id is required"));
    }

    if request.product_id.trim().is_empty() {
        return Err(ServiceError::bad_request("product_id is required"));
    }

    if request.quantity == 0 {
        return Err(ServiceError::bad_request(
            "quantity must be greater than zero",
        ));
    }

    if request.payment_method.trim().is_empty() {
        return Err(ServiceError::bad_request("payment_method is required"));
    }

    Ok(())
}

pub fn seed_catalog() -> HashMap<String, u64> {
    // NOTE: data is for now in memory, may be changed to use db.
    HashMap::from([
        ("laptop".to_string(), 120_000),
        ("keyboard".to_string(), 8_000),
        ("monitor".to_string(), 35_000),
    ])
}
