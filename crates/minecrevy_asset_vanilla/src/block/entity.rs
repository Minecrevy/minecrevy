use bevy::asset::Handle;
use bevy::ecs::system::Resource;
use minecrevy_asset::{block::BlockEntityType, index::ExtractIndexedAssets};

/// All [`BlockEntityType`]s in vanilla Minecraft.
#[derive(Resource, ExtractIndexedAssets)]
#[extract(asset = "BlockEntityType")]
pub struct BlockEntityTypes {
    pub furnace: Handle<BlockEntityType>,
    pub chest: Handle<BlockEntityType>,
    pub trapped_chest: Handle<BlockEntityType>,
    pub ender_chest: Handle<BlockEntityType>,
    pub jukebox: Handle<BlockEntityType>,
    pub dispenser: Handle<BlockEntityType>,
    pub dropper: Handle<BlockEntityType>,
    pub sign: Handle<BlockEntityType>,
    pub hanging_sign: Handle<BlockEntityType>,
    pub mob_spawner: Handle<BlockEntityType>,
    pub piston: Handle<BlockEntityType>,
    pub brewing_stand: Handle<BlockEntityType>,
    pub enchanting_table: Handle<BlockEntityType>,
    pub end_portal: Handle<BlockEntityType>,
    pub beacon: Handle<BlockEntityType>,
    pub skull: Handle<BlockEntityType>,
    pub daylight_detector: Handle<BlockEntityType>,
    pub hopper: Handle<BlockEntityType>,
    pub comparator: Handle<BlockEntityType>,
    pub banner: Handle<BlockEntityType>,
    pub structure_block: Handle<BlockEntityType>,
    pub end_gateway: Handle<BlockEntityType>,
    pub command_block: Handle<BlockEntityType>,
    pub shulker_box: Handle<BlockEntityType>,
    pub bed: Handle<BlockEntityType>,
    pub conduit: Handle<BlockEntityType>,
    pub barrel: Handle<BlockEntityType>,
    pub smoker: Handle<BlockEntityType>,
    pub blast_furnace: Handle<BlockEntityType>,
    pub lectern: Handle<BlockEntityType>,
    pub bell: Handle<BlockEntityType>,
    pub jigsaw: Handle<BlockEntityType>,
    pub campfire: Handle<BlockEntityType>,
    pub beehive: Handle<BlockEntityType>,
    pub sculk_sensor: Handle<BlockEntityType>,
    pub sculk_catalyst: Handle<BlockEntityType>,
    pub sculk_shrieker: Handle<BlockEntityType>,
    pub chiseled_bookshelf: Handle<BlockEntityType>,
}
