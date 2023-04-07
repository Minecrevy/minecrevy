use bevy::prelude::*;
use minecrevy_asset::{index::ExtractIndexedAssets, mob::villager::PoiType, tag::Tag};

/// All [`PoiType`] [`Tag`]s in vanilla Minecraft.
#[derive(Resource, ExtractIndexedAssets)]
#[extract(asset = "Tag<PoiType>")]
pub struct PoiTypes {
    pub acquirable_job_site: Handle<Tag<PoiType>>,
    pub village: Handle<Tag<PoiType>>,
    pub bee_home: Handle<Tag<PoiType>>,
}
