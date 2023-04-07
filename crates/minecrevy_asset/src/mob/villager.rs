use bevy::{prelude::Handle, reflect::TypeUuid, utils::HashSet};
use derive_more::AsRef;
use minecrevy_core::key::Key;

use crate::{
    block::{Block, BlockState},
    item::Item,
    sound::Sound,
};

// TODO
#[derive(TypeUuid, AsRef)]
#[uuid = "a0d69172-443e-474c-8273-adea28b8ac95"]
pub struct VillagerType {
    #[as_ref]
    pub key: Key,
}

#[derive(TypeUuid, AsRef)]
#[uuid = "6be34869-f35a-4090-b570-78fd550bbc8f"]
pub struct VillagerProfession {
    #[as_ref]
    pub key: Key,
    pub primary_poi: Handle<PoiType>,
    pub secondary_poi: HashSet<Handle<Block>>,
    pub requested_items: HashSet<Handle<Item>>,
    pub work_sound: Option<Handle<Sound>>,
}

#[derive(TypeUuid, AsRef)]
#[uuid = "51264281-5422-41db-b5df-be8dc6dc6e4d"]
pub struct PoiType {
    #[as_ref]
    pub key: Key,
    pub states: HashSet<Handle<BlockState>>,
    pub max_tickets: i32,
    pub max_distance: i32,
}
