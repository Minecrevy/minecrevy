//! An implementation of the Minecraft Anvil file format (`.mca`).

use std::{
    fs::File,
    io,
    path::{Path, PathBuf},
};

use bevy::utils::{Entry, HashMap};
use minecrevy_chunk::ChunkPos;
use minecrevy_nbt::Blob;

pub use self::{pos::*, region::*};
use crate::{
    pos::{RegionLocalChunkPos, RegionPos},
    region::AnvilRegion,
};

mod pos;
mod region;

/// A folder of Minecraft region files.
pub struct AnvilFolder {
    /// The folder where region files are stored.
    folder: PathBuf,
    /// The currently open region files.
    open: HashMap<RegionPos, AnvilRegion<File>>,
}

impl AnvilFolder {
    /// Constructs a new folder to load region files from.
    pub fn new(folder: impl Into<PathBuf>) -> Self {
        Self {
            folder: folder.into(),
            open: HashMap::default(),
        }
    }

    /// Returns the folder that region files are loaded from.
    pub fn folder(&self) -> &Path {
        &self.folder
    }

    /// Returns `true` if the chunk at the specified [`ChunkPos`] exists.
    pub fn contains(&mut self, pos: ChunkPos) -> bool {
        let (region, local) = Self::split(pos);

        if let Some(region) = self.open.get_mut(&region) {
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

    /// Writes the specified chunk [`Blob`] at the specified [`ChunkPos`].
    pub fn write(&mut self, pos: ChunkPos, chunk: Blob) -> io::Result<()> {
        let (region, local) = Self::split(pos);
        let region = self.region(region)?;

        region.write(local, chunk)
    }

    /// Closes/unloads the region at the specified [`RegionPos`].
    pub fn close(&mut self, pos: RegionPos) -> bool {
        self.open.remove(&pos).is_some()
    }

    /// Returns the number of chunks currently stored in the region at the specified [`RegionPos`].
    pub fn count_chunks(&mut self, pos: RegionPos) -> io::Result<u64> {
        self.region(pos)?.count()
    }

    fn region(&mut self, pos: RegionPos) -> io::Result<&mut AnvilRegion<File>> {
        match self.open.entry(pos) {
            Entry::Occupied(entry) => Ok(entry.into_mut()),
            Entry::Vacant(entry) => {
                let region = AnvilRegion::open(&self.folder, pos)?;
                Ok(entry.insert(region))
            }
        }
    }

    /// Splits the specified [`ChunkPos`] into its corresponding [`RegionPos`] and [`RegionLocalChunkPos`].
    fn split(chunk: ChunkPos) -> (RegionPos, RegionLocalChunkPos) {
        (chunk.into(), chunk.into())
    }
}
