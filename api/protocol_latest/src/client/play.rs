use enumflags2::BitFlags;
use glam::IVec3;
use uuid::Uuid;

use minecrevy_io_str::{McRead, McWrite};
use minecrevy_key::Key;

use crate::types::*;

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct TeleportConfirm {
    #[mcio(varint)]
    pub teleport_id: i32,
}

impl crate::Packet for TeleportConfirm {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct NBTQueryBlock {
    #[mcio(varint)]
    pub transaction_id: i32,
    #[mcio(compressed)]
    pub position: IVec3,
}

impl crate::Packet for NBTQueryBlock {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct DifficultyUpdate {
    pub difficulty: Difficulty,
}

impl crate::Packet for DifficultyUpdate {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct ChatMessage {
    #[mcio(max_len = 256)]
    pub message: String,
}

impl crate::Packet for ChatMessage {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct ClientStatusUpdate {
    pub action: ClientStatusAction,
}

impl crate::Packet for ClientStatusUpdate {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct ClientSettings {
    #[mcio(max_len = 16)]
    pub locale: String,
    pub view_dst: i8,
    #[mcio(varint)]
    pub chat_mode: i32,
    pub chat_colors: bool,
    pub skin_parts: u8,
    #[mcio(varint)]
    pub main_hand: i32,
    pub enable_text_filter: bool,
    pub allow_server_listings: bool,
}

impl crate::Packet for ClientSettings {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct TabCompletion {
    #[mcio(varint)]
    pub transaction_id: i32,
    #[mcio(max_len = 32500)]
    pub text: String,
}

impl crate::Packet for TabCompletion {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct ClickWindowButton {
    pub window_id: i8,
    pub button_id: i8,
}

impl crate::Packet for ClickWindowButton {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct ClickWindowSlot {
    pub window_id: u8,
    #[mcio(varint)]
    pub state_id: i32,
    pub slot: i16,
    pub button: i8,
    #[mcio(varint)]
    pub mode: i32,
    pub slots: Vec<(i16, Slot)>,
    pub clicked: Slot,
}

impl crate::Packet for ClickWindowSlot {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct CloseWindow {
    pub window_id: u8,
}

impl crate::Packet for CloseWindow {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct PluginMessage {
    pub channel: Key,
    #[mcio(length = "remaining")]
    pub data: Vec<u8>,
}

impl crate::Packet for PluginMessage {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct BookEdit {
    pub hand: Hand,
    pub pages: Vec<String>,
    pub title: Option<String>,
}

impl crate::Packet for BookEdit {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct NBTQueryEntity {
    #[mcio(varint)]
    pub transaction_id: i32,
    #[mcio(varint)]
    pub entity_id: i32,
}

impl crate::Packet for NBTQueryEntity {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct EntityInteraction {
    #[mcio(varint)]
    pub entity_id: i32,
    pub kind: EntityInteractionKind,
    // TODO: other fields have weird Option<T> semantics
}

impl crate::Packet for EntityInteraction {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct StructureGeneration {
    #[mcio(compressed)]
    pub position: IVec3,
    #[mcio(varint)]
    pub levels: i32,
    pub keep_jigsaws: bool,
}

impl crate::Packet for StructureGeneration {}

pub type KeepAlive = crate::server::KeepAlive;

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct LockDifficulty {
    pub locked: bool,
}

impl crate::Packet for LockDifficulty {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct PlayerPosition {
    pub position: DVec3,
    pub ground: bool,
}

impl crate::Packet for PlayerPosition {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct PlayerPositionAndRotation {
    pub position: DVec3,
    pub yaw: f32,
    pub pitch: f32,
    pub ground: bool,
}

impl crate::Packet for PlayerPositionAndRotation {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct PlayerRotation {
    pub yaw: f32,
    pub pitch: f32,
    pub ground: bool,
}

impl crate::Packet for PlayerRotation {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct PlayerGrounded {
    pub ground: bool,
}

impl crate::Packet for PlayerGrounded {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct VehicleMovement {
    pub position: DVec3,
    pub yaw: f32,
    pub pitch: f32,
}

impl crate::Packet for VehicleMovement {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct BoatSteer {
    pub left_paddle_turn: bool,
    pub right_paddle_turn: bool,
}

impl crate::Packet for BoatSteer {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct PickItem {
    #[mcio(varint)]
    pub slot: i32,
}

impl crate::Packet for PickItem {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct CraftRequest {
    pub window_id: i8,
    pub recipe: Key,
    pub make_all: bool,
}

impl crate::Packet for CraftRequest {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct PlayerAbilitiesUpdate {
    pub abilities: BitFlags<PlayerAbilities>,
}

impl crate::Packet for PlayerAbilitiesUpdate {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct BlockDig {
    pub status: BlockDigStatus,
    pub position: IVec3,
    pub face: Face,
}

impl crate::Packet for BlockDig {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct EntityAction {
    #[mcio(varint)]
    pub entity_id: i32,
    pub kind: EntityActionKind,
    #[mcio(varint)]
    pub jump_boost: i32,
}

impl crate::Packet for EntityAction {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct VehicleSteer {
    pub sideways: f32,
    pub forward: f32,
    pub flags: u8,
}

impl crate::Packet for VehicleSteer {}

pub type Pong = crate::server::Ping;

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct RecipeBookUpdate {
    pub kind: RecipeBookKind,
    pub open: bool,
    pub filter: bool,
}

impl crate::Packet for RecipeBookUpdate {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct DisplayedRecipe(pub Key);

impl crate::Packet for DisplayedRecipe {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct ItemName {
    pub name: String,
}

impl crate::Packet for ItemName {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct ResourcePackResponse {
    pub result: ResourcePackResult,
}

impl crate::Packet for ResourcePackResponse {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct DisplayedAdvancementTab {
    #[mcio(varint)]
    pub action: i32,
    // pub tab_id: Option<Key> // TODO: weird Option<T> semantics
}

impl crate::Packet for DisplayedAdvancementTab {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct TradeSelection {
    #[mcio(varint)]
    pub slot: i32,
}

impl crate::Packet for TradeSelection {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct BeaconEffectUpdate {
    #[mcio(varint)]
    pub primary: i32,
    #[mcio(varint)]
    pub secondary: i32,
}

impl crate::Packet for BeaconEffectUpdate {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct HeldItemChange {
    pub slot: i16, // mojang why is this different from serverside???
}

impl crate::Packet for HeldItemChange {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct CommandBlockUpdate {
    #[mcio(compressed)]
    pub position: IVec3,
    pub command: String,
    pub mode: CommandBlockMode,
    pub flags: i8,
}

impl crate::Packet for CommandBlockUpdate {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct CommandBlockMinecartUpdate {
    #[mcio(varint)]
    pub entity_id: i32,
    pub command: String,
    pub track_output: bool,
}

impl crate::Packet for CommandBlockMinecartUpdate {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct CreativeInventoryUpdate {
    pub slot: i16,
    pub clicked: Slot,
}

impl crate::Packet for CreativeInventoryUpdate {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct JigsawBlockUpdate {
    #[mcio(compressed)]
    pub position: IVec3,
    pub name: Key,
    pub target: Key,
    pub pool: Key,
    pub final_state: String,
    pub joint_type: String,
}

impl crate::Packet for JigsawBlockUpdate {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct StructureBlockUpdate {
    #[mcio(compressed)]
    pub position: IVec3,
    pub action: StructureBlockAction,
    pub mode: StructureBlockMode,
    pub name: String,
    pub offset: [i8; 3],
    pub size: [i8; 3],
    pub mirror: StructureBlockMirror,
    pub rotation: StructureBlockRotation,
    pub metadata: String,
    pub integrity: f32,
    #[mcio(varint)]
    pub seed: i64,
    pub flags: i8,
}

impl crate::Packet for StructureBlockUpdate {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct SignUpdate {
    #[mcio(compressed)]
    pub position: IVec3,
    #[mcio(inner::max_len = 384)]
    pub lines: [String; 4],
}

impl crate::Packet for SignUpdate {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct PlayerAnimation {
    pub hand: Hand,
}

impl crate::Packet for PlayerAnimation {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct Spectate {
    pub target_player: Uuid,
}

impl crate::Packet for Spectate {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct PlayerBlockPlacement {
    pub hand: Hand,
    #[mcio(compressed)]
    pub position: IVec3,
    pub face: Face,
    pub cursor_position: Vec3,
    pub inside_block: bool,
}

impl crate::Packet for PlayerBlockPlacement {}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct ItemUse {
    pub hand: Hand,
}

impl crate::Packet for ItemUse {}
