use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use common::{models::CreatePaymentResponse, zkp_auth::default_order_public_key};
use uuid::Uuid;
use zkp_auth_schnorr::{PublicKey, ReplayProtector, ReplayProtectorConfig};

pub type Payments = Arc<Mutex<HashMap<Uuid, CreatePaymentResponse>>>;
pub type SharedReplayProtector = Arc<Mutex<ReplayProtector>>;

#[derive(Clone)]
pub struct AppState {
    pub payments: Payments,
    pub order_service_public_key: PublicKey,
    pub replay_protector: SharedReplayProtector,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            payments: Arc::new(Mutex::new(HashMap::new())),
            order_service_public_key: default_order_public_key(),
            replay_protector: Arc::new(Mutex::new(ReplayProtector::new(
                ReplayProtectorConfig::default(),
            ))),
        }
    }
}
