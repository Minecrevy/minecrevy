use bevy::{prelude::Handle, reflect::TypeUuid};
use derive_more::AsRef;
use minecrevy_core::key::Key;

#[derive(TypeUuid, AsRef)]
#[uuid = "cb645144-0ae0-47b3-a8a0-c3a17bf36a2c"]
pub struct DimensionType {
    #[as_ref]
    pub key: Key,
    pub fixed_time: Option<i64>,
    pub has_skylight: bool,
    pub has_ceiling: bool,
    pub ultra_warm: bool,
    pub natural: bool,
    pub coordinate_scale: f64,
    pub create_dragon_flight: bool,
    pub piglin_safe: bool,
    pub bed_works: bool,
    pub respawn_anchor_works: bool,
    pub has_raids: bool,
    pub logical_height: i32,
    pub biome_zoomer: Handle<BiomeZoomer>,
    pub infiniburn: Key,       // TODO: handle?
    pub effects_location: Key, // TODO: handle?
    pub ambient_light: f32,
    pub brightness_ramp: [f32; 16],
}

#[derive(TypeUuid, AsRef)]
#[uuid = "0fb43292-5c90-46c2-a67d-81adbf417b18"]
pub struct BiomeZoomer {
    #[as_ref]
    pub key: Key,
}
