use bevy::{prelude::*, utils::HashMap};
use flexstr::SharedStr;
use minecrevy_key::SharedKey;

/// An item in Minecraft.
#[derive(Asset, TypePath)]
pub struct Item {
    /// The item ID.
    pub id: i32,
    /// The item [`Key`](minecrevy_key::Key).
    pub key: SharedKey,
    /// The key used for client-side translation.
    pub translation_key: SharedStr,
    /// The item rarity.
    pub rarity: Rarity,
    /// TODO: doc
    pub depletes: bool,
    pub max_stack_size: u8,
    pub max_damage: u16,
    pub edible: bool,
    pub fire_resistant: bool,
    pub block_id: SharedKey,
    pub eating_sound: SharedKey,
    pub drinking_sound: SharedKey,
}

/// [`Item`] rarity.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum Rarity {
    /// The most common rarity.
    Common,
    /// Quite common, but not as much as common.
    Uncommon,
    /// Less common than uncommon.
    Rare,
    /// Quite rare, even rarer than rare.
    Epic,
}

#[derive(Clone, PartialEq, Debug)]
pub struct ItemStack {
    pub item: Handle<Item>,
    pub count: u8,
    pub nbt: HashMap<String, minecrevy_nbt::Value>,
}
