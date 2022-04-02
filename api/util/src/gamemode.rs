use std::convert::Infallible;
use thiserror::Error;

/// The game mode of a Minecraft player.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "bevy", derive(bevy::prelude::Component))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "minecrevy_io_str", derive(minecrevy_io_str::McRead, minecrevy_io_str::McWrite))]
#[cfg_attr(feature = "minecrevy_io_str", io_repr(u8))]
pub enum GameMode {
    /// The survival mode. Players have finite health and food levels.
    Survival = 0,
    /// The creative mode. Players have infinite health and food levels, and no block or item resource limits.
    Creative = 1,
    /// The adventure mode. Players cannot break any block without the correct tools.
    Adventure = 2,
    /// The spectator mode. Players are invisible and can latch onto entities to spectate.
    Spectator = 3,
}

impl Default for GameMode {
    fn default() -> Self {
        Self::Survival
    }
}

/// The error type returned when a conversion from `f32` to [`GameMode`] fails.
#[derive(Error, Debug)]
#[error("{0} can't be represented as a GameMode")]
pub struct TryFromF32Error(pub f32);

impl TryFrom<f32> for GameMode {
    type Error = TryFromF32Error;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        let value = value.round();
        match value as i32 {
            0 => Ok(Self::Survival),
            1 => Ok(Self::Creative),
            2 => Ok(Self::Adventure),
            3 => Ok(Self::Spectator),
            _ => Err(TryFromF32Error(value))
        }
    }
}

impl TryFrom<GameMode> for f32 {
    type Error = Infallible;

    fn try_from(value: GameMode) -> Result<Self, Self::Error> {
        match value {
            GameMode::Survival => Ok(0.0),
            GameMode::Creative => Ok(1.0),
            GameMode::Adventure => Ok(2.0),
            GameMode::Spectator => Ok(3.0),
        }
    }
}
