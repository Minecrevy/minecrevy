use std::{
    fs::File,
    io::{self, Read, Seek, Write},
    path::Path,
};

use minecrevy_nbt::Blob;

use crate::{
    pos::{RegionLocalChunkPos, RegionPos},
    region::{sector::Sectors, sector_ptr::SectorPtrTable, timestamp::TimestampTable},
};

mod sector;
mod sector_ptr;
mod timestamp;

/// Any I/O type that can work like a standard [`File`]:
/// something that implements [`Read`], [`Seek`], and [`Write`].
pub trait Filelike: Read + Seek + Write {}
impl<F: Read + Seek + Write> Filelike for F {}

/// A Minecraft region file.
pub struct AnvilRegion<F: Filelike> {
    file: F,
}

impl<F: Filelike> AnvilRegion<F> {
    /// Constructs a new region backed by the specified `file`.
    pub fn new(file: F) -> Self {
        Self { file }
    }

    /// Returns `true` if the chunk at the specified [`RegionLocalChunkPos`] exists.
    pub fn contains(&mut self, pos: RegionLocalChunkPos) -> bool {
        // checks if the sector ptr exists (is non-zero) for this chunk pos
        self.sector_ptr_table()
            .read(pos)
            .map(|v| v.is_some())
            .unwrap_or(false)
    }

    /// Reads the chunk [`Blob`] at the specified [`RegionLocalChunkPos`].
    pub fn read(&mut self, pos: RegionLocalChunkPos) -> io::Result<Option<Blob>> {
        let Some(ptr) = self.sector_ptr_table().read(pos)? else {
            // the chunk is not stored
            return Ok(None);
        };

        let data = self.sectors().read(ptr)?;
        Ok(Some(data))
    }

    /// Writes the specified chunk [`Blob`] at the specified [`RegionLocalChunkPos`].
    pub fn write(&mut self, pos: RegionLocalChunkPos, chunk: Blob) -> io::Result<()> {
        todo!()
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

impl AnvilRegion<File> {
    /// Opens a region in the specified `folder` at the given [`pos`][`RegionPos`].
    pub fn open(folder: &Path, pos: RegionPos) -> io::Result<Self> {
        let path = folder.join(format!("r.{}.{}.mca", pos.x, pos.y));
        let file = File::options().read(true).write(true).open(path)?;
        Ok(Self::new(file))
    }
}
