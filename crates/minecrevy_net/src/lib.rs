use bevy::prelude::*;

use crate::packet::{ProtocolState, VersionedPacketsBuilder};

pub mod client;
pub mod error;
pub mod flow;
pub mod id;
pub mod packet;
pub mod plugin;
pub mod raw;

pub trait AppNetworkExt {
    fn packets<S: ProtocolState>(&mut self) -> Mut<VersionedPacketsBuilder<S>>;
}

impl AppNetworkExt for App {
    fn packets<S: ProtocolState>(&mut self) -> Mut<VersionedPacketsBuilder<S>> {
        self.world
            .get_resource_or_insert_with(|| VersionedPacketsBuilder::default())
    }
}
