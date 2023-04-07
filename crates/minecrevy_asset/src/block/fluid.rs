use bevy::{prelude::*, reflect::TypeUuid, utils::HashMap};
use derive_more::AsRef;
use minecrevy_core::{key::Key, str::CompactString};

use crate::block::BlockPropertyValue;

/// A mapping of [`Fluid`]s to their [`FluidState`]s.
#[derive(Resource)]
pub struct FluidStateMap {
    inner: HashMap<Handle<Fluid>, Vec<Handle<FluidState>>>,
}

/// A [`Fluid`] with specific [`BlockPropertyValue`]s.
#[derive(TypeUuid)]
#[uuid = "d48979bf-b82f-4802-85b1-bcdf7fd70f7f"]
pub struct FluidState {
    /// The fluid that the state refers to.
    pub fluid: Handle<Fluid>,
    /// The properties of the state.
    pub properties: HashMap<CompactString, BlockPropertyValue>,
}

/// A fluid block in Minecraft, like water or lava.
#[derive(TypeUuid, AsRef)]
#[uuid = "9a26dd86-f40d-4798-80ab-a4263d609d79"]
pub struct Fluid {
    /// The key that uniquely identifies the fluid.
    #[as_ref]
    pub key: Key,
}
