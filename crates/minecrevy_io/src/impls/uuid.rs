use std::io::{self, Read, Write};

use uuid::Uuid;

use crate::{
    std_ext::{ReadMinecraftExt, WriteMinecraftExt},
    McRead, McWrite,
};

impl McRead for Uuid {
    type Options = ();

    #[inline]
    fn read<R: Read>(mut reader: R, (): Self::Options) -> io::Result<Self> {
        Ok(Uuid::from_u128(reader.read_u128()?))
    }
}

impl McWrite for Uuid {
    type Options = ();

    #[inline]
    fn write<W: Write>(&self, mut writer: W, (): Self::Options) -> io::Result<()> {
        writer.write_u128(self.as_u128())
    }
}
