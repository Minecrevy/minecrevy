use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::io;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use minecrevy_chunk::ChunkPos;
use minecrevy_chunk::raw::RawChunk;

use crate::file::AnvilFile;
use crate::RegionPos;

/// Region manager of a given folder.
pub struct AnvilStorage {
    folder: PathBuf,
    files: HashMap<RegionPos, AnvilFile>,
}

impl AnvilStorage {
    /// Creates a new [`RegionStorage`] to manage regions for a given `folder`.
    pub fn new(folder: impl Into<PathBuf>) -> Self {
        Self {
            folder: folder.into(),
            files: HashMap::default(),
        }
    }

    /// Returns the `folder` this [`RegionStorage`] manages.
    pub fn folder(&self) -> &Path {
        &self.folder
    }

    /// Loads the [`RawChunk`] at the given [`chunk position`][`ChunkPos`].
    pub fn load_chunk(&mut self, pos: ChunkPos) -> io::Result<Option<RawChunk>> {
        let file = self.load(RegionPos::from(pos))?;
        file.read_chunk(pos)
    }

    /// Saves the [`RawChunk`] at the given [`chunk position`][`ChunkPos`].
    pub fn save_chunk(&mut self, pos: ChunkPos, chunk: RawChunk) -> io::Result<()> {
        let file = self.load(RegionPos::from(pos))?;
        file.write_chunk(pos, chunk)
    }

    /// Returns when the chunk at the given [`chunk position`][`ChunkPos`] was last written.
    pub fn chunk_last_written(&mut self, pos: ChunkPos) -> io::Result<Option<SystemTime>> {
        let file = self.load(RegionPos::from(pos))?;
        file.chunk_last_written(pos)
    }

    /// Loads the region at the given [`region position`][`RegionPos`], backed by a cache.
    fn load(&mut self, pos: RegionPos) -> io::Result<&mut AnvilFile> {
        match self.files.entry(pos) {
            Entry::Occupied(entry) => Ok(entry.into_mut()),
            Entry::Vacant(entry) => {
                let file = AnvilFile::open(&self.folder, pos)?;
                Ok(entry.insert(file))
            }
        }
    }

    /// Unloads the region at the given [`region position`][`RegionPos`] from the cache.
    pub fn unload(&mut self, pos: RegionPos) {
        self.files.remove(&pos);
    }
}
