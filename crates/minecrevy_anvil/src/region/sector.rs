use std::io::{self, Cursor, ErrorKind, Read, SeekFrom, Write};

use byteorder::{BigEndian, ReadBytesExt};
use minecrevy_nbt::Blob;

use crate::region::{
    sector_ptr::{SectorPtr, SectorPtrTable},
    timestamp::TimestampTable,
    Filelike,
};

/// A potentially-infinite collection of chunk [`Blob`]s.
pub struct Sectors<F: Filelike> {
    file: F,
}

impl<F: Filelike> Sectors<F> {
    /// The `file position` that sectors starts at.
    const START_POSITION: usize = SectorPtrTable::<F>::SIZE + TimestampTable::<F>::SIZE;

    /// The size (in bytes) of a single sector.
    const SECTOR_SIZE: usize = 4096;

    /// Constructs a new [`Sectors`] backed by the specified file.
    pub fn new(file: F) -> Self {
        Self { file }
    }

    /// Reads the [`Blob`] at the specified [`SectorPtr`].
    pub fn read(&mut self, ptr: SectorPtr) -> io::Result<Blob> {
        // go to the first sector's file position
        self.seek(ptr)?;

        // check the length of the chunk data
        let max_len = (ptr.len() as usize) * Self::SECTOR_SIZE;
        let len = usize::try_from(self.file.read_u32::<BigEndian>()?)
            .map_err(|_| io::Error::new(ErrorKind::InvalidData, "negative chunk data length"))?;
        if len > max_len {
            return Err(io::Error::new(
                ErrorKind::InvalidData,
                "chunk data length > max length",
            ));
        }

        let compression = Compression::try_from(self.file.read_u8()?)?;

        let mut data = vec![0; len];
        self.file.read_exact(&mut data)?;
        compression.read_blob(Cursor::new(data))
    }

    /// Writes the [`Blob`] at the specified [`SectorPtr`].
    pub fn write(&mut self, ptr: SectorPtr, chunk: Blob) -> io::Result<()> {
        todo!()
    }

    fn seek(&mut self, ptr: SectorPtr) -> io::Result<u64> {
        let position = Self::START_POSITION + (ptr.index() as usize) * Self::SECTOR_SIZE;
        self.file.seek(SeekFrom::Start(position as u64))
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Compression {
    GZip = 1,
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
}

impl Default for Compression {
    fn default() -> Self {
        Self::ZLib
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
