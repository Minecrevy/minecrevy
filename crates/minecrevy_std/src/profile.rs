//! Player profiles.

use bevy::{prelude::*, utils::Uuid};
use minecrevy_protocol::status::ResponseProfile;
use serde::{Deserialize, Serialize};

/// A player profile.
#[derive(Component, Serialize, Deserialize)]
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct Profile {
    /// The name of the player.
    pub name: String,
    /// The UUID of the player.
    pub uuid: Uuid,
    /// The properties of the profile.
    pub properties: Vec<Property>,
}

impl From<&Profile> for ResponseProfile {
    fn from(profile: &Profile) -> Self {
        Self {
            name: profile.name.clone(),
            id: profile.uuid,
        }
    }
}

/// A property of a [`Profile`].
#[derive(Serialize, Deserialize)]
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct Property {
    /// The name of the property.
    pub name: String,
    /// The value of the property.
    pub value: String,
    /// The signature of the property, if any.
    pub signature: Option<String>,
}
