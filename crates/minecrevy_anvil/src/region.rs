use std::{
    fmt,
    fs::File,
    io::{self, Cursor, Read, Seek, Write},
    path::Path,
    time::SystemTime,
};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use minecrevy_chunk::ChunkPos;
use minecrevy_nbt::Blob;
use serde::{de::DeserializeOwned, Serialize};
use tracing::warn_span;

use crate::region::{
    header::{read_header, HeaderTable, Offset, Timestamp},
    sector::{Sectors, UsedSectors},
};

mod header;
mod sector;

pub struct AnvilRegion {
    /// The file that stores the chunks.
    file: File,
    /// A table of where a chunk is stored in the file.
    offsets: HeaderTable<Offset>,
    /// A table of the last time each chunk was modified.
    timestamps: HeaderTable<Timestamp>,
    /// A bitmap of sectors in use by serialized chunks.
    used: UsedSectors,
    /// The compression used for writing chunks.
    compression: Compression,
}

impl AnvilRegion {
    /// The number of chunks per axis/side of a region.
    pub const CHUNKS_PER_AXIS: i32 = 32;

    pub fn new(file_path: &Path) -> io::Result<Self> {
        let _region = warn_span!("region", file = %file_path.display()).entered();

        let mut file = File::options()
            .create(true)
            .read(true)
            .write(true)
            .open(file_path)?;
        let (mut offsets, timestamps) = read_header(&mut file)?;
        let mut used = UsedSectors::default();

        // validate offset table
        let file_size = file.metadata()?.len();
        for (pos, offset) in &offsets.clone() {
            if let Some(offset) = offset {
                if offset.first_sector() < 2 {
                    tracing::warn!(
                        "region file has invalid sector offset at {}: sector {} overlaps region header",
                        pos,
                        offset.first_sector()
                    );
                    offsets.set(pos, None);
                } else if offset.num_sectors() == 0 {
                    tracing::warn!(
                        "region file has invalid sector offset at {}: num_sectors must be > 0",
                        pos,
                    );
                    offsets.set(pos, None);
                } else if (offset.first_sector() * Sectors::USIZE) as u64 > file_size {
                    tracing::warn!(
                        "region file has invalid sector offset at {}: sector out of bounds",
                        pos
                    );
                    offsets.set(pos, None);
                } else {
                    // offsets are correct, mark them as used.
                    used.force(offset);
                }
            }
        }

        Ok(Self {
            file,
            offsets,
            timestamps,
            used,
            compression: Compression::ZLib,
        })
    }

    pub fn read_blob(&mut self, pos: RegionLocalChunkPos) -> io::Result<Option<Blob>> {
        let Some(offset) = self.offsets.get(pos) else {
            // no chunk stored
            return Ok(None);
        };

        let mut data: Cursor<_> = {
            let mut data = vec![0u8; offset.num_bytes()];

            self.seek(offset)?;
            let read = self.file.read(&mut data)?;
            if read < Sectors::CHUNK_HEADER_SIZE {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "chunk {pos} has truncated header",
                ));
            }

            Cursor::new(data)
        };

        let chunk_len: usize = {
            let len = data.read_i32::<BigEndian>()?;
            if len == 0 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "chunk {pos} is allocated but has no data",
                ));
            };

            // The compression byte is included in the length field,
            // so we subtract it to get the actual chunk data's length.
            let chunk_len: usize = (len - 1).try_into().map_err(|_| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "chunk {pos} has negative length",
                )
            })?;

            chunk_len
        };
        let compression = Compression::try_from(data.read_u8()?)?;

        // slice the data to enforce chunk length
        // also skip the header
        let mut data = Cursor::new(
            &data.get_ref()[Sectors::CHUNK_HEADER_SIZE..(chunk_len + Sectors::CHUNK_HEADER_SIZE)],
        );
        let chunk = compression.read_blob(&mut data).map(Some)?;

        if data.position() < chunk_len as u64 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "chunk {pos} is truncated: expected {} but read {}",
                    chunk_len,
                    data.position(),
                ),
            ));
        }

        Ok(chunk)
    }

    pub fn read<T: DeserializeOwned>(&mut self, pos: RegionLocalChunkPos) -> io::Result<Option<T>> {
        let Some(offset) = self.offsets.get(pos) else {
            // no chunk stored
            return Ok(None);
        };

        let mut data: Cursor<_> = {
            let mut data = vec![0u8; offset.num_bytes()];

            self.seek(offset)?;
            let read = self.file.read(&mut data)?;
            if read < Sectors::CHUNK_HEADER_SIZE {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "chunk {pos} has truncated header",
                ));
            }

            Cursor::new(data)
        };

        let chunk_len: usize = {
            let len = data.read_i32::<BigEndian>()?;
            if len == 0 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "chunk {pos} is allocated but has no data",
                ));
            };

            // The compression byte is included in the length field,
            // so we subtract it to get the actual chunk data's length.
            let chunk_len: usize = (len - 1).try_into().map_err(|_| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "chunk {pos} has negative length",
                )
            })?;

            chunk_len
        };
        let compression = Compression::try_from(data.read_u8()?)?;

        // slice the data to enforce chunk length
        // also skip the header
        let mut data = Cursor::new(
            &data.get_ref()[Sectors::CHUNK_HEADER_SIZE..(chunk_len + Sectors::CHUNK_HEADER_SIZE)],
        );
        let chunk = compression.read::<T>(&mut data).map(Some)?;

        if data.position() < chunk_len as u64 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "chunk {pos} is truncated: expected {} but read {}",
                    chunk_len,
                    data.position(),
                ),
            ));
        }

        Ok(chunk)
    }

    pub fn write_blob(&mut self, pos: RegionLocalChunkPos, chunk: Blob) -> io::Result<()> {
        // saving the old offset to free it last
        // we don't free it now to prevent region corruption
        let old_offset = self.offsets.get(pos);

        let data = {
            // start with a zeroed-out length and the compression type byte
            // so that we can reuse the allocation to serialize the chunk data
            let mut data: Vec<u8> = vec![0, 0, 0, 0, self.compression.into()];
            // write the chunk data
            self.compression.write_blob(&mut data, chunk)?;

            // calculate the length of the serialized chunk + the compression type byte
            // and write it at the start of the Vec
            let chunk_len =
                i32::try_from(data.len() - Sectors::CHUNK_HEADER_SIZE + 1).map_err(|_| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        "serialized chunk length exceeds i32",
                    )
                })?;
            Cursor::new(&mut data).write_i32::<BigEndian>(chunk_len)?;

            data
        };

        // we need a new place to store the chunk data
        let new_offset = {
            // calculate the number of sectors needed to store the chunk data
            let num_sectors = Self::sectors_needed(data.len());
            if num_sectors > u8::MAX as usize {
                // TODO: support larger chunks
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "chunk data is too large: requires {num_sectors} sectors",
                ));
            }
            // allocate enough space in the bitmap
            self.used.allocate(num_sectors)
        };
        // write the chunk data to the new range of sectors
        self.seek(new_offset)?;
        self.file.write_all(&data)?;

        // update the tables and write the updated header to the file
        self.offsets.set(pos, Some(new_offset));
        self.timestamps
            .set(pos, Timestamp::from_system_time(SystemTime::now()));
        self.write_header()?;

        // finally free up the old offset if it existed
        if let Some(old_offset) = old_offset {
            self.used.free(old_offset);
        }

        Ok(())
    }

    pub fn write<T: Serialize>(&mut self, pos: RegionLocalChunkPos, chunk: T) -> io::Result<()> {
        // saving the old offset to free it last
        // we don't free it now to prevent region corruption
        let old_offset = self.offsets.get(pos);

        let data = {
            // start with a zeroed-out length and the compression type byte
            // so that we can reuse the allocation to serialize the chunk data
            let mut data: Vec<u8> = vec![0, 0, 0, 0, self.compression.into()];
            // write the chunk data
            self.compression.write(&mut data, chunk)?;

            // calculate the length of the serialized chunk + the compression type byte
            // and write it at the start of the Vec
            let chunk_len =
                i32::try_from(data.len() - Sectors::CHUNK_HEADER_SIZE + 1).map_err(|_| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        "serialized chunk length exceeds i32",
                    )
                })?;
            Cursor::new(&mut data).write_i32::<BigEndian>(chunk_len)?;

            data
        };

        // we need a new place to store the chunk data
        let new_offset = {
            // calculate the number of sectors needed to store the chunk data
            let num_sectors = Self::sectors_needed(data.len());
            if num_sectors > u8::MAX as usize {
                // TODO: support larger chunks
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "chunk data is too large: requires {num_sectors} sectors",
                ));
            }
            // allocate enough space in the bitmap
            self.used.allocate(num_sectors)
        };
        // write the chunk data to the new range of sectors
        self.seek(new_offset)?;
        self.file.write_all(&data)?;

        // update the tables and write the updated header to the file
        self.offsets.set(pos, Some(new_offset));
        self.timestamps
            .set(pos, Timestamp::from_system_time(SystemTime::now()));
        self.write_header()?;

        // finally free up the old offset if it existed
        if let Some(old_offset) = old_offset {
            self.used.free(old_offset);
        }

        Ok(())
    }

    pub fn remove(&mut self, pos: RegionLocalChunkPos) -> io::Result<()> {
        let Some(offset) = self.offsets.get(pos) else {
            // no chunk stored to remove
            return Ok(());
        };

        // update the tables and write the updated header to the file
        self.offsets.set(pos, None);
        self.timestamps
            .set(pos, Timestamp::from_system_time(SystemTime::now()));
        self.write_header()?;

        // free up the old offset
        self.used.free(offset);

        Ok(())
    }

    pub fn chunks(&mut self) -> ChunkPosIter<'_> {
        ChunkPosIter::new(self)
    }

    pub fn count_chunks(&self) -> usize {
        self.offsets.iter().flat_map(|(_, offset)| offset).count()
    }

    fn seek(&mut self, offset: Offset) -> io::Result<()> {
        let position = (offset.first_sector() * Sectors::USIZE) as u64;
        self.file.seek(io::SeekFrom::Start(position))?;
        Ok(())
    }

    fn write_header(&mut self) -> io::Result<()> {
        self.offsets.write(&mut self.file)?;
        self.timestamps.write(&mut self.file)?;
        Ok(())
    }

    fn pad_to_full_sector(&mut self) -> io::Result<()> {
        let file_size = self.file.metadata()?.len() as usize;
        let padded_size = Self::sectors_needed(file_size) * Sectors::USIZE;
        if file_size < padded_size {
            let pad_with = vec![0u8; padded_size - file_size];
            self.file.seek(io::SeekFrom::End(0))?;
            self.file.write_all(&pad_with)?;
        }
        Ok(())
    }

    /// Size includes the chunk's 5 byte header.
    fn sectors_needed(chunk_size: usize) -> usize {
        (chunk_size + Sectors::USIZE - 1) / Sectors::USIZE
    }
}

impl Drop for AnvilRegion {
    fn drop(&mut self) {
        if let Err(e) = self.pad_to_full_sector() {
            tracing::error!(error = %e, "failed to pad region to a full sector");
        }
    }
}

pub struct ChunkPosIter<'a> {
    region: &'a mut AnvilRegion,
    iter: std::vec::IntoIter<RegionLocalChunkPos>,
}

impl<'a> ChunkPosIter<'a> {
    pub fn new(region: &'a mut AnvilRegion) -> Self {
        Self {
            iter: region
                .offsets
                .iter()
                .map(|(pos, _)| pos)
                .collect::<Vec<_>>()
                .into_iter(),
            region,
        }
    }
}

impl Iterator for ChunkPosIter<'_> {
    type Item = (RegionLocalChunkPos, Blob);

    fn next(&mut self) -> Option<Self::Item> {
        let pos = self.iter.next()?;
        let chunk = self.region.read_blob(pos).unwrap()?;

        Some((pos, chunk))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Default)]
pub struct RegionPos {
    pub x: i32,
    pub z: i32,
}

impl RegionPos {
    pub fn as_filename(&self) -> String {
        let Self { x, z } = self;
        format!("r.{x}.{z}.mca")
    }
}

impl From<ChunkPos> for RegionPos {
    fn from(chunk: ChunkPos) -> Self {
        Self {
            x: chunk.x / AnvilRegion::CHUNKS_PER_AXIS,
            z: chunk.z / AnvilRegion::CHUNKS_PER_AXIS,
        }
    }
}

impl fmt::Display for RegionPos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { x, z } = self;
        write!(f, "region({x}, {z})")
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Default)]
pub struct RegionLocalChunkPos {
    pub x: u8,
    pub z: u8,
}

impl RegionLocalChunkPos {
    pub fn to_world(&self, region: RegionPos) -> ChunkPos {
        ChunkPos {
            x: (region.x * AnvilRegion::CHUNKS_PER_AXIS) + (self.x as i32),
            z: (region.z * AnvilRegion::CHUNKS_PER_AXIS) + (self.z as i32),
        }
    }
}

impl From<ChunkPos> for RegionLocalChunkPos {
    fn from(chunk: ChunkPos) -> Self {
        Self {
            x: u8::try_from(chunk.x.rem_euclid(AnvilRegion::CHUNKS_PER_AXIS)).unwrap(),
            z: u8::try_from(chunk.z.rem_euclid(AnvilRegion::CHUNKS_PER_AXIS)).unwrap(),
        }
    }
}

impl fmt::Display for RegionLocalChunkPos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { x, z } = self;
        write!(f, "local({x}, {z})")
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub enum Compression {
    GZip = 1,
    #[default]
    ZLib = 2,
    None = 3,
}

impl Compression {
    pub fn read_blob<R: Read>(&self, mut reader: R) -> io::Result<Blob> {
        match self {
            Compression::GZip => Blob::from_gzip_reader(&mut reader)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e)),
            Compression::ZLib => Blob::from_zlib_reader(&mut reader)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e)),
            Compression::None => Blob::from_reader(&mut reader)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e)),
        }
    }

    pub fn read<T: DeserializeOwned>(&self, reader: impl Read) -> io::Result<T> {
        let result = match self {
            Compression::GZip => minecrevy_nbt::from_gzip_reader::<_, T>(reader),
            Compression::ZLib => minecrevy_nbt::from_zlib_reader::<_, T>(reader),
            Compression::None => minecrevy_nbt::from_reader::<_, T>(reader),
        };

        result.map_err(|e| match e {
            minecrevy_nbt::Error::IoError(e) => e,
            e => io::Error::new(io::ErrorKind::InvalidData, e),
        })
    }

    pub fn write_blob<W: Write>(&self, mut writer: W, blob: Blob) -> io::Result<()> {
        match self {
            Compression::GZip => blob
                .to_gzip_writer(&mut writer)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?,
            Compression::ZLib => blob
                .to_zlib_writer(&mut writer)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?,
            Compression::None => blob
                .to_writer(&mut writer)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?,
        };
        Ok(())
    }

    pub fn write<T: Serialize>(&self, mut writer: impl Write, chunk: T) -> io::Result<()> {
        let result = match self {
            Compression::GZip => minecrevy_nbt::to_gzip_writer(&mut writer, &chunk, Some("")),
            Compression::ZLib => minecrevy_nbt::to_zlib_writer(&mut writer, &chunk, Some("")),
            Compression::None => minecrevy_nbt::to_writer(&mut writer, &chunk, Some("")),
        };

        result.map_err(|e| match e {
            minecrevy_nbt::Error::IoError(e) => e,
            e => io::Error::new(io::ErrorKind::InvalidData, e),
        })
    }
}

impl TryFrom<u8> for Compression {
    type Error = io::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::GZip),
            2 => Ok(Self::ZLib),
            3 => Ok(Self::None),
            v => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("invalid compression type: {}", v),
            )),
        }
    }
}

impl From<Compression> for u8 {
    fn from(value: Compression) -> Self {
        value as u8
    }
}

#[cfg(test)]
mod tests {
    use minecrevy_chunk::ChunkPos;

    use crate::{RegionLocalChunkPos, RegionPos};

    #[test]
    fn to_world() {
        {
            let chunk = ChunkPos { x: 0, z: 0 };
            let region = RegionPos::from(chunk);
            let local = RegionLocalChunkPos::from(chunk);

            assert_eq!(chunk, local.to_world(region));
        }
        {
            let chunk = ChunkPos { x: 1, z: 1 };
            let region = RegionPos::from(chunk);
            let local = RegionLocalChunkPos::from(chunk);

            assert_eq!(chunk, local.to_world(region));
        }
        {
            let chunk = ChunkPos { x: 4653, z: -626 };
            let region = RegionPos::from(chunk);
            let local = RegionLocalChunkPos::from(chunk);

            assert_eq!(chunk, local.to_world(region));
        }
        {
            let chunk = ChunkPos { x: -4, z: -66 };
            let region = RegionPos::from(chunk);
            let local = RegionLocalChunkPos::from(chunk);

            assert_eq!(chunk, local.to_world(region));
        }
    }
}
