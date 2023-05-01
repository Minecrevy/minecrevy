use std::{
    io::{self, Cursor, Read, Write},
    marker::PhantomData,
    mem::size_of,
    num::NonZeroU32,
    ops::Range,
    time::{Duration, SystemTime},
};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use thiserror::Error;

use crate::region::{sector::Sectors, AnvilRegion, RegionLocalChunkPos};

pub fn read_header(
    mut reader: impl Read,
) -> io::Result<(HeaderTable<Offset>, HeaderTable<Timestamp>)> {
    const TOTAL_SIZE: usize = HeaderTable::<Offset>::SIZE + HeaderTable::<Timestamp>::SIZE;

    let mut data = vec![0u8; TOTAL_SIZE];
    let read = reader.read(&mut data)?;
    if read > 0 && read != TOTAL_SIZE {
        tracing::warn!(
            "region file has truncated header: {read} bytes < {} bytes",
            TOTAL_SIZE
        );
    }

    let mut offsets = data;
    let timestamps = offsets.split_off(HeaderTable::<Offset>::SIZE);

    Ok((HeaderTable::from(offsets), HeaderTable::from(timestamps)))
}

#[derive(Clone, Debug)]
pub struct HeaderTable<V: HeaderValue> {
    data: Vec<u8>,
    _value_type: PhantomData<fn() -> V>,
}

impl<V: HeaderValue> HeaderTable<V> {
    /// The number of [`V`][`HeaderValue`]s in this table.
    pub const LEN: usize = 1024;

    /// The total size (in bytes) of this table.
    pub const SIZE: usize = Self::LEN * V::SIZE;

    pub fn write(&self, mut writer: impl Write) -> io::Result<()> {
        writer.write_all(&self.data)
    }

    pub fn get(&self, pos: RegionLocalChunkPos) -> Option<V> {
        let position = Self::to_position(pos);
        let mut view = Cursor::new(&self.data[position..]);

        let raw = view
            .read_u32::<BigEndian>()
            .unwrap_or_else(|_| unreachable!());
        V::new(raw)
    }

    pub fn set(&mut self, pos: RegionLocalChunkPos, value: Option<V>) {
        let position = Self::to_position(pos);
        let mut view = Cursor::new(&mut self.data[position..]);

        view.write_u32::<BigEndian>(V::get(value))
            .unwrap_or_else(|_| unreachable!());
    }

    pub fn iter(&self) -> HeaderTableIter<'_, V> {
        HeaderTableIter::new(self)
    }

    fn to_position(pos: RegionLocalChunkPos) -> usize {
        fn to_index(pos: RegionLocalChunkPos) -> usize {
            pos.x as usize + pos.z as usize * AnvilRegion::CHUNKS_PER_AXIS as usize
        }

        to_index(pos) * V::SIZE
    }

    fn from_position(position: usize) -> RegionLocalChunkPos {
        fn from_index(index: usize) -> RegionLocalChunkPos {
            RegionLocalChunkPos {
                x: (index % AnvilRegion::CHUNKS_PER_AXIS as usize) as u8,
                z: (index / AnvilRegion::CHUNKS_PER_AXIS as usize) as u8,
            }
        }

        from_index(position / V::SIZE)
    }
}

impl<V: HeaderValue> From<Vec<u8>> for HeaderTable<V> {
    fn from(data: Vec<u8>) -> Self {
        assert_eq!(Self::SIZE as u64, Sectors::SIZE);
        assert_eq!(Self::SIZE, data.len());
        Self {
            data,
            _value_type: PhantomData,
        }
    }
}

impl<'a, V: HeaderValue> IntoIterator for &'a HeaderTable<V> {
    type Item = (RegionLocalChunkPos, Option<V>);
    type IntoIter = HeaderTableIter<'a, V>;

    fn into_iter(self) -> Self::IntoIter {
        HeaderTableIter::new(self)
    }
}

pub struct HeaderTableIter<'a, V: HeaderValue> {
    data: Cursor<&'a [u8]>,
    _value_type: PhantomData<fn() -> V>,
}

impl<'a, V: HeaderValue> HeaderTableIter<'a, V> {
    pub fn new(table: &'a HeaderTable<V>) -> Self {
        Self {
            data: Cursor::new(&table.data),
            _value_type: PhantomData,
        }
    }
}

impl<V: HeaderValue> Iterator for HeaderTableIter<'_, V> {
    type Item = (RegionLocalChunkPos, Option<V>);

    fn next(&mut self) -> Option<Self::Item> {
        let position = usize::try_from(self.data.position()).unwrap_or_else(|_| unreachable!());

        if position >= HeaderTable::<V>::SIZE {
            return None;
        }

        let raw = self
            .data
            .read_u32::<BigEndian>()
            .unwrap_or_else(|_| unreachable!());
        let pos = HeaderTable::<V>::from_position(position);

        Some((pos, V::new(raw)))
    }
}

pub trait HeaderValue: Sized {
    const SIZE: usize = size_of::<Self>();

    fn new(raw: u32) -> Option<Self>;

    fn get(this: Option<Self>) -> u32;
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Offset(NonZeroU32);

impl Offset {
    pub fn first_sector(&self) -> usize {
        (self.0.get() >> 8 & 0xFFFFFF) as usize
    }

    pub fn num_sectors(&self) -> usize {
        (self.0.get() & 0xFF) as usize
    }

    /// Returns the number of sectors this offset points to, multiplied by the size of a single sector.
    pub fn num_bytes(&self) -> usize {
        self.num_sectors() * Sectors::USIZE
    }

    pub fn sectors(&self) -> Range<usize> {
        let first = self.first_sector();
        let len = self.num_sectors();
        first..(first + len)
    }
}

impl HeaderValue for Offset {
    fn new(raw: u32) -> Option<Self> {
        NonZeroU32::new(raw).map(Offset)
    }

    fn get(this: Option<Self>) -> u32 {
        this.map(|v| v.0.get()).unwrap_or(0)
    }
}

#[derive(Error, Debug)]
pub enum TryFromOffsetError {
    #[error("sectors 0 and 1 are reserved for the header")]
    SectorOverlapsHeader,
    #[error("num_sectors cannot be zero")]
    NumSectorsIsZero,
    #[error("offset is too large to fit in u32")]
    OffsetTooLarge,
}

impl TryFrom<Range<u32>> for Offset {
    type Error = TryFromOffsetError;

    fn try_from(sectors: Range<u32>) -> Result<Self, Self::Error> {
        let first_sector = sectors.start;
        let num_sectors = sectors.end - sectors.start;

        if first_sector < 2 {
            return Err(TryFromOffsetError::SectorOverlapsHeader);
        } else if num_sectors == 0 {
            return Err(TryFromOffsetError::NumSectorsIsZero);
        } else if num_sectors > u8::MAX as u32 {
            return Err(TryFromOffsetError::OffsetTooLarge);
        } else {
            let raw = first_sector << 8 | num_sectors;
            Offset::new(raw).ok_or(TryFromOffsetError::SectorOverlapsHeader)
        }
    }
}

impl TryFrom<Range<usize>> for Offset {
    type Error = TryFromOffsetError;

    fn try_from(sectors: Range<usize>) -> Result<Self, Self::Error> {
        let first_sector =
            u32::try_from(sectors.start).map_err(|_| TryFromOffsetError::OffsetTooLarge)?;
        let num_sectors = u32::try_from(sectors.end - sectors.start)
            .map_err(|_| TryFromOffsetError::OffsetTooLarge)?;

        if first_sector < 2 {
            return Err(TryFromOffsetError::SectorOverlapsHeader);
        } else if num_sectors == 0 {
            return Err(TryFromOffsetError::NumSectorsIsZero);
        } else if num_sectors > u8::MAX as u32 {
            return Err(TryFromOffsetError::OffsetTooLarge);
        } else {
            let raw = first_sector << 8 | num_sectors;
            Offset::new(raw).ok_or(TryFromOffsetError::SectorOverlapsHeader)
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Timestamp(NonZeroU32);

impl Timestamp {
    pub fn from_system_time(time: SystemTime) -> Option<Self> {
        let duration = time.duration_since(SystemTime::UNIX_EPOCH).ok()?;
        let epoch_secs = u32::try_from(duration.as_secs())
            .expect("can't represent epoch seconds as u32 anymore");
        Self::new(epoch_secs)
    }
}

impl HeaderValue for Timestamp {
    fn new(raw: u32) -> Option<Self> {
        NonZeroU32::new(raw).map(Timestamp)
    }

    fn get(this: Option<Self>) -> u32 {
        this.map(|v: Timestamp| v.0.get()).unwrap_or(0)
    }
}

impl From<Timestamp> for SystemTime {
    fn from(timestamp: Timestamp) -> Self {
        SystemTime::UNIX_EPOCH + Duration::from_secs(timestamp.0.get() as u64)
    }
}
