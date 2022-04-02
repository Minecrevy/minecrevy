//! Some utilities for working with raw packets.


/// Returns the number of bytes needed to represent the specified number as a VarInt.
pub fn var_i32_bytes(value: i32) -> usize {
    for i in 1..5 {
        if (value & -1 << i * 7) == 0 {
            return i;
        }
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
