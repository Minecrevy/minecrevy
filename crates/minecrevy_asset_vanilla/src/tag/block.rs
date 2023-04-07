use bevy::prelude::*;
use minecrevy_asset::{block::Block, index::ExtractIndexedAssets, tag::Tag};

/// All [`Block`] [`Tag`]s in vanilla Minecraft.
#[derive(Resource, ExtractIndexedAssets)]
#[extract(asset = "Tag<Block>")]
pub struct BlockTags {
    pub wool: Handle<Tag<Block>>,
    pub planks: Handle<Tag<Block>>,
    pub stone_bricks: Handle<Tag<Block>>,
    pub wooden_buttons: Handle<Tag<Block>>,
    pub buttons: Handle<Tag<Block>>,
    pub wool_carpets: Handle<Tag<Block>>,
    pub wooden_doors: Handle<Tag<Block>>,
    pub wooden_stairs: Handle<Tag<Block>>,
    pub wooden_slabs: Handle<Tag<Block>>,
    pub wooden_fences: Handle<Tag<Block>>,
    pub pressure_plates: Handle<Tag<Block>>,
    pub wooden_pressure_plates: Handle<Tag<Block>>,
    pub stone_pressure_plates: Handle<Tag<Block>>,
    pub wooden_trapdoors: Handle<Tag<Block>>,
    pub doors: Handle<Tag<Block>>,
    pub saplings: Handle<Tag<Block>>,
    pub logs_that_burn: Handle<Tag<Block>>,
    pub overworld_natural_logs: Handle<Tag<Block>>,
    pub logs: Handle<Tag<Block>>,
    pub dark_oak_logs: Handle<Tag<Block>>,
    pub oak_logs: Handle<Tag<Block>>,
    pub birch_logs: Handle<Tag<Block>>,
    pub acacia_logs: Handle<Tag<Block>>,
    pub jungle_logs: Handle<Tag<Block>>,
    pub spruce_logs: Handle<Tag<Block>>,
    pub mangrove_logs: Handle<Tag<Block>>,
    pub crimson_stems: Handle<Tag<Block>>,
    pub warped_stems: Handle<Tag<Block>>,
    pub bamboo_blocks: Handle<Tag<Block>>,
    pub wart_blocks: Handle<Tag<Block>>,
    pub banners: Handle<Tag<Block>>,
    pub sand: Handle<Tag<Block>>,
    pub stairs: Handle<Tag<Block>>,
    pub slabs: Handle<Tag<Block>>,
    pub walls: Handle<Tag<Block>>,
    pub anvil: Handle<Tag<Block>>,
    pub rails: Handle<Tag<Block>>,
    pub leaves: Handle<Tag<Block>>,
    pub trapdoors: Handle<Tag<Block>>,
    pub small_flowers: Handle<Tag<Block>>,
    pub beds: Handle<Tag<Block>>,
    pub fences: Handle<Tag<Block>>,
    pub tall_flowers: Handle<Tag<Block>>,
    pub flowers: Handle<Tag<Block>>,
    pub piglin_repellents: Handle<Tag<Block>>,
    pub gold_ores: Handle<Tag<Block>>,
    pub iron_ores: Handle<Tag<Block>>,
    pub diamond_ores: Handle<Tag<Block>>,
    pub redstone_ores: Handle<Tag<Block>>,
    pub lapis_ores: Handle<Tag<Block>>,
    pub coal_ores: Handle<Tag<Block>>,
    pub emerald_ores: Handle<Tag<Block>>,
    pub copper_ores: Handle<Tag<Block>>,
    pub candles: Handle<Tag<Block>>,
    pub dirt: Handle<Tag<Block>>,
    pub terracotta: Handle<Tag<Block>>,
    pub completes_find_tree_tutorial: Handle<Tag<Block>>,
    pub flower_pots: Handle<Tag<Block>>,
    pub enderman_holdable: Handle<Tag<Block>>,
    pub ice: Handle<Tag<Block>>,
    pub valid_spawn: Handle<Tag<Block>>,
    pub impermeable: Handle<Tag<Block>>,
    pub underwater_bonemeals: Handle<Tag<Block>>,
    pub coral_blocks: Handle<Tag<Block>>,
    pub wall_corals: Handle<Tag<Block>>,
    pub coral_plants: Handle<Tag<Block>>,
    pub corals: Handle<Tag<Block>>,
    pub bamboo_plantable_on: Handle<Tag<Block>>,
    pub standing_signs: Handle<Tag<Block>>,
    pub wall_signs: Handle<Tag<Block>>,
    pub signs: Handle<Tag<Block>>,
    pub ceiling_hanging_signs: Handle<Tag<Block>>,
    pub wall_hanging_signs: Handle<Tag<Block>>,
    pub all_hanging_signs: Handle<Tag<Block>>,
    pub all_signs: Handle<Tag<Block>>,
    pub dragon_immune: Handle<Tag<Block>>,
    pub dragon_transparent: Handle<Tag<Block>>,
    pub wither_immune: Handle<Tag<Block>>,
    pub wither_summon_base_blocks: Handle<Tag<Block>>,
    pub beehives: Handle<Tag<Block>>,
    pub crops: Handle<Tag<Block>>,
    pub bee_growables: Handle<Tag<Block>>,
    pub portals: Handle<Tag<Block>>,
    pub fire: Handle<Tag<Block>>,
    pub nylium: Handle<Tag<Block>>,
    pub beacon_base_blocks: Handle<Tag<Block>>,
    pub soul_speed_blocks: Handle<Tag<Block>>,
    pub wall_post_override: Handle<Tag<Block>>,
    pub climbable: Handle<Tag<Block>>,
    pub fall_damage_resetting: Handle<Tag<Block>>,
    pub shulker_boxes: Handle<Tag<Block>>,
    pub hoglin_repellents: Handle<Tag<Block>>,
    pub soul_fire_base_blocks: Handle<Tag<Block>>,
    pub strider_warm_blocks: Handle<Tag<Block>>,
    pub campfires: Handle<Tag<Block>>,
    pub guarded_by_piglins: Handle<Tag<Block>>,
    pub prevent_mob_spawning_inside: Handle<Tag<Block>>,
    pub fence_gates: Handle<Tag<Block>>,
    pub unstable_bottom_center: Handle<Tag<Block>>,
    pub mushroom_grow_block: Handle<Tag<Block>>,
    pub infiniburn_overworld: Handle<Tag<Block>>,
    pub infiniburn_nether: Handle<Tag<Block>>,
    pub infiniburn_end: Handle<Tag<Block>>,
    pub base_stone_overworld: Handle<Tag<Block>>,
    pub stone_ore_replaceables: Handle<Tag<Block>>,
    pub deepslate_ore_replaceables: Handle<Tag<Block>>,
    pub base_stone_nether: Handle<Tag<Block>>,
    pub overworld_carver_replaceables: Handle<Tag<Block>>,
    pub nether_carver_replaceables: Handle<Tag<Block>>,
    pub candle_cakes: Handle<Tag<Block>>,
    pub cauldrons: Handle<Tag<Block>>,
    pub crystal_sound_blocks: Handle<Tag<Block>>,
    pub inside_step_sound_blocks: Handle<Tag<Block>>,
    pub occludes_vibration_signals: Handle<Tag<Block>>,
    pub dampens_vibrations: Handle<Tag<Block>>,
    pub dripstone_replaceable: Handle<Tag<Block>>,
    pub cave_vines: Handle<Tag<Block>>,
    pub moss_replaceable: Handle<Tag<Block>>,
    pub lush_ground_replaceable: Handle<Tag<Block>>,
    pub azalea_root_replaceable: Handle<Tag<Block>>,
    pub small_dripleaf_placeable: Handle<Tag<Block>>,
    pub big_dripleaf_placeable: Handle<Tag<Block>>,
    pub snow: Handle<Tag<Block>>,
    pub mineable_with_axe: Handle<Tag<Block>>,
    pub mineable_with_hoe: Handle<Tag<Block>>,
    pub mineable_with_pickaxe: Handle<Tag<Block>>,
    pub mineable_with_shovel: Handle<Tag<Block>>,
    pub needs_diamond_tool: Handle<Tag<Block>>,
    pub needs_iron_tool: Handle<Tag<Block>>,
    pub needs_stone_tool: Handle<Tag<Block>>,
    pub features_cannot_replace: Handle<Tag<Block>>,
    pub lava_pool_stone_cannot_replace: Handle<Tag<Block>>,
    pub geode_invalid_blocks: Handle<Tag<Block>>,
    pub frog_prefer_jump_to: Handle<Tag<Block>>,
    pub sculk_replaceable: Handle<Tag<Block>>,
    pub sculk_replaceable_world_gen: Handle<Tag<Block>>,
    pub ancient_city_replaceable: Handle<Tag<Block>>,
    pub animals_spawnable_on: Handle<Tag<Block>>,
    pub axolotls_spawnable_on: Handle<Tag<Block>>,
    pub goats_spawnable_on: Handle<Tag<Block>>,
    pub mooshrooms_spawnable_on: Handle<Tag<Block>>,
    pub parrots_spawnable_on: Handle<Tag<Block>>,
    pub polar_bears_spawnable_on_alternate: Handle<Tag<Block>>,
    pub rabbits_spawnable_on: Handle<Tag<Block>>,
    pub foxes_spawnable_on: Handle<Tag<Block>>,
    pub wolves_spawnable_on: Handle<Tag<Block>>,
    pub frogs_spawnable_on: Handle<Tag<Block>>,
    pub azalea_grows_on: Handle<Tag<Block>>,
    pub replaceable_plants: Handle<Tag<Block>>,
    pub convertable_to_mud: Handle<Tag<Block>>,
    pub mangrove_logs_can_grow_through: Handle<Tag<Block>>,
    pub mangrove_roots_can_grow_through: Handle<Tag<Block>>,
    pub dead_bush_may_place_on: Handle<Tag<Block>>,
    pub snaps_goat_horn: Handle<Tag<Block>>,
    pub snow_layer_cannot_survive_on: Handle<Tag<Block>>,
    pub snow_layer_can_survive_on: Handle<Tag<Block>>,
    pub invalid_spawn_inside: Handle<Tag<Block>>,
}