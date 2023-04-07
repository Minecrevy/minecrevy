use bevy::asset::Handle;
use bevy::ecs::system::Resource;
use minecrevy_asset::{
    index::ExtractIndexedAssets,
    item::{ArmorMaterial, EquipmentSlot, ToolMaterial},
};

/// All [`EquipmentSlot`]s in vanilla Minecraft.
#[derive(Resource, ExtractIndexedAssets)]
#[extract(asset = "EquipmentSlot")]
pub struct EquipmentSlots {
    /// The main hand of a mob, where tools and weapons are equipped.
    pub main_hand: Handle<EquipmentSlot>,
    /// The off hand of a mob, where miscellaneous items are used.
    pub off_hand: Handle<EquipmentSlot>,
    /// The feet armor slot of a mob.
    pub feet: Handle<EquipmentSlot>,
    /// The leg armor slot of a mob.
    pub legs: Handle<EquipmentSlot>,
    /// The chest armor slot of a mob.
    pub chest: Handle<EquipmentSlot>,
    /// The head armor slot of a mob.
    pub head: Handle<EquipmentSlot>,
}

/// All [`ArmorMaterial`]s in vanilla Minecraft.
#[derive(Resource, ExtractIndexedAssets)]
#[extract(asset = "ArmorMaterial")]
pub struct ArmorMaterials {
    /// Leather armor.
    pub leather: Handle<ArmorMaterial>,
    /// Turtle armor.
    pub turtle: Handle<ArmorMaterial>,
    /// Chainmail armor.
    pub chainmail: Handle<ArmorMaterial>,
    /// Iron armor.
    pub iron: Handle<ArmorMaterial>,
    /// Gold armor.
    pub gold: Handle<ArmorMaterial>,
    /// Diamond armor.
    pub diamond: Handle<ArmorMaterial>,
    /// Netherite armor.
    pub netherite: Handle<ArmorMaterial>,
}

/// All [`ToolMaterial`]s in vanilla Minecraft.
#[derive(Resource, ExtractIndexedAssets)]
#[extract(asset = "ToolMaterial")]
pub struct ToolMaterials {
    /// Wooden tools.
    pub wood: Handle<ToolMaterial>,
    /// Stone tools.
    pub stone: Handle<ToolMaterial>,
    /// Iron tools.
    pub iron: Handle<ToolMaterial>,
    /// Diamond tools.
    pub diamond: Handle<ToolMaterial>,
    /// Gold tools.
    pub gold: Handle<ToolMaterial>,
    /// Netherite tools.
    pub netherite: Handle<ToolMaterial>,
}
