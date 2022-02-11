//! Some utilities for working with raw packets.

/// Returns the number of bytes needed to represent the specified number as a VarInt.
pub const fn var_i32_bytes(value: i32) -> usize {
    let mut len = 1;
    while len < 5 {
        if (value & -1) << (7 * len) == 0 {
            return len;
        }
        len += 1;
    }
    return 5;
}

/// Returns the specified byte slice as a hexdump
pub fn hex_string(bytes: &[u8]) -> String {
    let mut result = bytes.iter()
        .fold(String::new(), |mut a, b| {
            use std::fmt::Write;
            write!(a, "{:02X} ", b).unwrap();
            a
        });
    // remove trailing whitespace
    result.truncate(result.trim_end_matches(' ').len());
    
    result
}
