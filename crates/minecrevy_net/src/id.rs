use std::net::SocketAddr;

use bevy::{
    prelude::*,
    utils::{HashMap, HashSet},
};
use derive_more::From;

use crate::raw::RawClient;

#[derive(From, Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum ClientId {
    Addr(SocketAddr),
    Entity(Entity),
}

#[derive(Resource)]
pub struct ClientIndex {
    entities: HashMap<SocketAddr, Entity>,
}

/// Public API
impl ClientIndex {
    pub fn entity(&self, id: impl Into<ClientId>) -> Entity {
        let id = id.into();
        self.get_entity(id)
            .unwrap_or_else(|| panic!("unknown client: {id:?}"))
    }

    pub fn get_entity(&self, id: impl Into<ClientId>) -> Option<Entity> {
        match id.into() {
            ClientId::Addr(addr) => self.entities.get(&addr).copied(),
            ClientId::Entity(entity) => Some(entity),
        }
    }
}

/// ECS Systems
impl ClientIndex {
    pub fn manage_index(
        mut index: ResMut<Self>,
        clients: Query<(Entity, &RawClient), Added<RawClient>>,
        mut disconnected: RemovedComponents<RawClient>,
    ) {
        for (entity, client) in clients.iter() {
            index.entities.insert(client.addr(), entity);
        }

        let disconnected = HashSet::from_iter(disconnected.iter());
        index.entities.retain(|_, v| !disconnected.contains(v));
    }
}
