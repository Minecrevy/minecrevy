use std::{fmt, hash::Hash, marker::PhantomData, ops::Index};

use bevy::{
    asset::{Asset, HandleId},
    ecs::system::SystemParam,
    prelude::*,
    utils::HashMap,
};
use minecrevy_core::key::Key;

pub type KeyedAssets<'w, 's, T: Asset> = IndexedAssets<'w, 's, Key, T>;

/// [System parameter](SystemParam) that provides shared access to all [`Asset`]s of type `T`,
/// additionally indexed by key `K`.
#[derive(SystemParam)]
pub struct IndexedAssets<'w, 's, K: AssetKey, T: Asset> {
    assets: Res<'w, Assets<T>>,
    index: Res<'w, AssetIndex<K, T>>,
    #[system_param(ignore)]
    _no_state: PhantomData<&'s ()>,
}

impl<'w, K: AssetKey, T: Asset> IndexedAssets<'w, '_, K, T> {
    /// Returns an iterator over all [`Asset`]s of type `T`.
    pub fn iter(&'w self) -> impl Iterator<Item = (HandleId, &'w T)> {
        self.assets.iter()
    }

    /// Returns a [`Handle`] to the [`Asset`] indexed by the given key `K`.
    pub fn get(&'w self, key: &K) -> Option<&'w Handle<T>> {
        self.index.get(key)
    }
}

impl<'w, K: AssetKey, T: Asset> IntoIterator for &'w IndexedAssets<'w, '_, K, T> {
    type Item = (HandleId, &'w T);
    type IntoIter = impl Iterator<Item = (HandleId, &'w T)>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<K: AssetKey, T: Asset> Index<&K> for IndexedAssets<'_, '_, K, T> {
    type Output = Handle<T>;

    fn index(&self, key: &K) -> &Self::Output {
        &self.index[key]
    }
}

/// An immutable index of all [`Asset`]s of type `T`, by a key type `K`.
#[derive(Resource, Deref, Clone, Debug)]
pub struct AssetIndex<K: AssetKey, T: Asset> {
    inner: HashMap<K, Handle<T>>,
}

/// A key type suitable for [`Asset`] indexing.
pub trait AssetKey: fmt::Display + Eq + Hash + Clone + Send + Sync + 'static {}

impl<K> AssetKey for K where K: fmt::Display + Eq + Hash + Clone + Send + Sync + 'static {}

impl<K: AssetKey, T: Asset + AsRef<K>> AssetIndex<K, T> {
    /// Constructs a new index from the provided [`Assets`], using `Weak` [`Handle`]s.
    pub fn new(assets: &Assets<T>) -> Self {
        Self {
            inner: assets
                .iter()
                .map(|(id, v)| (v.as_ref().clone(), Handle::weak(id)))
                .collect(),
        }
    }

    /// A system that builds the index based on the current assets.
    /// Place this system after all asset registering for a given type is finished.
    pub fn build(commands: &mut Commands, assets: Res<Assets<T>>) {
        commands.insert_resource(Self::new(&assets));

        debug!(
            "built {} asset index for {}",
            std::any::type_name::<K>(),
            std::any::type_name::<T>()
        );
    }

    /// A system that rebuilds the index if the assets changed.
    pub fn rebuild_on_change(assets: Res<Assets<T>>, mut index: ResMut<Self>) {
        if assets.is_changed() {
            *index = Self::new(&assets);

            debug!(
                "rebuilt {} asset index for {} because an asset was changed, added, or removed",
                std::any::type_name::<K>(),
                std::any::type_name::<T>()
            );
        }
    }
}

impl<K: AssetKey, T: Asset> Index<&K> for AssetIndex<K, T> {
    type Output = Handle<T>;

    fn index(&self, key: &K) -> &Self::Output {
        self.get(key)
            .unwrap_or_else(|| panic!("{} {} is not indexed", std::any::type_name::<T>(), key))
    }
}

pub use minecrevy_asset_macros::ExtractIndexedAssets;

pub trait ExtractIndexedAssets<K: AssetKey> {
    type Asset: Asset + AsRef<K>;

    fn extract(index: &AssetIndex<K, Self::Asset>) -> Self;
}
