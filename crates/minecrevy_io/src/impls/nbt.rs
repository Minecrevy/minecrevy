use std::io;

use minecrevy_nbt::Blob;
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    options::{Compression, NbtOptions},
    McRead, McWrite, ProtocolVersion,
};

impl McRead for Blob {
    type Options = NbtOptions;

    fn read<R: io::Read>(
        reader: R,
        options: Self::Options,
        _version: ProtocolVersion,
    ) -> io::Result<Self> {
        let mut reader: Box<dyn io::Read> = match options.max_len {
            Some(max_len) => Box::new(reader.take(max_len as u64)),
            None => Box::new(reader),
        };

        Ok(match options.compression {
            Compression::None => Blob::from_reader(&mut reader)?,
            Compression::GZip => Blob::from_gzip_reader(&mut reader)?,
            Compression::ZLib => Blob::from_zlib_reader(&mut reader)?,
        })
    }
}

impl McWrite for Blob {
    type Options = NbtOptions;

    fn write<W: io::Write>(
        &self,
        mut writer: W,
        options: Self::Options,
        _version: ProtocolVersion,
    ) -> io::Result<()> {
        let mut buf = vec![];
        match options.compression {
            Compression::None => self.to_writer(&mut buf)?,
            Compression::GZip => self.to_gzip_writer(&mut buf)?,
            Compression::ZLib => self.to_zlib_writer(&mut buf)?,
        };

        if let Some(max_len) = options.max_len {
            if buf.len() > max_len {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("max length of {max_len} reached"),
                ));
            }
        }

        writer.write_all(&buf)?;
        Ok(())
    }
}

/// A serde-enabled value serialized/dserialized as NBT.
pub struct Nbt<T>(pub T);

impl<T: DeserializeOwned> McRead for Nbt<T> {
    type Options = NbtOptions;

    fn read<R: io::Read>(
        reader: R,
        options: Self::Options,
        _: ProtocolVersion,
    ) -> io::Result<Self> {
        let reader: Box<dyn io::Read> = match options.max_len {
            Some(max_len) => Box::new(reader.take(max_len as u64)),
            _ => Box::new(reader),
        };

        let result = match options.compression {
            Compression::None => minecrevy_nbt::from_reader::<_, T>(reader),
            Compression::GZip => minecrevy_nbt::from_gzip_reader::<_, T>(reader),
            Compression::ZLib => minecrevy_nbt::from_zlib_reader::<_, T>(reader),
        };

        let value = result.map_err(|e| match e {
            minecrevy_nbt::Error::IoError(e) => e,
            e => io::Error::new(io::ErrorKind::InvalidData, e),
        })?;

        Ok(Nbt(value))
    }
}

impl<T: Serialize> McWrite for Nbt<T> {
    type Options = NbtOptions;

    fn write<W: io::Write>(
        &self,
        mut writer: W,
        options: Self::Options,
        _: ProtocolVersion,
    ) -> io::Result<()> {
        let mut buf = vec![];
        match options.compression {
            Compression::None => minecrevy_nbt::to_writer(&mut buf, &self.0, options.header)?,
            Compression::GZip => minecrevy_nbt::to_gzip_writer(&mut buf, &self.0, options.header)?,
            Compression::ZLib => minecrevy_nbt::to_zlib_writer(&mut buf, &self.0, options.header)?,
        };

        if let Some(max_len) = options.max_len {
            if buf.len() > max_len {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("max length of {max_len} reached"),
                ));
            }
        }

        writer.write_all(&buf)?;
        Ok(())
    }
}
