//! I/O byte primitives for working with Minecraft protocol data types.

#[cfg(feature = "async-tokio")]
pub mod asynchronous;
pub mod blocking;
#[cfg(feature = "codec")]
pub mod codec;
pub mod packet;

/// Returns the number of bytes needed to represent the specified number as a VarInt.
pub fn var_i32_bytes(value: i32) -> usize {
    for i in 1..5 {
        if (value & -1 << i * 7) == 0 {
            return i;
        }
    }
    return 5;
}
