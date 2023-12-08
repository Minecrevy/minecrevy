//! A Minecraft networking library, integrated with [Bevy](bevy).

#![warn(missing_docs)]

use bevy::{app::PluginGroupBuilder, prelude::*};
use minecrevy_io::{McRead, McWrite};

use crate::{
    client::{Client, Paused, ProtocolState},
    packet::{IncomingPackets, PacketIds, Recv},
    server::{ControlServer, ServerPlugin},
};

pub mod client;
pub mod packet;
pub mod server;

/// [`PluginGroup`] for the [`NetworkPlugin`] and [`ServerPlugin`].
pub struct NetworkServerPlugins;

impl PluginGroup for NetworkServerPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(NetworkPlugin)
            .add(ServerPlugin)
    }
}

/// [`SystemSet`]s for the [`NetworkPlugin`].
#[derive(SystemSet, Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum NetworkSets {
    /// The set of systems that read incoming packets from clients, and buffers them for processing.
    ///
    /// This set is configured to run before [`NetworkSets::EmitPacketEvents`].
    ReadPackets,
    /// The set of systems that emit packet events.
    EmitPacketEvents,
}

/// [`Plugin`] for basic network functionality.
pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<IncomingPackets>();
        app.init_resource::<PacketIds>();

        app.configure_sets(
            PreUpdate,
            (NetworkSets::ReadPackets, NetworkSets::EmitPacketEvents).chain(),
        );

        // NetworkSets::ReadPackets
        app.add_systems(
            PreUpdate,
            Self::read_incoming_packets.in_set(NetworkSets::ReadPackets),
        );
    }
}

impl NetworkPlugin {
    /// [`System`] that reads incoming packets from clients,
    /// and buffers them into [`IncomingPackets`] for later event emission.
    fn read_incoming_packets(
        packets: Res<PacketIds>,
        mut incoming: ResMut<IncomingPackets>,
        mut clients: Query<(Entity, &Client, &ProtocolState, &mut Paused)>,
    ) {
        for (entity, client, &state, mut paused) in clients.iter_mut() {
            if paused.0 {
                continue;
            }

            for packet in client.incoming.try_iter() {
                let id = packet.id;

                incoming.insert(state, packet, entity);

                if packets.is_stateful(state, id) {
                    paused.0 = true;
                    break;
                }
            }
        }
    }

    /// [`System`] supplier that emits [`Recv<T>`] events for incoming packets.
    fn emit_packet_events<T: McRead + Send + Sync + 'static>(
        state: ProtocolState,
    ) -> impl FnMut(Res<PacketIds>, Res<IncomingPackets>, EventWriter<Recv<T>>) {
        move |ids: Res<PacketIds>,
              incoming: Res<IncomingPackets>,
              mut events: EventWriter<Recv<T>>| {
            let id = ids.incoming::<T>(state).unwrap();

            for (client, packet) in incoming.drain::<T>(state, id) {
                match packet {
                    Ok(packet) => events.send(Recv { client, packet }),
                    Err(err) => {
                        error!("Error reading packet from {client:?}: {}", err);
                    }
                }
            }
        }
    }
}

/// Extension trait for [`App`] to add network-related functionality.
pub trait AppNetworkExt {
    /// Registers the given incoming packet type with the given [`ProtocolState`] and packet ID.
    ///
    /// If `stateful` is `true`, then the client will be paused until the packet is handled.
    fn add_incoming_packet<T: McRead + Send + Sync + 'static>(
        &mut self,
        state: ProtocolState,
        id: i32,
        stateful: bool,
    ) -> &mut Self;

    /// Registers the given outgoing packet type with the given [`ProtocolState`] and packet ID.
    fn add_outgoing_packet<T: McWrite + Send + Sync + 'static>(
        &mut self,
        state: ProtocolState,
        id: i32,
    ) -> &mut Self;
}

impl AppNetworkExt for App {
    fn add_incoming_packet<T: McRead + Send + Sync + 'static>(
        &mut self,
        state: ProtocolState,
        id: i32,
        stateful: bool,
    ) -> &mut Self {
        // Save the packet ID for event emission.
        let mut ids = self.world.resource_mut::<PacketIds>();
        ids.insert_incoming::<T>(state, id, stateful);

        // NetworkSets::EmitPacketEvents
        self.add_event::<Recv<T>>();
        self.add_systems(
            PreUpdate,
            NetworkPlugin::emit_packet_events::<T>(state).in_set(NetworkSets::EmitPacketEvents),
        );

        self
    }

    fn add_outgoing_packet<T: McWrite + Send + Sync + 'static>(
        &mut self,
        state: ProtocolState,
        id: i32,
    ) -> &mut Self {
        // Save the packet ID.
        let mut ids = self.world.resource_mut::<PacketIds>();
        ids.insert_outgoing::<T>(state, id, false);

        self
    }
}

/// [`System`] supplier that tells the [`Server`](server::Server) to start listening for connections.
pub fn start_server(
    address: impl Into<String>,
    port: u16,
) -> impl FnMut(EventWriter<ControlServer>) {
    let address = address.into();
    move |mut control: EventWriter<ControlServer>| {
        control.send(ControlServer::Start {
            address: address.clone(),
            port,
        });
    }
}
