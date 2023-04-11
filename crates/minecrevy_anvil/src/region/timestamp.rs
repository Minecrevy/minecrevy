use std::{
    io::{self, SeekFrom},
    mem::size_of,
    num::NonZeroU32,
    time::{Duration, SystemTime},
};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use crate::{
    pos::RegionLocalChunkPos,
    region::{sector_ptr::SectorPtrTable, Filelike},
};

/// A table of 1024 [`Timestamp`]s.
pub struct TimestampTable<F: Filelike> {
    file: F,
}

impl<F: Filelike> TimestampTable<F> {
    /// The number of [`Timestamp`]s in the table.
    pub const LENGTH: usize = 1024;

    /// The total size in bytes of the table.
    pub const SIZE: usize = Self::LENGTH * Timestamp::SIZE;

    /// The `file position` that the table starts at.
    const START_POSITION: usize = SectorPtrTable::<F>::SIZE;

    /// Constructs a new table backed by the specified file.
    pub fn new(file: F) -> Self {
        Self { file }
    }

    /// Reads the [`Timestamp`] at the specified [`RegionLocalChunkPos`].
    pub fn read(&mut self, pos: RegionLocalChunkPos) -> io::Result<Option<Timestamp>> {
        // Seek to the timestamp's position in the table.
        self.seek(pos)?;
        // Read the timestamp's value from the table.
        let raw = self.file.read_u32::<BigEndian>()?;
        Ok(Timestamp::new(raw))
    }

    /// Writes the [`Timestamp`] at the specified [`RegionLocalChunkPos`].
    pub fn write(
        &mut self,
        pos: RegionLocalChunkPos,
        timestamp: Option<Timestamp>,
    ) -> io::Result<()> {
        // Seek to the timestamp's position in the table.
        self.seek(pos)?;
        // Write the timestamp's value to the table.
        let raw = Timestamp::get(timestamp);
        self.file.write_u32::<BigEndian>(raw)
    }

    fn seek(&mut self, pos: RegionLocalChunkPos) -> io::Result<u64> {
        let position =
            (Self::START_POSITION as u64) + pos.as_table_index() * (Timestamp::SIZE as u64);
        self.file.seek(SeekFrom::Start(position))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Timestamp(NonZeroU32);

impl Timestamp {
    /// The size in bytes of a single [`Timestamp`].
    pub const SIZE: usize = size_of::<Self>();

    pub fn new(raw: u32) -> Option<Self> {
        NonZeroU32::new(raw).map(Timestamp)
    }

    pub fn get(this: Option<Self>) -> u32 {
        this.map(|Timestamp(raw)| raw.get()).unwrap_or(0)
    }

    pub fn from_system_time(time: SystemTime) -> Option<Self> {
        let duration = time.duration_since(SystemTime::UNIX_EPOCH).ok()?;
        let epoch_secs = u32::try_from(duration.as_secs())
            .expect("can't represent epoch seconds as u32 anymore");
        Self::new(epoch_secs)
    }
}

impl From<Timestamp> for SystemTime {
    fn from(timestamp: Timestamp) -> Self {
        SystemTime::UNIX_EPOCH + Duration::from_secs(timestamp.0.get() as u64)
    }
}
