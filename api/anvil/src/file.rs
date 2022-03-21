use std::{fmt, io};
use std::fs::File;
use std::path::Path;
use std::time::SystemTime;

use minecrevy_chunk::ChunkPos;
use minecrevy_chunk::raw::RawChunk;

use crate::io::AnvilIo;
use crate::RegionPos;

/// A currently open region (`.mca`) file. A single region contains a `32x32` section of chunks.
pub struct AnvilFile {
    pos: RegionPos,
    io: AnvilIo<File>,
}

impl AnvilFile {
    pub fn open(folder: impl AsRef<Path>, pos: RegionPos) -> io::Result<Self> {
        let path = folder.as_ref().join(format!("r.{}.{}.mca", pos.x, pos.z));
        let file = File::options().read(true).write(true).open(path)?;

        Ok(Self {
            pos,
            io: AnvilIo::new(file),
        })
    }

    /// Reads the [`RawChunk`] at the given [`chunk position`][`ChunkPos`].
    #[inline]
    pub fn read_chunk(&mut self, pos: ChunkPos) -> io::Result<Option<RawChunk>> {
        self.io.read_chunk(pos)
    }

    /// Writes the [`RawChunk`] at the given [`chunk position`][`ChunkPos`].
    #[inline]
    pub fn write_chunk(&mut self, pos: ChunkPos, chunk: RawChunk) -> io::Result<()> {
        self.io.write_chunk(pos, chunk)
    }

    /// Returns when the chunk at the given [`chunk position`][`ChunkPos`] was last written.
    #[inline]
    pub fn chunk_last_written(&mut self, pos: ChunkPos) -> io::Result<Option<SystemTime>> {
        self.io.chunk_last_written(pos)
    }
}

impl fmt::Debug for AnvilFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AnvilFile")
            .field("pos", &self.pos)
            .finish()
    }
}
