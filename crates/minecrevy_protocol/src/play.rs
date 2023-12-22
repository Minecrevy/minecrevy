//! Minecraft protocol packet definitions in the `Play` state.

use std::io;

use bevy::prelude::*;
use minecrevy_io::{
    args::{IVec3Args, IntArgs, ListArgs, ListLength, OptionArgs, OptionTag},
    McWrite,
};
use minecrevy_key::{KeyArgs, SharedKey};

/// The initial packet sent when entering the `Play` state.
#[derive(Clone, PartialEq, Debug)]
pub struct Login {
    /// The player's entity ID.
    pub entity_id: i32,
    /// Whether the world is hardcore (i.e. one life).
    pub is_hardcore: bool,
    /// The server's worlds.
    pub dimensions: Vec<SharedKey>,
    /// The server's maximum player count.
    pub max_players: i32,
    /// The server's view distance.
    pub view_distance: i32,
    /// The server's simulation distance.
    pub simulation_distance: i32,
    /// Whether the server is in reduced debug info mode.
    pub reduced_debug_info: bool,
    /// Whether the player should see the respawn screen when they die.
    pub enable_respawn_screen: bool,
    /// Whether the player can only craft recipes they've unlocked.
    pub do_limited_crafting: bool,
    /// The dimension type of the world the player is in.
    pub dimension_type: SharedKey,
    /// The name of the world the player is in.
    pub dimension_name: SharedKey,
    /// The seed of the world the player is in, hashed.
    pub hashed_seed: i64,
    /// The player's game mode.
    pub game_mode: GameMode,
    /// The player's previous game mode.
    pub previous_game_mode: PreviousGameMode,
    /// Whether the player's world is a debug world.
    pub is_debug: bool,
    /// Whether the player's world is a superflat world.
    pub is_flat: bool,
    /// The player's last death location, if any.
    pub death_location: Option<DeathLocation>,
    /// The number of ticks until the player can use a portal again.
    pub portal_cooldown: i32,
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
                inner: KeyArgs::default(),
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
        self.dimension_type.write(&mut writer, KeyArgs::default())?;
        self.dimension_name.write(&mut writer, KeyArgs::default())?;
        self.hashed_seed
            .write(&mut writer, IntArgs { varint: false })?;
        self.game_mode.write(&mut writer, ())?;
        self.previous_game_mode.write(&mut writer, ())?;
        self.is_debug.write(&mut writer, ())?;
        self.is_flat.write(&mut writer, ())?;
        self.death_location.write(
            &mut writer,
            OptionArgs {
                tag: OptionTag::Bool,
                inner: (),
            },
        )?;
        self.portal_cooldown
            .write(&mut writer, IntArgs { varint: true })?;
        Ok(())
    }
}

/// The player's game mode.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum GameMode {
    /// Survival mode.
    #[default]
    Survival,
    /// Creative mode.
    Creative,
    /// Adventure mode.
    Adventure,
    /// Spectator mode.
    Spectator,
}

impl McWrite for GameMode {
    type Args = ();

    fn write(&self, writer: impl io::Write, (): Self::Args) -> io::Result<()> {
        let id: u8 = match self {
            Self::Survival => 0,
            Self::Creative => 1,
            Self::Adventure => 2,
            Self::Spectator => 3,
        };
        id.write(writer, ())
    }
}

/// The player's game mode.
#[derive(Deref, DerefMut)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct PreviousGameMode(pub Option<GameMode>);

impl McWrite for PreviousGameMode {
    type Args = ();

    fn write(&self, writer: impl io::Write, (): Self::Args) -> io::Result<()> {
        let id: i8 = match self.0 {
            None => -1,
            Some(GameMode::Survival) => 0,
            Some(GameMode::Creative) => 1,
            Some(GameMode::Adventure) => 2,
            Some(GameMode::Spectator) => 3,
        };
        id.write(writer, ())
    }
}

/// The player's location of their last death.
#[derive(Clone, PartialEq, Debug)]
pub struct DeathLocation {
    /// The dimension the player died in.
    pub dimension_name: SharedKey,
    /// The location the player died at.
    pub location: IVec3,
}

impl McWrite for DeathLocation {
    type Args = ();

    fn write(&self, mut writer: impl io::Write, (): Self::Args) -> io::Result<()> {
        self.dimension_name.write(&mut writer, KeyArgs::default())?;
        self.location
            .write(&mut writer, IVec3Args { compressed: true })?;
        Ok(())
    }
}

/// A packet sent by the server to synchronize the player's position.
#[derive(Clone, PartialEq, Debug)]
pub struct SynchronizePlayerPosition {
    /// The player's X coordinate.
    pub x: f64,
    /// The player's Y coordinate.
    pub y: f64,
    /// The player's Z coordinate.
    pub z: f64,
    /// The player's yaw.
    pub yaw: f32,
    /// The player's pitch.
    pub pitch: f32,
    /// A bitmask of which prior fields are relative to the player's current position.
    pub flags: u8,
    /// The teleport ID.
    pub teleport_id: i32,
}

impl McWrite for SynchronizePlayerPosition {
    type Args = ();

    fn write(&self, mut writer: impl io::Write, (): Self::Args) -> io::Result<()> {
        self.x.write(&mut writer, ())?;
        self.y.write(&mut writer, ())?;
        self.z.write(&mut writer, ())?;
        self.yaw.write(&mut writer, ())?;
        self.pitch.write(&mut writer, ())?;
        self.flags.write(&mut writer, ())?;
        self.teleport_id
            .write(&mut writer, IntArgs { varint: true })?;
        Ok(())
    }
}

/// A packet sent by the server to set the player's default spawn position.
#[derive(Clone, PartialEq, Debug)]
pub struct SetDefaultSpawnPosition {
    /// The player's spawn position.
    pub location: IVec3,
    /// The player's spawn angle.
    pub angle: f32,
}

impl McWrite for SetDefaultSpawnPosition {
    type Args = ();

    fn write(&self, mut writer: impl io::Write, (): Self::Args) -> io::Result<()> {
        self.location
            .write(&mut writer, IVec3Args { compressed: true })?;
        self.angle.write(writer, ())?;
        Ok(())
    }
}
