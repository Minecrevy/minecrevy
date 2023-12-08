//! Configurable arguments for customizing how encode and decode operations
//! work for data types.
//!
//! - Want to use a VarInt? Set the `varint` flag in [`IntArgs`].
//! - Want to protect memory usage for strings? Set the `max_len` arg in [`StringArgs`].

/// Arguments for reading and writing 32-bit and 64-bit integers.
#[derive(Clone, Debug, Default)]
pub struct IntArgs {
    /// Specifies that the integer should be encoded and decoded in a
    /// variable-length format.
    ///
    /// Implementation details about VarInts can be found [here][1].
    ///
    /// [1]: https://wiki.vg/Protocol#VarInt_and_VarLong
    pub varint: bool,
}

/// Arguments for reading and writing strings.
#[derive(Clone, Debug, Default)]
pub struct StringArgs {
    /// Specifies that the encoded/decoded string should not exceed the specified
    /// length.
    ///
    /// Setting this option to [`None`] simply means there is no length checking.
    pub max_len: Option<usize>,
}

/// Arguments for reading and writing lists.
#[derive(Clone, Debug, Default)]
pub struct ListArgs<TArgs> {
    /// Specifies how the length of the encoded/decoded list should be calculated.
    pub length: ListLength,
    /// Allows the specification of arguments for the inner type being processed.
    ///
    /// For example, you may want to encode a [`Vec<String>`],
    /// but ensure that every string is at most some length by setting
    /// `inner.max_len` in [`StringArgs`].
    pub inner: TArgs,
}

/// Specifies how the length should be calculated when encoding or decoding a
/// collection of values.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub enum ListLength {
    /// Specifies that the collection should be prefixed with a length encoded
    /// as a VarInt.
    #[default]
    VarInt,
    /// Specifies that the collection should be prefixed with a length encoded
    /// as an `i8`.
    Byte,
    /// Specifies that the collection's length should be calculated based on
    /// the bytes remaining in the stream.
    Remaining,
}

/// Arguments for reading and writing arrays.
#[derive(Clone, Debug, Default)]
pub struct ArrayArgs<TArgs> {
    /// Allows the specification of arguments for the inner type being processed.
    pub inner: TArgs,
}

/// Arguments for reading and writing optional values.
#[derive(Clone, Debug, Default)]
pub struct OptionArgs<TArgs> {
    /// Specifies how the existence of an optional value should be calculated.
    pub tag: OptionTag,
    /// Allows the specification of options for the inner type being processed.
    ///
    /// For example, you may want to encode a [`Option<String>`],
    /// but ensure that the inner string is at most some length by setting
    /// `inner.max_len` in [`StringArgs`].
    pub inner: TArgs,
}

/// Specifies how the existence of an optional value is calculated.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub enum OptionTag {
    /// The optional value is known to exist through a prefixed boolean.
    #[default]
    Bool,
    /// The optional value is known to exist through the number of remaining
    /// bytes in the stream being greater than zero.
    Remaining,
}

/// Arguments for reading and writing NBT blobs.
#[derive(Clone, Debug, Default)]
pub struct NbtArgs {
    /// The compression algorithm
    pub compression: Compression,
    /// The maximum length of the blob.
    pub max_len: Option<usize>,
    /// The optional header used for serialization.
    pub header: Option<&'static str>,
}

/// The compression algorithm to be used for NBT blobs.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Compression {
    /// No compression.
    #[default]
    None,
    /// GZIP compression.
    GZip,
    /// ZLIB compression.
    ZLib,
}

/// Arguments for reading and writing 3-dimensional signed integer vectors.
#[derive(Clone, Debug, Default)]
pub struct IVec3Args {
    /// Whether the coordinate should be compressed to 64 bits.
    ///
    /// See the [position data type][1] for more info.
    ///
    /// [1]: https://wiki.vg/Protocol#Position
    pub compressed: bool,
}
