use core::fmt;

use bevy::prelude::*;
use minecrevy_chunk::ChunkPos;

/// The number of chunks per axis of a single region.
const CHUNKS_PER_REGION_AXIS: i32 = 32;

/// Region coordinates.
#[derive(Component)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Default)]
pub struct RegionPos {
    pub x: i32,
    pub z: i32,
}

impl RegionPos {
    pub fn new(x: i32, z: i32) -> Self {
        Self { x, z }
    }
}

impl From<ChunkPos> for RegionPos {
    fn from(chunk: ChunkPos) -> Self {
        Self {
            x: chunk.x / CHUNKS_PER_REGION_AXIS,
            z: chunk.z / CHUNKS_PER_REGION_AXIS,
        }
    }
}

impl fmt::Display for RegionPos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { x, z } = self;
        write!(f, "region({x}, {z})")
    }
}

/// Region-local [`ChunkPos`]. In comparison, normal [`ChunkPos`] are world-local.
#[derive(Component)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Default)]
pub struct RegionLocalChunkPos {
    pub x: u32,
    pub z: u32,
}

impl RegionLocalChunkPos {
    /// Converts this region-local chunk position into an index for the offset and timestamp table.
    pub fn to_table_index(&self) -> u64 {
        (self.x + self.z * CHUNKS_PER_REGION_AXIS as u32).into()
    }

    pub fn from_table_index(index: u64) -> Self {
        Self {
            x: (index % CHUNKS_PER_REGION_AXIS as u64) as u32,
            z: (index / CHUNKS_PER_REGION_AXIS as u64) as u32,
        }
    }

    pub fn to_world(&self, region: RegionPos) -> ChunkPos {
        ChunkPos {
            x: (region.x * CHUNKS_PER_REGION_AXIS) + (self.x as i32),
            z: (region.z * CHUNKS_PER_REGION_AXIS) + (self.z as i32),
        }
    }
}

impl From<ChunkPos> for RegionLocalChunkPos {
    fn from(chunk: ChunkPos) -> Self {
        Self {
            x: (chunk.x.abs() % CHUNKS_PER_REGION_AXIS) as u32,
            z: (chunk.z.abs() % CHUNKS_PER_REGION_AXIS) as u32,
        }
    }
}

impl fmt::Display for RegionLocalChunkPos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { x, z } = self;
        write!(f, "localchunk({x}, {z})")
    }
}

#[cfg(test)]
mod tests {
    use minecrevy_chunk::ChunkPos;

    use crate::{pos::RegionLocalChunkPos, RegionPos};

    #[test]
    fn to_world() {
        {
            let chunk = ChunkPos { x: 0, z: 0 };
            let region = RegionPos::from(chunk);
            let local = RegionLocalChunkPos::from(chunk);

            assert_eq!(chunk, local.to_world(region));
        }
        {
            let chunk = ChunkPos { x: 1, z: 1 };
            let region = RegionPos::from(chunk);
            let local = RegionLocalChunkPos::from(chunk);

            assert_eq!(chunk, local.to_world(region));
        }
        {
            let chunk = ChunkPos { x: 4653, z: -626 };
            let region = RegionPos::from(chunk);
            let local = RegionLocalChunkPos::from(chunk);

            assert_eq!(chunk, local.to_world(region));
        }
        {
            let chunk = ChunkPos { x: -4, z: -66 };
            let region = RegionPos::from(chunk);
            let local = RegionLocalChunkPos::from(chunk);

            assert_eq!(chunk, local.to_world(region));
        }
    }

    #[test]
    fn table_index_round_trip() {
        {
            let pos = RegionLocalChunkPos { x: 0, z: 0 };
            let index = pos.to_table_index();
            assert_eq!(pos, RegionLocalChunkPos::from_table_index(index))
        }
        {
            let pos = RegionLocalChunkPos { x: 1, z: 1 };
            let index = pos.to_table_index();
            assert_eq!(pos, RegionLocalChunkPos::from_table_index(index))
        }
        {
            let pos = RegionLocalChunkPos { x: 17, z: 23 };
            let index = pos.to_table_index();
            assert_eq!(pos, RegionLocalChunkPos::from_table_index(index))
        }
    }
}
