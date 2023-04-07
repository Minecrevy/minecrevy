use std::io::{self, Read, Write};

use crate::{
    options::{ArrayOptions, IntOptions, OptionOptions, OptionTag},
    std_ext::{ReadMinecraftExt, WriteMinecraftExt},
    McRead, McWrite,
};

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

impl<T: McRead, const N: usize> McRead for [T; N] {
    type Options = ArrayOptions<T::Options>;

    fn read<R: Read>(mut reader: R, options: Self::Options) -> io::Result<Self> {
        std::array::try_from_fn(|_| T::read(&mut reader, options.inner.clone()))
    }
}

impl<T: McWrite, const N: usize> McWrite for [T; N] {
    type Options = ArrayOptions<T::Options>;

    fn write<W: Write>(&self, mut writer: W, options: Self::Options) -> io::Result<()> {
        for val in self {
            val.write(&mut writer, options.inner.clone())?;
        }
        Ok(())
    }
}

impl<T: McRead> McRead for Option<T> {
    type Options = OptionOptions<T::Options>;

    fn read<R: Read>(mut reader: R, options: Self::Options) -> io::Result<Self> {
        match options.tag {
            OptionTag::Bool => {
                if reader.read_bool()? {
                    T::read(reader, options.inner).map(Some)
                } else {
                    Ok(None)
                }
            }
            OptionTag::Remaining => match T::read(reader, options.inner) {
                Ok(v) => Ok(Some(v)),
                Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => Ok(None),
                Err(e) => return Err(e),
            },
        }
    }
}

impl<T: McWrite> McWrite for Option<T> {
    type Options = OptionOptions<T::Options>;

    fn write<W: Write>(&self, mut writer: W, options: Self::Options) -> io::Result<()> {
        match options.tag {
            OptionTag::Bool => writer.write_bool(self.is_some())?,
            OptionTag::Remaining => {}
        }

        if let Some(val) = self {
            val.write(writer, options.inner)?;
        }

        Ok(())
    }
}
