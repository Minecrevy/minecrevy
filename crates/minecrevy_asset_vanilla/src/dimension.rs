use bevy::prelude::{Handle, Resource};
use minecrevy_asset::{dimension::DimensionType, index::ExtractIndexedAssets};

/// All [`DimensionType`]s in vanilla Minecraft.
#[derive(Resource, ExtractIndexedAssets)]
#[extract(asset = "DimensionType")]
pub struct DimensionTypes {
    pub overworld: Handle<DimensionType>,
    pub the_nether: Handle<DimensionType>,
    pub the_end: Handle<DimensionType>,
    pub overworld_caves: Handle<DimensionType>,
}
