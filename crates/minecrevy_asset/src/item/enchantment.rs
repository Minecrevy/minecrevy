use bevy::{prelude::Handle, reflect::TypeUuid};
use derive_more::AsRef;
use minecrevy_core::key::Key;

use crate::item::equipment::EquipmentSlot;

// TODO
/// An instance of an [`Enchantment`] with a corresponding level.
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct EnchantmentValue {
    /// The used enchantment.
    pub enchantment: Handle<Enchantment>,
    /// The enchantment level.
    pub level: i32,
}

// TODO
#[derive(TypeUuid, AsRef, Clone, PartialEq, Debug)]
#[uuid = "8177c565-9dc6-492c-9843-eda6220963f7"]
pub struct Enchantment {
    /// The key that uniquely identifies the enchantment.
    #[as_ref]
    pub key: Key,
    /// The valid slots that the enchantment can be used in.
    pub slots: Vec<Handle<EquipmentSlot>>,
    /// The rarity of the enchantment.
    pub rarity: EnchantmentRarity,
    /// The category of the enchantment.
    pub category: Handle<EnchantmentCategory>,
    /// The localization key to the enchantment's description.
    pub description: Key,
    /// The minimum level of the enchantment.
    pub min_level: i32,
    /// The maximum level of the enchantment.
    pub max_level: i32,
    /// Whether the enchantment CANNOT be applied with an enchantment table.
    pub treasure_only: bool,
    /// Whether the enchantment is considered a 'curse', like the Curse of Binding or Vanishing.
    pub curse: bool,
    pub tradeable: bool,
    pub discoverable: bool,
}

/// A category of [`Enchantment`].
#[derive(TypeUuid, AsRef, Clone, PartialEq, Debug)]
#[uuid = "53cabf51-10a5-4af6-80c5-ced391cf90f9"]
pub struct EnchantmentCategory {
    /// The key that uniquely identifies the category.
    #[as_ref]
    pub key: Key,
}

// TODO
/// How often an enchantment is found through gameplay.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Default)]
pub enum EnchantmentRarity {
    /// Common.
    #[default]
    Common,
    /// Uncommon.
    Uncommon,
    /// Rare.
    Rare,
    /// Very rare.
    VeryRare,
}
