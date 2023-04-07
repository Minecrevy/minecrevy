use bevy::{prelude::Handle, reflect::TypeUuid, utils::HashMap};
use derive_more::AsRef;
use minecrevy_core::key::Key;

use crate::{recipe::Ingredient, sound::Sound};

/// A named inventory slot meant for armor, tools, etc.
#[derive(TypeUuid, AsRef, Clone, PartialEq, Eq, Debug)]
#[uuid = "59a06ebd-92de-4799-999b-553003683158"]
pub struct EquipmentSlot {
    /// The key that uniquely identifies the slot.
    #[as_ref]
    pub key: Key,
    /// The type of slot.
    pub ty: EquipmentSlotType,
}

/// The type of [`EquipmentSlot`].
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum EquipmentSlotType {
    /// Handheld equipment.
    Hand,
    /// Worn equipment.
    Armor,
}

// TODO
/// The material stats for a piece of armor.
#[derive(TypeUuid, AsRef, Clone, PartialEq, Debug)]
#[uuid = "180e030a-f7e5-44d8-82f6-f784edcb0c82"]
pub struct ArmorMaterial {
    /// The key that uniquely identifies the armor material.
    #[as_ref]
    pub key: Key,
    /// The number of uses before breaking, per [`EquipmentSlot`].
    pub durability: HashMap<Handle<EquipmentSlot>, i32>,
    /// The defense stat, per [`EquipmentSlot`].
    pub defense: HashMap<Handle<EquipmentSlot>, i32>,
    /// The ability of the material to be enchanted. Higher is better.
    pub enchantment_value: i32,
    /// The sound played when the material is equipped.
    pub equip_sound: Handle<Sound>,
    pub toughness: f32,
    /// The material's ability to reduce knockback.
    pub knockback_resistance: f32,
    pub repair_ingredient: Ingredient,
}

/// The material stats for a tool or weapon.
#[derive(TypeUuid, AsRef, Clone, PartialEq, Debug)]
#[uuid = "790c7e1b-615f-473f-ba6e-d8690e0b79ea"]
pub struct ToolMaterial {
    /// The key that uniquely identifies the tool material.
    #[as_ref]
    pub key: Key,
    /// The harvest level of tools made from the materail.
    pub level: i32,
    /// The durability of the material.
    pub uses: i32,
    /// The speed at which blocks are harvested by the material.
    pub speed: f32,
    /// The damage done by weapons made with the material.
    pub damage: f32,
    /// The ability of the material to be enchanted. Higher is better.
    pub enchantment_value: i32,
    /// The ingredient used to repair the material.
    pub repair_ingredient: Ingredient,
}
