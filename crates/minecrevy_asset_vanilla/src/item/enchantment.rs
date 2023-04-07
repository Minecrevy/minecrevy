use bevy::asset::Handle;
use bevy::ecs::system::Resource;
use minecrevy_asset::{
    index::ExtractIndexedAssets,
    item::{Enchantment, EnchantmentCategory},
};

/// All [`Enchantment`]s in vanilla Minecraft.
#[derive(Resource, ExtractIndexedAssets)]
#[extract(asset = "Enchantment")]
pub struct Enchantments {
    /// Protection
    pub protection: Handle<Enchantment>,
    /// Fire Protection
    pub fire_protection: Handle<Enchantment>,
    /// Feather Falling
    pub feather_falling: Handle<Enchantment>,
    /// Blast Protection
    pub blast_protection: Handle<Enchantment>,
    /// Projectile Protection
    pub projectile_protection: Handle<Enchantment>,
    /// Respiration
    pub respiration: Handle<Enchantment>,
    /// Aqua Affinity
    pub aqua_affinity: Handle<Enchantment>,
    /// Thorns
    pub thorns: Handle<Enchantment>,
    /// Depth Strider
    pub depth_strider: Handle<Enchantment>,
    /// Frost Walker
    pub frost_walker: Handle<Enchantment>,
    /// Curse of Binding
    pub binding_curse: Handle<Enchantment>,
    /// Soul Speed
    pub soul_speed: Handle<Enchantment>,
    /// Sharpness
    pub sharpness: Handle<Enchantment>,
    /// Smite
    pub smite: Handle<Enchantment>,
    /// Bane of Arthropods
    pub bane_of_arthropods: Handle<Enchantment>,
    /// Knockback
    pub knockback: Handle<Enchantment>,
    /// Fire Aspect
    pub fire_aspect: Handle<Enchantment>,
    /// Looting
    pub looting: Handle<Enchantment>,
    /// Sweeping Edge
    pub sweeping: Handle<Enchantment>,
    /// Efficiency
    pub efficiency: Handle<Enchantment>,
    /// Silk Touch
    pub silk_touch: Handle<Enchantment>,
    /// Unbreaking
    pub unbreaking: Handle<Enchantment>,
    /// Fortune
    pub fortune: Handle<Enchantment>,
    /// Power
    pub power: Handle<Enchantment>,
    /// Punch
    pub punch: Handle<Enchantment>,
    /// Flame
    pub flame: Handle<Enchantment>,
    /// Infinity
    pub infinity: Handle<Enchantment>,
    /// Luck of the Sea
    pub luck_of_the_sea: Handle<Enchantment>,
    /// Lure
    pub lure: Handle<Enchantment>,
    /// Loyalty
    pub loyalty: Handle<Enchantment>,
    /// Impaling
    pub impaling: Handle<Enchantment>,
    /// Riptide
    pub riptide: Handle<Enchantment>,
    /// Channeling
    pub channeling: Handle<Enchantment>,
    /// Multishot
    pub multishot: Handle<Enchantment>,
    /// Quick Charge
    pub quick_charge: Handle<Enchantment>,
    /// Piercing
    pub piercing: Handle<Enchantment>,
    /// Mending
    pub mending: Handle<Enchantment>,
    /// Curse of Vanishing
    pub vanishing_curse: Handle<Enchantment>,
}

/// All [`EnchantmentCategory`]s in vanilla Minecraft.
#[derive(Resource, ExtractIndexedAssets)]
#[extract(asset = "EnchantmentCategory")]
pub struct EnchantmentCategories {
    /// Enchantments that apply to any kind of armor piece.
    pub armor: Handle<EnchantmentCategory>,
    /// Enchantments that apply to boots.
    pub armor_feet: Handle<EnchantmentCategory>,
    /// Enchantments that apply to leggings.
    pub armor_legs: Handle<EnchantmentCategory>,
    /// Enchantments that apply to chestplates.
    pub armor_chest: Handle<EnchantmentCategory>,
    /// Enchantments that apply to helmets.
    pub armor_head: Handle<EnchantmentCategory>,
    /// Enchantments that apply to weapons.
    pub weapon: Handle<EnchantmentCategory>,
    /// Enchantments that apply to pickaxes, axes, and shovels.
    /// That is, anything used to break blocks.
    pub digger: Handle<EnchantmentCategory>,
    /// Enchantments that apply to fishing rods.
    pub fishing_rod: Handle<EnchantmentCategory>,
    /// Enchantments that only apply to tridents.
    pub trident: Handle<EnchantmentCategory>,
    /// Enchantments that can apply to anything that can be damaged.
    pub breakable: Handle<EnchantmentCategory>,
    /// Enchantments that only apply to bows.
    pub bow: Handle<EnchantmentCategory>,
    /// Enchantments that can apply to anything that can be weared.
    pub wearable: Handle<EnchantmentCategory>,
    /// Enchantments that only apply to crossbows.
    pub crossbow: Handle<EnchantmentCategory>,
    /// Enchantments that can apply to anything that can disappear.
    pub vanishable: Handle<EnchantmentCategory>,
}
