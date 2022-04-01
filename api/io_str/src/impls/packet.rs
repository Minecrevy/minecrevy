use std::io::{self, Read, Write};

use minecrevy_io_buf::{RawPacket, ReadMinecraftExt, WriteMinecraftExt};

use crate::{McRead, McWrite};

impl McRead for RawPacket {
    type Options = ();

    #[inline]
    fn read<R: Read>(mut reader: R, (): Self::Options) -> io::Result<Self> {
        reader.read_packet()
    }
}

impl McWrite for RawPacket {
    type Options = ();

    #[inline]
    fn write<W: Write>(&self, mut writer: W, (): Self::Options) -> io::Result<()> {
        writer.write_packet(self)
    }
}
