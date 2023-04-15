use std::{
    fmt,
    io::{self, Read, Write},
    ops::RangeBounds,
};

use strum::{EnumIter, FromRepr, IntoEnumIterator, IntoStaticStr};

use crate::{McRead, McWrite};

/// The Minecraft protocol version sent during protocol handshake.
#[repr(i32)]
#[derive(FromRepr, EnumIter, IntoStaticStr)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum ProtocolVersion {
    /// Minecraft 1.7.2
    V1_7_2 = 4,
    /// Minecraft 1.7.6
    V1_7_6 = 5,
    /// Minecraft 1.8
    V1_8 = 47,
    /// Minecraft 1.9
    V1_9 = 107,
    /// Minecraft 1.9.1
    V1_9_1 = 108,
    /// Minecraft 1.9.2
    V1_9_2 = 109,
    /// Minecraft 1.9.4
    V1_9_4 = 110,
    /// Minecraft 1.10
    V1_10 = 210,
    /// Minecraft 1.11
    V1_11 = 315,
    /// Minecraft 1.11.1
    V1_11_1 = 316,
    /// Minecraft 1.12
    V1_12 = 335,
    /// Minecraft 1.12.1
    V1_12_1 = 338,
    /// Minecraft 1.12.2
    V1_12_2 = 340,
    /// Minecraft 1.13
    V1_13 = 393,
    /// Minecraft 1.13.1
    V1_13_1 = 401,
    /// Minecraft 1.13.2
    V1_13_2 = 404,
    /// Minecraft 1.14
    V1_14 = 477,
    /// Minecraft 1.14.1
    V1_14_1 = 480,
    /// Minecraft 1.14.2
    V1_14_2 = 485,
    /// Minecraft 1.14.3
    V1_14_3 = 490,
    /// Minecraft 1.14.4
    V1_14_4 = 498,
    /// Minecraft 1.15
    V1_15 = 573,
    /// Minecraft 1.15.1
    V1_15_1 = 575,
    /// Minecraft 1.15.2
    V1_15_2 = 578,
    /// Minecraft 1.16
    V1_16 = 735,
    /// Minecraft 1.16.1
    V1_16_1 = 736,
    /// Minecraft 1.16.2
    V1_16_2 = 751,
    /// Minecraft 1.16.3
    V1_16_3 = 753,
    /// Minecraft 1.16.4
    V1_16_4 = 754,
    /// Minecraft 1.17
    V1_17 = 755,
    /// Minecraft 1.17.1
    V1_17_1 = 756,
    /// Minecraft 1.18
    V1_18 = 757,
    /// Minecraft 1.18.2
    V1_18_2 = 758,
    /// Minecraft 1.19
    V1_19 = 759,
    /// Minecraft 1.19.1
    V1_19_1 = 760,
    /// Minecraft 1.19.3
    V1_19_3 = 761,
    /// Minecraft 1.19.4
    V1_19_4 = 762,
}

impl ProtocolVersion {
    /// The oldest supported Minecraft version.
    pub fn min() -> Self {
        Self::iter().next().unwrap()
    }

    /// The most recently supported Minecraft version.
    pub fn max() -> Self {
        Self::iter().last().unwrap()
    }

    /// Gets the protocol version number.
    pub fn get(&self) -> i32 {
        *self as i32
    }
}

impl fmt::Display for ProtocolVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let variant_name: &'static str = self.into();
        let name: String = variant_name[1..].replace('_', ".");
        write!(f, "{name}")
    }
}

#[derive(McRead, McWrite)]
struct Version(#[options(varint = true)] pub i32);

impl McRead for Option<ProtocolVersion> {
    type Options = ();

    fn read<R: Read>(reader: R, (): Self::Options, v: ProtocolVersion) -> io::Result<Self> {
        let version = Version::read(reader, (), v)?.0;
        Ok(ProtocolVersion::from_repr(version))
    }
}

impl McWrite for Option<ProtocolVersion> {
    type Options = ();

    fn write<W: Write>(&self, writer: W, (): Self::Options, v: ProtocolVersion) -> io::Result<()> {
        let version = self.map(|v| v.get()).unwrap_or(-1);
        Version(version).write(writer, (), v)
    }
}

/// A trait for creating iterators of [`ProtocolVersion`] from [`RangeBounds`].
pub trait VersionRangeIter {
    /// Returns an iterator of [`ProtocolVersion`]s.
    fn iter(&self) -> Box<dyn Iterator<Item = ProtocolVersion> + '_>;
}

impl<T: RangeBounds<ProtocolVersion>> VersionRangeIter for T {
    fn iter(&self) -> Box<dyn Iterator<Item = ProtocolVersion> + '_> {
        Box::new(ProtocolVersion::iter().filter(move |v| self.contains(v)))
    }
}
