use std::io::{Read, Write};

use glam::{DVec2, DVec3, IVec2, IVec3, Vec2, Vec3};

use crate::{McRead, McWrite};

macro_rules! vec_impl {
    ($($ty:ty as $arr:ty,)*) => {
        $(
        impl McRead for $ty {
            type Options = ();

            fn read<R: Read>(reader: R, (): Self::Options) -> std::io::Result<Self> {
                let arr = <$arr>::read(reader, Default::default())?;
                Ok(<$ty>::from(arr))
            }
        }
        impl McWrite for $ty {
            type Options = ();

            fn write<W: Write>(&self, writer: W, (): Self::Options) -> std::io::Result<()> {
                self.to_array().write(writer, Default::default())
            }
        }
        )*
    };
}

vec_impl!(
    IVec2 as [i32; 2],
    Vec2 as [f32; 2],
    Vec3 as [f32; 3],
    DVec2 as [f64; 2],
    DVec3 as [f64; 3],
);

/// Configurable options for parsing [`IVec3`]s.
#[derive(Clone, Debug, Default)]
pub struct IVec3Options {
    /// Whether the coordinate should be compressed to 64 bits.
    ///
    /// See the [position data type][1] for more info.
    ///
    /// [1]: https://wiki.vg/Protocol#Position
    pub compressed: bool,
}

impl McRead for IVec3 {
    type Options = IVec3Options;

    fn read<R: Read>(reader: R, options: Self::Options) -> std::io::Result<Self> {
        if options.compressed {
            let val = u64::read(reader, ())?;
            Ok(uncompress_ivec3(val))
        } else {
            let [x, y, z] = <[i32; 3]>::read(reader, Default::default())?;
            Ok(IVec3::new(x, y, z))
        }
    }
}

impl McWrite for IVec3 {
    type Options = IVec3Options;

    fn write<W: Write>(&self, mut writer: W, options: Self::Options) -> std::io::Result<()> {
        if options.compressed {
            let val = compress_ivec3(*self);
            val.write(&mut writer, ())?;
        } else {
            self.to_array().write(writer, Default::default())?;
        }
        Ok(())
    }
}

fn uncompress_ivec3(v: u64) -> IVec3 {
    IVec3::new(
        (v >> 38) as i32,
        (v & 0xFFF) as i32,
        ((v >> 12) & 0x3FFFFFF) as i32,
    )
}

fn compress_ivec3(v: IVec3) -> u64 {
    ((v.x as u64 & 0x3FFFFFF) << 38) | ((v.z as u64 & 0x3FFFFFF) << 12) | (v.y as u64 & 0xFFF)
}
