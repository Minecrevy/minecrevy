use bevy::prelude::*;
use minecrevy_ecs::label::Networking;
use minecrevy_net::socket::{Play, Socket};
use minecrevy_protocol_latest::server;
use minecrevy_text::Text;
use crate::entity::player::TabList;

/// Handles [`TabList`] updates.
pub struct TabListPlugin;

impl Plugin for TabListPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(Self::update_tab_list.label(Networking));
    }
}

impl TabListPlugin {
    fn update_tab_list(mut players: Query<(Socket<Play>, Option<&TabList>), Changed<TabList>>) {
        for (mut socket, tablist) in players.iter_mut() {
            if let Some(tablist) = tablist {
                socket.send(server::TabListHeaderAndFooter {
                    header: tablist.header.clone().unwrap_or(Text::empty()),
                    footer: tablist.footer.clone().unwrap_or(Text::empty()),
                });
            }
        }
    }
}
