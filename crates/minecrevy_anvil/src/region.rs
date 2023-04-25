use std::{
    fs::File,
    io::{self, Read, Seek, Write},
    path::Path,
    time::SystemTime,
};

use minecrevy_nbt::Blob;

use crate::{
    pos::{RegionLocalChunkPos, RegionPos},
    region::{
        sector::Sectors,
        sector_ptr::{SectorPtr, SectorPtrTable},
        timestamp::TimestampTable,
    },
};

mod sector;
mod sector_ptr;
mod timestamp;

/// A Minecraft region file.
pub struct AnvilRegion<F> {
    file: F,
}

/// Public API
impl<F> AnvilRegion<F> {
    /// Constructs a new region backed by the specified `file`.
    pub fn new(file: F) -> Self {
        Self { file }
    }
}

impl<F: Seek + Read> AnvilRegion<F> {
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

    /// Reads the timestamp of the last time the chunk at the specified [`RegionLocalChunkPos`] was modified.
    pub fn last_modified(&mut self, pos: RegionLocalChunkPos) -> io::Result<Option<SystemTime>> {
        let Some(timestamp) = self.timestamp_table().read(pos)? else {
                // the chunk is not stored
                return Ok(None);
            };

        Ok(Some(timestamp.into()))
    }

    /// Returns the number of chunks currently stored in the region.
    pub fn count(&mut self) -> io::Result<u64> {
        self.sector_ptr_table().count()
    }

    /// Iterates over all chunk [`Blob`]s currently stored in the region.
    pub fn iter(&mut self) -> io::Result<impl Iterator<Item = (RegionLocalChunkPos, Blob)> + '_> {
        let ptrs: Vec<(RegionLocalChunkPos, SectorPtr)> = self
            .sector_ptr_table()
            .iter()?
            .flat_map(|(pos, ptr)| Some((pos, ptr?)))
            .collect();

        Ok(ptrs
            .into_iter()
            .filter_map(|(pos, ptr)| Some((pos, self.sectors().read(ptr).ok()?))))
    }
}

impl<F: Seek + Write> AnvilRegion<F> {
    /// Writes the specified chunk [`Blob`] at the specified [`RegionLocalChunkPos`].
    pub fn write(&mut self, _pos: RegionLocalChunkPos, _chunk: Blob) -> io::Result<()> {
        todo!()
    }

    pub fn flush(&mut self) -> io::Result<()> {
        self.file.flush()
    }
}

/// Private Implementation
impl<F> AnvilRegion<F> {
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

/// Helper methods for [`File`]-based regions.
impl AnvilRegion<File> {
    pub fn open(file_path: &Path) -> io::Result<Self> {
        let file = File::options()
            .create(true)
            .read(true)
            .write(true)
            .open(file_path)?;
        Ok(Self::new(file))
    }

    /// Opens a region in the specified `folder` at the given [`pos`][`RegionPos`].
    pub fn open_at(folder: &Path, pos: RegionPos) -> io::Result<Self> {
        let path = folder.join(format!("r.{}.{}.mca", pos.x, pos.z));
        Self::open(&path)
    }
}
