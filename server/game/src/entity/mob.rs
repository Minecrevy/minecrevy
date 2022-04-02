//! Components and systems for all Minecraft monster entities.

use bevy::prelude::*;

use minecrevy_util::MainHand;

use crate::entity::living::LivingEntityBundle;

/// Component [`Bundle`] for all Minecraft monster entities.
#[derive(Bundle, Clone, Debug, Default)]
pub struct MobBundle {
    /// See [`LivingEntityBundle`] for documentation.
    #[bundle]
    pub living: LivingEntityBundle,
}

/// Standard set of flags for monsters.
#[derive(Component, Clone, PartialEq, Debug)]
pub struct MobFlags {
    /// True if the mob should have AI enabled, such as walking, targeting and attacking.
    pub ai: bool,
    /// The main hand that the mob uses.
    pub hand: MainHand,
    /// True if the mob will attack-on-sight.
    pub aggressive: bool,
}

impl Default for MobFlags {
    fn default() -> Self {
        Self {
            ai: true,
            hand: MainHand::Right,
            aggressive: false,
        }
    }
}
