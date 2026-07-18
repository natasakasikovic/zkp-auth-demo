use common::models::CreatePaymentRequest;

use crate::error::ServiceError;

pub fn validate_payment_request(request: &CreatePaymentRequest) -> Result<(), ServiceError> {
    if request.amount == 0 {
        return Err(ServiceError::bad_request(
            "amount must be greater than zero",
        ));
    }

    if request.payment_method.trim().is_empty() {
        return Err(ServiceError::bad_request("payment method is required"));
    }

    Ok(())
}
