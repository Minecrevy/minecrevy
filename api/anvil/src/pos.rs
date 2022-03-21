use minecrevy_chunk::ChunkPos;
use std::fmt;

/// Region coordinates.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct RegionPos {
    /// A region's `X` coordinate.
    pub x: i32,
    /// A region's `Z` coordinate.
    pub z: i32,
}

impl RegionPos {
    pub const CHUNKS_PER_AXIS: i32 = 32;
}

impl fmt::Debug for RegionPos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "region({}, {})", self.x, self.z)
    }
}

impl From<ChunkPos> for RegionPos {
    fn from(chunk: ChunkPos) -> Self {
        Self {
            x: chunk.x / RegionPos::CHUNKS_PER_AXIS,
            z: chunk.z / RegionPos::CHUNKS_PER_AXIS,
        }
    }
}

/// Chunk coordinates in region-space (i.e. local to a region).
/// In comparison, [`ChunkPos`] are in world-space.
pub(crate) struct LocalChunkPos {
    x: i32,
    z: i32,
}

impl LocalChunkPos {
    pub fn as_table_index(&self) -> u64 {
        u64::try_from(self.x + self.z * RegionPos::CHUNKS_PER_AXIS).unwrap()
    }
}

impl From<ChunkPos> for LocalChunkPos {
    fn from(chunk: ChunkPos) -> Self {
        Self {
            x: chunk.x % RegionPos::CHUNKS_PER_AXIS,
            z: chunk.z % RegionPos::CHUNKS_PER_AXIS,
        }
    }
}
