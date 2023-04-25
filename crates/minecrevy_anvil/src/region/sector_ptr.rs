use std::{
    io::{self, BufReader, Read, Seek, SeekFrom, Take, Write},
    mem::size_of,
    num::NonZeroU32,
    ops::Range,
};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use crate::pos::RegionLocalChunkPos;

/// A table of 1024 [`SectorPtr`]s.
pub struct SectorPtrTable<F> {
    file: F,
}

impl<F> SectorPtrTable<F> {
    /// The number of [`SectorPtr`]s in the table.
    pub const LENGTH: usize = 1024;

    /// The total size in bytes of the table.
    pub const SIZE: usize = Self::LENGTH * SectorPtr::SIZE;

    /// The `file position` that the table starts at.
    pub const START_POSITION: usize = 0;

    /// Constructs a new table backed by the specified file.
    pub fn new(file: F) -> Self {
        Self { file }
    }
}

impl<F: Seek + Read> SectorPtrTable<F> {
    /// Reads the [`SectorPtr`] at the specified [`RegionLocalChunkPos`].
    pub fn read(&mut self, pos: RegionLocalChunkPos) -> io::Result<Option<SectorPtr>> {
        // go to the sector ptr's table position
        self.seek(pos)?;
        // read the sector ptr's value
        let raw = self.file.read_u32::<BigEndian>()?;
        Ok(SectorPtr::new(raw))
    }

    /// Returns an iterator of all [`SectorPtr`]s in the table.
    pub fn iter(&mut self) -> io::Result<SectorPtrTableIter<&mut F>> {
        SectorPtrTableIter::new(&mut self.file)
    }

    /// Counts the number of non-zero [`SectorPtr`]s in the table.
    pub fn count(&mut self) -> io::Result<u64> {
        Ok(self
            .iter()?
            .flat_map(|(_, ptr)| ptr)
            .filter(|ptr| ptr.len() > 0)
            .count() as u64)
    }
}

impl<F: Seek + Write> SectorPtrTable<F> {
    /// Writes the [`SectorPtr`] at the specified [`RegionLocalChunkPos`].
    pub fn write(&mut self, pos: RegionLocalChunkPos, ptr: Option<SectorPtr>) -> io::Result<()> {
        // go to the sector ptr's table position
        self.seek(pos)?;
        // write the sector ptr's value
        let raw = SectorPtr::get(ptr);
        self.file.write_u32::<BigEndian>(raw)?;
        Ok(())
    }
}

impl<F: Seek> SectorPtrTable<F> {
    fn seek(&mut self, pos: RegionLocalChunkPos) -> io::Result<u64> {
        let position = Self::file_position(pos);
        self.file.seek(SeekFrom::Start(position))
    }

    fn file_position(pos: RegionLocalChunkPos) -> u64 {
        (Self::START_POSITION as u64) + pos.to_table_index() * (SectorPtr::SIZE as u64)
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

    /// Returns the range of indices that this [`SectorPtr`] points to.
    pub fn indices(&self) -> Range<u32> {
        let index = self.index();
        let len = u32::from(self.len());

        index..(index + len)
    }
}

pub struct SectorPtrTableIter<F> {
    file: BufReader<Take<F>>,
    index: u64,
    finished: bool,
}

impl<F: Seek + Read> SectorPtrTableIter<F> {
    pub fn new(mut file: F) -> io::Result<Self> {
        file.seek(SeekFrom::Start(SectorPtrTable::<F>::START_POSITION as u64))?;
        Ok(Self {
            file: BufReader::new(file.take(SectorPtrTable::<F>::SIZE as u64)),
            index: 0,
            finished: false,
        })
    }
}

impl<F: Read> Iterator for SectorPtrTableIter<F> {
    type Item = (RegionLocalChunkPos, Option<SectorPtr>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        // read the sector ptr's value
        let Ok(raw) = self.file.read_u32::<BigEndian>() else {
            self.finished = true;
            return None;
        };

        self.index += 1;
        let pos = RegionLocalChunkPos::from_table_index(self.index);

        Some((pos, SectorPtr::new(raw)))
    }
}
