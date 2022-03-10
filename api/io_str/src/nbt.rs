use std::io::{Read, Write};

use serde::{Deserialize, Serialize};

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
}

impl McRead for nbt::Blob {
    type Options = NbtOptions;

    fn read<R: Read>(mut reader: R, options: Self::Options) -> std::io::Result<Self> {
        Ok(match options.compression {
            None => Self::from_reader(&mut reader)?,
            Some(Compression::Gzip) => Self::from_gzip_reader(&mut reader)?,
            Some(Compression::Zlib) => Self::from_zlib_reader(&mut reader)?,
        })
    }
}

impl McWrite for nbt::Blob {
    type Options = NbtOptions;

    fn write<W: Write>(&self, mut writer: W, options: Self::Options) -> std::io::Result<()> {
        match options.compression {
            None => self.to_writer(&mut writer)?,
            Some(Compression::Gzip) => self.to_gzip_writer(&mut writer)?,
            Some(Compression::Zlib) => self.to_zlib_writer(&mut writer)?,
        }
        Ok(())
    }
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
        match options.compression {
            None => nbt::to_writer(&mut writer, self, None)?,
            Some(Compression::Gzip) => nbt::to_gzip_writer(&mut writer, self, None)?,
            Some(Compression::Zlib) => nbt::to_zlib_writer(&mut writer, self, None)?,
        }
        Ok(())
    }
}

/// Wrapper type that allows any [`Serialize`]/[`Deserialize`] type to be encoded/decoded as an NBT tag.
#[derive(Clone, PartialEq, Debug)]
pub struct Nbt<T>(pub T);

impl<T: for<'de> Deserialize<'de>> McRead for Nbt<T> {
    type Options = NbtOptions;

    fn read<R: Read>(reader: R, options: Self::Options) -> std::io::Result<Self> {
        Ok(Nbt(match options.compression {
            None => nbt::from_reader(reader)?,
            Some(Compression::Gzip) => nbt::from_gzip_reader(reader)?,
            Some(Compression::Zlib) => nbt::from_zlib_reader(reader)?,
        }))
    }
}

impl<T: Serialize> McWrite for Nbt<T> {
    type Options = NbtOptions;

    fn write<W: Write>(&self, mut writer: W, options: Self::Options) -> std::io::Result<()> {
        match options.compression {
            None => nbt::to_writer(&mut writer, &self.0, None)?,
            Some(Compression::Gzip) => nbt::to_gzip_writer(&mut writer, &self.0, None)?,
            Some(Compression::Zlib) => nbt::to_zlib_writer(&mut writer, &self.0, None)?,
        }
        Ok(())
    }
}
