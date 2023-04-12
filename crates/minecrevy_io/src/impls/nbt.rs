use std::io;

use minecrevy_nbt::Blob;

use crate::{
    options::{Compression, NbtOptions},
    McRead, McWrite,
};

impl McRead for Blob {
    type Options = NbtOptions;

    fn read<R: io::Read>(reader: R, options: Self::Options) -> io::Result<Self> {
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

    fn write<W: io::Write>(&self, mut writer: W, options: Self::Options) -> io::Result<()> {
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
