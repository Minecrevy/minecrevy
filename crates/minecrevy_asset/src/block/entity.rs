use std::collections::HashSet;

use bevy::{prelude::*, reflect::TypeUuid};
use derive_more::AsRef;
use minecrevy_core::key::Key;

use crate::block::Block;

#[derive(TypeUuid, AsRef)]
#[uuid = "44f63fb9-9947-4819-a830-96a9350715c0"]
pub struct BlockEntityType {
    #[as_ref]
    pub key: Key,
    pub valid: HashSet<Handle<Block>>,
}
