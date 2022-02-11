use std::io::{Read, Write};

use crate::{McRead, McWrite};

/// The type of compression used for encoding and decoding an NBT tag.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum Compression {
    /// Uses the gzip compression format.
    Gzip,
    /// Uses the zlib compression format.
    Zlib,
}

impl From<&str> for Compression {
    fn from(s: &str) -> Self {
        match s {
            "gzip" => Self::Gzip,
            "zlib" => Self::Zlib,
            _ => panic!("invalid compression type")
        }
    }
}

/// Configurable options for parsing [`nbt::Value`]s.
#[derive(Clone, Debug, Default)]
pub struct NbtOptions {
    /// The compression type, if any, for encoding and decoding an NBT tag.
    pub compression: Option<Compression>,
    /// The textual header for an NBT compound when being encoded.
    pub header: Option<String>,
}

impl McRead for nbt::Value {
    type Options = NbtOptions;

    fn read<R: Read>(reader: R, options: Self::Options) -> std::io::Result<Self> {
        Ok(match options.compression {
            None => nbt::from_reader(reader)?,
            Some(Compression::Gzip) => nbt::from_gzip_reader(reader)?,
            Some(Compression::Zlib) => nbt::from_zlib_reader(reader)?,
        })
    }
}

impl McWrite for nbt::Value {
    type Options = NbtOptions;

    fn write<W: Write>(&self, mut writer: W, options: Self::Options) -> std::io::Result<()> {
        let header = options.header.as_ref().map(|s| s.as_str());
        match options.compression {
            None => nbt::to_writer(&mut writer, self, header)?,
            Some(Compression::Gzip) => nbt::to_gzip_writer(&mut writer, self, header)?,
            Some(Compression::Zlib) => nbt::to_zlib_writer(&mut writer, self, header)?,
        }
        Ok(())
    }
}
