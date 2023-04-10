use bevy::{app::PluginGroupBuilder, prelude::*};

use crate::{
    protocol::flow::{
        handshake::HandshakeFlowPlugin, login::LoginFlowPlugin, status::StatusFlowPlugin,
    },
    raw::RawNetworkPlugin,
};

/// This [`PluginGroup`] will add all the default networking plugins needed to handle
/// pre-gameplay Minecraft protocol flow. Gameplay flow is specifically excluded as it
/// requires a much larger breadth of features to fully implement.
///
/// See [`MinimalNetworkPlugins`] if interested in providing a custom Status and Login flow.
pub struct DefaultNetworkPlugins;

impl PluginGroup for DefaultNetworkPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(RawNetworkPlugin)
            .add(HandshakeFlowPlugin)
            .add(StatusFlowPlugin)
            .add(LoginFlowPlugin)
    }
}

/// This [`PluginGroup`] will add the minimal networking plugins required to handle
/// Minecraft protocol handshake flow. Everything else after (status and login)
/// are left for implementation.
///
/// See [`DefaultNetworkPlugins`] if interested in a more holistic builtin pre-gameplay flow.
pub struct MinimalNetworkPlugins;

impl PluginGroup for MinimalNetworkPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(RawNetworkPlugin)
            .add(HandshakeFlowPlugin)
    }
}
