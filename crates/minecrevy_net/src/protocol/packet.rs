use std::collections::BTreeSet;

use bevy::prelude::*;
use minecrevy_core::ecs::CommandExt;
use minecrevy_io::ProtocolVersion;

use crate::protocol::{
    registry::VersionedPacketsBuilder,
    state::{Handshake, Login, Play, ProtocolState, Status},
};

/// A [`Plugin`] that provides a [`VersionedPackets`] [`Resource`].
///
/// During [`Startup`], register via the [`VersionedPacketsBuilder`] [`Resource`],
/// as the former one is read-only.
pub struct PacketsPlugin {
    /// The [`ProtocolVersion`]s of Minecraft to support.
    supported_versions: BTreeSet<ProtocolVersion>,
}

impl PacketsPlugin {
    pub fn new(supported_versions: impl IntoIterator<Item = ProtocolVersion>) -> Self {
        Self {
            supported_versions: BTreeSet::from_iter(supported_versions),
        }
    }
}

impl Plugin for PacketsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(VersionedPacketsBuilder::<Handshake>::new(
            self.supported_versions.iter().copied(),
        ))
        .insert_resource(VersionedPacketsBuilder::<Status>::new(
            self.supported_versions.iter().copied(),
        ))
        .insert_resource(VersionedPacketsBuilder::<Login>::new(
            self.supported_versions.iter().copied(),
        ))
        .insert_resource(VersionedPacketsBuilder::<Play>::new(
            self.supported_versions.iter().copied(),
        ));

        app.add_systems(
            PostStartup,
            (
                Self::build_registry::<Handshake>,
                Self::build_registry::<Status>,
                Self::build_registry::<Login>,
                Self::build_registry::<Play>,
            ),
        );
    }
}

impl PacketsPlugin {
    fn build_registry<S: ProtocolState>(mut commands: Commands) {
        commands.replace_resource(|builder: VersionedPacketsBuilder<S>| builder.build());
    }
}
