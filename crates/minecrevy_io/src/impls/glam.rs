use std::io::{self, Read, Write};

use glam::{DVec2, DVec3, IVec2, IVec3, Vec2, Vec3};

use crate::{
    args::{ArrayArgs, IVec3Args},
    McRead, McWrite,
};

macro_rules! impl_vec {
    ($($ty:ty as $arr:ty,)*) => {
        $(
        impl McRead for $ty {
            type Args = ();

            fn read(reader: impl Read, (): Self::Args) -> io::Result<Self> {
                let arr = <$arr>::read(reader, Default::default())?;
                Ok(<$ty>::from(arr))
            }
        }
        impl McWrite for $ty {
            type Args = ();

            fn write(&self, writer: impl Write, (): Self::Args) -> io::Result<()> {
                self.to_array().write(writer, Default::default())
            }
        }
        )*
    };
}

impl_vec!(
    IVec2 as [i32; 2],
    Vec2 as [f32; 2],
    Vec3 as [f32; 3],
    DVec2 as [f64; 2],
    DVec3 as [f64; 3],
);

impl McRead for IVec3 {
    type Args = IVec3Args;

    fn read(reader: impl Read, args: Self::Args) -> io::Result<Self> {
        if args.compressed {
            let val = u64::read(reader, ())?;
            Ok(uncompress_ivec3(val))
        } else {
            let [x, y, z] = <[i32; 3]>::read(reader, ArrayArgs::default())?;
            Ok(IVec3::new(x, y, z))
        }
    }
}

impl McWrite for IVec3 {
    type Args = IVec3Args;

    fn write(&self, mut writer: impl Write, args: Self::Args) -> io::Result<()> {
        if args.compressed {
            let val = compress_ivec3(*self);
            val.write(&mut writer, ())?;
        } else {
            self.to_array().write(writer, ArrayArgs::default())?;
        }
        Ok(())
    }
}

fn uncompress_ivec3(v: u64) -> IVec3 {
    IVec3::new(
        (v >> 38) as i32,
        (v & 0xFFF) as i32,
        ((v >> 12) & 0x03FF_FFFF) as i32,
    )
}

fn compress_ivec3(v: IVec3) -> u64 {
    ((v.x as u64 & 0x03FF_FFFF) << 38) | ((v.z as u64 & 0x03FF_FFFF) << 12) | (v.y as u64 & 0xFFF)
}
