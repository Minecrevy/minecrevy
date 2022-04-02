/// The difficulty levels of Minecraft.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "minecrevy_io_str", derive(minecrevy_io_str::McRead, minecrevy_io_str::McWrite))]
#[cfg_attr(feature = "minecrevy_io_str", io_repr(u8))]
pub enum Difficulty {
    /// No mobs spawn.
    Peaceful = 0,
    /// Mobs spawn, but do little damage.
    Easy = 1,
    /// Mobs spawn, and do moderate damage.
    Normal = 2,
    /// Mobs spawn, and do a lot of damage.
    Hard = 3,
}
