/// The hand being used.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "minecrevy_io_str", derive(minecrevy_io_str::McRead, minecrevy_io_str::McWrite))]
#[cfg_attr(feature = "minecrevy_io_str", io_repr(varint))]
pub enum Hand {
    /// The main hand.
    Main = 0,
    /// The off hand.
    Off = 1,
}

/// The hand being used.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "bevy", derive(bevy::prelude::Component))]
#[cfg_attr(feature = "minecrevy_io_str", derive(minecrevy_io_str::McRead, minecrevy_io_str::McWrite))]
#[cfg_attr(feature = "minecrevy_io_str", io_repr(varint))]
pub enum MainHand {
    /// The left hand.
    Left = 0,
    /// The right hand.
    Right = 1,
}

impl Default for MainHand {
    fn default() -> Self {
        Self::Right
    }
}


