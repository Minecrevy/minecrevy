use std::io::{Read, Write};

use minecrevy_math::vector::Vector;

use crate::{McRead, McWrite};

macro_rules! vector_impl {
    ($(< $dim:literal , $ty:ty >),*) => {
        $(
        impl McRead for Vector<$dim, $ty> {
            type Options = ();

            #[inline]
            fn read<R: Read>(reader: R, (): Self::Options) -> std::io::Result<Self> {
                let arr = <[$ty; $dim]>::read(reader, Default::default())?;
                Ok(Vector::from(arr))
            }
        }
        impl McWrite for Vector<$dim, $ty> {
            type Options = ();

            #[inline]
            fn write<W: Write>(&self, writer: W, (): Self::Options) -> std::io::Result<()> {
                self.0.write(writer, Default::default())
            }
        }
        )*
    };
}

vector_impl!(
    <2, i32>,
    <2, f32>,
    <2, f64>,
    <3, f32>,
    <3, f64>
);

/// Configurable options for parsing [`Vector`]s.
#[derive(Clone, Debug, Default)]
pub struct VectorOptions {
    /// Whether the coordinate should be compressed to 64 bits.
    ///
    /// See the [position data type][1] for more info.
    ///
    /// [1]: https://wiki.vg/Protocol#Position
    pub compressed: bool,
}

impl McRead for Vector<3, i32> {
    type Options = VectorOptions;

    fn read<R: Read>(reader: R, options: Self::Options) -> std::io::Result<Self> {
        if options.compressed {
            let val = u64::read(reader, ())?;
            Ok(uncompress_vector(val))
        } else {
            let arr = <[i32; 3]>::read(reader, Default::default())?;
            Ok(Vector::new(arr))
        }
    }
}

impl McWrite for Vector<3, i32> {
    type Options = VectorOptions;

    fn write<W: Write>(&self, mut writer: W, options: Self::Options) -> std::io::Result<()> {
        if options.compressed {
            let val = compress_vector(*self);
            val.write(&mut writer, ())?;
        } else {
            self.0.write(writer, Default::default())?;
        }
        Ok(())
    }
}

#[inline]
fn uncompress_vector(v: u64) -> Vector<3, i32> {
    Vector::new([
        (v >> 38) as i32,
        (v & 0xFFF) as i32,
        ((v >> 12) & 0x3FFFFFF) as i32,
    ])
}

#[inline]
fn compress_vector(v: Vector<3, i32>) -> u64 {
    let Vector([x, y, z]) = v;
    ((x as u64 & 0x3FFFFFF) << 38) | ((z as u64 & 0x3FFFFFF) << 12) | (y as u64 & 0xFFF)
}
