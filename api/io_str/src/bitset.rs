use crate::{McRead, McWrite};

/// A network-compatible version of Java's [`BitSet`].
///
/// [`BitSet`]: https://docs.oracle.com/javase/8/docs/api/java/util/BitSet.html
#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, McRead, McWrite)]
pub struct BitSet {
    bits: Vec<u64>,
}

impl BitSet {
    /// Constructs a new, empty [`BitSet`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Constructs a [`BitSet`] from the provided [`Vec<u64>`].
    pub fn from_vec(bits: Vec<u64>) -> Self {
        Self { bits }
    }

    /// Returns true if the bit at the provided index is set.
    pub fn get(&self, bit_idx: usize) -> bool {
        let word_idx = Self::word_idx(bit_idx);
        if let Some(&word) = self.bits.get(word_idx) {
            word & (1 << bit_idx) != 0
        } else {
            false
        }
    }

    /// Sets the bit at the provided index.
    pub fn set(&mut self, bit_idx: usize) {
        let word_idx = Self::word_idx(bit_idx);
        self.ensure_capacity(word_idx);

        self.bits[word_idx] |= 1 << bit_idx;
    }

    /// Unsets the bit at the provided index.
    pub fn unset(&mut self, bit_idx: usize) {
        let word_idx = Self::word_idx(bit_idx);

        if let Some(word) = self.bits.get_mut(word_idx) {
            *word &= !(1 << bit_idx);
        }
    }

    /// Flips the bit at the provided index.
    pub fn flip(&mut self, bit_idx: usize) {
        let word_idx = Self::word_idx(bit_idx);
        self.ensure_capacity(word_idx);

        self.bits[word_idx] ^= 1 << bit_idx;
    }

    /// Gets the number of addressable bits in this set.
    pub fn len(&self) -> usize {
        self.bits.len() * u64::BITS as usize
    }

    /// Extracts a slice containing the entire bit set.
    pub fn as_slice(&self) -> &[u64] {
        self.bits.as_slice()
    }

    /// Extracts a mutable slice containing the entire bit set.
    pub fn as_mut_slice(&mut self) -> &mut [u64] {
        self.bits.as_mut_slice()
    }

    /// Consumes `self`, returning the internal [`Vec<u64>`].
    pub fn into_vec(self) -> Vec<u64> {
        self.bits
    }
}

impl BitSet {
    fn ensure_capacity(&mut self, word_idx: usize) {
        let ensure_len = word_idx + 1;
        while self.bits.len() < ensure_len {
            self.bits.push(0);
        }
    }

    fn word_idx(bit_idx: usize) -> usize {
        bit_idx >> 6
    }
}
