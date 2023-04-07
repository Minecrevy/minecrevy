use bevy::{prelude::*, reflect::TypeUuid, utils::HashMap};
use derive_more::AsRef;
use minecrevy_core::{color::Rgb, key::Key, math::Direction, str::CompactString};

use crate::{index::KeyedAssets, item::Item, sound::SoundGroup};

pub use self::entity::*;
pub use self::fluid::*;

mod entity;
mod fluid;

/// [System parameter](bevy::ecs::system::SystemParam) that provides shared access to all [`Block`]s,
/// additionally indexed by [`Key`].
pub type Blocks<'w, 's> = KeyedAssets<'w, 's, Block>;

/// A mapping of [`Block`]s to their [`BlockState`]s.
#[derive(Resource)]
pub struct BlockStateMap {
    inner: HashMap<Handle<Block>, Vec<Handle<BlockState>>>,
}

/// A [`Block`] with specific [`BlockPropertyValue`]s.
#[derive(TypeUuid)]
#[uuid = "91e17275-edfe-4a2e-96d8-bce21cc6929a"]
pub struct BlockState {
    /// The block that the state refers to.
    pub block: Handle<Block>,
    /// How much light (0-15) is emitted by the block.
    pub light_emission: i32,
    /// The color of the state's block material.
    pub material_color: Handle<BlockMaterialColor>,
    /// The properties of the state.
    pub properties: HashMap<CompactString, BlockPropertyValue>,
}

/// A block property value.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum BlockPropertyValue {
    /// A boolean.
    Bool(bool),
    /// A 32-bit signed integer.
    Int(i32),
    /// A 3-dimensional direction.
    Direction(Direction),
}

// TODO
#[derive(TypeUuid, AsRef)]
#[uuid = "4d6f5f6b-d7b7-4bad-ac67-4d6e0a508a26"]
pub struct Block {
    /// The key that identifies the block.
    #[as_ref]
    pub key: Key,
    /// The material type that the block has.
    pub material: Handle<BlockMaterial>,
    /// True if mobs can collide with the block.
    pub collision_enabled: bool,
    /// The sounds that the block makes.
    pub sound_group: Handle<SoundGroup>,
    pub explosion_resistance: f32,
    pub destroy_time: f32,
    pub requires_correct_tool_for_drops: bool,
    pub randomly_ticks: bool,
    pub friction: f32,
    pub speed_factor: f32,
    pub jump_factor: f32,
    // pub loot_table: Option<Handle<LootTable>>, TODO
    pub can_occlude: bool,
    pub is_air: bool,
    /// The item that the block corresponds to.
    pub item: Option<Handle<Item>>,
}

#[derive(TypeUuid, Clone, PartialEq, Eq, Debug)]
#[uuid = "224092fb-acbb-4533-ab21-16d8a25f8197"]
pub struct BlockMaterial {
    /// The color of the block material.
    pub color: Handle<BlockMaterialColor>,
    /// The block's reaction to being pushed by a piston.
    pub push_reaction: PushReaction,
    pub blocks_motion: bool,
    pub flammable: bool,
    pub liquid: bool,
    pub solid_blocking: bool,
    pub replaceable: bool,
    pub solid: bool,
}

#[derive(TypeUuid, AsRef, Clone, PartialEq, Eq, Debug)]
#[uuid = "90f0b346-037b-4e8b-8a02-28af80b012ac"]
pub struct BlockMaterialColor {
    #[as_ref]
    pub key: Key,
    /// The network ID of the  material.w
    pub id: i32,
    /// The color of the block material.
    pub color: Rgb,
}

/// How a block reacts to being pushed by a piston.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Default)]
pub enum PushReaction {
    /// The block is pushed.
    #[default]
    Normal,
    /// The block is destroyed.
    Destroy,
    /// Nothing happens.
    Block,
    /// The block is ignored.
    Ignore,
    PushOnly,
}
