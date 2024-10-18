use std::io;

use fastnbt::{DeOpts, SerOpts};

use crate::{
    args::{NbtRead, NbtWrite},
    McRead, McWrite,
};

impl McRead for fastnbt::Value {
    type Args = NbtRead;

    fn read(reader: impl io::Read, read: Self::Args) -> io::Result<Self> {
        match read {
            NbtRead::Network => Ok(
                fastnbt::from_reader_with_opts(reader, DeOpts::network_nbt())
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?,
            ),
            NbtRead::Other => Ok(fastnbt::from_reader_with_opts(reader, DeOpts::new())
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?),
        }
    }
}

impl McWrite for fastnbt::Value {
    type Args = NbtWrite;

    fn write(&self, writer: impl io::Write, write: Self::Args) -> io::Result<()> {
        match write {
            NbtWrite::Network => fastnbt::to_writer_with_opts(writer, self, SerOpts::network_nbt())
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e)),
            NbtWrite::Other(name) => {
                fastnbt::to_writer_with_opts(writer, self, SerOpts::new().root_name(name))
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
            }
        }
    }
}
