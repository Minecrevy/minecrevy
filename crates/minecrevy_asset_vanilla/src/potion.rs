use bevy::prelude::{Handle, Resource};
use minecrevy_asset::{index::ExtractIndexedAssets, potion::Potion};

#[derive(Resource, ExtractIndexedAssets)]
#[extract(asset = "Potion")]
pub struct Potions {
    pub empty: Handle<Potion>,
    pub water: Handle<Potion>,
    pub mundane: Handle<Potion>,
    pub thick: Handle<Potion>,
    pub awkward: Handle<Potion>,
    pub night_vision: Handle<Potion>,
    pub long_night_vision: Handle<Potion>,
    pub invisibility: Handle<Potion>,
    pub long_invisibility: Handle<Potion>,
    pub leaping: Handle<Potion>,
    pub long_leaping: Handle<Potion>,
    pub strong_leaping: Handle<Potion>,
    pub fire_resistance: Handle<Potion>,
    pub long_fire_resistance: Handle<Potion>,
    pub swiftness: Handle<Potion>,
    pub long_swiftness: Handle<Potion>,
    pub strong_swiftness: Handle<Potion>,
    pub slowness: Handle<Potion>,
    pub long_slowness: Handle<Potion>,
    pub strong_slowness: Handle<Potion>,
    pub turtle_master: Handle<Potion>,
    pub long_turtle_master: Handle<Potion>,
    pub strong_turtle_master: Handle<Potion>,
    pub water_breathing: Handle<Potion>,
    pub long_water_breathing: Handle<Potion>,
    pub healing: Handle<Potion>,
    pub strong_healing: Handle<Potion>,
    pub harming: Handle<Potion>,
    pub strong_harming: Handle<Potion>,
    pub poison: Handle<Potion>,
    pub long_poison: Handle<Potion>,
    pub strong_poison: Handle<Potion>,
    pub regeneration: Handle<Potion>,
    pub long_regeneration: Handle<Potion>,
    pub strong_regeneration: Handle<Potion>,
    pub strength: Handle<Potion>,
    pub long_strength: Handle<Potion>,
    pub strong_strength: Handle<Potion>,
    pub weakness: Handle<Potion>,
    pub long_weakness: Handle<Potion>,
    pub luck: Handle<Potion>,
    pub slow_falling: Handle<Potion>,
    pub long_slow_falling: Handle<Potion>,
}