//! Components and systems that are common across all **living** Minecraft entities.

use bevy::prelude::*;

use minecrevy_math::vector::Vector;

use crate::entity::EntityBundle;

/// Component [`Bundle`] for all **living** Minecraft entities.
#[derive(Bundle, Clone, Debug, Default)]
pub struct LivingEntityBundle {
    /// See [`EntityBundle`] for documentation.
    #[bundle]
    pub entity: EntityBundle,
    // TODO: hand active component
    /// See [`Health`] for documentation.
    pub health: Health,
    // TODO potions component
    /// See [`Arrows`] for documentation.
    pub arrows: Arrows,
    /// See [`Stingers`] for documentation.
    pub stingers: Stingers,
    /// See [`Sleeping`] for documentation.
    pub sleeping: Sleeping,
}

/// How much health an entity has, and the maximum amount it can have.
#[derive(Component, Clone, PartialEq, Debug)]
pub struct Health {
    /// The entity's current health.
    pub value: f32,
    /// The entity's maximum health.
    pub max: f32,
}

impl Default for Health {
    fn default() -> Self {
        Self {
            value: 20.0,
            max: 20.0,
        }
    }
}

/// How many arrows are stuck in the entity.
#[derive(Component, Clone, PartialEq, Debug, Default)]
pub struct Arrows(pub i32);

/// How many bee stingers are stuck in the entity.
#[derive(Component, Clone, PartialEq, Debug, Default)]
pub struct Stingers(pub i32);

/// Where the entity is currently sleeping.
#[derive(Component, Clone, PartialEq, Debug, Default)]
pub struct Sleeping(pub Option<Vector<3, i32>>);

/// Which hand the entity is using.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum Hand {
    /// The entity's main hand.
    Main,
    /// The entity's off hand.
    Off,
}

impl Default for Hand {
    fn default() -> Self {
        Self::Main
    }
}
