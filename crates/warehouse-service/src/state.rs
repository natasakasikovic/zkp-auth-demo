use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::domain::inventory::{InventoryItem, seed_inventory};

pub type Inventory = Arc<Mutex<HashMap<String, InventoryItem>>>;

#[derive(Clone)]
pub struct AppState {
    pub inventory: Inventory,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            inventory: Arc::new(Mutex::new(seed_inventory())),
        }
    }
}
