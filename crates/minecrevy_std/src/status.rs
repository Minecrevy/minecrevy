//! This module contains the [`StatusPlugin`], which handles status packets.

use std::io::Cursor;

use base64::Engine;
use bevy::prelude::*;
use image::{imageops::FilterType, ImageOutputFormat};
use minecrevy_net::{client::ClientQReadOnly, packet::Recv};
use minecrevy_protocol::{
    common::PingRequest,
    status::{Request, Response, ResponsePlayers, ResponseVersion},
    PacketHandlerSet, ServerProtocolPlugin,
};
use minecrevy_text::Text;

use crate::{handshake::ConnectionInfo, profile::Profile, CorePlugin, DynamicImage, PlayerCount};

/// [`Plugin`] for handling status packets.
#[derive(Default)]
pub struct StatusPlugin {
    /// The text displayed in the server list.
    pub motd: Option<Text>,
    /// The filename of the favicon to display in the server list.
    pub favicon_filename: Option<String>,
    /// Whether or not to show the list of players in the server list.
    ///
    /// Be mindful that this can be a privacy concern.
    pub show_players: bool,
}

impl Plugin for StatusPlugin {
    fn build(&self, app: &mut App) {
        assert!(
            app.is_plugin_added::<ServerProtocolPlugin>(),
            "{} must be added before {}",
            std::any::type_name::<ServerProtocolPlugin>(),
            std::any::type_name::<Self>(),
        );
        assert!(
            app.is_plugin_added::<CorePlugin>(),
            "{} must be added before {}",
            std::any::type_name::<CorePlugin>(),
            std::any::type_name::<Self>(),
        );

        // Insert the player list visibility.
        app.insert_resource(ShowPlayers(self.show_players));

        // Use the provided MOTD, or the default if none was provided.
        app.insert_resource(self.motd.clone().map(Motd).unwrap_or_default());
        app.init_resource::<Favicon>();

        // Load the favicon if one was provided.
        if let Some(filename) = self.favicon_filename.clone() {
            app.add_systems(Startup, Self::load_favicon(filename));
        }

        // Handle status::Request and status::Ping packets.
        app.add_systems(
            Update,
            (Self::handle_status_requests, Self::handle_status_pings)
                .in_set(PacketHandlerSet::Status),
        );
    }
}

impl StatusPlugin {
    /// Returns a [`System`] that loads the favicon from the given filename.
    pub fn load_favicon(filename: String) -> impl FnMut(Res<AssetServer>, ResMut<Favicon>) {
        move |asset_server: Res<AssetServer>, mut favicon: ResMut<Favicon>| {
            let handle = asset_server.load(filename.clone());
            favicon.0 = Some(handle);
        }
    }

    /// [`System`] that handles displaying the MOTD and favicon to clients in the server list.
    pub fn handle_status_requests(
        mut requests: EventReader<Recv<Request>>,
        clients: Query<(ClientQReadOnly, &ConnectionInfo)>,
        players: Query<&Profile>,
        counts: Res<PlayerCount>,
        show_players: Res<ShowPlayers>,
        motd: Res<Motd>,
        favicon: Res<Favicon>,
        mut images: ResMut<Assets<DynamicImage>>,
        mut favicon_cache: Local<Option<FaviconCache>>,
    ) {
        // Update the favicon cache.
        'favicon: {
            let is_cache_dirty = favicon_cache.as_ref().map(|fc| &fc.handle) != favicon.0.as_ref();
            if is_cache_dirty {
                let Some(favicon) = favicon.0.as_ref() else {
                    // favicon was removed
                    *favicon_cache = None;
                    break 'favicon;
                };

                let Some(image) = images.get(favicon) else {
                    // Favicon was not loaded yet.
                    *favicon_cache = None;
                    break 'favicon;
                };

                // Resize the image to 64x64 if necessary.
                // The vanilla client won't display favicons that aren't 64x64.
                if image.width() != 64 || image.height() != 64 {
                    warn!("Favicon {:?} is not 64x64, resizing", favicon.path());
                    let resized = image.resize_to_fill(64, 64, FilterType::Nearest);

                    // Replace the loaded image with the resized one.
                    images.insert(favicon, DynamicImage(resized));
                }

                // Reborrow the image (in case it was resized).
                let Some(image) = images.get(favicon) else {
                    error!("Favicon was unloaded somehow?");
                    *favicon_cache = None;
                    break 'favicon;
                };

                // Encode the image to base64 PNG.
                let mut bytes: Cursor<Vec<u8>> = Cursor::new(Vec::new());
                if let Err(e) = image.write_to(&mut bytes, ImageOutputFormat::Png) {
                    error!("Could not encode favicon as a PNG: {}", e);
                    *favicon_cache = None;
                    break 'favicon;
                }
                let b64 = base64::engine::general_purpose::STANDARD.encode(bytes.into_inner());

                *favicon_cache = Some(FaviconCache {
                    handle: favicon.clone(),
                    data: format!("data:image/png;base64,{}", b64),
                });
                info!("Favicon loaded and cached.");
            }
        }

        for Recv { client, packet: _ } in requests.read() {
            let Ok((client, info)) = clients.get(*client) else {
                continue;
            };

            client.send(Response {
                version: ResponseVersion {
                    name: "Ping Server".into(),
                    protocol: info.protocol_version,
                },
                players: ResponsePlayers {
                    max: counts.max,
                    online: counts.online,
                    sample: if **show_players {
                        players.iter().map(|v| v.into()).collect::<Vec<_>>()
                    } else {
                        Vec::new()
                    },
                },
                description: motd.0.clone(),
                favicon: favicon_cache.as_ref().map(|f| &f.data).cloned(),
                enforces_secure_chat: None,
                previews_chat: None,
            });
        }
    }

    /// [`System`] that responds to clients' ping requests.
    pub fn handle_status_pings(
        mut packets: EventReader<Recv<PingRequest>>,
        clients: Query<ClientQReadOnly>,
    ) {
        for Recv { client, packet } in packets.read() {
            let Ok(client) = clients.get(*client) else {
                continue;
            };

            client.send(packet.clone());
        }
    }
}

/// [`Resource`] for whether or not to show the list of players in the server list.
#[derive(Resource, Deref, DerefMut)]
#[derive(Clone, PartialEq, Debug)]
pub struct ShowPlayers(pub bool);

/// [`Resource`] for the message of the day. Displayed in the server list.
#[derive(Resource, Deref, DerefMut)]
#[derive(Clone, PartialEq, Debug)]
pub struct Motd(pub Text);

impl Default for Motd {
    fn default() -> Self {
        Self(Text::from("A Minecraft Server"))
    }
}

/// [`Resource`] for the image to display in the server list.
#[derive(Resource, Deref, DerefMut)]
#[derive(Clone, PartialEq, Debug, Default)]
pub struct Favicon(pub Option<Handle<DynamicImage>>);

/// [`Local`] used by [`StatusPlugin`] to cache the base64-encoded favicon.
pub struct FaviconCache {
    /// The [`Handle`] to the currently encoded favicon.
    handle: Handle<DynamicImage>,
    /// The base64-encoded favicon PNG.
    data: String,
}
