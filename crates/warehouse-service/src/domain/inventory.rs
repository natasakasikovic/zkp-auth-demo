use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct InventoryItem {
    pub product_id: String,
    pub available_quantity: u32,
}

impl InventoryItem {
    pub fn new(product_id: impl Into<String>, available_quantity: u32) -> Self {
        Self {
            product_id: product_id.into(),
            available_quantity,
        }
    }

    pub fn reserve(&mut self, quantity: u32) -> Result<(), InventoryReservationError> {
        if quantity == 0 {
            return Err(InventoryReservationError::InvalidQuantity);
        }

        if self.available_quantity < quantity {
            return Err(InventoryReservationError::InsufficientStock {
                requested_quantity: quantity,
                available_quantity: self.available_quantity,
            });
        }

        self.available_quantity -= quantity;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum InventoryReservationError {
    InvalidQuantity,
    InsufficientStock {
        requested_quantity: u32,
        available_quantity: u32,
    },
}

pub fn seed_inventory() -> HashMap<String, InventoryItem> {
    // 
    HashMap::from([
        ("laptop".to_string(), InventoryItem::new("laptop", 5)),
        ("keyboard".to_string(), InventoryItem::new("keyboard", 20)),
        ("monitor".to_string(), InventoryItem::new("monitor", 8)),
    ])
}
