use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
pub struct HealthResponse {
    pub service: &'static str,
    pub status: &'static str,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateOrderRequest {
    pub customer_id: String,
    pub product_id: String,
    pub quantity: u32,
    pub payment_method: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateOrderResponse {
    pub order_id: Uuid,
    pub status: OrderStatus,
    pub reservation_id: Uuid,
    pub payment_id: Uuid,
    pub total_amount: u64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OrderStatus {
    Created,
}

#[derive(Debug, Clone, Serialize)]
pub struct InventoryItemResponse {
    pub product_id: String,
    pub available_quantity: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReserveStockRequest {
    pub order_id: Uuid,
    pub product_id: String,
    pub quantity: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReserveStockResponse {
    pub reservation_id: Uuid,
    pub product_id: String,
    pub reserved_quantity: u32,
    pub remaining_quantity: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePaymentRequest {
    pub order_id: Uuid,
    pub amount: u64,
    pub payment_method: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePaymentResponse {
    pub payment_id: Uuid,
    pub order_id: Uuid,
    pub status: PaymentStatus,
    pub amount: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentStatus {
    Approved,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}
