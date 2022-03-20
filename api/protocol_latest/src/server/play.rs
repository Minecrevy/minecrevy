use std::collections::HashMap;

use enumflags2::BitFlags;
use glam::{DVec3, IVec3};
use uuid::Uuid;

use minecrevy_io_str::{McRead, McWrite, Nbt};
use minecrevy_key::Key;
use minecrevy_text::Text;

use crate::types::*;

/// Spawns a vehicle or non-living entity.
#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct SpawnEntity {
    /// The entity's network ID. Usually created per-entity from an atomic counter.
    ///
    /// Network IDs are not persisted through server restarts, nor guaranteed unique through restarts.
    #[options(varint = true)]
    pub net_id: i32,
    /// The entity's [`unique ID`][`Uuid`].
    ///
    /// These are statistically unique through server restarts.
    pub uuid: Uuid,
    /// The entity's type as a numerical ID.
    #[options(varint = true)]
    pub ty: i32,
    /// The entity's position.
    pub position: DVec3,
    /// The entity's pitch as an [`Angle`].
    pub pitch: Angle,
    /// The entity's yaw as an [`Angle`].
    pub yaw: Angle,
    /// Extra data associated with the entity.
    ///
    /// See https://wiki.vg/Object_Data for more.
    pub data: i32,
    /// The entity's velocity.
    pub velocity: [i16; 3],
}

impl crate::Packet for SpawnEntity {}

/// Spawns one or more experience orbs.
#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct SpawnExpOrb {
    /// The experience orb's network ID.
    #[options(varint = true)]
    pub net_id: i32,
    /// The experience orb's position.
    pub position: DVec3,
    /// How much experience the orb will reward when collected.
    pub count: i16,
}

impl crate::Packet for SpawnExpOrb {}

/// Spawns a living entity.
#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct SpawnLivingEntity {
    /// The entity's network ID.
    #[options(varint = true)]
    pub net_id: i32,
    /// The entity's [`unique id`][`Uuid`].
    pub uuid: Uuid,
    /// The entity's type as a numerical ID.
    #[options(varint = true)]
    pub ty: i32,
    /// The entity's position.
    pub position: DVec3,
    /// The entity's yaw as an [`Angle`].
    pub yaw: Angle,
    /// The entity's pitch as an [`Angle`].
    pub pitch: Angle,
    /// The entity's head yaw as an [`Angle`].
    pub head_yaw: Angle,
    /// The entity's velocity.
    pub velocity: [i16; 3],
}

impl crate::Packet for SpawnLivingEntity {}

/// Spawns a painting.
#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct SpawnPainting {
    /// The painting's network ID.
    #[options(varint = true)]
    pub net_id: i32,
    /// The painting's [`unique id`][`Uuid`].
    pub uuid: Uuid,
    /// The painting's type as a numerical ID.
    #[options(varint = true)]
    pub painting_id: i32,
    /// The painting's position from the painting's center.
    #[options(compressed = true)]
    pub position: IVec3,
    /// The painting's faced direction.
    pub direction: CardinalDirection,
}

impl crate::Packet for SpawnPainting {}

/// Spawns a player.
#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct SpawnPlayer {
    /// The player's network ID.
    #[options(varint = true)]
    pub net_id: i32,
    /// The player's [`unique ID`][`Uuid`].
    pub uuid: Uuid,
    /// The player's position.
    pub position: DVec3,
    /// The player's yaw as an [`Angle`].
    pub yaw: Angle,
    /// The player's pitch as an [`Angle`].
    pub pitch: Angle,
}

impl crate::Packet for SpawnPlayer {}

/// Spawns a sculk vibration signal.
#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct SpawnVibrationSignal {
    /// The vibration's source position.
    #[options(compressed = true)]
    pub position: IVec3,
    /// The vibration's destination.
    pub destination: SignalDestination,
    /// How long until the vibration arrives at the destination.
    #[options(varint = true)]
    pub arrival_ticks: i32,
}

impl crate::Packet for SpawnVibrationSignal {}

/// Animates an entity.
#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct EntityAnimation {
    /// The entity's network ID.
    #[options(varint = true)]
    pub entity_id: i32,
    pub animation: Animation,
}

impl crate::Packet for EntityAnimation {}

/// Sets the player's statistics.
#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct StatisticsUpdate {
    pub statistics: Vec<Statistic>,
}

impl crate::Packet for StatisticsUpdate {}

/// Acknowledges that a block break occurred.
#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct AckBlockBreak {
    #[options(compressed = true)]
    pub position: IVec3,
    #[options(varint = true)]
    pub block: i32,
    #[options(varint = true)]
    pub status: i32,
    pub successful: bool,
}

impl crate::Packet for AckBlockBreak {}

/// Animates a block being broken.
#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct BlockBreakAnimation {
    #[options(varint = true)]
    pub entity_id: i32,
    #[options(compressed = true)]
    pub position: IVec3,
    /// 0..=9
    pub stage: u8,
}

impl crate::Packet for BlockBreakAnimation {}

/// Spawns a block entity.
#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct SpawnBlockEntity {
    #[options(compressed = true)]
    pub position: IVec3,
    #[options(varint = true)]
    pub ty: i32,
    pub data: nbt::Value,
}

impl crate::Packet for SpawnBlockEntity {}

/// Performs a block action.
#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct BlockAction {
    #[options(compressed = true)]
    pub position: IVec3,
    pub action_id: u8,
    pub action_param: u8,
    #[options(varint = true)]
    pub ty: i32,
}

impl crate::Packet for BlockAction {}

/// Sets a block to a new state.
#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct BlockUpdate {
    #[options(compressed = true)]
    pub position: IVec3,
    #[options(varint = true)]
    pub block_id: i32,
}

impl crate::Packet for BlockUpdate {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
#[io_repr(varint)]
pub enum BossBarUpdate {
    Add(AddBossBar),
    Remove(RemoveBossBar),
    UpdateHealth(UpdateHealthBossBar),
    UpdateTitle(UpdateTitleBossBar),
    UpdateStyle(UpdateStyleBossBar),
    UpdateFlags(UpdateFlagsBossBar),
}

impl crate::Packet for BossBarUpdate {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct AddBossBar {
    pub title: Text,
    pub health: f32,
    pub color: BossBarColor,
    pub style: BossBarStyle,
    pub flags: u8,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct RemoveBossBar;

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct UpdateHealthBossBar(pub f32);

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct UpdateTitleBossBar(pub Text);

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct UpdateStyleBossBar(pub BossBarColor, pub BossBarStyle);

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct UpdateFlagsBossBar(pub u8);

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct DifficultyUpdate {
    /// The difficulty that should appear in the client's option menu.
    pub difficulty: Difficulty,
    /// True if the difficulty should be unchangeable.
    pub locked: bool,
}

impl crate::Packet for DifficultyUpdate {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct ChatMessage {
    pub message: Text,
    pub kind: MessageKind,
    pub sender: Uuid,
}

impl crate::Packet for ChatMessage {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct ClearTitles {
    pub reset: bool,
}

impl crate::Packet for ClearTitles {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct TabCompletion {
    #[options(varint = true)]
    pub id: i32,
    #[options(varint = true)]
    pub start: i32,
    #[options(varint = true)]
    pub len: i32,
    pub matches: Vec<TabCompletionMatch>,
}

impl crate::Packet for TabCompletion {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct DeclareCommands {
    pub nodes: Vec<CommandNode>,
    // #[options(varint = true)]
    pub root_idx: i32,
}

impl crate::Packet for DeclareCommands {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct CloseWindow {
    pub window_id: u8,
}

impl crate::Packet for CloseWindow {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct WindowSlotsUpdate {
    pub window_id: u8,
    #[options(varint = true)]
    pub state_id: i32,
    pub slots: Vec<Slot>,
    pub held: Slot,
}

impl crate::Packet for WindowSlotsUpdate {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct WindowPropertyUpdate {
    pub window_id: u8,
    pub property: i16,
    pub value: i16,
}

impl crate::Packet for WindowPropertyUpdate {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct WindowSlotUpdate {
    pub window_id: u8,
    #[options(varint = true)]
    pub state_id: i32,
    pub slot_idx: i16,
    pub slot: Slot,
}

impl crate::Packet for WindowSlotUpdate {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct ItemCooldown {
    #[options(varint = true)]
    pub item_id: i32,
    #[options(varint = true)]
    pub cooldown_ticks: i32,
}

impl crate::Packet for ItemCooldown {}

pub type PluginMessage = crate::client::PluginMessage;

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct NamedSoundEffect {
    pub name: Key,
    #[options(varint = true)]
    pub category: i32,
    pub position: IVec3,
    pub volume: f32,
    pub pitch: f32,
}

impl crate::Packet for NamedSoundEffect {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct EntityStatus {
    pub entity_id: i32,
    pub status: u8,
}

impl crate::Packet for EntityStatus {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct Explosion {
    pub position: Vec3,
    pub strength: f32,
    pub offsets: Vec<BlockOffset>,
    pub push_velocity: Vec3,
}

impl crate::Packet for Explosion {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct ChunkUnload {
    pub chunk_x: i32,
    pub chunk_y: i32,
}

impl crate::Packet for ChunkUnload {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct GameStateUpdate {
    pub reason: u8,
    pub value: f32,
}

impl crate::Packet for GameStateUpdate {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct OpenHorseWindow {
    pub window_id: u8,
    #[options(varint = true)]
    pub num_slots: i32,
    pub entity_id: i32,
}

impl crate::Packet for OpenHorseWindow {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct CreateWorldBorder {
    pub x: f64,
    pub z: f64,
    pub old_diameter: f64,
    pub new_diameter: f64,
    #[options(varint = true)]
    pub speed: i64,
    #[options(varint = true)]
    pub portal_tp_bound: i32,
    #[options(varint = true)]
    pub warning_blocks: i32,
    #[options(varint = true)]
    pub warning_time: i32,
}

impl crate::Packet for CreateWorldBorder {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct KeepAlive(pub i64);

impl crate::Packet for KeepAlive {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct ChunkDataAndLightUpdate {
    pub chunk_coords: IVec2,
    pub chunk: ChunkData,
    pub light: LightData,
}

impl crate::Packet for ChunkDataAndLightUpdate {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct GameEffect {
    pub effect_id: i32,
    #[options(compressed = true)]
    pub location: IVec3,
    pub data: i32,
    pub disable_relative_volume: bool,
}

impl crate::Packet for GameEffect {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct SpawnParticle {
    pub particle_id: i32,
    pub long_dst: bool,
    pub position: DVec3,
    pub offset: Vec3,
    pub value: f32,
    pub count: i32,
    pub data: ParticleData, // TODO
}

impl crate::Packet for SpawnParticle {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct LightUpdate {
    pub chunk_coords: IVec2,
    pub light: LightData,
}

impl crate::Packet for LightUpdate {}

/// The first packet sent by the server upon transitioning to the Play state.
#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct JoinGame {
    /// The player's network entity id (EID).
    pub id: i32,
    /// Whether the server is in hardcore mode (one life, generally).
    pub hardcore: bool,
    /// The player's current gamemode.
    pub gamemode: GameMode,
    /// The player's previous gamemode.
    pub previous_gamemode: PreviousGameMode,
    /// The keys for all worlds in the server.
    pub world_keys: Vec<Key>,
    /// A dimension and biome registry.
    pub dimension_codec: Nbt<DimensionCodec>,
    /// The dimension type data.
    pub dimension_type: Nbt<DimensionType>,
    /// The key of the player's current world.
    pub world_key: Key,
    /// First 8 bytes of SHA256 hash of the world's seed.
    pub hashed_seed: i64,
    /// The maximum players. Currently ignored by the client.
    #[options(varint = true)]
    pub max_players: i32,
    /// The maximum render distance enforced by the server (2-32).
    #[options(varint = true)]
    pub view_dst: i32,
    /// The distance that clients will process things like entities.
    #[options(varint = true)]
    pub sim_dst: i32,
    /// Whether the client receives additional info in the debug screen.
    pub reduced_debug_info: bool,
    /// Whether to display the respawn screen on death. Otherwise, immediately respawn.
    pub respawn_screen: bool,
    /// Whether the world is a debug mode world.
    pub debug: bool,
    /// Whether the world is super-flat.
    pub flat: bool,
}

impl crate::Packet for JoinGame {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct MapUpdate {
    #[options(varint = true)]
    pub map_id: i32,
    pub scale: i8,
    pub locked: bool,
    pub track_pos: bool,
    pub icons: Vec<Icon>,
    pub columns: u8,
    // TODO: rest of fields
}

impl crate::Packet for MapUpdate {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct TradeList {
    #[options(varint = true)]
    pub window_id: i32,
    #[options(length = "byte")]
    pub trades: Vec<Trade>,
    #[options(varint = true)]
    pub villager_level: i32,
    #[options(varint = true)]
    pub exp: i32,
    pub is_normal_villager: bool,
    pub can_restock: bool,
}

impl crate::Packet for TradeList {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct EntityPosition {
    pub entity_id: i32,
    pub delta: [i16; 3],
    pub ground: bool,
}

impl crate::Packet for EntityPosition {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct EntityPositionAndRotation {
    pub entity_id: i32,
    pub delta: [i16; 3],
    pub yaw: Angle,
    pub pitch: Angle,
    pub ground: bool,
}

impl crate::Packet for EntityPositionAndRotation {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct EntityRotation {
    pub entity_id: i32,
    pub yaw: Angle,
    pub pitch: Angle,
    pub ground: bool,
}

impl crate::Packet for EntityRotation {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct VehicleMovement {
    pub position: DVec3,
    pub yaw: f32,
    pub pitch: f32,
}

impl crate::Packet for VehicleMovement {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct OpenBook {
    pub hand: Hand,
}

impl crate::Packet for OpenBook {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct OpenWindow {
    #[options(varint = true)]
    pub id: i32,
    #[options(varint = true)]
    pub kind: i32,
    pub title: Text,
}

impl crate::Packet for OpenWindow {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct OpenSignEditor {
    pub position: IVec3,
}

impl crate::Packet for OpenSignEditor {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct Ping(pub i32);

impl crate::Packet for Ping {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct CraftResponse {
    pub window_id: i8,
    pub recipe: Key,
}

impl crate::Packet for CraftResponse {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct PlayerAbilitiesUpdate {
    pub flags: BitFlags<PlayerAbilities>,
    pub flying_speed: f32,
    pub fov_modifier: f32,
}

impl crate::Packet for PlayerAbilitiesUpdate {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct EndCombat {
    #[options(varint = true)]
    pub duration: i32,
    pub entity_id: i32,
}

impl crate::Packet for EndCombat {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct EnterCombat;

impl crate::Packet for EnterCombat {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct EndCombatDeath {
    #[options(varint = true)]
    pub player_id: i32,
    pub entity_id: i32,
    pub message: Text,
}

impl crate::Packet for EndCombatDeath {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct TabListUpdate {
    pub actions: TabListActions,
}

impl crate::Packet for TabListUpdate {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct FacePlayer {
    pub mode: FaceMode,
    pub target: DVec3,
    pub entity: Option<FaceEntity>,
}

impl crate::Packet for FacePlayer {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct PlayerPositionAndRotation {
    pub position: DVec3,
    pub yaw: f32,
    pub pitch: f32,
    pub flags: i8,
    #[options(varint = true)]
    pub tp_id: i32,
    pub dismount: bool,
}

impl crate::Packet for PlayerPositionAndRotation {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
#[io_repr(varint)]
pub enum UnlockRecipes {
    Init(InitRecipes),
    Add(ChangeRecipes),
    Remove(ChangeRecipes),
}

impl crate::Packet for UnlockRecipes {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct DestroyEntities {
    #[options(inner.varint = true)]
    pub entities: Vec<i32>,
}

impl crate::Packet for DestroyEntities {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct RemoveEntityEffect {
    #[options(varint = true)]
    pub entity_id: i32,
    #[options(varint = true)]
    pub effect_id: i32,
}

impl crate::Packet for RemoveEntityEffect {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct ResourcePackRequest {
    pub url: String,
    #[options(max_len = 40)]
    pub hash: String,
    pub forced: bool,
    pub prompt: Option<Text>,
}

impl crate::Packet for ResourcePackRequest {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct Respawn {
    pub dimension: Nbt<DimensionType>,
    pub dimension_name: Key,
    pub hashed_seed: i64,
    pub gamemode: GameMode,
    pub previous_gamemode: PreviousGameMode,
    pub debug: bool,
    pub flat: bool,
    pub copy_metadata: bool,
}

impl crate::Packet for Respawn {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct EntityHeadLook {
    #[options(varint = true)]
    pub entity_id: i32,
    pub head_yaw: Angle,
}

impl crate::Packet for EntityHeadLook {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct MultiBlockUpdate {
    pub chunk_section_position: i64,
    pub invert_trust_edges: bool,
    #[options(inner.varint = true)]
    pub blocks: Vec<i64>,
}

impl crate::Packet for MultiBlockUpdate {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct SelectAdvancementTab {
    pub id: Option<Key>,
}

impl crate::Packet for SelectAdvancementTab {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct ActionBarMessage {
    pub message: Text,
}

impl crate::Packet for ActionBarMessage {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct WorldBorderCenter {
    pub x: f64,
    pub z: f64,
}

impl crate::Packet for WorldBorderCenter {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct WorldBorderLerpSize {
    pub old_diameter: f64,
    pub new_diameter: f64,
    #[options(varint = true)]
    pub speed: i64,
}

impl crate::Packet for WorldBorderLerpSize {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct WorldBorderSize {
    pub diameter: f64,
}

impl crate::Packet for WorldBorderSize {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct WorldBorderWarningDelay {
    #[options(varint = true)]
    pub warning_time: i32,
}

impl crate::Packet for WorldBorderWarningDelay {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct WorldBorderWarningReach {
    pub warning_blocks: i32,
}

impl crate::Packet for WorldBorderWarningReach {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct CameraUpdate {
    #[options(varint = true)]
    pub entity_id: i32,
}

impl crate::Packet for CameraUpdate {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct HeldItemUpdate {
    pub slot: i8,
}

impl crate::Packet for HeldItemUpdate {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct ViewPositionUpdate {
    #[options(varint = true)]
    pub chunk_x: i32,
    #[options(varint = true)]
    pub chunk_z: i32,
}

impl crate::Packet for ViewPositionUpdate {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct ViewDistanceUpdate {
    #[options(varint = true)]
    pub view_dst: i32,
}

impl crate::Packet for ViewDistanceUpdate {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct SpawnPosition {
    #[options(compressed = true)]
    pub position: IVec3,
    pub angle: f32,
}

impl crate::Packet for SpawnPosition {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct ShowScoreboard {
    pub display_kind: ScoreboardDisplayKind,
    #[options(max_len = 16)]
    pub score_name: String,
}

impl crate::Packet for ShowScoreboard {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct EntityMetadata {
    #[options(varint = true)]
    pub entity_id: i32,
    pub metadata: Metadata,
}

impl crate::Packet for EntityMetadata {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct LeashEntity {
    pub leashed_entity_id: i32,
    pub leashing_entity_id: i32,
}

impl crate::Packet for LeashEntity {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct EntityVelocityUpdate {
    #[options(varint = true)]
    pub entity_id: i32,
    pub velocity: [i16; 3],
}

impl crate::Packet for EntityVelocityUpdate {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct EntityEquipmentUpdate {
    #[options(varint = true)]
    pub entity_id: i32,
    // TODO: weird array
}

impl crate::Packet for EntityEquipmentUpdate {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct ExperienceUpdate {
    pub exp_bar: f32,
    #[options(varint = true)]
    pub exp_level: i32,
    #[options(varint = true)]
    pub exp_total: i32,
}

impl crate::Packet for ExperienceUpdate {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct HealthUpdate {
    pub health: f32,
    #[options(varint = true)]
    pub food: i32,
    pub saturation: f32,
}

impl crate::Packet for HealthUpdate {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct ScoreboardObjectiveUpdate {
    #[options(max_len = 16)]
    pub name: String,
    pub mode: i8,
    // TODO: weird Option<T> semantics
}

impl crate::Packet for ScoreboardObjectiveUpdate {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct PassengersUpdate {
    #[options(varint = true)]
    pub entity_id: i32,
    #[options(inner.varint = true)]
    pub passengers: Vec<i32>,
}

impl crate::Packet for PassengersUpdate {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
#[io_repr(i8)]
pub enum TeamUpdate {
    CreateTeam(CreateTeam),
    RemoveTeam(RemoveTeam),
    UpdateTeam(UpdateTeam),
    AddEntities(AddTeamEntities),
    RemoveEntities(RemoveTeamEntities),
}

impl crate::Packet for TeamUpdate {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct ScoreUpdate {
    #[options(max_len = 40)]
    pub entity_name: String,
    pub action: i8,
    #[options(max_len = 16)]
    pub objective_name: String,
    // TODO: value field with weird Option<T> semantics
}

impl crate::Packet for ScoreUpdate {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct SimulationDistanceUpdate {
    #[options(varint = true)]
    pub sim_dst: i32,
}

impl crate::Packet for SimulationDistanceUpdate {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct SubTitle(pub Text);

impl crate::Packet for SubTitle {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct TimeUpdate {
    pub world_age: i64,
    pub time_of_day: i64,
}

impl crate::Packet for TimeUpdate {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct Title(pub Text);

impl crate::Packet for Title {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct TitleTimes {
    pub fade_in: i32,
    pub stay: i32,
    pub fade_out: i32,
}

impl crate::Packet for TitleTimes {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct EntitySoundEffect {
    #[options(varint = true)]
    pub sound_id: i32,
    #[options(varint = true)]
    pub sound_category: i32,
    #[options(varint = true)]
    pub entity_id: i32,
    pub volume: f32,
    pub pitch: f32,
}

impl crate::Packet for EntitySoundEffect {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct SoundEffect {
    #[options(varint = true)]
    pub sound_id: i32,
    #[options(varint = true)]
    pub sound_category: i32,
    pub position: IVec3,
    pub volume: f32,
    pub pitch: f32,
}

impl crate::Packet for SoundEffect {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct SoundStop {
    pub flags: i8,
    // TODO source/sound fields with weird Option<T> semantics
}

impl crate::Packet for SoundStop {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct TabListHeaderAndFooter {
    pub header: Text,
    pub footer: Text,
}

impl crate::Packet for TabListHeaderAndFooter {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct NBTQueryResponse {
    #[options(varint = true)]
    pub transaction_id: i32,
    pub data: nbt::Value,
}

impl crate::Packet for NBTQueryResponse {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct ItemPickup {
    #[options(varint = true)]
    pub collected_entity_id: i32,
    #[options(varint = true)]
    pub collector_entity_id: i32,
    #[options(varint = true)]
    pub pickup_item_count: i32,
}

impl crate::Packet for ItemPickup {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct EntityTeleport {
    #[options(varint = true)]
    pub entity_id: i32,
    pub position: DVec3,
    pub yaw: Angle,
    pub pitch: Angle,
    pub ground: bool,
}

impl crate::Packet for EntityTeleport {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct AdvancementList {
    pub clear_current: bool,
    pub registry: HashMap<Key, Advancement>, // TODO: finish display field
    // pub remove: Vec<Key>,
    // pub progress: HashMap<Key, AdvancementProgress>,
}

impl crate::Packet for AdvancementList {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct EntityAttributes {
    pub entity_id: i32,
    pub attributes: Vec<EntityAttribute>,
}

impl crate::Packet for EntityAttributes {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct EntityPotionEffect {
    #[options(varint = true)]
    pub entity_id: i32,
    #[options(varint = true)]
    pub effect_id: i32,
    pub amplifier: i8,
    #[options(varint = true)]
    pub duration: i32,
    pub flags: u8,
}

impl crate::Packet for EntityPotionEffect {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct DeclareRecipes {
    // TODO
}

impl crate::Packet for DeclareRecipes {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct Tags {
    pub tags: HashMap<Key, Vec<Tag>>,
}

impl crate::Packet for Tags {}
