use std::io::{self, Read, Write};

use uuid::Uuid;

use crate::{
    prelude::{ReadMinecraftExt, WriteMinecraftExt},
    McRead, McWrite,
};

impl McRead for Uuid {
    type Args = ();

    #[inline]
    fn read(mut reader: impl Read, (): Self::Args) -> io::Result<Self> {
        Ok(Uuid::from_u128(reader.read_u128()?))
    }
}

impl McWrite for Uuid {
    type Args = ();

    #[inline]
    fn write(&self, mut writer: impl Write, (): Self::Args) -> io::Result<()> {
        writer.write_u128(self.as_u128())
    }
}
