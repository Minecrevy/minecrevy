use minecrevy_io_str_derive::{McRead, McWrite};
use crate::ListLength;

#[derive(McRead, McWrite)]
struct Test {
    #[options(varint = true)]
    len: i32,
    #[options(max_len = 16)]
    name: String,
    b: u8,
    #[options(length = ListLength::VarInt, inner.max_len = 16)]
    values: Vec<String>,
}

#[derive(McRead, McWrite)]
#[io_repr(varint)]
enum DiscriminantTest {
    AddPlayer = 0,
    SetGameMode = 1,
    SetPing = 2,
    SetDisplayName = 3,
    RemovePlayer = 4,
}

#[derive(McRead, McWrite)]
#[io_repr(varint)]
enum EnumTest {
    AddPlayer(String),
    SetGameMode(i32),
    SetPing(#[options(varint = true)] i32),
    SetDisplayName(String),
    RemovePlayer(u8),
}

