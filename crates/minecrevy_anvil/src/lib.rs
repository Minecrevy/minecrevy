use std::{fs::ReadDir, io, path::PathBuf};

use minecrevy_chunk::ChunkPos;
use minecrevy_nbt::Blob;
use once_cell::sync::Lazy;
use regex::Regex;
use schnellru::{ByLength, LruMap};
use serde::{de::DeserializeOwned, Serialize};

pub use self::region::*;

mod region;

/// A collection of Minecraft region files stored in a single folder.
pub struct AnvilStorage {
    /// The folder all accessible regions are stored in.
    folder: PathBuf,
    /// The currently opened region files.
    cache: LruMap<RegionPos, AnvilRegion>,
}

impl AnvilStorage {
    /// The maximum size of the LRU region cache.
    const CACHE_SIZE: u32 = 256;

    /// Constructs an anvil region storage from the specified `folder`.
    pub fn new(folder: impl Into<PathBuf>) -> Self {
        Self {
            folder: folder.into(),
            cache: LruMap::new(ByLength::new(Self::CACHE_SIZE)),
        }
    }

    /// Reads the [chunk](Blob) at the specified [pos](ChunkPos).
    pub fn read_blob(&mut self, pos: ChunkPos) -> io::Result<Option<Blob>> {
        let region = self.region(RegionPos::from(pos))?;

        region.read_blob(RegionLocalChunkPos::from(pos))
    }

    /// Reads the chunk at the specified [pos](ChunkPos).
    pub fn read<T: DeserializeOwned>(&mut self, pos: ChunkPos) -> io::Result<Option<T>> {
        let region = self.region(RegionPos::from(pos))?;

        region.read::<T>(RegionLocalChunkPos::from(pos))
    }

    /// Writes the provided [chunk](Blob) at the specified [pos](ChunkPos).
    pub fn write_blob(&mut self, pos: ChunkPos, chunk: Option<Blob>) -> io::Result<()> {
        let region = self.region(RegionPos::from(pos))?;

        if let Some(chunk) = chunk {
            region.write_blob(RegionLocalChunkPos::from(pos), chunk)
        } else {
            region.remove(RegionLocalChunkPos::from(pos))
        }
    }

    /// Writes the provided chunk at the specified [pos](ChunkPos).
    pub fn write<T: Serialize>(&mut self, pos: ChunkPos, chunk: Option<T>) -> io::Result<()> {
        let region = self.region(RegionPos::from(pos))?;

        if let Some(chunk) = chunk {
            region.write(RegionLocalChunkPos::from(pos), chunk)
        } else {
            region.remove(RegionLocalChunkPos::from(pos))
        }
    }

    /// Constructs an iterator over all [`RegionPos`] contained in the folder.
    pub fn regions(&self) -> io::Result<RegionPosIter> {
        RegionPosIter::new(self)
    }

    /// Returns the [`AnvilRegion`] containing the specified [`ChunkPos`].
    /// May fail during file validation.
    pub fn region(&mut self, pos: RegionPos) -> io::Result<&mut AnvilRegion> {
        // ensure the parent folder exists so we can create region files as necessary
        std::fs::create_dir_all(&self.folder)?;

        self.cache
            .get_or_insert_fallible(pos, || {
                let file_path = self.folder.join(pos.as_filename());
                AnvilRegion::new(&file_path)
            })
            .map(|v| v.unwrap_or_else(|| unreachable!()))
    }
}

/// An iterator of all [`RegionPos`] currently stored in an [`AnvilStorage`] folder.
pub struct RegionPosIter {
    entries: ReadDir,
}

impl RegionPosIter {
    fn new(storage: &AnvilStorage) -> io::Result<Self> {
        Ok(Self {
            entries: storage.folder.read_dir()?,
        })
    }
}

impl Iterator for RegionPosIter {
    type Item = RegionPos;

    fn next(&mut self) -> Option<Self::Item> {
        const FILE_NAME_REGEX: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"r\.(\-?\d+)\.(\-?\d+)\.mca").unwrap());

        let entry = self.entries.next()?.ok()?;
        let file_type = entry.file_type().ok()?;
        if !file_type.is_file() {
            return None;
        }

        let file_name = entry.file_name();
        let file_name = file_name.to_str()?;
        let captures = FILE_NAME_REGEX.captures(file_name)?;
        let x: i32 = captures.get(1).unwrap().as_str().parse().ok()?;
        let z: i32 = captures.get(2).unwrap().as_str().parse().ok()?;
        Some(RegionPos { x, z })
    }
}
