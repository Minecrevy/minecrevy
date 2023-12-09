//! Utility functions for the `minecrevy_io` crate.

use std::sync::OnceLock;

/// Returns the number of bytes required to encode the given value as a varint.
pub fn varint_bytes(value: i32) -> usize {
    static VARINT_EXACT_BYTE_LENGTHS: OnceLock<[usize; 33]> = OnceLock::<[usize; 33]>::new();

    VARINT_EXACT_BYTE_LENGTHS.get_or_init(|| {
        std::array::from_fn(|i| {
            if i == 32 {
                // Special case for zero
                1
            } else {
                ((31.0 - (i as f64 - 1.0)) / 7.0).ceil() as usize
            }
        })
    })[value.leading_zeros() as usize]
}
