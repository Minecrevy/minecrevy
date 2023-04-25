//! An implementation of the Minecraft Anvil file format (`.mca`).

use std::{
    fs::{File, ReadDir},
    io,
    path::{Path, PathBuf},
    time::SystemTime,
};

use bevy::utils::{Entry, HashMap};
use minecrevy_chunk::ChunkPos;
use minecrevy_nbt::Blob;
use once_cell::sync::Lazy;
use regex::Regex;

pub use self::{pos::*, region::*};
use crate::{pos::RegionLocalChunkPos, region::AnvilRegion};

mod pos;
mod region;

/// A folder of Minecraft region files.
pub struct AnvilStorage {
    /// The folder where region files are stored.
    folder: PathBuf,
    /// The currently cached (open) region files.
    cache: HashMap<RegionPos, AnvilRegion<File>>,
}

impl AnvilStorage {
    /// Constructs a new folder to load region files from.
    pub fn new(folder: impl Into<PathBuf>) -> Self {
        Self {
            folder: folder.into(),
            cache: HashMap::default(),
        }
    }

    /// Returns the folder that region files are loaded from.
    pub fn folder(&self) -> &Path {
        &self.folder
    }

    /// Returns `true` if the chunk at the specified [`ChunkPos`] exists.
    pub fn contains(&mut self, pos: ChunkPos) -> bool {
        let (region, local) = Self::split(pos);

        if let Some(region) = self.cache.get_mut(&region) {
            region.contains(local)
        } else {
            false
        }
    }

    /// Reads the chunk [`Blob`] at the specified [`ChunkPos`].
    pub fn read(&mut self, pos: ChunkPos) -> io::Result<Option<Blob>> {
        let (region, local) = Self::split(pos);
        let region = self.region(region)?;

        region.read(local)
    }

    /// Reads the last time the chunk at the specified [`ChunkPos`] was modified.
    pub fn last_modified(&mut self, pos: ChunkPos) -> io::Result<Option<SystemTime>> {
        let (region, local) = Self::split(pos);
        let region = self.region(region)?;

        region.last_modified(local)
    }

    /// Writes the specified chunk [`Blob`] at the specified [`ChunkPos`].
    pub fn write(&mut self, pos: ChunkPos, chunk: Blob) -> io::Result<()> {
        let (region, local) = Self::split(pos);
        let region = self.region(region)?;

        region.write(local, chunk)
    }

    /// Closes/unloads the region at the specified [`RegionPos`].
    pub fn close(&mut self, pos: RegionPos) -> bool {
        self.cache.remove(&pos).is_some()
    }

    /// Returns the number of chunks currently stored in the region at the specified [`RegionPos`].
    pub fn count_chunks(&mut self, pos: RegionPos) -> io::Result<u64> {
        self.region(pos)?.count()
    }

    pub fn regions(&mut self) -> io::Result<RegionsIter> {
        RegionsIter::new(self)
    }

    pub fn region(&mut self, pos: RegionPos) -> io::Result<&mut AnvilRegion<File>> {
        std::fs::create_dir_all(&self.folder)?;

        match self.cache.entry(pos) {
            Entry::Occupied(entry) => Ok(entry.into_mut()),
            Entry::Vacant(entry) => {
                let region = AnvilRegion::open_at(&self.folder, pos)?;
                Ok(entry.insert(region))
            }
        }
    }

    /// Splits the specified [`ChunkPos`] into its corresponding [`RegionPos`] and [`RegionLocalChunkPos`].
    fn split(chunk: ChunkPos) -> (RegionPos, RegionLocalChunkPos) {
        (chunk.into(), chunk.into())
    }
}

pub struct RegionsIter {
    entries: ReadDir,
}

impl RegionsIter {
    fn new(storage: &mut AnvilStorage) -> io::Result<Self> {
        Ok(Self {
            entries: storage.folder.read_dir()?,
        })
    }
}

impl Iterator for RegionsIter {
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
        let y: i32 = captures.get(2).unwrap().as_str().parse().ok()?;
        Some(RegionPos::new(x, y))
    }
}
