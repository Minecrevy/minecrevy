//! Components and systems for Minecraft armor-stand entities.

use bevy::prelude::*;

use crate::entity::living::LivingEntityBundle;

/// Component [`Bundle`] for Minecraft armor-stand entities.
#[derive(Bundle, Clone, Debug, Default)]
pub struct ArmorStandBundle {
    /// See [`LivingEntityBundle`] for documentation.
    #[bundle]
    pub living: LivingEntityBundle,
    /// See [`ArmorStandFlags`] for documentation.
    pub flags: ArmorStandFlags,
    /// See [`ArmorStandRotations`] for documentation.
    pub rotations: ArmorStandRotations,
}

/// Standard set of flags for armor-stands.
#[derive(Component, Clone, PartialEq, Debug)]
pub struct ArmorStandFlags {
    /// True if the armor-stand is miniature.
    pub small: bool,
    /// True if the armor-stand has arms.
    pub arms: bool,
    /// True if the armor-stand has a base plate.
    pub base_plate: bool,
    /// True if the armor-stand is invisible, undetectable by clients.
    pub marker: bool,
}

impl Default for ArmorStandFlags {
    fn default() -> Self {
        Self {
            small: false,
            arms: true,
            base_plate: true,
            marker: false,
        }
    }
}

/// Parts of the armor-stand that can be separately rotated.
#[derive(Component, Clone, PartialEq, Debug)]
pub struct ArmorStandRotations {
    /// The rotation of the armor-stand's head.
    pub head: Vec3,
    /// The rotation of the armor-stand's body.
    pub body: Vec3,
    /// The rotation of the armor-stand's left arm.
    pub left_arm: Vec3,
    /// The rotation of the armor-stand's right arm.
    pub right_arm: Vec3,
    /// The rotation of the armor-stand's left leg.
    pub left_leg: Vec3,
    /// The rotation of the armor-stand's right leg.
    pub right_leg: Vec3,
}

impl Default for ArmorStandRotations {
    fn default() -> Self {
        Self {
            head: Vec3::new(0.0, 0.0, 0.0),
            body: Vec3::new(0.0, 0.0, 0.0),
            left_arm: Vec3::new(-10.0, 0.0, -10.0),
            right_arm: Vec3::new(-15.0, 0.0, 10.0),
            left_leg: Vec3::new(-1.0, 0.0, -1.0),
            right_leg: Vec3::new(1.0, 0.0, 1.0),
        }
    }
}
