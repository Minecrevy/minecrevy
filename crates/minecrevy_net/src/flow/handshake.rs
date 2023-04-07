use bevy::prelude::*;
use minecrevy_protocol::{
    c2s::handshake::{Handshake as HandshakePacket, NextState},
    version::ProtocolVersion,
};

use crate::{client::Client, packet::Handshake};

/// Adds systems to handle the Minecraft protocol handshake flow.
pub struct HandshakeFlowPlugin;

impl Plugin for HandshakeFlowPlugin {
    fn build(&self, app: &mut App) {
        todo!()
    }
}

impl HandshakeFlowPlugin {
    pub fn handshake(mut commands: Commands, mut clients: Query<(Entity, Client<Handshake>)>) {
        let _span = debug_span!("handshake").entered();

        for (entity, mut client) in &mut clients {
            let Ok(handshake) = client.read::<HandshakePacket>() else {
                error!(client = %client.addr(), "received malformed handshake packet");
                continue
            };
            let Some(handshake) = handshake else { continue; };

            let next = handshake.next;

            let mut entity = commands.entity(entity);
            match next {
                NextState::Status => todo!(),
                NextState::Login => todo!(),
            }
            entity.insert(ClientInfo::from(handshake));
        }
    }
}

#[derive(Component, Clone, Debug)]
pub struct ClientInfo {
    pub version: ProtocolVersion,
    /// The server address that the client used to connect.
    pub server_address: String,
    /// The server port that the client used to connect.
    pub server_port: u16,
}

impl From<HandshakePacket> for ClientInfo {
    fn from(handshake: HandshakePacket) -> Self {
        Self {
            version: ProtocolVersion(handshake.version),
            server_address: handshake.address,
            server_port: handshake.port,
        }
    }
}
