use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use common::zkp_auth::default_order_public_key;
use zkp_auth_schnorr::{PublicKey, ReplayProtector, ReplayProtectorConfig};

use crate::domain::inventory::{InventoryItem, seed_inventory};

pub type Inventory = Arc<Mutex<HashMap<String, InventoryItem>>>;
pub type SharedReplayProtector = Arc<Mutex<ReplayProtector>>;

#[derive(Clone)]
pub struct AppState {
    pub inventory: Inventory,
    pub order_service_public_key: PublicKey,
    pub replay_protector: SharedReplayProtector,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            inventory: Arc::new(Mutex::new(seed_inventory())),
            order_service_public_key: default_order_public_key(),
            replay_protector: Arc::new(Mutex::new(ReplayProtector::new(
                ReplayProtectorConfig::default(),
            ))),
        }
    }
}
