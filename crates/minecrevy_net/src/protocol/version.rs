use std::fmt;

use minecrevy_io::{McRead, McWrite};
use strum::{EnumIter, FromRepr, IntoStaticStr};

#[derive(McRead, McWrite, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct ProtocolVersion(#[options(varint = true)] pub i32);

impl ProtocolVersion {
    pub fn snapshot(&self) -> Option<SnapshotVersion> {
        SnapshotVersion::new(*self)
    }

    pub fn release(&self) -> Option<ReleaseVersion> {
        if self.snapshot().is_some() {
            return None;
        }
        ReleaseVersion::from_repr(self.0)
    }
}

impl From<ReleaseVersion> for ProtocolVersion {
    fn from(value: ReleaseVersion) -> Self {
        Self(value as i32)
    }
}

impl From<SnapshotVersion> for ProtocolVersion {
    fn from(value: SnapshotVersion) -> Self {
        Self(value.0 | SnapshotVersion::BIT)
    }
}

#[derive(
    FromRepr, EnumIter, IntoStaticStr, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash,
)]
#[repr(i32)]
pub enum ReleaseVersion {
    V1_7_2 = 4,
    V1_7_6 = 5,
    V1_8 = 47,
    V1_9 = 107,
    V1_9_1 = 108,
    V1_9_2 = 109,
    V1_9_4 = 110,
    V1_10 = 210,
    V1_11 = 315,
    V1_11_1 = 316,
    V1_12 = 335,
    V1_12_1 = 338,
    V1_12_2 = 340,
    V1_13 = 393,
    V1_13_1 = 401,
    V1_13_2 = 404,
    V1_14 = 477,
    V1_14_1 = 480,
    V1_14_2 = 485,
    V1_14_3 = 490,
    V1_14_4 = 498,
    V1_15 = 573,
    V1_15_1 = 575,
    V1_15_2 = 578,
    V1_16 = 735,
    V1_16_1 = 736,
    V1_16_2 = 751,
    V1_16_3 = 753,
    V1_16_4 = 754,
    V1_17 = 755,
    V1_17_1 = 756,
    V1_18 = 757,
    V1_18_2 = 758,
    V1_19 = 759,
    V1_19_1 = 760,
    V1_19_3 = 761,
    V1_19_4 = 762,
}

impl ReleaseVersion {
    pub fn v(self) -> ProtocolVersion {
        ProtocolVersion::from(self)
    }
}

impl fmt::Display for ReleaseVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let variant: &'static str = self.into();
        let name: String = variant[1..].replace('_', ".");
        write!(f, "{name}")
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct SnapshotVersion(pub i32);

impl SnapshotVersion {
    const BIT: i32 = 1 << 30;

    pub fn new(version: ProtocolVersion) -> Option<Self> {
        if version.0 & Self::BIT != 0 {
            Some(Self(version.0 & !Self::BIT))
        } else {
            None
        }
    }

    pub fn v(self) -> ProtocolVersion {
        ProtocolVersion::from(self)
    }
}
