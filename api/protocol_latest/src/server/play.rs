use std::collections::HashMap;
use std::marker::PhantomData;

use enumflags2::BitFlags;
use flexstr::SharedStr;
use uuid::Uuid;

use minecrevy_io_str::{McRead, McWrite, Nbt};
use minecrevy_key::Key;
use minecrevy_math::vector::Vector;
use minecrevy_protocol::Packet;
use minecrevy_text::{Text, TextPosition};
use minecrevy_util::{Difficulty, Direction2d, GameMode, Hand};

use crate::types::*;

/// Spawns a vehicle or non-living entity.
#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
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
    pub position: Vector<3, f64>,
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

/// Spawns one or more experience orbs.
#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct SpawnExpOrb {
    /// The experience orb's network ID.
    #[options(varint = true)]
    pub net_id: i32,
    /// The experience orb's position.
    pub position: Vector<3, f64>,
    /// How much experience the orb will reward when collected.
    pub count: i16,
}

/// Spawns a living entity.
#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
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
    pub position: Vector<3, f64>,
    /// The entity's yaw as an [`Angle`].
    pub yaw: Angle,
    /// The entity's pitch as an [`Angle`].
    pub pitch: Angle,
    /// The entity's head yaw as an [`Angle`].
    pub head_yaw: Angle,
    /// The entity's velocity.
    pub velocity: [i16; 3],
}

/// Spawns a painting.
#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
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
    pub position: Vector<3, i32>,
    /// The painting's faced direction.
    pub direction: Direction2d,
}

/// Spawns a player.
#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct SpawnPlayer {
    /// The player's network ID.
    #[options(varint = true)]
    pub net_id: i32,
    /// The player's [`unique ID`][`Uuid`].
    pub uuid: Uuid,
    /// The player's position.
    pub position: Vector<3, f64>,
    /// The player's yaw as an [`Angle`].
    pub yaw: Angle,
    /// The player's pitch as an [`Angle`].
    pub pitch: Angle,
}

/// Spawns a sculk vibration signal.
#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct SpawnVibrationSignal {
    /// The vibration's source position.
    #[options(compressed = true)]
    pub position: Vector<3, i32>,
    /// The vibration's destination.
    pub destination: SignalDestination,
    /// How long until the vibration arrives at the destination.
    #[options(varint = true)]
    pub arrival_ticks: i32,
}

/// Animates an entity.
#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct EntityAnimation {
    /// The entity's network ID.
    #[options(varint = true)]
    pub entity_id: i32,
    pub animation: Animation,
}

/// Sets the player's statistics.
#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct StatisticsUpdate {
    pub statistics: Vec<Statistic>,
}

/// Acknowledges that a block break occurred.
#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct AckBlockBreak {
    #[options(compressed = true)]
    pub position: Vector<3, i32>,
    #[options(varint = true)]
    pub block: i32,
    #[options(varint = true)]
    pub status: i32,
    pub successful: bool,
}

/// Animates a block being broken.
#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct BlockBreakAnimation {
    #[options(varint = true)]
    pub entity_id: i32,
    #[options(compressed = true)]
    pub position: Vector<3, i32>,
    /// 0..=9
    pub stage: u8,
}

/// Spawns a block entity.
#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct SpawnBlockEntity {
    #[options(compressed = true)]
    pub position: Vector<3, i32>,
    #[options(varint = true)]
    pub ty: i32,
    pub data: nbt::Value,
}

/// Performs a block action.
#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct BlockAction {
    #[options(compressed = true)]
    pub position: Vector<3, i32>,
    pub action_id: u8,
    pub action_param: u8,
    #[options(varint = true)]
    pub ty: i32,
}

/// Sets a block to a new state.
#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct BlockUpdate {
    #[options(compressed = true)]
    pub position: Vector<3, i32>,
    #[options(varint = true)]
    pub block_id: i32,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
#[io_repr(varint)]
pub enum BossBarUpdate {
    Add(AddBossBar),
    Remove(RemoveBossBar),
    UpdateHealth(UpdateHealthBossBar),
    UpdateTitle(UpdateTitleBossBar),
    UpdateStyle(UpdateStyleBossBar),
    UpdateFlags(UpdateFlagsBossBar),
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct AddBossBar {
    pub title: Text,
    pub health: f32,
    pub color: BossBarColor,
    pub style: BossBarStyle,
    pub flags: u8,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct RemoveBossBar;

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct UpdateHealthBossBar(pub f32);

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct UpdateTitleBossBar(pub Text);

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct UpdateStyleBossBar(pub BossBarColor, pub BossBarStyle);

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct UpdateFlagsBossBar(pub u8);

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct DifficultyUpdate {
    /// The difficulty that should appear in the client's option menu.
    pub difficulty: Difficulty,
    /// True if the difficulty should be unchangeable.
    pub locked: bool,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct ChatMessage {
    pub message: Text,
    pub position: TextPosition,
    pub sender: Uuid,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct ClearTitles {
    pub reset: bool,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct TabCompletion {
    #[options(varint = true)]
    pub id: i32,
    #[options(varint = true)]
    pub start: i32,
    #[options(varint = true)]
    pub len: i32,
    pub matches: Vec<TabCompletionMatch>,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct DeclareCommands {
    pub nodes: Vec<CommandNode>,
    // #[options(varint = true)]
    pub root_idx: i32,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct CloseWindow {
    pub window_id: u8,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct WindowSlotsUpdate {
    pub window_id: u8,
    #[options(varint = true)]
    pub state_id: i32,
    pub slots: Vec<Slot>,
    pub held: Slot,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct WindowPropertyUpdate {
    pub window_id: u8,
    pub property: i16,
    pub value: i16,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct WindowSlotUpdate {
    pub window_id: u8,
    #[options(varint = true)]
    pub state_id: i32,
    pub slot_idx: i16,
    pub slot: Slot,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct ItemCooldown {
    #[options(varint = true)]
    pub item_id: i32,
    #[options(varint = true)]
    pub cooldown_ticks: i32,
}

pub type PluginMessage = crate::client::PluginMessage;

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct NamedSoundEffect {
    pub name: Key,
    #[options(varint = true)]
    pub category: i32,
    pub position: Vector<3, i32>,
    pub volume: f32,
    pub pitch: f32,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct EntityStatus {
    pub entity_id: i32,
    pub status: u8,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct Explosion {
    pub position: Vector<3, f32>,
    pub strength: f32,
    pub offsets: Vec<BlockOffset>,
    pub push_velocity: Vector<3, f32>,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct ChunkUnload {
    pub chunk_x: i32,
    pub chunk_y: i32,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
#[io_repr(u8)]
pub enum GameStateUpdate {
    /// The player couldn't respawn at their normal respawn location.
    NoRespawnBlock(PhantomData<f32>),
    /// It has stopped raining in the player's world.
    BeginRaining(PhantomData<f32>),
    /// It has started raining in the player's world.
    EndRaining(PhantomData<f32>),
    /// The player's [`GameMode`] has changed.
    GameMode(AsPrimitive<GameMode, f32>),
    /// The player beat the game (killed the EnderDragon and jumped through the spawned portal).
    WinGame {
        /// True if the endgame credits should be shown.
        roll_credits: BoolNum<f32>,
    },
    /// The player is playing the demo version of the game.
    DemoEvent(f32),
    /// The player has been struck by an arrow.
    ArrowHitPlayer(PhantomData<f32>),
    /// The amount of rain falling in the player's world has changed.
    RainLevelChange(f32),
    /// The amount of thunder sounding off in the player's world has changed.
    ThunderLevelChange(f32),
    /// Plays the pufferfish sting sound effect.
    PlayPufferfishStingSound(PhantomData<f32>),
    /// Shows the elder guardian overlay and plays its sound.
    PlayElderGuardianMobAppearance(PhantomData<f32>),
    /// Tells the player they are respawning.
    Respawn {
        /// True if the respawn screen should be skipped.
        immediate: BoolNum<f32>,
    },
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct OpenHorseWindow {
    pub window_id: u8,
    #[options(varint = true)]
    pub num_slots: i32,
    pub entity_id: i32,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
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

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct KeepAlive(pub i64);

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct ChunkDataAndLightUpdate {
    pub chunk_coords: Vector<2, i32>,
    pub chunk: ChunkData,
    pub light: LightData,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct GameEffect {
    pub effect_id: i32,
    #[options(compressed = true)]
    pub location: Vector<3, i32>,
    pub data: i32,
    pub disable_relative_volume: bool,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct SpawnParticle {
    pub particle_id: i32,
    pub long_dst: bool,
    pub position: Vector<3, f64>,
    pub offset: Vector<3, f32>,
    pub value: f32,
    pub count: i32,
    pub data: ParticleData, // TODO
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct LightUpdate {
    pub chunk_coords: Vector<2, i32>,
    pub light: LightData,
}

/// The first packet sent by the server upon transitioning to the Play state.
#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
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
    pub worlds: Vec<Key>,
    /// A dimension and biome registry.
    pub dimension_registry: Nbt<DimensionRegistry>,
    /// The dimension type data.
    pub dimension_type: Nbt<DimensionType>,
    /// The key of the player's current world.
    pub world: Key,
    /// First 8 bytes of SHA256 hash of the world's seed.
    pub seed: i64,
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

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
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

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
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

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct EntityPosition {
    pub entity_id: i32,
    pub delta: [i16; 3],
    pub ground: bool,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct EntityPositionAndRotation {
    pub entity_id: i32,
    pub delta: [i16; 3],
    pub yaw: Angle,
    pub pitch: Angle,
    pub ground: bool,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct EntityRotation {
    pub entity_id: i32,
    pub yaw: Angle,
    pub pitch: Angle,
    pub ground: bool,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct VehicleMovement {
    pub position: Vector<3, f64>,
    pub yaw: f32,
    pub pitch: f32,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct OpenBook {
    pub hand: Hand,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct OpenWindow {
    #[options(varint = true)]
    pub id: i32,
    #[options(varint = true)]
    pub kind: i32,
    pub title: Text,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct OpenSignEditor {
    pub position: Vector<3, i32>,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct Ping(pub i32);

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct CraftResponse {
    pub window_id: i8,
    pub recipe: Key,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct PlayerAbilitiesUpdate {
    pub flags: BitFlags<PlayerAbilities>,
    pub flying_speed: f32,
    pub fov_modifier: f32,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct EndCombat {
    #[options(varint = true)]
    pub duration: i32,
    pub entity_id: i32,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct EnterCombat;

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct EndCombatDeath {
    #[options(varint = true)]
    pub player_id: i32,
    pub entity_id: i32,
    pub message: Text,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct TabListUpdate {
    pub actions: TabListActions,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct FacePlayer {
    pub mode: FaceMode,
    pub target: Vector<3, f64>,
    pub entity: Option<FaceEntity>,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct PlayerPositionAndRotation {
    pub position: Vector<3, f64>,
    pub yaw: f32,
    pub pitch: f32,
    // TODO: strongly typed bit mask
    pub flags: u8,
    #[options(varint = true)]
    pub tp_id: i32,
    pub dismount: bool,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
#[io_repr(varint)]
pub enum UnlockRecipes {
    Init(InitRecipes),
    Add(ChangeRecipes),
    Remove(ChangeRecipes),
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct DestroyEntities {
    #[options(inner.varint = true)]
    pub entities: Vec<i32>,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct RemoveEntityEffect {
    #[options(varint = true)]
    pub entity_id: i32,
    #[options(varint = true)]
    pub effect_id: i32,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct ResourcePackRequest {
    pub url: SharedStr,
    #[options(max_len = 40)]
    pub hash: SharedStr,
    pub forced: bool,
    pub prompt: Option<Text>,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
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

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct EntityHeadLook {
    #[options(varint = true)]
    pub entity_id: i32,
    pub head_yaw: Angle,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct MultiBlockUpdate {
    pub chunk_section_position: i64,
    pub invert_trust_edges: bool,
    #[options(inner.varint = true)]
    pub blocks: Vec<i64>,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct SelectAdvancementTab {
    pub id: Option<Key>,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct ActionBarMessage(pub Text);

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct WorldBorderCenter {
    pub x: f64,
    pub z: f64,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct WorldBorderLerpSize {
    pub old_diameter: f64,
    pub new_diameter: f64,
    #[options(varint = true)]
    pub speed: i64,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct WorldBorderSize {
    pub diameter: f64,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct WorldBorderWarningDelay {
    #[options(varint = true)]
    pub warning_time: i32,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct WorldBorderWarningReach {
    pub warning_blocks: i32,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct CameraUpdate {
    #[options(varint = true)]
    pub entity_id: i32,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct HeldItemUpdate {
    pub slot: i8,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct ViewPositionUpdate {
    #[options(varint = true)]
    pub chunk_x: i32,
    #[options(varint = true)]
    pub chunk_z: i32,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct ViewDistanceUpdate {
    #[options(varint = true)]
    pub view_dst: i32,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct SpawnPosition {
    #[options(compressed = true)]
    pub position: Vector<3, i32>,
    pub pitch: f32,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct ShowScoreboard {
    pub display_kind: ScoreboardDisplayKind,
    #[options(max_len = 16)]
    pub score_name: SharedStr,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct EntityMetadata {
    #[options(varint = true)]
    pub entity_id: i32,
    pub metadata: Metadata,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct LeashEntity {
    pub leashed_entity_id: i32,
    pub leashing_entity_id: i32,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct EntityVelocityUpdate {
    #[options(varint = true)]
    pub entity_id: i32,
    pub velocity: [i16; 3],
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct EntityEquipmentUpdate {
    #[options(varint = true)]
    pub entity_id: i32,
    // TODO: weird array
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct ExperienceUpdate {
    pub exp_bar: f32,
    #[options(varint = true)]
    pub exp_level: i32,
    #[options(varint = true)]
    pub exp_total: i32,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct HealthUpdate {
    pub health: f32,
    #[options(varint = true)]
    pub food: i32,
    pub saturation: f32,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct ScoreboardObjectiveUpdate {
    #[options(max_len = 16)]
    pub name: SharedStr,
    pub mode: i8,
    // TODO: weird Option<T> semantics
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct PassengersUpdate {
    #[options(varint = true)]
    pub entity_id: i32,
    #[options(inner.varint = true)]
    pub passengers: Vec<i32>,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
#[io_repr(i8)]
pub enum TeamUpdate {
    CreateTeam(CreateTeam),
    RemoveTeam(RemoveTeam),
    UpdateTeam(UpdateTeam),
    AddEntities(AddTeamEntities),
    RemoveEntities(RemoveTeamEntities),
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct ScoreUpdate {
    #[options(max_len = 40)]
    pub entity_name: SharedStr,
    pub action: i8,
    #[options(max_len = 16)]
    pub objective_name: SharedStr,
    // TODO: value field with weird Option<T> semantics
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct SimulationDistanceUpdate {
    #[options(varint = true)]
    pub sim_dst: i32,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct SubTitle(pub Text);

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct TimeUpdate {
    pub world_age: i64,
    pub time_of_day: i64,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct Title(pub Text);

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct TitleTimes {
    pub fade_in: i32,
    pub stay: i32,
    pub fade_out: i32,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
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

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct SoundEffect {
    #[options(varint = true)]
    pub sound_id: i32,
    #[options(varint = true)]
    pub sound_category: i32,
    pub position: Vector<3, i32>,
    pub volume: f32,
    pub pitch: f32,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct SoundStop {
    pub flags: i8,
    // TODO source/sound fields with weird Option<T> semantics
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct TabListHeaderAndFooter {
    pub header: Text,
    pub footer: Text,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct NBTQueryResponse {
    #[options(varint = true)]
    pub transaction_id: i32,
    pub data: nbt::Value,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct ItemPickup {
    #[options(varint = true)]
    pub collected_entity_id: i32,
    #[options(varint = true)]
    pub collector_entity_id: i32,
    #[options(varint = true)]
    pub pickup_item_count: i32,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct EntityTeleport {
    #[options(varint = true)]
    pub entity_id: i32,
    pub position: Vector<3, f64>,
    pub yaw: Angle,
    pub pitch: Angle,
    pub ground: bool,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct AdvancementList {
    pub clear_current: bool,
    pub registry: HashMap<Key, Advancement>, // TODO: finish display field
    // pub remove: Vec<Key>,
    // pub progress: HashMap<Key, AdvancementProgress>,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct EntityAttributes {
    pub entity_id: i32,
    pub attributes: Vec<EntityAttribute>,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
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

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct DeclareRecipes {
    // TODO
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct Tags {
    pub tags: HashMap<Key, Vec<Tag>>,
}
