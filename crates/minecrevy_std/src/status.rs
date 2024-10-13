//! This module contains the [`StatusPlugin`], which handles status packets.

use std::{
    io::{self, Cursor},
    path::PathBuf,
};

use base64::Engine;
use bevy::{
    asset::{io::Reader, AssetLoader, LoadContext},
    prelude::*,
    utils::ConditionalSendFuture,
};
use image::{imageops::FilterType, ImageFormat};
use minecrevy_net::{client::PacketWriter, packet::Recv};
use minecrevy_protocol::{
    status::{Ping, Request, Response, ResponsePlayers, ResponseProfile, ResponseVersion},
    ServerProtocolPlugin,
};
use minecrevy_text::Text;
use thiserror::Error;
use uuid::Uuid;

use crate::{
    handshake::{ClientInfo, HandshakePlugin},
    CorePlugin, PlayerCount,
};

/// [`Plugin`] for handling status packets.
///
/// Configurable [`Resource`]s:
/// - [`ServerProtocol`]: The protocol version to send to clients.
/// - [`ServerProtocolName`]: The name of the protocol version to send to clients.
/// - [`Motd`]: The message of the day displayed in the server list.
/// - [`PlayerSample`]: The list of sample player names to display in the server list.
/// - [`PlayerCount`]: The number of players to display in the server list, online and maximum.
/// - [`ServerListFavicon`]: The favicon to display in the server list.
#[derive(Default)]
pub struct StatusPlugin {
    /// The path of the favicon to display in the server list.
    pub favicon_path: Option<PathBuf>,
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
        assert!(
            app.is_plugin_added::<HandshakePlugin>(),
            "{} must be added before {}",
            std::any::type_name::<HandshakePlugin>(),
            std::any::type_name::<Self>(),
        );

        app.init_resource::<ServerProtocol>();
        app.init_resource::<Motd>();
        app.init_resource::<PlayerSample>();
        app.init_resource::<ServerListFavicon>();
        app.init_asset::<Favicon>()
            .init_asset_loader::<FaviconLoader>();

        // Load the favicon if one was provided.
        if let Some(path) = self.favicon_path.clone() {
            app.add_systems(Startup, Self::load_favicon(path));
        }

        // Handle status::Request and status::Ping packets.
        app.add_observer(Self::on_status_request);
        app.add_observer(Self::on_status_ping);
    }
}

impl StatusPlugin {
    /// Returns a [`System`] that loads the favicon from the given filename.
    pub fn load_favicon(path: PathBuf) -> impl IntoSystem<(), (), ()> {
        IntoSystem::into_system(
            move |asset_server: Res<AssetServer>, mut favicon: ResMut<ServerListFavicon>| {
                let handle = asset_server.load(path.clone());
                favicon.0 = Some(handle);
            },
        )
    }

    /// [`Observer`] [`System`] that handles displaying the MOTD and favicon to clients in the server list.
    #[expect(clippy::too_many_arguments)]
    pub fn on_status_request(
        trigger: Trigger<Recv<Request>>,
        mut writer: PacketWriter,
        counts: Res<PlayerCount>,
        version_name: Res<ServerProtocolName>,
        version: Res<ServerProtocol>,
        motd: Res<Motd>,
        sample: Res<PlayerSample>,
        favicon: Res<ServerListFavicon>,
        favicons: Res<Assets<Favicon>>,
        client_info: Query<&ClientInfo>,
    ) {
        let writer = writer.client(trigger.entity());

        let favicon = favicon
            .0
            .as_ref()
            .and_then(|handle| favicons.get(handle))
            .map(|f| f.base64.clone());

        let version = match *version {
            ServerProtocol::Echo => client_info
                .get(trigger.entity())
                .map(|i| i.protocol_version)
                .unwrap_or(0),
            ServerProtocol::Version(v) => v,
        };

        writer.send(&Response {
            version: ResponseVersion {
                name: version_name.0.clone(),
                protocol: version,
            },
            players: ResponsePlayers {
                max: counts.max,
                online: counts.online,
                sample: sample
                    .0
                    .clone()
                    .into_iter()
                    .map(|name| ResponseProfile {
                        name,
                        id: Uuid::nil(),
                    })
                    .collect(),
            },
            description: motd.0.clone(),
            favicon,
            enforces_secure_chat: None,
            previews_chat: None,
        });
    }

    /// [`Observer`] [`System`] that responds to clients' ping requests.
    pub fn on_status_ping(trigger: Trigger<Recv<Ping>>, mut writer: PacketWriter) {
        let packet = &trigger.event().0;
        let writer = writer.client(trigger.entity());

        // Echo the client's payload back to them.
        writer.send(packet);
    }
}

/// [`Resource`] that stores the protocol version of the server to send to clients.
#[derive(Resource)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum ServerProtocol {
    /// The client's protocol version wlll be sent back to them.
    #[default]
    Echo,
    /// This specific protocol version will be sent to clients.
    Version(i32),
}

/// [`Resource`] that stores the name of the protocol version to send to clients.
#[derive(Resource)]
#[derive(Clone, PartialEq, Debug)]
pub struct ServerProtocolName(pub String);

impl Default for ServerProtocolName {
    fn default() -> Self {
        Self("Ping Server".into())
    }
}

/// [`Resource`] for the message of the day. Displayed in the server list.
#[derive(Resource, Deref, DerefMut)]
#[derive(Clone, PartialEq, Debug)]
pub struct Motd(pub Text);

impl Default for Motd {
    fn default() -> Self {
        Self(Text::from("A Minecraft Server"))
    }
}

/// [`Resource`] for the list of sample player names to display in the server list.
#[derive(Resource, Deref, DerefMut)]
#[derive(Clone, PartialEq, Debug, Default)]
pub struct PlayerSample(pub Vec<String>);

/// [`Resource`] that stores a [`Handle`] to the [`Favicon`], which is used to
/// display an image in the server list. The favicon is automatically resized
/// to 64x64 pixels. If no favicon is provided, no image will be sent to clients.
///
/// Clients appear to cache the favicon, so it will not be removed if the server
/// stops sending it.
#[derive(Resource, Default)]
pub struct ServerListFavicon(pub Option<Handle<Favicon>>);

/// [`Asset`] that wraps [`image::DynamicImage`]s.
#[derive(Asset, TypePath)]
pub struct Favicon {
    /// The image data.
    pub image: image::DynamicImage,
    /// The base64-encoded image data.
    pub base64: String,
}

/// [`AssetLoader`] for [`DynamicImage`]s.
#[derive(Default)]
pub struct FaviconLoader;

/// Error type for [`DynamicImageLoader`].
#[derive(Error, Debug)]
pub enum FaviconLoaderError {
    /// Error variant for image laoding.
    #[error("Could not load image: {0}")]
    Io(#[from] io::Error),
    /// Error variant for image parsing.
    #[error("Could not parse image: {0}")]
    Image(#[from] image::ImageError),
}

impl AssetLoader for FaviconLoader {
    type Asset = Favicon;
    type Settings = ();
    type Error = FaviconLoaderError;

    fn extensions(&self) -> &[&str] {
        &["png", "jpg", "webp"]
    }

    fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        load_context: &mut LoadContext,
    ) -> impl ConditionalSendFuture<Output = Result<Self::Asset, Self::Error>> {
        async move {
            let extension = load_context.path().extension().unwrap().to_str().unwrap();

            let format = match extension {
                "png" => image::ImageFormat::Png,
                "jpg" => image::ImageFormat::Jpeg,
                "webp" => image::ImageFormat::WebP,
                _ => unreachable!("Unsupported image format: {}", extension),
            };

            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;

            let mut image = image::load_from_memory_with_format(&bytes, format)?;

            // Resize the image to 64x64 if it isn't already.
            if image.width() != 64 || image.height() != 64 {
                image = image.resize(64, 64, FilterType::Nearest);
            }

            // Encode the image to base64 PNG.
            let mut bytes: Cursor<Vec<u8>> = Cursor::new(Vec::new());
            image.write_to(&mut bytes, ImageFormat::Png)?;
            let b64 = base64::engine::general_purpose::STANDARD.encode(bytes.into_inner());

            Ok(Favicon {
                image,
                base64: format!("data:image/png;base64,{}", b64),
            })
        }
    }
}
