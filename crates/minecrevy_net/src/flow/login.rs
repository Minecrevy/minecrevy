use bevy::{prelude::*, utils::HashSet};
use minecrevy_core::key::Key;

use crate::{
    client::{Client, ClientEntered},
    packet::Login,
};

/// Adds systems to handle the Minecraft protocol login flow.
pub struct LoginFlowPlugin;

impl Plugin for LoginFlowPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LoginHandlers::default());
        app.add_systems(Update, (Self::begin_login, Self::finish_login).chain());
    }
}

/// ECS Systems
impl LoginFlowPlugin {
    pub fn begin_login(
        mut commands: Commands,
        clients: Query<(Entity, Client<Login>), ClientEntered<Login>>,
    ) {
        // Begin channel negotiations
        for (entity, _) in &clients {
            commands
                .entity(entity)
                .insert(FinishedLoginHandlers::default());
        }
    }

    pub fn finish_login(
        all_channels: Res<LoginHandlers>,
        clients: Query<(Client<Login>, &FinishedLoginHandlers), Changed<FinishedLoginHandlers>>,
    ) {
        for (client, channels) in &clients {
            // Send Login Success if all channels are finished negotiating.
            if channels.is_finished(&all_channels) {
                todo!()
            }
        }
    }
}

pub trait AppLoginFlowExt {
    /// Adds a login handler that **MUST** signal finish for the login flow to end.
    fn add_login_handler(&mut self, channel: LoginHandler) -> &mut Self;
}

impl AppLoginFlowExt for App {
    fn add_login_handler(&mut self, channel: LoginHandler) -> &mut Self {
        let mut channels = self.world.resource_mut::<LoginHandlers>();
        channels.insert(channel);

        self
    }
}

pub struct EncryptionRequested();

/// The set of [`LoginHandler`]s that have finished negotation for a given client.
/// Login flow for a client will not be ended until this set is a superset of the
/// [set of all registered channels](LoginHandlers).
///
/// A login flow channel can signal it is finished negotating by inserting its [`ChannelKey`]
/// with [`FinishedLoginHandlers::finish`].
#[derive(Component, Deref, Default)]
pub struct FinishedLoginHandlers(HashSet<LoginHandler>);

impl FinishedLoginHandlers {
    /// Adds the given channel to the set of finished channels.
    pub fn finish(&mut self, channel: LoginHandler) {
        self.0.insert(channel);
    }

    /// Returns `true` when all registered channels have finished negotation.
    pub fn is_finished(&self, all_channels: &LoginHandlers) -> bool {
        self.0.is_superset(&all_channels.0)
    }
}

/// The set of all registered [`LoginHandler`]s.
#[derive(Resource, Deref, Default)]
pub struct LoginHandlers(HashSet<LoginHandler>);

impl LoginHandlers {
    pub fn insert(&mut self, channel: LoginHandler) {
        self.0.insert(channel);
    }
}

/// The identifier of a login handler.
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct LoginHandler(pub Key);
