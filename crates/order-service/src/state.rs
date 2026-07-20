use std::{
    collections::HashMap,
    env,
    sync::{Arc, Mutex},
};

use common::models::CreateOrderResponse;
use reqwest::Client;
use uuid::Uuid;

use crate::domain::order::seed_catalog;

pub type Orders = Arc<Mutex<HashMap<Uuid, CreateOrderResponse>>>;

#[derive(Clone)]
pub struct AppState {
    pub http_client: Client,
    pub warehouse_url: String,
    pub payment_url: String,
    pub catalog: HashMap<String, u64>,
    pub orders: Orders,
}

impl AppState {
    pub fn from_env() -> Self {
        let warehouse_url =
            env::var("WAREHOUSE_URL").unwrap_or_else(|_| "http://127.0.0.1:3001".to_string());
        let payment_url =
            env::var("PAYMENT_URL").unwrap_or_else(|_| "http://127.0.0.1:3002".to_string());

        Self {
            http_client: Client::new(),
            warehouse_url,
            payment_url,
            catalog: seed_catalog(),
            orders: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}
