use std::io::{self, Read, Write};
use std::marker::PhantomData;

use minecrevy_io_buf::{ReadMinecraftExt, WriteMinecraftExt};

use crate::{options::*, McRead, McWrite};

macro_rules! mcread_impl_primitive {
    ($($ty:ty => $fn:expr,)+) => {
        $(
        impl McRead for $ty {
            type Options = ();

            #[inline]
            fn read<R: Read>(mut reader: R, (): Self::Options) -> io::Result<Self> {
                $fn(&mut reader)
            }
        }
        )+
    };
}

macro_rules! mcwrite_impl_primitive {
    ($($ty:ty => $fn:expr,)+) => {
        $(
        impl McWrite for $ty {
            type Options = ();

            #[inline]
            fn write<W: Write>(&self, mut writer: W, (): Self::Options) -> io::Result<()> {
                $fn(&mut writer, *self)
            }
        }
        )+
    };
}

mcread_impl_primitive!(
    u8 => ReadMinecraftExt::read_u8,
    u16 => ReadMinecraftExt::read_u16,
    u32 => ReadMinecraftExt::read_u32,
    u64 => ReadMinecraftExt::read_u64,
    u128 => ReadMinecraftExt::read_u128,
    i8 => ReadMinecraftExt::read_i8,
    i16 => ReadMinecraftExt::read_i16,
    i128 => ReadMinecraftExt::read_i128,
    f32 => ReadMinecraftExt::read_f32,
    f64 => ReadMinecraftExt::read_f64,
);

mcwrite_impl_primitive!(
    u8 => WriteMinecraftExt::write_u8,
    u16 => WriteMinecraftExt::write_u16,
    u32 => WriteMinecraftExt::write_u32,
    u64 => WriteMinecraftExt::write_u64,
    u128 => WriteMinecraftExt::write_u128,
    i8 => WriteMinecraftExt::write_i8,
    i16 => WriteMinecraftExt::write_i16,
    i128 => WriteMinecraftExt::write_i128,
    f32 => WriteMinecraftExt::write_f32,
    f64 => WriteMinecraftExt::write_f64,
);

impl McRead for i32 {
    type Options = IntOptions;

    fn read<R: Read>(mut reader: R, options: Self::Options) -> io::Result<Self> {
        match options.varint {
            true => reader.read_var_i32(),
            false => reader.read_i32(),
        }
    }
}

impl McWrite for i32 {
    type Options = IntOptions;

    fn write<W: Write>(&self, mut writer: W, options: Self::Options) -> io::Result<()> {
        match options.varint {
            true => writer.write_var_i32(*self),
            false => writer.write_i32(*self),
        }
    }
}

impl McRead for i64 {
    type Options = IntOptions;

    fn read<R: Read>(mut reader: R, options: Self::Options) -> io::Result<Self> {
        match options.varint {
            true => reader.read_var_i64(),
            false => reader.read_i64(),
        }
    }
}

impl McWrite for i64 {
    type Options = IntOptions;

    fn write<W: Write>(&self, mut writer: W, options: Self::Options) -> io::Result<()> {
        match options.varint {
            true => writer.write_var_i64(*self),
            false => writer.write_i64(*self),
        }
    }
}

impl McRead for bool {
    type Options = ();

    #[inline]
    fn read<R: Read>(mut reader: R, (): Self::Options) -> io::Result<Self> {
        Ok(reader.read_u8()? != 0x00)
    }
}

impl McWrite for bool {
    type Options = ();

    #[inline]
    fn write<W: Write>(&self, mut writer: W, (): Self::Options) -> io::Result<()> {
        writer.write_u8(if *self { 0x01 } else { 0x00 })
    }
}

impl<T: McRead> McRead for PhantomData<T> {
    type Options = T::Options;

    fn read<R: Read>(reader: R, options: Self::Options) -> io::Result<Self> {
        T::read(reader, options)?;
        Ok(PhantomData)
    }
}

impl<T: Default + McWrite> McWrite for PhantomData<T> {
    type Options = T::Options;

    fn write<W: Write>(&self, writer: W, options: Self::Options) -> io::Result<()> {
        T::default().write(writer, options)
    }
}
