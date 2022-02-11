//! Configurable options for customizing how encode and decode operations operate for data types.
//!
//! - Want to use a VarInt? Set the `varint` flag in [`IntOptions`].
//! - Want to protect memory usage for strings? Set the `max_len` option in [`StringOptions`].
//!
//! It's really that simple!

/// Configurable options for parsing [`i32`]s and [`i64`]s in the Minecraft protocol.
#[derive(Clone, Debug, Default)]
pub struct IntOptions {
    /// Specifies that the integer should be encoded and decoded in a variable-length format.
    ///
    /// Implementation details about VarInts can be found [here][1].
    ///
    /// [1]: https://wiki.vg/Protocol#VarInt_and_VarLong
    pub varint: bool,
}

/// Configurable options for parsing [`String`]s in the Minecraft protocol.
#[derive(Clone, Debug, Default)]
pub struct StringOptions {
    /// Specifies that the encoded/decoded string should not exceed the specified length.
    ///
    /// Setting this option to [`None`] simply means there is no length checking.
    pub max_len: Option<usize>,
}

/// Configurable options for parsing lists of things in the Minecraft protocol.
///
/// The Minecraft protocol can be pretty arbitrary in its execution of "serialize multiple of this type."
#[derive(Clone, Debug, Default)]
pub struct ListOptions<TOptions> {
    /// Specifies how the length of the encoded/decoded list should be calculated.
    pub length: ListLength,
    /// Allows the specification of options for the inner type being processed.
    ///
    /// For example, you may want to encode a [`Vec<String>`],
    /// but ensure that every string is at most some length by setting `inner.max_len` in [`StringOptions`].
    pub inner: TOptions,
}

/// Configurable options for parsing exact sequences of things in the Minecraft protocol.
#[derive(Clone, Debug, Default)]
pub struct ArrayOptions<TOptions> {
    /// Allows the specification of options for the inner type being processed.
    pub inner: TOptions,
}

/// Specifies how the length should be calculated when encoding or decoding a collection of values.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum ListLength {
    /// Specifies that the collection should be prefixed with a length encoded as a VarInt.
    VarInt,
    /// Specifies that the collection's length should be calculated from the bytes remaining in the stream.
    Remaining,
}

impl From<&str> for ListLength {
    fn from(v: &str) -> Self {
        match v {
            "varint" => Self::VarInt,
            "remaining" => Self::Remaining,
            _ => panic!("invalid length option"),
        }
    }
}

impl Default for ListLength {
    fn default() -> Self {
        Self::VarInt
    }
}

/// Configurable options for parsing an optionally present value in the Minecraft protocol.
#[derive(Clone, Debug, Default)]
pub struct OptionOptions<TOptions> {
    /// Specifies how the existence of an optional value should be calculated.
    pub existence: OptionExistence,
    /// Allows the specification of options for the inner type being processed.
    ///
    /// For example, you may want to encode a [`Option<String>`],
    /// but ensure that every string is at most some length by setting `inner.max_len` in [`StringOptions`].
    pub inner: TOptions,
}

/// Specifies how the existence of an optional value is calculated.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum OptionExistence {
    /// Specifies that the optional value is known to exist through a prefixed boolean.
    Bool,
    /// Specifies that the optional value is known to exist through the number of remaining bytes in the stream.
    Remaining,
}

impl From<&str> for OptionExistence {
    fn from(v: &str) -> Self {
        match v {
            "bool" => Self::Bool,
            "remaining" => Self::Remaining,
            _ => panic!("invalid existence option"),
        }
    }
}

impl Default for OptionExistence {
    fn default() -> Self {
        Self::Bool
    }
}
