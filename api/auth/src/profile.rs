use flexstr::SharedStr;
use uuid::Uuid;

/// A Minecraft player profile.
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "bevy", derive(bevy::prelude::Component))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Profile {
    /// The [`Uuid`] that uniquely represents a player account.
    id: Uuid,
    /// The username of a player account.
    pub name: SharedStr,
    /// Any properties associated with a player account.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Vec::is_empty"))]
    pub properties: Vec<ProfileProperty>,
}

impl Profile {
    /// Creates a new profile from the provided uuid and username.
    pub fn new(id: Uuid, name: impl Into<SharedStr>) -> Self {
        Self {
            id,
            name: name.into(),
            properties: vec![],
        }
    }

    /// Creates a new profile from the provided username, using a generated uuid.
    pub fn new_offline(name: SharedStr) -> Self {
        // Randomly generated UUIDv4
        const NAMESPACE_MINECREVY: Uuid = Uuid::from_bytes(
            [0xf5, 0x27, 0x82, 0x58, 0xc0, 0x6c, 0x41, 0xff, 0xbd, 0x1d, 0x02, 0xa0, 0xb1, 0x2e, 0xb9, 0x7b]
        );

        Self::new(Uuid::new_v3(&NAMESPACE_MINECREVY, name.as_bytes()), name)
    }

    /// Creates a new profile from the provided uuid and username, as well as a list of associated properties.
    pub fn with_properties(id: Uuid, name: impl Into<SharedStr>, properties: Vec<ProfileProperty>) -> Self {
        Self {
            id,
            name: name.into(),
            properties,
        }
    }

    pub fn without_properties(&mut self) -> &mut Self {
        self.properties.clear();
        self
    }

    /// Returns the uuid of a player account, or a generated uuid for an offline account.
    #[inline]
    pub fn id(&self) -> Uuid {
        self.id
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProfileProperty {
    pub name: SharedStr,
    pub value: SharedStr,
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none"))]
    pub signature: Option<SharedStr>,
}
