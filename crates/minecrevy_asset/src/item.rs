use std::num::NonZeroU32;

use bevy::reflect::TypeUuid;
use derive_more::AsRef;
use minecrevy_core::key::Key;

pub use self::{enchantment::*, equipment::*, stack::*};
use crate::index::KeyedAssets;

mod enchantment;
mod equipment;
mod stack;

/// [System parameter](bevy::ecs::system::SystemParam) that provides shared access to all [`Item`]s,
/// additionally indexed by [`Key`].
pub type Items<'w, 's> = KeyedAssets<'w, 's, Item>;

// TODO
#[derive(TypeUuid, AsRef, Clone, Debug)]
#[uuid = "032997ce-07c3-4c21-a8c9-d791aeb74471"]
pub struct Item {
    #[as_ref]
    pub key: Key,
    pub id: u32,
    pub translation: String,
    pub kind: ItemKind,
}

impl Item {
    /// Returns the maximum amount of this item that can be "stacked" in one inventory slot.
    pub fn max_stack_size(&self) -> u32 {
        match self.kind {
            ItemKind::Stackable(size) => size.get(),
            ItemKind::Damageable(_) => 1,
        }
    }

    /// Returns the maximum amount of damage an instance of this item can take before it is destroyed.
    pub fn max_damage(&self) -> Option<NonZeroU32> {
        match self.kind {
            ItemKind::Stackable(_) => None,
            ItemKind::Damageable(damage) => Some(damage),
        }
    }
}

// TODO
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ItemKind {
    Stackable(NonZeroU32),
    Damageable(NonZeroU32),
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Default)]
pub enum Rarity {
    /// Common.
    #[default]
    Common,
    /// Uncommon.
    Uncommon,
    /// Rare.
    Rare,
}
