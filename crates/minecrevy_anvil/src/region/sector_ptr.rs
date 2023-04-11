use std::{
    io::{self, SeekFrom},
    mem::size_of,
    num::NonZeroU32,
};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use crate::{pos::RegionLocalChunkPos, region::Filelike};

/// A table of 1024 [`SectorPtr`]s.
pub struct SectorPtrTable<F: Filelike> {
    file: F,
}

impl<F: Filelike> SectorPtrTable<F> {
    /// The number of [`SectorPtr`]s in the table.
    pub const LENGTH: usize = 1024;

    /// The total size in bytes of the table.
    pub const SIZE: usize = Self::LENGTH * SectorPtr::SIZE;

    /// The `file position` that the table starts at.
    const START_POSITION: usize = 0;

    /// Constructs a new table backed by the specified file.
    pub fn new(file: F) -> Self {
        Self { file }
    }

    /// Reads the [`SectorPtr`] at the specified [`RegionLocalChunkPos`].
    pub fn read(&mut self, pos: RegionLocalChunkPos) -> io::Result<Option<SectorPtr>> {
        // go to the sector ptr's table position
        self.seek(pos)?;
        // read the sector ptr's value
        let raw = self.file.read_u32::<BigEndian>()?;
        Ok(SectorPtr::new(raw))
    }

    /// Writes the [`SectorPtr`] at the specified [`RegionLocalChunkPos`].
    pub fn write(&mut self, pos: RegionLocalChunkPos, ptr: Option<SectorPtr>) -> io::Result<()> {
        // go to the sector ptr's table position
        self.seek(pos)?;
        // write the sector ptr's value
        let raw = SectorPtr::get(ptr);
        self.file.write_u32::<BigEndian>(raw)?;
        Ok(())
    }

    fn seek(&mut self, pos: RegionLocalChunkPos) -> io::Result<u64> {
        let position =
            (Self::START_POSITION as u64) + pos.as_table_index() * (SectorPtr::SIZE as u64);
        self.file.seek(SeekFrom::Start(position))
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct SectorPtr(NonZeroU32);

impl SectorPtr {
    /// The size in bytes of a single [`SectorPtr`].
    pub const SIZE: usize = size_of::<Self>();

    pub fn new(raw: u32) -> Option<Self> {
        NonZeroU32::new(raw).map(SectorPtr)
    }

    pub fn get(this: Option<Self>) -> u32 {
        this.map(|SectorPtr(raw)| raw.get()).unwrap_or(0)
    }

    /// The `index` of the first sector this [`SectorPtr`] points to.
    #[inline]
    pub fn index(&self) -> u32 {
        self.0.get() >> 8 & 0xFF_FF_FF
    }

    /// The number of sectors this [`SectorPtr`] points to.
    #[inline]
    pub fn len(&self) -> u8 {
        (self.0.get() & 0xFF) as u8
    }
}
