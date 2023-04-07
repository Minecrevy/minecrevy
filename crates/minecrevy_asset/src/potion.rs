use bevy::reflect::TypeUuid;
use derive_more::AsRef;
use minecrevy_core::key::Key;

use crate::effect::EffectValue;

/// A drinkable or throwable group of [`EffectValue`]s.
#[derive(TypeUuid, AsRef)]
#[uuid = "2c33e39a-0b92-4e21-97da-53ef748e564d"]
pub struct Potion {
    /// The key that uniquely identifies the potion.
    #[as_ref]
    pub key: Key,
    /// The effects that the potion give.
    pub effects: Vec<EffectValue>,
}
