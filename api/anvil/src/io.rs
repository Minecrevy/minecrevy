use std::io::{self, Read, Seek, Write};
use std::time::SystemTime;

use minecrevy_chunk::ChunkPos;
use minecrevy_chunk::raw::RawChunk;

use crate::LocalChunkPos;

pub(crate) use self::sector::*;
pub(crate) use self::sector_ptr::*;
pub(crate) use self::timestamp::*;

mod sector_ptr;
mod sector;
mod timestamp;

/// Handles region and chunk I/O for any underlying type that implements [`Read`], [`Seek`], and [`Write`].
pub struct AnvilIo<F: Read + Seek + Write> {
    file: F,
}

impl<F: Read + Seek + Write> AnvilIo<F> {
    pub fn new(file: F) -> Self {
        Self { file }
    }

    /// Reads the [`RawChunk`] at the given [`chunk position`][`ChunkPos`].
    pub fn read_chunk(&mut self, pos: ChunkPos) -> io::Result<Option<RawChunk>> {
        let pos = LocalChunkPos::from(pos);

        let ptr = match self.sector_ptr_table().read(pos)? {
            Some(ptr) => ptr,
            None => return Ok(None),
        };

        let data = self.sectors().read(ptr)?;

        Ok(Some(RawChunk::new(data)))
    }

    /// Writes the [`RawChunk`] at the given [`chunk position`][`ChunkPos`].
    pub fn write_chunk(&mut self, _pos: ChunkPos, _chunk: RawChunk) -> io::Result<()> {
        todo!()
    }

    /// Returns when the chunk at the given [`chunk position`][`ChunkPos`] was last written.
    pub fn chunk_last_written(&mut self, pos: ChunkPos) -> io::Result<Option<SystemTime>> {
        let pos = LocalChunkPos::from(pos);

        let timestamp = match self.timestamp_table().read(pos)? {
            Some(timestamp) => timestamp,
            None => return Ok(None),
        };

        Ok(Some(SystemTime::from(timestamp)))
    }

    fn sector_ptr_table(&mut self) -> SectorPtrTable<&mut F> {
        SectorPtrTable::new(&mut self.file)
    }

    fn timestamp_table(&mut self) -> TimestampTable<&mut F> {
        TimestampTable::new(&mut self.file)
    }

    fn sectors(&mut self) -> Sectors<&mut F> {
        Sectors::new(&mut self.file)
    }
}
