use std::fmt;

use minecrevy_io::ProtocolVersion;
use strum::{EnumIter, FromRepr, IntoEnumIterator, IntoStaticStr};
use thiserror::Error;

#[derive(FromRepr, EnumIter, IntoStaticStr)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
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
    pub fn min() -> ReleaseVersion {
        Self::iter()
            .next()
            .unwrap_or_else(|| unreachable!("no release versions"))
    }

    pub fn max() -> ReleaseVersion {
        Self::iter()
            .last()
            .unwrap_or_else(|| unreachable!("no release versions"))
    }

    pub fn v(self) -> ProtocolVersion {
        ProtocolVersion::from(self)
    }
}

#[derive(Error, Debug)]
#[error("protocol version does not match a valid release version {0}")]
pub struct TryIntoReleaseError(pub ProtocolVersion);

impl TryFrom<ProtocolVersion> for ReleaseVersion {
    type Error = TryIntoReleaseError;

    fn try_from(version: ProtocolVersion) -> Result<Self, Self::Error> {
        ReleaseVersion::from_repr(version.0).ok_or_else(|| TryIntoReleaseError(version))
    }
}

impl From<ReleaseVersion> for ProtocolVersion {
    fn from(version: ReleaseVersion) -> Self {
        ProtocolVersion(version as i32)
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

    pub fn v(self) -> ProtocolVersion {
        ProtocolVersion::from(self)
    }
}

#[derive(Error, Debug)]
#[error("protocol version does not match a valid snapshot version: {0}")]
pub struct TryIntoSnapshotError(pub ProtocolVersion);

impl TryFrom<ProtocolVersion> for SnapshotVersion {
    type Error = TryIntoSnapshotError;

    fn try_from(version: ProtocolVersion) -> Result<Self, Self::Error> {
        if version.0 & SnapshotVersion::BIT != 0 {
            Ok(Self(version.0 & !SnapshotVersion::BIT))
        } else {
            Err(TryIntoSnapshotError(version))
        }
    }
}

impl From<SnapshotVersion> for ProtocolVersion {
    fn from(version: SnapshotVersion) -> Self {
        ProtocolVersion(version.0 | SnapshotVersion::BIT)
    }
}
