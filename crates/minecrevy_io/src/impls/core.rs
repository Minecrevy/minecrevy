use std::io::{self, Read, Write};

use crate::{
    args::{ArrayArgs, IntArgs, ListArgs, OptionArgs, OptionTag, StringArgs},
    prelude::{ReadMinecraftExt, WriteMinecraftExt},
    McRead, McWrite,
};

macro_rules! mcread_impl_primitive {
    ($($ty:ty => $fn:expr,)+) => {
        $(
        impl McRead for $ty {
            type Args = ();

            #[inline]
            fn read(mut reader: impl Read, (): Self::Args) -> io::Result<Self> {
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
            type Args = ();

            #[inline]
            fn write(&self, mut writer: impl Write, (): Self::Args) -> io::Result<()> {
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
    type Args = IntArgs;

    fn read(mut reader: impl Read, args: Self::Args) -> io::Result<Self> {
        if args.varint {
            reader.read_var_i32()
        } else {
            reader.read_i32()
        }
    }
}

impl McWrite for i32 {
    type Args = IntArgs;

    fn write(&self, mut writer: impl Write, args: Self::Args) -> io::Result<()> {
        if args.varint {
            writer.write_var_i32(*self)
        } else {
            writer.write_i32(*self)
        }
    }
}

impl McRead for i64 {
    type Args = IntArgs;

    fn read(mut reader: impl Read, args: Self::Args) -> io::Result<Self> {
        if args.varint {
            reader.read_var_i64()
        } else {
            reader.read_i64()
        }
    }
}

impl McWrite for i64 {
    type Args = IntArgs;

    fn write(&self, mut writer: impl Write, args: Self::Args) -> io::Result<()> {
        if args.varint {
            writer.write_var_i64(*self)
        } else {
            writer.write_i64(*self)
        }
    }
}

impl McRead for bool {
    type Args = ();

    #[inline]
    fn read(mut reader: impl Read, (): Self::Args) -> io::Result<Self> {
        reader.read_bool()
    }
}

impl McWrite for bool {
    type Args = ();

    #[inline]
    fn write(&self, mut writer: impl Write, (): Self::Args) -> io::Result<()> {
        writer.write_bool(*self)
    }
}

impl<T: McRead, const N: usize> McRead for [T; N] {
    type Args = ArrayArgs<T::Args>;

    fn read(mut reader: impl Read, args: Self::Args) -> io::Result<Self> {
        // TODO: use std::array::try_from_fn when stabilized
        let mut vec = Vec::with_capacity(N);
        for _ in 0..N {
            vec.push(T::read(&mut reader, args.inner.clone())?);
        }
        vec.try_into()
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "failed to convert vec to array"))
    }
}

impl<T: McWrite, const N: usize> McWrite for [T; N] {
    type Args = ArrayArgs<T::Args>;

    fn write(&self, mut writer: impl Write, args: Self::Args) -> io::Result<()> {
        for val in self {
            val.write(&mut writer, args.inner.clone())?;
        }
        Ok(())
    }
}

impl<'a, T: McWrite> McWrite for &'a [T] {
    type Args = ListArgs<T::Args>;

    fn write(&self, mut writer: impl Write, args: Self::Args) -> io::Result<()> {
        for val in *self {
            val.write(&mut writer, args.inner.clone())?;
        }
        Ok(())
    }
}

impl McWrite for &str {
    type Args = StringArgs;

    fn write(&self, mut writer: impl Write, args: Self::Args) -> io::Result<()> {
        match args.max_len {
            Some(max_len) if self.len() > max_len => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!(
                        "exceeded max string length (max: {}, actual: {})",
                        max_len,
                        self.len()
                    ),
                ))
            }
            _ => {}
        }

        writer.write_var_i32_len(self.len())?;
        writer.write_all(self.as_bytes())?;
        Ok(())
    }
}

impl<T: McRead> McRead for Option<T> {
    type Args = OptionArgs<T::Args>;

    fn read(mut reader: impl Read, args: Self::Args) -> io::Result<Self> {
        match args.tag {
            OptionTag::Bool => {
                if reader.read_bool()? {
                    T::read(reader, args.inner).map(Some)
                } else {
                    Ok(None)
                }
            }
            OptionTag::Remaining => match T::read(reader, args.inner) {
                Ok(v) => Ok(Some(v)),
                Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => Ok(None),
                Err(e) => Err(e),
            },
        }
    }
}

impl<T: McWrite> McWrite for Option<T> {
    type Args = OptionArgs<T::Args>;

    fn write(&self, mut writer: impl Write, args: Self::Args) -> io::Result<()> {
        match args.tag {
            OptionTag::Bool => writer.write_bool(self.is_some())?,
            OptionTag::Remaining => {}
        }

        if let Some(val) = self {
            val.write(writer, args.inner)?;
        }

        Ok(())
    }
}
