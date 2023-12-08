//! Standard library for Minecrevy servers.

#![warn(missing_docs)]

use std::io;

use bevy::{
    asset::{AssetLoader, AsyncReadExt},
    prelude::*,
    utils::BoxedFuture,
};
use thiserror::Error;

pub mod handshake;
pub mod status;

/// [`Plugin`] that provides core functionality for Minecrevy servers.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct CorePlugin {
    /// The maximum number of players allowed at once.
    pub max_players: i32,
}

impl Default for CorePlugin {
    fn default() -> Self {
        Self { max_players: 20 }
    }
}

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        assert!(
            app.is_plugin_added::<AssetPlugin>(),
            "{} must be added before {}",
            std::any::type_name::<AssetPlugin>(),
            std::any::type_name::<Self>(),
        );

        app.insert_resource(PlayerCount {
            online: 0,
            max: self.max_players,
        });

        app.init_asset::<DynamicImage>()
            .init_asset_loader::<DynamicImageLoader>();
    }
}

/// [`Resource`] for the current and maximum player count.
#[derive(Resource)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct PlayerCount {
    /// The number of players currently online.
    pub online: i32,
    /// The maximum number of players allowed at once.
    pub max: i32,
}

impl Default for PlayerCount {
    fn default() -> Self {
        Self { online: 0, max: 20 }
    }
}

impl PlayerCount {
    /// Returns `true` if the server is at maximum configured capacity.
    pub fn is_full(&self) -> bool {
        self.online >= self.max
    }
}

/// [`Asset`] that wraps [`image::DynamicImage`]s.
#[derive(Asset, TypePath, Deref, DerefMut)]
#[repr(transparent)]
pub struct DynamicImage(pub image::DynamicImage);

/// [`AssetLoader`] for [`DynamicImage`]s.
#[derive(Default)]
pub struct DynamicImageLoader;

/// Error type for [`DynamicImageLoader`].
#[derive(Error, Debug)]
pub enum DynamicImageLoaderError {
    /// Error variant for image laoding.
    #[error("Could not load image: {0}")]
    Io(#[from] io::Error),
    /// Error variant for image parsing.
    #[error("Could not parse image: {0}")]
    Image(#[from] image::ImageError),
}

impl AssetLoader for DynamicImageLoader {
    type Asset = DynamicImage;
    type Settings = ();
    type Error = DynamicImageLoaderError;

    fn load<'a>(
        &'a self,
        reader: &'a mut bevy::asset::io::Reader,
        _settings: &'a Self::Settings,
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let extension = load_context.path().extension().unwrap().to_str().unwrap();

            let format = match extension {
                "png" => image::ImageFormat::Png,
                "jpg" => image::ImageFormat::Jpeg,
                "webp" => image::ImageFormat::WebP,
                _ => unreachable!("Unsupported image format: {}", extension),
            };

            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;

            let image = image::load_from_memory_with_format(&bytes, format)?;
            Ok(DynamicImage(image))
        })
    }

    fn extensions(&self) -> &[&str] {
        &["png", "jpg", "webp"]
    }
}
