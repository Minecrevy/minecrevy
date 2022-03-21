use std::io::{Cursor, Read, Seek, SeekFrom, Write};
use std::io;

use byteorder::{BigEndian, ReadBytesExt};

use crate::io::{SectorPtr, SectorPtrTable, TimestampTable};

pub struct Sectors<F: Read + Seek + Write> {
    file: F,
}

impl<F: Read + Seek + Write> Sectors<F> {
    /// The `file position` that the table starts at.
    const START_POSITION: usize = SectorPtrTable::<F>::SIZE + TimestampTable::<F>::SIZE;

    /// The size in bytes of a single sector.
    const SECTOR_SIZE: usize = 4096;

    #[inline]
    pub fn new(file: F) -> Self {
        Self { file }
    }

    pub fn read(&mut self, ptr: SectorPtr) -> io::Result<nbt::Blob> {
        // Seek to the file position of the first sector.
        let position = Self::START_POSITION + (ptr.index() as usize) * Self::SECTOR_SIZE;
        self.file.seek(SeekFrom::Start(position as u64))?;

        // Read the length of the following chunk data, and check that it doesn't exceed the max length.
        let max_length = (ptr.len() as usize) * Self::SECTOR_SIZE;
        let length = usize::try_from(self.file.read_i32::<BigEndian>()?)
            .map_err(|_| io::Error::new(
                io::ErrorKind::InvalidData,
                "negative chunk data length encountered",
            ))?;
        if length > max_length {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "chunk data length exceeded maximum sector data length",
            ));
        }

        // Read the type of compression used for the following chunk data.
        let compression = Compression::try_from(self.file.read_u8()?)?;

        // Read the chunk data and decompress it (if needed).
        let mut data = vec![0; length];
        self.file.read_exact(&mut data)?;
        compression.read_blob(Cursor::new(data))
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Compression {
    GZip = 1,
    ZLib = 2,
    None = 3,
}

impl Compression {
    pub fn read_blob<R: Read>(&self, mut reader: R) -> io::Result<nbt::Blob> {
        match self {
            Compression::GZip => nbt::Blob::from_gzip_reader(&mut reader)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e)),
            Compression::ZLib => nbt::Blob::from_zlib_reader(&mut reader)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e)),
            Compression::None => nbt::Blob::from_reader(&mut reader)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e)),
        }
    }

    pub fn write_blob<W: Write>(&self, mut writer: W, blob: nbt::Blob) -> io::Result<()> {
        match self {
            Compression::GZip => blob.to_gzip_writer(&mut writer)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?,
            Compression::ZLib => blob.to_zlib_writer(&mut writer)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?,
            Compression::None => blob.to_writer(&mut writer)
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
            ))
        }
    }
}
