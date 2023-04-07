use bevy::prelude::*;

use crate::item::Item;

// TODO
#[derive(Clone, PartialEq, Debug)]
pub struct ItemStack {
    item: Handle<Item>,
    pub quantity: u32,
}

impl ItemStack {
    /// Constructs a new item stack with the specified item handle and quantity.
    pub fn new(item: Handle<Item>, quantity: u32) -> Self {
        Self { item, quantity }
    }
}

impl From<Handle<Item>> for ItemStack {
    fn from(item: Handle<Item>) -> Self {
        Self::new(item, 1)
    }
}
