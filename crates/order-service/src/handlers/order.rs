use axum::{Json, extract::State};
use common::models::{CreateOrderRequest, CreateOrderResponse, OrderStatus};
use uuid::Uuid;

use crate::{
    client::{payment_client::create_payment, warehouse_client::reserve_stock},
    domain::order::validate_order_request,
    error::ServiceError,
    state::AppState,
};

pub async fn create_order(
    State(state): State<AppState>,
    Json(request): Json<CreateOrderRequest>,
) -> Result<Json<CreateOrderResponse>, ServiceError> {
    validate_order_request(&request)?;

    let unit_price = state
        .catalog
        .get(&request.product_id)
        .copied()
        .ok_or_else(|| {
            ServiceError::bad_request(format!(
                "product '{}' is not in the catalog",
                request.product_id
            ))
        })?;

    let total_amount = unit_price * u64::from(request.quantity);
    let order_id = Uuid::new_v4();

    let reservation = reserve_stock(&state, &request, order_id).await?;
    let payment = create_payment(&state, &request, order_id, total_amount).await?;

    let response = CreateOrderResponse {
        order_id,
        status: OrderStatus::Created,
        reservation_id: reservation.reservation_id,
        payment_id: payment.payment_id,
        total_amount,
    };

    state
        .orders
        .lock()
        .map_err(|_| ServiceError::internal("order state is unavailable"))?
        .insert(order_id, response.clone());

    Ok(Json(response))
}
