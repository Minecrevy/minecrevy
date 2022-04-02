//! The module containing code for manipulating [`Ticks`].

use std::time::Duration;
use thiserror::Error;

/// A number of `ticks`. Ticks are the standard unit of time measurement for the vanilla minecraft server.
///
/// The standard server operates on 20 ticks per second.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, Default)]
pub struct Ticks(pub i32);

impl Ticks {
    /// The number of ticks per second for vanilla.
    pub const TICKS_PER_SECOND: i32 = 20;

    /// The number of milliseconds per tick for vanilla.
    pub const MILLISECONDS_PER_TICK: i32 = 1000 / Self::TICKS_PER_SECOND;
}

impl From<Ticks> for i32 {
    #[inline]
    fn from(ticks: Ticks) -> Self {
        ticks.0
    }
}

/// The error type returned when a [`Duration`] to [`Ticks`] conversion fails.
#[derive(Error, Debug)]
#[error("{0:?} can't be represented as a tick amount")]
pub struct TryFromDurationError(pub Duration);

impl TryFrom<Duration> for Ticks {
    type Error = TryFromDurationError;

    fn try_from(duration: Duration) -> Result<Self, Self::Error> {
        let millis = i32::try_from(duration.as_millis()).map_err(|_| TryFromDurationError(duration))?;
        Ok(Self(millis / Self::MILLISECONDS_PER_TICK))
    }
}

/// The error type returned when a [`Ticks`] to [`Duration`] conversion fails.
#[derive(Error, Debug)]
#[error("{0:?} can't be represented as a duration")]
pub struct TryFromTicksError(pub Ticks);

impl TryFrom<Ticks> for Duration {
    type Error = TryFromTicksError;

    fn try_from(ticks: Ticks) -> Result<Self, Self::Error> {
        let millis = u64::try_from(ticks.0 * Ticks::MILLISECONDS_PER_TICK)
            .map_err(|_| TryFromTicksError(ticks))?;
        Ok(Duration::from_millis(millis))
    }
}
