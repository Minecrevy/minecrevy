use std::any::type_name;
use std::io;
use std::io::{Read, Write};

use enumflags2::{BitFlag, BitFlags};
use enumflags2::_internal::RawBitFlags;

use minecrevy_io_buf::{ReadMinecraftExt, WriteMinecraftExt};

use crate::{McRead, McWrite};

macro_rules! impl_mcread {
    ($($ty:ty = $method:ident,)+) => {
        $(
        impl<T> McRead for BitFlags<T, $ty>
        where
            T: BitFlag + RawBitFlags<Numeric=$ty>,
        {
            type Options = ();

            fn read<R: Read>(mut reader: R, _options: Self::Options) -> io::Result<Self> {
                let bits = reader.$method()?;
                BitFlags::from_bits(bits)
                    .map_err(|_| io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("invalid bitset for {}: {}", type_name::<T>(), bits),
                    ))
            }
        }
        )+
    };
}

impl_mcread!(
    u8 = read_u8,
    u16 = read_u16,
    u32 = read_u32,
    u64 = read_u64,
    u128 = read_u128,
);

macro_rules! impl_mcwrite {
    ($($ty:ty = $method:ident,)+) => {
        $(
        impl<T> McWrite for BitFlags<T, $ty>
        where
            T: BitFlag + RawBitFlags<Numeric=$ty>
        {
            type Options = ();

            #[inline]
            fn write<W: Write>(&self, mut writer: W, _options: Self::Options) -> io::Result<()> {
                writer.$method(self.bits())
            }
        }
        )+
    };
}

impl_mcwrite!(
    u8 = write_u8,
    u16 = write_u16,
    u32 = write_u32,
    u64 = write_u64,
    u128 = write_u128,
);
