//! Components and systems that are common across all Minecraft entities.

use std::num::NonZeroI32;
use std::sync::atomic::{AtomicI32, Ordering};

use bevy::prelude::*;

use minecrevy_math::complex::Quaternion;
use minecrevy_math::index::Y;
use minecrevy_math::vector::Vector3;
use minecrevy_protocol_latest::types::Pose as ProtoPose;
use minecrevy_text::Text;

pub mod armorstand;
pub mod living;
pub mod mob;
pub mod player;
pub mod metadata;

/// Component [`Bundle`] for all Minecraft entities.
#[derive(Bundle, Clone, Debug, Default)]
pub struct EntityBundle {
    /// See [`NetworkId`] for documentation.
    pub id: NetworkId,
    /// See [`Position`] for documentation.
    pub position: Position,
    /// See [`Rotation`] for documentation.
    pub rotation: Rotation,
    /// See [`EntityFlags`] for documentation.
    pub flags: EntityFlags,
    /// See [`Fire`] for documentation.
    pub fire: Fire,
    /// See [`RemainingAir`] for documentation.
    pub air_remaining: AirRemaining,
    /// See [`CustomName`] for documentation.
    pub custom_name: CustomName,
    /// See [`Silent`] for documentation.
    pub silent: Silent,
    /// See [`Gravity`] for documentation.
    pub gravity: Gravity,
    /// See [`Pose`] for documentation.
    pub pose: Pose,
    /// See [`Frozen`] for documentation.
    pub frozen: Frozen,
}

/// A monotonically increasing unique numerical identifier for entities sent to clients.
///
/// Since all loaded Minecraft entities have a `NetworkId` component,
/// use [`With<NetworkId>`] to ECS filter for them.
#[derive(Component, Clone, Eq, PartialEq, Debug)]
pub struct NetworkId(NonZeroI32);

impl NetworkId {
    /// Returns the next network id.
    pub fn next() -> Self {
        static COUNTER: AtomicI32 = AtomicI32::new(1);

        let next = COUNTER.fetch_add(1, Ordering::Relaxed);
        let next = NonZeroI32::new(next).expect("ALL NETWORK IDs EXHAUSTED");

        Self(next)
    }

    /// Returns the `NonZeroI32` representing this id.
    #[inline]
    pub const fn get(&self) -> NonZeroI32 {
        self.0
    }

    /// Returns the `i32` representing this id.
    #[inline]
    pub const fn val(&self) -> i32 {
        self.0.get()
    }
}

impl Default for NetworkId {
    #[inline]
    fn default() -> Self {
        Self::next()
    }
}

/// Position of the entity in the world.
#[derive(Component, Clone, PartialEq, Debug, Default)]
pub struct Position(pub Vector3<f64>);

/// Rotation of the entity.
#[derive(Component, Clone, PartialEq, Debug, Default)]
pub struct Rotation(pub Quaternion<f64>);

impl Rotation {
    #[inline]
    pub fn direction(&self) -> Vector3<f64> {
        self.0.direction()
    }

    #[inline]
    pub fn axes_angles_deg(&self) -> Vector3<f64> {
        let mut vec = self.0.axes_angles_deg();
        vec[Y] = -vec[Y];
        vec
    }
}

/// How much longer the entity is on fire.
#[derive(Component, Clone, Debug, Default)]
pub struct Fire(pub Timer);

/// Standard set of flags for all Minecraft entities.
#[derive(Component, Clone, PartialEq, Debug, Default)]
pub struct EntityFlags {
    /// True if the entity is crouching.
    pub crouching: bool,
    /// True if the entity is sprinting.
    pub sprinting: bool,
    /// True if the entity is swimming.
    pub swimming: bool,
    /// True if the entity is invisible.
    pub invisible: bool,
    /// True if the entity is glowing.
    pub glowing: bool,
    /// True if the entity is flying with an elytra.
    pub elytra_flying: bool,
}

/// How much longer the entity can breathe underwater.
#[derive(Component, Clone, Debug, Default)]
pub struct AirRemaining(pub Timer);

/// The overhead name of the entity, and whether the name is visible.
#[derive(Component, Clone, PartialEq, Debug, Default)]
pub struct CustomName {
    /// The custom name of the entity.
    pub name: Option<Text>,
    /// True if the custom name is visible.
    pub visible: bool,
}

/// True if the entity makes any noise.
#[derive(Component, Clone, PartialEq, Debug, Default)]
pub struct Silent(pub bool);

/// True if the entity is affected by gravity.
#[derive(Component, Clone, PartialEq, Debug)]
pub struct Gravity(pub bool);

impl Default for Gravity {
    #[inline]
    fn default() -> Self {
        Self(true)
    }
}

/// The pose that an entity is currently doing.
#[derive(Component, Copy, Clone, PartialEq, Debug)]
pub enum Pose {
    /// The default pose, just standing.
    Standing,
    /// Falling pose.
    FallFlying,
    /// Sleeping pose.
    Sleeping,
    /// Swimming pose.
    Swimming,
    /// Spin attack pose.
    SpinAttack,
    /// Crouching pose.
    Crouching,
    /// Long jumping pose.
    LongJumping,
    /// Dying pose.
    Dying,
}

impl From<ProtoPose> for Pose {
    fn from(v: ProtoPose) -> Self {
        match v {
            ProtoPose::Standing => Pose::Standing,
            ProtoPose::FallFlying => Pose::FallFlying,
            ProtoPose::Sleeping => Pose::Sleeping,
            ProtoPose::Swimming => Pose::Swimming,
            ProtoPose::SpinAttack => Pose::SpinAttack,
            ProtoPose::Crouching => Pose::Crouching,
            ProtoPose::LongJumping => Pose::LongJumping,
            ProtoPose::Dying => Pose::Dying,
        }
    }
}

impl From<Pose> for ProtoPose {
    fn from(v: Pose) -> Self {
        match v {
            Pose::Standing => ProtoPose::Standing,
            Pose::FallFlying => ProtoPose::FallFlying,
            Pose::Sleeping => ProtoPose::Sleeping,
            Pose::Swimming => ProtoPose::Swimming,
            Pose::SpinAttack => ProtoPose::SpinAttack,
            Pose::Crouching => ProtoPose::Crouching,
            Pose::LongJumping => ProtoPose::LongJumping,
            Pose::Dying => ProtoPose::Dying,
        }
    }
}

impl Default for Pose {
    #[inline]
    fn default() -> Self {
        Self::Standing
    }
}

/// How much longer the entity is frozen in powdered snow.
#[derive(Component, Clone, Debug, Default)]
pub struct Frozen(pub Timer);
