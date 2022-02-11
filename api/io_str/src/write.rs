use std::io::{self, Write};

use uuid::Uuid;

use minecrevy_io_buf::{RawPacket, WriteMinecraftExt};
pub use minecrevy_io_str_derive::McWrite;
use crate::ArrayOptions;

use crate::options::{IntOptions, ListLength, ListOptions, OptionExistence, OptionOptions, StringOptions};

/// The `McWrite` trait allows for converting data types into bytes.
///
/// Implementors of the `McWrite` trait are typically packets or primitive data types.
pub trait McWrite: Sized {
    /// The type of options available to configure the write operation.
    type Options: Clone + Default;

    /// Writes this value as a series of bytes to the specified writer,
    /// optionally configured via the specified options.
    fn write<W: Write>(&self, writer: W, options: Self::Options) -> io::Result<()>;
}

macro_rules! mcwrite_impl_primitive {
    ($($ty:ty => $fn:expr,)+) => {
        $(
        impl McWrite for $ty {
            type Options = ();

            #[inline]
            fn write<W: Write>(&self, mut writer: W, _: Self::Options) -> io::Result<()> {
                $fn(&mut writer, *self)
            }
        }
        )+
    };
}

mcwrite_impl_primitive!(
    u8 => WriteMinecraftExt::write_u8,
    u16 => WriteMinecraftExt::write_u16,
    u32 => WriteMinecraftExt::write_u32,
    u64 => WriteMinecraftExt::write_u64,
    u128 => WriteMinecraftExt::write_u128,
    i8 => WriteMinecraftExt::write_i8,
    i16 => WriteMinecraftExt::write_i16,
    i64 => WriteMinecraftExt::write_i64,
    i128 => WriteMinecraftExt::write_i128,
    f32 => WriteMinecraftExt::write_f32,
    f64 => WriteMinecraftExt::write_f64,
);

impl McWrite for i32 {
    type Options = IntOptions;

    fn write<W: Write>(&self, mut writer: W, options: Self::Options) -> io::Result<()> {
        match options.varint {
            true => writer.write_var_i32(*self),
            false => writer.write_i32(*self),
        }
    }
}

impl McWrite for bool {
    type Options = ();

    #[inline]
    fn write<W: Write>(&self, mut writer: W, _: Self::Options) -> io::Result<()> {
        writer.write_u8(if *self { 0x01 } else { 0x00 })
    }
}

impl McWrite for Uuid {
    type Options = ();

    #[inline]
    fn write<W: Write>(&self, mut writer: W, _: Self::Options) -> io::Result<()> {
        writer.write_u128(self.as_u128())
    }
}

impl McWrite for String {
    type Options = StringOptions;

    fn write<W: Write>(&self, mut writer: W, options: Self::Options) -> io::Result<()> {
        match options.max_len {
            Some(max_len) if self.len() > max_len => return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("exceeded max string length (max: {}, actual: {})", max_len, self.len()),
            )),
            _ => {}
        }

        writer.write_var_i32_len(self.len())?;
        writer.write_all(self.as_bytes())?;
        Ok(())
    }
}

impl<T: McWrite> McWrite for Vec<T> {
    type Options = ListOptions<T::Options>;

    fn write<W: Write>(&self, mut writer: W, options: Self::Options) -> io::Result<()> {
        match options.length {
            ListLength::VarInt => writer.write_var_i32_len(self.len())?,
            ListLength::Remaining => { /* no length prefix since its inferred */ }
        }
        for element in self {
            element.write(&mut writer, options.inner.clone())?;
        }
        Ok(())
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

impl<T: McWrite> McWrite for Option<T> {
    type Options = OptionOptions<T::Options>;

    fn write<W: Write>(&self, mut writer: W, options: Self::Options) -> io::Result<()> {
        match options.existence {
            OptionExistence::Bool => writer.write_bool(self.is_some())?,
            OptionExistence::Remaining => {}
        }

        if let Some(val) = self {
            val.write(writer, options.inner)?;
        }

        Ok(())
    }
}

impl McWrite for RawPacket {
    type Options = ();

    #[inline]
    fn write<W: Write>(&self, mut writer: W, _options: Self::Options) -> io::Result<()> {
        writer.write_packet(self)
    }
}
