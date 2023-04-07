use bevy::{prelude::Handle, reflect::TypeUuid, utils::HashSet};
use derive_more::AsRef;
use minecrevy_core::key::Key;

use crate::block::Block;

pub mod villager;

/// A kind of interactable entity in Minecraft.
#[derive(TypeUuid, AsRef)]
#[uuid = "be271f39-97c9-455f-a6d7-9250987f4b45"]
pub struct MobType {
    /// The key that uniquely identifies the mob type.
    #[as_ref]
    pub key: Key,
    /// The entity type's category of mob.
    pub category: Handle<MobCategory>,
    /// The blocks that the entity type is immune from.
    pub immune_to: HashSet<Handle<Block>>,
    /// True if the entity type should be serialized to disk.
    pub saved: bool,
    /// True if the entity type can be summoned with /summon.
    pub summonable: bool,
    /// True if the entity type is immune to fire.
    pub fire_immune: bool,
    /// True if the entity type can spawn far away from a player.
    pub spawns_far_from_player: bool,
    /// How far (in blocks) clients will track the entity type.
    pub client_tracking_range: i32,
    /// How often (in game ticks) the entity type will be updated.
    pub update_interval: i32,
    // pub description: Option<Text>, TODO
    // pub loot_table: Option<Handle<LootTable>>, TODO
    pub dimensions: MobDimensions,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct MobDimensions {
    pub width: f32,
    pub height: f32,
    fixed: bool,
}

#[derive(TypeUuid, AsRef)]
#[uuid = "3b898330-aa4e-4043-927f-8efb8ea582a4"]
pub struct MobCategory {
    #[as_ref]
    pub key: Key,
    pub maximum_per_chunk: i32,
    pub friendly: bool,
    pub persistent: bool,
    pub despawn_distance: i32,
}
