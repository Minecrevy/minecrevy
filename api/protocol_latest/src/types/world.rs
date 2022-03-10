use serde::{Deserialize, Serialize};
use minecrevy_key::Key;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct DimensionCodec {
    #[serde(rename = "minecraft:dimension_type")]
    dimension_type_registry: DimensionTypeRegistry,
    #[serde(rename = "minecraft:worldgen/biome")]
    biome_registry: BiomeRegistry,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct DimensionTypeRegistry {
    #[serde(rename = "type")]
    pub ty: Key,
    pub value: Vec<DimensionTypeEntry>,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct DimensionTypeEntry {
    pub name: Key,
    pub id: i32,
    pub element: DimensionType,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct DimensionType {
    pub piglin_safe: i8,
    pub natural: i8,
    pub ambient_light: f32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fixed_time: Option<i64>,
    pub infiniburn: String,
    pub respawn_anchor_works: i8,
    pub has_skylight: i8,
    pub bed_works: i8,
    pub effects: Key,
    pub has_raids: i8,
    pub min_y: i32,
    pub height: i32,
    pub logical_height: i32,
    pub coordinate_scale: f32,
    pub ultrawarm: i8,
    pub has_ceiling: i8,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct BiomeRegistry {
    #[serde(rename = "type")]
    pub ty: Key,
    pub value: Vec<BiomeEntry>,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct BiomeEntry {
    pub name: Key,
    pub id: i32,
    pub element: BiomeProperties,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct BiomeProperties {
    pub precipitation: String,
    pub depth: f32,
    pub temperature: f32,
    pub scale: f32,
    pub downfall: f32,
    pub category: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temperature_modifier: Option<String>,
    pub effects: BiomeEffects,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub particle: Option<BiomeParticle>,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct BiomeEffects {
    pub sky_color: i32,
    pub water_fog_color: i32,
    pub fog_color: i32,
    pub water_color: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub foliage_color: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grass_color: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grass_color_modifier: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub music: Option<nbt::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ambient_sound: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additions_sound: Option<nbt::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mood_sound: Option<nbt::Value>,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct BiomeParticle {
    pub probability: f32,
    pub options: nbt::Value,
}
