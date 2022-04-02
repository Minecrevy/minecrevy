use bevy::prelude::*;

/// Denotes operations that interact with the remote clients.
#[derive(Clone, Eq, PartialEq, Hash, Debug, SystemLabel)]
pub struct Networking;
