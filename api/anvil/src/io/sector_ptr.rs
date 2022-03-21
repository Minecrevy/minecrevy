use std::io::{self, Read, Seek, SeekFrom, Write};
use std::mem::size_of;
use std::num::NonZeroU32;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use crate::pos::LocalChunkPos;

pub struct SectorPtrTable<F: Read + Seek + Write> {
    file: F,
}

impl<F: Read + Seek + Write> SectorPtrTable<F> {
    /// The number of [`SectorPtr`]s in the table.
    pub const LENGTH: usize = 1024;

    /// The total size in bytes of the table.
    pub const SIZE: usize = Self::LENGTH * SectorPtr::SIZE;

    #[inline]
    pub fn new(file: F) -> Self {
        Self { file }
    }

    pub(crate) fn read(&mut self, pos: LocalChunkPos) -> io::Result<Option<SectorPtr>> {
        // Seek to the sector pointer's position in the table.
        self.seek(pos)?;
        // Read the sector pointer's value from the table.
        let raw = self.file.read_u32::<BigEndian>()?;
        Ok(SectorPtr::from_raw(raw))
    }

    pub(crate) fn write(&mut self, pos: LocalChunkPos, ptr: Option<SectorPtr>) -> io::Result<()> {
        // Seek to the sector pointer's position in the table.
        self.seek(pos)?;
        // Write the sector pointer's value to the table.
        let raw = SectorPtr::into_raw(ptr);
        self.file.write_u32::<BigEndian>(raw)
    }

    #[inline]
    fn seek(&mut self, pos: LocalChunkPos) -> io::Result<u64> {
        let position = pos.as_table_index() * (SectorPtr::SIZE as u64);
        self.file.seek(SeekFrom::Start(position))
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct SectorPtr(NonZeroU32);

impl SectorPtr {
    /// The size in bytes of a single [`SectorPtr`].
    pub const SIZE: usize = size_of::<Self>();

    fn from_raw(raw: u32) -> Option<Self> {
        NonZeroU32::new(raw)
            .map(SectorPtr)
    }

    fn into_raw(this: Option<Self>) -> u32 {
        this.map(|SectorPtr(raw)| raw.get())
            .unwrap_or(0)
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
