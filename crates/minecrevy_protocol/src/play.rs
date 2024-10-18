//! Minecraft protocol packet definitions in the `Play` state.

use std::io;

use bevy::math::{DVec3, Vec2, Vec3};
use enumflags2::BitFlags;
use minecrevy_io::{
    args::{IntArgs, ListArgs, ListLength, OptionArgs, OptionTag, StringArgs},
    McRead, McWrite,
};

#[derive(Clone, PartialEq, Debug)]
pub struct GameEvent {
    pub event: u8,
    pub value: f32,
}

impl GameEvent {
    pub const WAIT_ON_CHUNKS: Self = Self {
        event: 13,
        value: 0.0,
    };
}

impl McWrite for GameEvent {
    type Args = ();

    fn write(&self, mut writer: impl io::Write, (): Self::Args) -> io::Result<()> {
        self.event.write(&mut writer, ())?;
        self.value.write(&mut writer, ())?;
        Ok(())
    }
}

/// Clientbound packet that sends the player's initial information after login.
#[derive(Clone, PartialEq, Debug)]
pub struct Login {
    pub entity_id: i32,
    pub is_hardcore: bool,
    pub dimensions: Vec<String>,
    pub max_players: i32,
    pub view_distance: i32,
    pub simulation_distance: i32,
    pub reduced_debug_info: bool,
    pub enable_respawn_screen: bool,
    pub do_limited_crafting: bool,
    pub dimension_type: i32,
    pub dimension_name: String,
    pub hashed_seed: i64,
    pub game_mode: u8,
    pub previous_game_mode: i8,
    pub is_debug: bool,
    pub is_flat: bool,
    pub death_dimension_name_and_location: Option<(String, Vec3)>,
    pub portal_cooldown: i32,
    pub enforces_secure_chat: bool,
}

impl McWrite for Login {
    type Args = ();

    fn write(&self, mut writer: impl io::Write, (): Self::Args) -> io::Result<()> {
        self.entity_id
            .write(&mut writer, IntArgs { varint: false })?;
        self.is_hardcore.write(&mut writer, ())?;
        self.dimensions.write(
            &mut writer,
            ListArgs {
                length: ListLength::VarInt,
                inner: StringArgs {
                    max_len: Some(32767),
                },
            },
        )?;
        self.max_players
            .write(&mut writer, IntArgs { varint: true })?;
        self.view_distance
            .write(&mut writer, IntArgs { varint: true })?;
        self.simulation_distance
            .write(&mut writer, IntArgs { varint: true })?;
        self.reduced_debug_info.write(&mut writer, ())?;
        self.enable_respawn_screen.write(&mut writer, ())?;
        self.do_limited_crafting.write(&mut writer, ())?;
        self.dimension_type
            .write(&mut writer, IntArgs { varint: true })?;
        self.dimension_name.write(
            &mut writer,
            StringArgs {
                max_len: Some(32767),
            },
        )?;
        self.hashed_seed
            .write(&mut writer, IntArgs { varint: false })?;
        self.game_mode.write(&mut writer, ())?;
        self.previous_game_mode.write(&mut writer, ())?;
        self.is_debug.write(&mut writer, ())?;
        self.is_flat.write(&mut writer, ())?;
        self.death_dimension_name_and_location.write(
            &mut writer,
            OptionArgs {
                tag: OptionTag::Bool,
                inner: (
                    StringArgs {
                        max_len: Some(32767),
                    },
                    (),
                ),
            },
        )?;
        self.portal_cooldown
            .write(&mut writer, IntArgs { varint: true })?;
        self.enforces_secure_chat.write(&mut writer, ())?;
        Ok(())
    }
}

/// Clientbound packet that updates the player's position.
/// Clients must confirm the teleport with a [`ConfirmTeleport`] packet.
#[derive(Clone, PartialEq, Debug)]
pub struct SyncPlayerPosition {
    /// The new position of the player.
    pub position: DVec3,
    /// The new rotation of the player.
    /// X: Yaw, Y: Pitch.
    pub rotation: Vec2,
    /// Bitfield for which values above are relative to the player's current
    /// position.
    pub flags: BitFlags<SyncPlayerPositionFlags>,
    /// The teleport ID. Must be confirmed
    pub teleport_id: i32,
}

/// The flags for the [`SyncPlayerPosition`] packet.
#[enumflags2::bitflags]
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SyncPlayerPositionFlags {
    /// The X coordinate is relative to the player's current position.
    X = 0x01,
    /// The Y coordinate is relative to the player's current position.
    Y = 0x02,
    /// The Z coordinate is relative to the player's current position.
    Z = 0x04,
    /// The pitch is relative to the player's current rotation.
    Pitch = 0x08,
    /// The yaw is relative to the player's current rotation.
    Yaw = 0x10,
}

impl McWrite for SyncPlayerPosition {
    type Args = ();

    fn write(&self, mut writer: impl io::Write, (): Self::Args) -> io::Result<()> {
        self.position.write(&mut writer, ())?;
        self.rotation.write(&mut writer, ())?;
        self.flags.write(&mut writer, ())?;
        self.teleport_id.write(writer, IntArgs { varint: true })?;
        Ok(())
    }
}

/// Serverbound packet that confirms a teleport sent by the server
/// via [`SyncPlayerPosition`].
#[derive(Clone, PartialEq, Debug)]
pub struct ConfirmTeleport {
    /// The teleport ID to confirm.
    pub teleport_id: i32,
}

impl McRead for ConfirmTeleport {
    type Args = ();

    fn read(mut reader: impl io::Read, (): Self::Args) -> io::Result<Self> {
        Ok(Self {
            teleport_id: i32::read(&mut reader, IntArgs { varint: true })?,
        })
    }
}

/// Serverbound packet that updates the player's position.
///
/// See also [`SetPlayerRotation`] and [`SetPlayerPositionAndRotation`].
#[derive(Clone, PartialEq, Debug)]
pub struct SetPlayerPosition {
    /// The new position of the player.
    /// Y is the feet position, not the eyes.
    pub position: DVec3,
    /// True if the player is on the ground.
    pub on_ground: bool,
}

impl McRead for SetPlayerPosition {
    type Args = ();

    fn read(mut reader: impl io::Read, (): Self::Args) -> io::Result<Self> {
        Ok(Self {
            position: DVec3::read(&mut reader, ())?,
            on_ground: bool::read(&mut reader, ())?,
        })
    }
}

/// Serverbound packet that updates the player's rotation.
///
/// See also [`SetPlayerPosition`] and [`SetPlayerPositionAndRotation`].
#[derive(Clone, PartialEq, Debug)]
pub struct SetPlayerRotation {
    /// The new rotation of the player.
    /// X: Yaw, Y: Pitch.
    pub rotation: Vec2,
    /// True if the player is on the ground.
    pub on_ground: bool,
}

impl McRead for SetPlayerRotation {
    type Args = ();

    fn read(mut reader: impl io::Read, (): Self::Args) -> io::Result<Self> {
        Ok(Self {
            rotation: Vec2::read(&mut reader, ())?,
            on_ground: bool::read(&mut reader, ())?,
        })
    }
}

/// Serverbound packet that updates the player's position and rotation
/// at the same time.
///
/// See also [`SetPlayerPosition`] and [`SetPlayerRotation`].
#[derive(Clone, PartialEq, Debug)]
pub struct SetPlayerPositionAndRotation {
    /// The new position of the player.
    /// Y is the feet position, not the eyes.
    pub position: DVec3,
    /// The new rotation of the player.
    /// X: Yaw, Y: Pitch.
    pub rotation: Vec2,
    /// True if the player is on the ground.
    pub on_ground: bool,
}

impl McRead for SetPlayerPositionAndRotation {
    type Args = ();

    fn read(mut reader: impl io::Read, (): Self::Args) -> io::Result<Self> {
        Ok(Self {
            position: DVec3::read(&mut reader, ())?,
            rotation: Vec2::read(&mut reader, ())?,
            on_ground: bool::read(&mut reader, ())?,
        })
    }
}

/// Serverbound packet that updates the player's on-ground status.
///
/// See also [`SetPlayerPosition`], [`SetPlayerRotation`], and
/// [`SetPlayerPositionAndRotation`].
#[derive(Clone, PartialEq, Debug)]
pub struct SetPlayerOnGround {
    /// True if the player is on the ground.
    pub on_ground: bool,
}

impl McRead for SetPlayerOnGround {
    type Args = ();

    fn read(mut reader: impl io::Read, (): Self::Args) -> io::Result<Self> {
        Ok(Self {
            on_ground: bool::read(&mut reader, ())?,
        })
    }
}

/// Clientbound packet that updates the player's abilities.
#[derive(Clone, PartialEq, Debug)]
pub struct SyncPlayerAbilities {
    /// Bitfield of player abilities.
    pub flags: BitFlags<PlayerAbilities>,
    /// The player's flying speed, defaults to `0.05`.
    pub flying_speed: f32,
    /// The field of view modifier, defaults to `0.1`.
    pub fov_modifier: f32,
}

impl McWrite for SyncPlayerAbilities {
    type Args = ();

    fn write(&self, mut writer: impl io::Write, (): Self::Args) -> io::Result<()> {
        self.flags.write(&mut writer, ())?;
        self.flying_speed.write(&mut writer, ())?;
        self.fov_modifier.write(&mut writer, ())?;
        Ok(())
    }
}

/// The flags for the [`SyncPlayerAbilities`] and [`SetPlayerAbilities`] packets.
#[enumflags2::bitflags]
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PlayerAbilities {
    /// The player cannot take damage.
    Invulnerable = 0x01,
    /// The player is flying.
    Flying = 0x02,
    /// The player can fly.
    AllowFlying = 0x04,
    /// The player instantly breaks blocks.
    InstantBreak = 0x08,
}
