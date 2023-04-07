use bevy::prelude::*;
use minecrevy_asset::{block::Fluid, index::ExtractIndexedAssets, tag::Tag};

/// All [`Fluid`] [`Tag`]s in vanilla Minecraft.
#[derive(Resource, ExtractIndexedAssets)]
#[extract(asset = "Tag<Fluid>")]
pub struct FluidTags {
    pub water: Handle<Tag<Fluid>>,
    pub lava: Handle<Tag<Fluid>>,
}
