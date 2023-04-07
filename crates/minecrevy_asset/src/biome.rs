use bevy::{prelude::Handle, reflect::TypeUuid};
use derive_more::AsRef;
use minecrevy_core::{color::Rgb, key::Key};

use crate::sound::Sound;

#[derive(TypeUuid, AsRef)]
#[uuid = "84b90c29-3af5-452d-bbcd-40b3900bca4b"]
pub struct BiomeCategory {
    #[as_ref]
    pub key: Key,
}

#[derive(TypeUuid, AsRef)]
#[uuid = "81a1a62f-dded-4a3b-8f9c-c08753646c94"]
pub struct Biome {
    #[as_ref]
    pub key: Key,
    pub climate: BiomeClimateSettings,
    pub depth: f32,
    pub scale: f32,
    pub category: Handle<BiomeCategory>,
}

pub struct BiomeClimateSettings {
    pub precipitation: BiomePrecipitation,
    pub temperature: f32,
    pub downfall: f32,
    pub modifier: BiomeTemperatureModifier,
}

pub enum BiomePrecipitation {
    None,
    Rain,
    Snow,
}

pub enum BiomeTemperatureModifier {
    None,
    Frozen,
}

pub struct BiomeSpecialEffects {
    pub sky_color: Rgb,
    pub water_fog_color: Rgb,
    pub fog_color: Rgb,
    pub water_color: Rgb,
    pub foliage_color_override: Option<Rgb>,
    pub grass_color_override: Option<Rgb>,
    pub grass_color_modifier: BiomeGrassColorModifier,
    pub music: BiomeMusic,
    pub ambient_sound: Option<Handle<Sound>>,
    pub ambient_addition: Option<BiomeAmbientAdditionSettings>,
    pub ambient_mood: Option<BiomeAmbientMoodSettings>,
}

pub enum BiomeGrassColorModifier {
    None,
    DarkForest,
    Swamp,
}

pub struct BiomeMusic {
    pub sound: Handle<Sound>,
    pub min_delay: i32,
    pub max_delay: i32,
    pub replace_current_music: bool,
}

pub struct BiomeAmbientAdditionSettings {
    pub sound: Handle<Sound>,
    pub tick_chance: f64,
}

pub struct BiomeAmbientMoodSettings {
    pub sound: Handle<Sound>,
    pub tick_delay: i32,
    pub block_search_extent: i32,
    pub sound_position_offset: f64,
}
