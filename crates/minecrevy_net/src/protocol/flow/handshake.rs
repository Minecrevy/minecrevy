use bevy::{ecs::system::EntityCommands, prelude::*};
use minecrevy_io::{McRead, McWrite, Packet};

use crate::protocol::{
    client::{Client, ClientConnected, PacketQueue, PacketRegistry},
    registry::{Packets, VersionedPackets, VersionedPacketsBuilder},
    state::{Handshake, Login, ProtocolState, Status},
    version::{ProtocolVersion, ReleaseVersion},
};

/// A [`SystemSet`] for handling packets as part of the protocol handshake.
#[derive(SystemSet, Clone, PartialEq, Eq, Debug, Hash)]
pub struct HandshakeFlow;

/// Adds systems to handle the Minecraft protocol handshake flow.
pub struct HandshakeFlowPlugin;

impl Plugin for HandshakeFlowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Self::register_packets);

        app.add_systems(
            PreUpdate,
            (Self::start_handshake, Self::handshake)
                .chain()
                .in_set(HandshakeFlow),
        );
    }
}

/// ECS Systems
impl HandshakeFlowPlugin {
    /// Registers the packets needed during handshake flow.
    fn register_packets(mut handshake: ResMut<VersionedPacketsBuilder<Handshake>>) {
        handshake.add_incoming::<HandshakePacket>(0x00, ReleaseVersion::V1_7_2.v()..);
    }

    /// Inserts components to enable [`Client<Handshake>`] querying.
    fn start_handshake(
        mut commands: Commands,
        handshake: Res<VersionedPackets<Handshake>>,
        clients: Query<Entity, ClientConnected>,
    ) {
        let Some((_, registry)) = handshake.min() else {
            panic!(
                "{} registry does not exist, protocol handshake flow cannot run without a handshake packet",
                std::any::type_name::<Packets<Handshake>>()
            );
        };

        for entity in &clients {
            commands.entity(entity).insert((
                PacketQueue::<Handshake>::default(),
                PacketRegistry(registry.clone()),
            ));
        }
    }

    /// Reads incoming handshake packets and performs the standard protocol handshake flow.
    fn handshake(
        mut commands: Commands,
        status: Res<VersionedPackets<Status>>,
        login: Res<VersionedPackets<Login>>,
        mut clients: Query<(Entity, Client<Handshake>)>,
    ) {
        fn change_state<S: ProtocolState>(
            entity: &mut EntityCommands,
            state: &VersionedPackets<S>,
            version: ProtocolVersion,
        ) -> bool {
            let Some(registry) = state.get(version) else {
                warn!(
                    "{} registry does not exist, cannot enter {} flow without one.",
                    std::any::type_name::<Packets<S>>(),
                    std::any::type_name::<S>()
                );
                entity.despawn();
                return false;
            };

            entity.insert((
                PacketQueue::<S>::default(),
                PacketRegistry(registry.clone()),
            ));
            return true;
        }

        for (entity, mut client) in &mut clients {
            let _handshake = debug_span!("handshake", client = %client.addr()).entered();

            let mut entity = commands.entity(entity);

            let Some(handshake) = client.read::<HandshakePacket>() else { continue; };
            let Ok(handshake) = handshake else {
                error!("received malformed handshake packet");
                entity.despawn();
                continue
            };

            match handshake.next {
                NextState::Status => {
                    if !change_state(&mut entity, &status, handshake.version) {
                        continue;
                    }
                }
                NextState::Login => {
                    if !change_state(&mut entity, &login, handshake.version) {
                        continue;
                    }
                }
            }
            entity.insert(ClientInfo::from(handshake));
        }
    }
}

/// Client information stored on a client's [`Entity`] as given by the [`HandshakePacket`].
#[derive(Component, Clone, Debug)]
pub struct ClientInfo {
    /// The client's protocol version.
    pub version: ProtocolVersion,
    /// The server address that the client used to connect.
    pub server_address: String,
    /// The server port that the client used to connect.
    pub server_port: u16,
}

impl From<HandshakePacket> for ClientInfo {
    fn from(handshake: HandshakePacket) -> Self {
        Self {
            version: handshake.version,
            server_address: handshake.address,
            server_port: handshake.port,
        }
    }
}

/// Sent by the client to begin Minecraft protocol communication.
#[derive(Packet, McRead, McWrite, Clone, PartialEq, Debug)]
pub struct HandshakePacket {
    /// The protocol version.
    pub version: ProtocolVersion,
    /// The server address.
    #[options(max_len = 255)]
    pub address: String,
    /// The server port.
    pub port: u16,
    /// The intended handshake flow.
    pub next: NextState,
}

/// The intended handshake flow sent by the [`Handshake`] packet.
#[derive(McRead, McWrite, Clone, Copy, PartialEq, Eq, Debug, Hash)]
#[io_repr(varint)]
pub enum NextState {
    /// The client just wants server information.
    Status = 1,
    /// The client wants to login and play.
    Login = 2,
}
