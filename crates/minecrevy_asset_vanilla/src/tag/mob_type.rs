use bevy::prelude::*;
use minecrevy_asset::{index::ExtractIndexedAssets, mob::MobType, tag::Tag};

/// All [`MobType`] [`Tag`]s in vanilla Minecraft.
#[derive(Resource, ExtractIndexedAssets)]
#[extract(asset = "Tag<MobType>")]
pub struct MobTypeTags {
    pub skeletons: Handle<Tag<MobType>>,
    pub raiders: Handle<Tag<MobType>>,
    pub beehive_inhabitors: Handle<Tag<MobType>>,
    pub arrows: Handle<Tag<MobType>>,
    pub impact_projectiles: Handle<Tag<MobType>>,
    pub powder_snow_walkable_mobs: Handle<Tag<MobType>>,
    pub axolotl_always_hostiles: Handle<Tag<MobType>>,
    pub axolotl_hunt_targets: Handle<Tag<MobType>>,
    pub freeze_immune_entity_ttypes: Handle<Tag<MobType>>,
    pub freeze_hurts_extra_types: Handle<Tag<MobType>>,
    pub frog_food: Handle<Tag<MobType>>,
}
