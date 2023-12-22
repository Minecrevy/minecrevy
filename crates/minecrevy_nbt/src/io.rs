use std::{
    io,
    ops::{Deref, DerefMut},
};

use minecrevy_io::{
    args::{Compression, NbtArgs},
    McRead, McWrite,
};
use serde::{de::DeserializeOwned, Serialize};

pub struct Nbt<T>(pub T);

impl<T> Deref for Nbt<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Nbt<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: DeserializeOwned> McRead for Nbt<T> {
    type Args = NbtArgs;

    fn read(reader: impl io::Read, args: Self::Args) -> io::Result<Self> {
        let reader = reader.take(args.max_len.unwrap_or(u64::MAX));

        match args.compression {
            Compression::None => Ok(Nbt(crate::from_reader(reader)?)),
            Compression::GZip => Ok(Nbt(crate::from_gzip_reader(reader)?)),
            Compression::ZLib => Ok(Nbt(crate::from_zlib_reader(reader)?)),
        }
    }
}

impl<T: Serialize> McWrite for Nbt<T> {
    type Args = NbtArgs;

    fn write(&self, mut writer: impl io::Write, args: Self::Args) -> io::Result<()> {
        match args.compression {
            Compression::None => Ok(crate::to_writer(&mut writer, &self.0, args.header)?),
            Compression::GZip => Ok(crate::to_gzip_writer(&mut writer, &self.0, args.header)?),
            Compression::ZLib => Ok(crate::to_zlib_writer(&mut writer, &self.0, args.header)?),
        }
    }
}
