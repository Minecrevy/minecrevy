//! Minecraft protocol packet definitions in the `Config` state.

use std::io;

use bevy::utils::HashMap;
use minecrevy_io::{
    args::{Compression, IntArgs, NbtArgs, OptionArgs, OptionTag, StringArgs},
    McRead, McWrite,
};
use minecrevy_key::SharedKey;
use minecrevy_nbt::io::Nbt;
use minecrevy_text::{Text, TextArgs, TextStyle};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A packet sent by the client to give basic information about itself.
#[derive(Clone, PartialEq, Debug)]
pub struct ClientInformation {
    /// The client's locale.
    pub locale: String,
    /// The client's view distance.
    pub view_distance: i8,
    /// The visibility of chat messages for this client.
    pub chat_mode: i32,
    /// Whether chat colors are enabled for this client.
    pub chat_colors: bool,
    /// Bitmask of displayed skin parts.
    pub displayed_skin_parts: u8,
    /// The main hand of the player.
    pub main_hand: i32,
    /// Whether to enable text filtering for this client for signs and books.
    pub enable_text_filtering: bool,
    /// Hide this client from the server's server list.
    pub allow_server_listings: bool,
}

impl McRead for ClientInformation {
    type Args = ();

    fn read(mut reader: impl io::Read, (): Self::Args) -> io::Result<Self> {
        Ok(Self {
            locale: String::read(&mut reader, StringArgs { max_len: Some(16) })?,
            view_distance: i8::read(&mut reader, ())?,
            chat_mode: i32::read(&mut reader, IntArgs { varint: true })?,
            chat_colors: bool::read(&mut reader, ())?,
            displayed_skin_parts: u8::read(&mut reader, ())?,
            main_hand: i32::read(&mut reader, IntArgs { varint: true })?,
            enable_text_filtering: bool::read(&mut reader, ())?,
            allow_server_listings: bool::read(&mut reader, ())?,
        })
    }
}

/// A packet sent by the server to indicate that the configuration stage is finished.
/// The client responds with the same packet.
///
/// # Event Consumption Notes
///
/// This is a **stateful** packet. Further incoming packets are paused for
/// reading until the client is unpaused.
#[derive(Clone, PartialEq, Debug)]
pub struct Finish;

impl McRead for Finish {
    type Args = ();

    fn read(_reader: impl io::Read, _args: Self::Args) -> io::Result<Self> {
        Ok(Self)
    }
}

impl McWrite for Finish {
    type Args = ();

    fn write(&self, _writer: impl io::Write, _args: Self::Args) -> io::Result<()> {
        Ok(())
    }
}

/// A packet sent by the client to indicate if the resource pack was accepted.
#[derive(Clone, PartialEq, Debug)]
pub struct ResourcePackResponse {
    /// The UUID of the resource pack.
    pub uuid: Uuid,
    /// The result of the resource pack request.
    pub result: i32,
}

impl McRead for ResourcePackResponse {
    type Args = ();

    fn read(mut reader: impl io::Read, (): Self::Args) -> io::Result<Self> {
        Ok(Self {
            uuid: Uuid::read(&mut reader, ())?,
            result: i32::read(&mut reader, IntArgs { varint: true })?,
        })
    }
}

/// A packet sent by the server to customize the client's registries
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct RegistryData {
    /// The registry of trim patterns for armor.
    #[serde(rename = "minecraft:trim_pattern")]
    pub trim_pattern: Registry<TrimPattern>,
    /// The registry of trim materials for armor.
    #[serde(rename = "minecraft:trim_material")]
    pub trim_material: Registry<TrimMaterial>,
    /// The registry of chat types.
    #[serde(rename = "minecraft:chat_type")]
    pub chat_type: Registry<ChatType>,
    /// The registry of dimension types.
    #[serde(rename = "minecraft:dimension_type")]
    pub dimension_type: Registry<DimensionType>,
    /// The registry of damage types.
    #[serde(rename = "minecraft:damage_type")]
    pub damage_type: Registry<DamageType>,
    /// The registry of biomes.
    #[serde(rename = "minecraft:worldgen/biome")]
    pub biome: Registry<Biome>,
}

impl McWrite for RegistryData {
    type Args = ();

    fn write(&self, writer: impl io::Write, (): Self::Args) -> io::Result<()> {
        Nbt(self).write(
            writer,
            NbtArgs {
                header: None,
                compression: Compression::None,
                max_len: None,
            },
        )
    }
}

/// A data-driven registry.
#[derive(Serialize, Deserialize)]
#[derive(Clone, PartialEq, Debug)]
pub struct Registry<T> {
    /// The type of the registry.
    #[serde(rename = "type")]
    pub typ: SharedKey,
    /// The entries of the registry.
    pub value: Vec<RegistryEntry<T>>,
}

/// An entry in a [`Registry`].
#[derive(Serialize, Deserialize)]
#[derive(Clone, PartialEq, Debug)]
pub struct RegistryEntry<T> {
    /// The name of the registry entry.
    pub name: SharedKey,
    /// The protocol id of the registry entry.
    pub id: i32,
    /// The data of the registry entry.
    pub element: T,
}

/// A trim pattern for armor.
#[derive(Serialize, Deserialize)]
#[derive(Clone, PartialEq, Debug)]
pub struct TrimPattern {
    /// The trim pattern model to be rendered on top of the armor.
    pub asset_id: String,
    /// The template item to be used on the smithing table.
    pub template_item: String,
    /// The name of the trim pattern to be displayed on the armor tooltip.
    pub description: Text,
    /// Whether the trim pattern is a decal.
    pub decal: bool,
}

/// A trim material for armor.
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct TrimMaterial {
    /// The trim color model to be rendered on top of the armor.
    pub asset_name: SharedKey,
    /// The ingredient item to be used on the smithing table.
    pub ingredient: SharedKey,
    /// The color index of the trim material.
    pub item_model_index: f32,
    /// The override armor material to be used when crafting the armor.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub override_armor_materials: HashMap<SharedKey, String>,
    /// The name of the trim material to be displayed on the armor tooltip.
    pub description: Text,
}

/// Defines different types of in-game chat and how they are formatted.
#[derive(Serialize, Deserialize)]
#[derive(Clone, PartialEq, Debug)]
pub struct ChatType {
    /// The chat decoration.
    pub chat: ChatDecoration,
    /// The narration decoration.
    pub narration: ChatDecoration,
}

/// A chat decoration.
#[derive(Serialize, Deserialize)]
#[derive(Clone, PartialEq, Debug)]
pub struct ChatDecoration {
    /// The translation key representing the chat format.
    pub translation_key: String,
    /// The style of the chat.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub style: Option<TextStyle>,
    /// Placeholders used when formatting the string give by the translation key.
    pub parameters: Vec<ChatPlaceholder>,
}

/// A placeholder for a chat message.
#[derive(Serialize, Deserialize)]
#[derive(Clone, PartialEq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ChatPlaceholder {
    /// The name of the player who sent the message.
    Sender,
    /// The name of the player who received the message.
    Target,
    /// The content of the message.
    Content,
}

/// A dimension type.
#[derive(Serialize, Deserialize)]
#[derive(Clone, PartialEq, Debug)]
pub struct DimensionType {
    /// The time of day in ticks, if fixed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fixed_time: Option<i64>,
    /// Whether the dimension has skylight access or not.
    pub has_skylight: bool,
    /// Whether the dimension hasa bedrock ceiling or not.
    /// When `true`, lava spreads faster.
    pub has_ceiling: bool,
    /// Whether water evaporates and sponges dry or not.
    /// Also causes lava to spread thinner.
    pub ultrawarm: bool,
    /// When `true`, nether portals can spawn zombified piglins.
    /// When `false`, compasses spin randomly.
    pub natural: bool,
    /// The multiplier applied to coordinates when traveling to the dimension.
    pub coordinate_scale: f64,
    /// Whether players can use a bed to sleep.
    pub bed_works: bool,
    /// Whether players can charge and use respawn anchors.
    pub respawn_anchor_works: bool,
    /// The minimum y coordinate.
    pub min_y: i32,
    /// The maximum build height.
    pub height: i32,
    /// The maximum height where chorus fruit and nether portals will bring players.
    pub logical_height: i32,
    /// The block tag to use for infiniburn.
    pub infiniburn: String,
    /// Changes cloud level, sky type, forced light map, and constant ambient light.
    pub effects: SharedKey,
    /// How much light the dimension has.
    pub ambient_light: f32,
    /// Whether piglins shake and transform into zombified piglins.
    pub piglin_safe: bool,
    /// Whether players with the Bad Omen effect trigger raids.
    pub has_raids: bool,
    /// The maximum allowed light level for mob spawning.
    pub monster_spawn_light_level: LightLevel,
    /// The maximum allowed block light level for mob spawning.
    pub monster_spawn_block_light_limit: i32,
}

/// A light level.
#[derive(Serialize, Deserialize)]
#[derive(Clone, PartialEq, Debug)]
#[serde(untagged)]
pub enum LightLevel {
    /// A constant light level.
    Constant(i32),
    /// A light level that varies.
    Distribution {
        /// The type of distribution.
        #[serde(rename = "type")]
        typ: SharedKey,
        /// The distribution parameters.
        value: DistributionParams,
    },
}

/// The parameters for an integer distribution.
#[derive(Serialize, Deserialize)]
#[derive(Clone, PartialEq, Debug)]
pub struct DistributionParams {
    /// The minimum value of the distribution.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_inclusive: Option<i32>,
    /// The maximum value of the distribution.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_inclusive: Option<i32>,
}

/// A damage type.
#[derive(Serialize, Deserialize)]
#[derive(Clone, PartialEq, Debug)]
pub struct DamageType {
    /// The id of the death message to display.
    pub message_id: String,
    /// How the damage type scales with difficulty.
    pub scaling: DamageScalingType,
    /// The exhaustion level to apply to the player when damaged by this damage type.
    pub exhaustion: f32,
    /// The effect of the damage type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effects: Option<DamageTypeEffect>,
    /// The type of death message to display.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub death_message_type: Option<DeathMessageType>,
}

/// Whether the damage type scales with difficulty.
#[derive(Serialize, Deserialize)]
#[derive(Clone, PartialEq, Debug, Default)]
#[serde(rename_all = "snake_case")]
pub enum DamageScalingType {
    /// Damage does not scale with difficulty.
    #[default]
    Never,
    /// Damage scales with difficulty when caused by a living non-player.
    WhenCausedByLivingNonPlayer,
    /// Damage scales with difficulty.
    Always,
}

/// The effect of a damage type.
#[derive(Serialize, Deserialize)]
#[derive(Clone, PartialEq, Debug, Default)]
#[serde(rename_all = "snake_case")]
pub enum DamageTypeEffect {
    /// The default effect.
    #[default]
    Hurt,
    /// The effect for damaged by thorns enchantment.
    Thorns,
    /// The effect for damaged by drowning.
    Drowning,
    /// The effect for damaged by fire or lava.
    Burning,
    /// The effect for damaged by cactus.
    Poking,
    /// The effect for damaged by powdered snow.
    Freezing,
}

/// The type of death message to display.
#[derive(Serialize, Deserialize)]
#[derive(Clone, PartialEq, Debug, Default)]
#[serde(rename_all = "snake_case")]
pub enum DeathMessageType {
    /// The default message type.
    #[default]
    Default,
    /// The message type for death by falling.
    FallVariants,
    /// The message type for death by magic.
    IntentionalGameDesign,
}

/// A biome.
#[derive(Serialize, Deserialize)]
#[derive(Clone, PartialEq, Debug)]
pub struct Biome {
    /// Whether the biome has precipitation.
    pub has_precipitation: bool,
    /// The temperature factor of the biome.
    pub temperature: f32,
    /// Modifies the temperature of the biome.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temperature_modifier: Option<BiomeTempModifier>,
    /// The downfall factor of the biome.
    pub downfall: f32,
    /// Biome special effects.
    pub effects: BiomeEffects,
}

/// Modifies the temperature of a biome.
#[derive(Serialize, Deserialize)]
#[derive(Clone, PartialEq, Debug, Default)]
#[serde(rename_all = "snake_case")]
pub enum BiomeTempModifier {
    /// Static temperature.
    #[default]
    None,
    /// Pockets of warmer temperature.
    Frozen,
}

/// Biome special effects.
#[derive(Serialize, Deserialize)]
#[derive(Clone, PartialEq, Debug)]
pub struct BiomeEffects {
    /// The fog color of the biome.
    pub fog_color: i32,
    /// The color of the water.
    pub water_color: i32,
    /// The color of the water fog.
    pub water_fog_color: i32,
    /// The color of the sky.
    pub sky_color: i32,
    /// The color of the foliage.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub foliage_color: Option<i32>,
    /// The color of the grass.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grass_color: Option<i32>,
    /// The color modifier for the grass.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grass_color_modifier: Option<GrassColorModifier>,
    /// The particle effects to play in the biome.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub particle: Option<BiomeParticle>,
    /// The ambient sound effects to play in the biome.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ambient_sound: Option<SharedKey>,
    /// The mood sound effects to play in the biome.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mood_sound: Option<BiomeMoodSound>,
    /// The addition sound effects to play in the biome.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additions_sound: Option<BiomeAdditionsSound>,
    /// The music to play in the biome.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub music: Option<BiomeMusic>,
}

/// The color modifier for the grass.
#[derive(Serialize, Deserialize)]
#[derive(Clone, PartialEq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum GrassColorModifier {
    /// Static grass color.
    None,
    /// Dark forest grass color.
    DarkForest,
    /// Swamp grass color.
    Swamp,
}

/// Biome particle effects.
#[derive(Serialize, Deserialize)]
#[derive(Clone, PartialEq, Debug)]
pub struct BiomeParticle {
    /// The probability of spawning the particle per tick.
    pub probability: f32,
    /// The particle effect options.
    pub options: BiomeParticleOption,
}

/// Biome particle effect options.
#[derive(Serialize, Deserialize)]
#[derive(Clone, PartialEq, Debug)]
pub struct BiomeParticleOption {
    /// The id of the particle to spawn.
    #[serde(rename = "type")]
    pub typ: SharedKey,
}

/// Biome mood sound effects.
#[derive(Serialize, Deserialize)]
#[derive(Clone, PartialEq, Debug)]
pub struct BiomeMoodSound {
    /// The id of the sound to play.
    pub sound: SharedKey,
    /// The minimum tick delay between playing the sound.
    pub tick_delay: i32,
    /// The radius used for the block search to calculate moodiness.
    pub block_search_extent: i32,
    /// The distance offset from the player to play the sound..
    pub offset: f64,
}

/// Biome additional sound effects.
#[derive(Serialize, Deserialize)]
#[derive(Clone, PartialEq, Debug)]
pub struct BiomeAdditionsSound {
    /// The id of the sound to play.
    pub sound: SharedKey,
    /// The chance of playing the sound per tick.
    pub tick_chance: f64,
}

/// Biome music effects.
#[derive(Serialize, Deserialize)]
#[derive(Clone, PartialEq, Debug)]
pub struct BiomeMusic {
    /// The id of the sound to play.
    pub sound: SharedKey,
    /// The minimum tick delay between playing the sound.
    pub min_delay: i32,
    /// The maximum tick delay between playing the sound.
    pub max_delay: i32,
    /// Whether the music is a replaceable music disc.
    pub replace_current_music: bool,
}

/// A packet sent by the server to request the client remove one or all resource packs.
#[derive(Clone, PartialEq, Debug)]
pub struct RemoveResourcePack {
    /// The UUID of the resource pack to remove, or `None` to remove all resource packs.
    pub uuid: Option<Uuid>,
}

impl McWrite for RemoveResourcePack {
    type Args = ();

    fn write(&self, writer: impl io::Write, (): Self::Args) -> io::Result<()> {
        self.uuid.write(
            writer,
            OptionArgs {
                tag: OptionTag::Bool,
                inner: (),
            },
        )
    }
}

/// A packet sent by the server to request the client add a resource pack.
#[derive(Clone, PartialEq, Debug)]
pub struct AddResourcePack {
    /// The UUID of the resource pack.
    pub uuid: Uuid,
    /// The URL of the resource pack.
    pub url: String,
    /// The hash of the resource pack.
    pub hash: String,
    /// Whether the resource pack is required. If a forced resource pack is not
    /// accepted, the client will be disconnected.
    pub forced: bool,
    /// The message to display to the client.
    pub prompt: Option<Text>,
}

impl McWrite for AddResourcePack {
    type Args = ();

    fn write(&self, mut writer: impl io::Write, (): Self::Args) -> io::Result<()> {
        self.uuid.write(&mut writer, ())?;
        self.url.write(
            &mut writer,
            StringArgs {
                max_len: Some(32767),
            },
        )?;
        self.hash
            .write(&mut writer, StringArgs { max_len: Some(40) })?;
        self.forced.write(&mut writer, ())?;
        self.prompt.write(
            &mut writer,
            OptionArgs {
                tag: OptionTag::Bool,
                inner: TextArgs::default(),
            },
        )?;
        Ok(())
    }
}
