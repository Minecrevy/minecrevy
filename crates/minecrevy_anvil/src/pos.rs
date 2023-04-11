use bevy::prelude::*;
use minecrevy_chunk::ChunkPos;

/// The number of chunks per axis of a single region.
const CHUNKS_PER_REGION_AXIS: i32 = 32;

/// Region coordinates.
#[derive(Component, Deref, DerefMut)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Default)]
pub struct RegionPos(IVec2);

impl From<ChunkPos> for RegionPos {
    fn from(chunk: ChunkPos) -> Self {
        Self(IVec2 {
            x: chunk.x / CHUNKS_PER_REGION_AXIS,
            y: chunk.y / CHUNKS_PER_REGION_AXIS,
        })
    }
}

/// Region-local [`ChunkPos`]. In comparison, normal [`ChunkPos`] are world-local.
#[derive(Component, Deref, DerefMut)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Default)]
pub struct RegionLocalChunkPos(UVec2);

impl RegionLocalChunkPos {
    /// Converts this region-local chunk position into an index for the offset and timestamp table.
    pub fn as_table_index(&self) -> u64 {
        (self.x + self.y * CHUNKS_PER_REGION_AXIS as u32).into()
    }
}

impl From<ChunkPos> for RegionLocalChunkPos {
    fn from(chunk: ChunkPos) -> Self {
        Self(UVec2 {
            x: (chunk.x % CHUNKS_PER_REGION_AXIS) as u32,
            y: (chunk.y % CHUNKS_PER_REGION_AXIS) as u32,
        })
    }
}
