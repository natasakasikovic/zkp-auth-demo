use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use common::models::CreatePaymentResponse;
use uuid::Uuid;

pub type Payments = Arc<Mutex<HashMap<Uuid, CreatePaymentResponse>>>;

#[derive(Clone)]
pub struct AppState {
    pub payments: Payments,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            payments: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}
