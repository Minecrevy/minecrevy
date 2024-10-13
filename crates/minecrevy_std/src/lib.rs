//! Standard library for Minecrevy servers.

#![warn(missing_docs)]

use bevy::prelude::*;

pub mod handshake;
pub mod status;

/// [`Plugin`] that provides core functionality for Minecrevy servers.
///
/// Configurable [`Resource`]s:
/// - [`PlayerCount`]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Default)]
pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        assert!(
            app.is_plugin_added::<AssetPlugin>(),
            "{} must be added before {}",
            std::any::type_name::<AssetPlugin>(),
            std::any::type_name::<Self>(),
        );

        app.init_resource::<PlayerCount>();
    }
}

/// [`Resource`] for the current and maximum player count.
#[derive(Resource)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct PlayerCount {
    /// The number of players currently online.
    pub online: i32,
    /// The maximum number of players allowed at once.
    pub max: i32,
}

impl Default for PlayerCount {
    fn default() -> Self {
        Self { online: 0, max: 20 }
    }
}

impl PlayerCount {
    /// Returns `true` if the server is at maximum configured capacity.
    pub fn is_full(&self) -> bool {
        self.online >= self.max
    }
}
