use bevy::{prelude::*, utils::HashMap};
use flexstr::SharedStr;
use minecrevy_key::SharedKey;

use crate::item::Item;

#[derive(Asset, TypePath)]
pub struct Block {
    pub id: i32,
    pub name: SharedStr,
    pub translation_key: SharedStr,
    pub explosion_resistance: f32,
    pub friction: f32,
    pub speed_factor: f32,
    pub jump_factor: f32,
    pub has_dynamic_shape: bool,
    pub default_state: Handle<BlockState>,
    pub loot_table_location: SharedKey,
    pub offset: Vec2,
    pub default_hardness: f32,
    pub item: Handle<Item>,
    pub is_block_entity: bool,
    pub has_gravity: bool,
}

#[derive(Asset, TypePath)]
pub struct BlockState {
    pub block: Handle<Block>,
    pub properties: HashMap<SharedStr, SharedStr>,
}
