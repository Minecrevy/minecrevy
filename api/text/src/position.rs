#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "minecrevy_io_str", derive(minecrevy_io_str::McRead, minecrevy_io_str::McWrite))]
#[cfg_attr(feature = "minecrevy_io_str", io_repr(u8))]
pub enum TextPosition {
    Chat = 0,
    System = 1,
    ActionBar = 2,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "minecrevy_io_str", derive(minecrevy_io_str::McRead, minecrevy_io_str::McWrite))]
#[cfg_attr(feature = "minecrevy_io_str", io_repr(varint))]
pub enum ChatVisibility {
    Enabled = 0,
    CommandsOnly = 1,
    Hidden = 2,
}
