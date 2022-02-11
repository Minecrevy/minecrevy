use std::io::{self, ErrorKind, Read};
use std::mem::MaybeUninit;

use uuid::Uuid;

use minecrevy_io_buf::{RawPacket, ReadMinecraftExt};
pub use minecrevy_io_str_derive::McRead;

use crate::options::*;

/// The `McRead` trait allows for constructing data types from bytes.
///
/// Implementors of the `McRead` trait are typically packets or primitive data types.
pub trait McRead: Sized {
    /// The type of options available to configure the read operation.
    type Options: Clone + Default;

    /// Returns a value constructed from a series of bytes received from the specified reader,
    /// optionally configured via the specified options.
    fn read<R: Read>(reader: R, options: Self::Options) -> io::Result<Self>;
}

macro_rules! mcread_impl_primitive {
    ($($ty:ty => $fn:expr,)+) => {
        $(
        impl McRead for $ty {
            type Options = ();

            #[inline]
            fn read<R: Read>(mut reader: R, _: Self::Options) -> io::Result<Self> {
                $fn(&mut reader)
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
    i64 => ReadMinecraftExt::read_i64,
    i128 => ReadMinecraftExt::read_i128,
    f32 => ReadMinecraftExt::read_f32,
    f64 => ReadMinecraftExt::read_f64,
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

impl McRead for bool {
    type Options = ();

    #[inline]
    fn read<R: Read>(mut reader: R, _: Self::Options) -> io::Result<Self> {
        Ok(reader.read_u8()? != 0x00)
    }
}

impl McRead for Uuid {
    type Options = ();

    #[inline]
    fn read<R: Read>(mut reader: R, _: Self::Options) -> io::Result<Self> {
        Ok(Uuid::from_u128(reader.read_u128()?))
    }
}

impl McRead for String {
    type Options = StringOptions;

    fn read<R: Read>(mut reader: R, options: Self::Options) -> io::Result<Self> {
        // Read the len value and check upper bound
        let len = reader.read_var_i32_len()?;
        match options.max_len {
            Some(max_len) if len > max_len => return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("exceeded max string length (max: {}, actual: {})", max_len, len),
            )),
            _ => {}
        }

        // Read the actual string as bytes
        let mut bytes = vec![0; len];
        reader.read_exact(&mut bytes)?;

        // Try to convert the bytes into valid UTF-8
        String::from_utf8(bytes)
            .map_err(|_| io::Error::new(
                io::ErrorKind::InvalidData,
                "string has invalid UTF-8 characters",
            ))
    }
}

impl<T: McRead> McRead for Vec<T> {
    type Options = ListOptions<T::Options>;

    fn read<R: Read>(mut reader: R, options: Self::Options) -> io::Result<Self> {
        match options.length {
            ListLength::Remaining => {
                let mut result = Vec::new();
                loop {
                    match T::read(&mut reader, options.inner.clone()) {
                        Ok(v) => result.push(v),
                        Err(e) if e.kind() == ErrorKind::UnexpectedEof => break,
                        Err(e) => return Err(e),
                    }
                }
                Ok(result)
            }
            ListLength::VarInt => {
                let len = reader.read_var_i32_len()?;
                let mut result = Vec::with_capacity(len);
                for _ in 0..len {
                    result.push(T::read(&mut reader, options.inner.clone())?);
                }
                Ok(result)
            }
        }
    }
}

impl<T: McRead, const N: usize> McRead for [T; N] {
    type Options = ArrayOptions<T::Options>;

    fn read<R: Read>(mut reader: R, options: Self::Options) -> io::Result<Self> {
        let mut result = MaybeUninit::<T>::uninit_array::<N>();
        for val in &mut result {
            val.write(T::read(&mut reader, options.inner.clone())?);
        }
        // SAFETY: At this point, the array is always fully initialized,
        // otherwise we've already returned with an error.
        unsafe { Ok(MaybeUninit::array_assume_init(result)) }
    }
}

impl<T: McRead> McRead for Option<T> {
    type Options = OptionOptions<T::Options>;

    fn read<R: Read>(mut reader: R, options: Self::Options) -> io::Result<Self> {
        match options.existence {
            OptionExistence::Bool => {
                if reader.read_bool()? {
                    T::read(reader, options.inner).map(Some)
                } else {
                    Ok(None)
                }
            }
            OptionExistence::Remaining => {
                match T::read(reader, options.inner) {
                    Ok(v) => Ok(Some(v)),
                    Err(e) if e.kind() == ErrorKind::UnexpectedEof => Ok(None),
                    Err(e) => return Err(e),
                }
            }
        }
    }
}

impl McRead for RawPacket {
    type Options = ();

    #[inline]
    fn read<R: Read>(mut reader: R, _options: Self::Options) -> io::Result<Self> {
        reader.read_packet()
    }
}
