use std::{fmt, iter::once};

use bevy::{asset::Asset, prelude::*, reflect::TypeUuid};
use derive_more::{AsRef, From};
use minecrevy_core::key::Key;

/// An [`Asset`] handle or [`Tag`] handle.
#[derive(From, Clone, PartialEq, Eq, Debug, Hash)]
pub enum AssetOrTag<T: Asset> {
    /// An asset.
    Asset(Handle<T>),
    /// A tag.
    Tag(Handle<Tag<T>>),
}

impl<T: Asset> AssetOrTag<T> {
    /// Returns an iterator of all referenced handles, unpacking in case of [`Tag`](AssetOrTag::Tag).
    pub fn iter<'a>(&'a self, tags: &'a Assets<Tag<T>>) -> impl Iterator<Item = &'a Handle<T>> {
        match self {
            AssetOrTag::Asset(asset) => {
                Box::new(once(asset)) as Box<dyn Iterator<Item = &Handle<T>>>
            }
            AssetOrTag::Tag(tag) => {
                Box::new(tags.get(tag).into_iter().flat_map(|tag| &tag.handles))
            }
        }
    }

    /// Creates an untyped copy of the [`Asset`] handle or [`Tag`] handle.
    pub fn clone_untyped(&self) -> HandleUntyped {
        match self {
            AssetOrTag::Asset(asset) => asset.clone_untyped(),
            AssetOrTag::Tag(tag) => tag.clone_untyped(),
        }
    }
}

/// A group of [`Asset`]s of type `T`.
#[derive(TypeUuid, AsRef)]
#[uuid = "bf55b917-c95c-4d74-ab79-4baddefd6037"]
pub struct Tag<T: Asset> {
    /// The namespaced key identifying the tag.
    #[as_ref]
    pub key: Key,
    handles: Vec<Handle<T>>,
}

impl<T: Asset> Tag<T> {
    /// Returns true if this tag contains the specified handle to an asset.
    pub fn contains(&self, handle: &Handle<T>) -> bool {
        self.handles.contains(handle)
    }
}

impl<T: Asset> Clone for Tag<T> {
    fn clone(&self) -> Self {
        Self {
            key: self.key.clone(),
            handles: self.handles.clone(),
        }
    }
}

impl<T: Asset> PartialEq for Tag<T> {
    fn eq(&self, other: &Self) -> bool {
        self.handles == other.handles
    }
}

impl<T: Asset> Eq for Tag<T> {}

impl<T: Asset> fmt::Debug for Tag<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Tag")
            .field("values", &self.handles)
            .finish()
    }
}
