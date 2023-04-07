use bevy::asset::Handle;
use bevy::ecs::system::Resource;
use minecrevy_asset::{
    index::ExtractIndexedAssets,
    mob::villager::{VillagerProfession, VillagerType},
};

/// All [`VillagerType`]s in vanilla Minecraft.
#[derive(Resource, ExtractIndexedAssets)]
#[extract(asset = "VillagerType")]
pub struct VillagerTypes {
    /// Villagers that live in the desert.
    pub desert: Handle<VillagerType>,
    /// Villagers that live in the jungle.
    pub jungle: Handle<VillagerType>,
    /// Villagers that live in the plains.
    pub plains: Handle<VillagerType>,
    /// Villagers that live in the savanna.
    pub savanna: Handle<VillagerType>,
    /// Villagers that live in snowy places.
    pub snow: Handle<VillagerType>,
    /// Villagers that live in the swamps.
    pub swamp: Handle<VillagerType>,
    /// Villagers that live in taiga biomes.
    pub taiga: Handle<VillagerType>,
}

/// All [`VillagerProfession`]s in vanilla Minecraft.
#[derive(Resource, ExtractIndexedAssets)]
#[extract(asset = "VillagerProfession")]
pub struct VillagerProfessions {
    /// Villagers without any profession.
    pub none: Handle<VillagerProfession>,
    /// Armorer villagers.
    pub armorer: Handle<VillagerProfession>,
    /// Butcher villagers.
    pub butcher: Handle<VillagerProfession>,
    /// Cartographer villagers.
    pub cartographer: Handle<VillagerProfession>,
    /// Cleric villagers.
    pub cleric: Handle<VillagerProfession>,
    /// Farmer villagers.
    pub farmer: Handle<VillagerProfession>,
    /// Fisherman villagers.
    pub fisherman: Handle<VillagerProfession>,
    /// Fletcher villagers.
    pub fletcher: Handle<VillagerProfession>,
    /// Leatherworker villagers.
    pub leatherworker: Handle<VillagerProfession>,
    /// Librarian villagers.
    pub librarian: Handle<VillagerProfession>,
    /// Mason villagers.
    pub mason: Handle<VillagerProfession>,
    /// Nitwits.
    pub nitwit: Handle<VillagerProfession>,
    /// Shepherd villagers.
    pub shepherd: Handle<VillagerProfession>,
    /// Toolsmith villagers.
    pub toolsmith: Handle<VillagerProfession>,
    /// Weaponsmith villagers.
    pub weaponsmith: Handle<VillagerProfession>,
}
