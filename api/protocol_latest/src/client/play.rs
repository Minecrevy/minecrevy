use enumflags2::BitFlags;
use glam::IVec3;
use uuid::Uuid;

use minecrevy_io_str::{McRead, McWrite};
use minecrevy_key::Key;
use minecrevy_protocol::Packet;

use crate::types::*;

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct TeleportConfirm {
    #[options(varint = true)]
    pub teleport_id: i32,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct NBTQueryBlock {
    #[options(varint = true)]
    pub transaction_id: i32,
    #[options(compressed = true)]
    pub position: IVec3,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct DifficultyUpdate {
    pub difficulty: Difficulty,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct ChatMessage {
    #[options(max_len = 256)]
    pub message: String,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct ClientStatusUpdate {
    pub action: ClientStatusAction,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct ClientSettings {
    #[options(max_len = 16)]
    pub locale: String,
    pub view_dst: i8,
    #[options(varint = true)]
    pub chat_mode: i32,
    pub chat_colors: bool,
    pub skin_parts: u8,
    #[options(varint = true)]
    pub main_hand: i32,
    pub enable_text_filter: bool,
    pub allow_server_listings: bool,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct TabCompletion {
    #[options(varint = true)]
    pub transaction_id: i32,
    #[options(max_len = 32500)]
    pub text: String,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct ClickWindowButton {
    pub window_id: i8,
    pub button_id: i8,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct ClickWindowSlot {
    pub window_id: u8,
    #[options(varint = true)]
    pub state_id: i32,
    pub slot: i16,
    pub button: i8,
    #[options(varint = true)]
    pub mode: i32,
    pub slots: Vec<(i16, Slot)>,
    pub clicked: Slot,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct CloseWindow {
    pub window_id: u8,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct PluginMessage {
    pub channel: Key,
    #[options(length = "remaining")]
    pub data: Vec<u8>,
}

impl PluginMessage {
    pub fn brand(brand: String) -> Self {
        Self {
            channel: Key::new("minecraft", "brand").unwrap(),
            data: brand.into_bytes(),
        }
    }
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct BookEdit {
    pub hand: Hand,
    pub pages: Vec<String>,
    pub title: Option<String>,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct NBTQueryEntity {
    #[options(varint = true)]
    pub transaction_id: i32,
    #[options(varint = true)]
    pub entity_id: i32,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct EntityInteraction {
    #[options(varint = true)]
    pub entity_id: i32,
    pub kind: EntityInteractionKind,
    // TODO: other fields have weird Option<T> semantics
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct StructureGeneration {
    #[options(compressed = true)]
    pub position: IVec3,
    #[options(varint = true)]
    pub levels: i32,
    pub keep_jigsaws: bool,
}

pub type KeepAlive = crate::server::KeepAlive;

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct LockDifficulty {
    pub locked: bool,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct PlayerPosition {
    pub position: DVec3,
    pub ground: bool,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct PlayerPositionAndRotation {
    pub position: DVec3,
    pub yaw: f32,
    pub pitch: f32,
    pub ground: bool,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct PlayerRotation {
    pub yaw: f32,
    pub pitch: f32,
    pub ground: bool,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct PlayerGrounded {
    pub ground: bool,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct VehicleMovement {
    pub position: DVec3,
    pub yaw: f32,
    pub pitch: f32,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct BoatSteer {
    pub left_paddle_turn: bool,
    pub right_paddle_turn: bool,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct PickItem {
    #[options(varint = true)]
    pub slot: i32,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct CraftRequest {
    pub window_id: i8,
    pub recipe: Key,
    pub make_all: bool,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct PlayerAbilitiesUpdate {
    pub abilities: BitFlags<PlayerAbilities>,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct BlockDig {
    pub status: BlockDigStatus,
    pub position: IVec3,
    pub face: Face,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct EntityAction {
    #[options(varint = true)]
    pub entity_id: i32,
    pub kind: EntityActionKind,
    #[options(varint = true)]
    pub jump_boost: i32,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct VehicleSteer {
    pub sideways: f32,
    pub forward: f32,
    pub flags: u8,
}

pub type Pong = crate::server::Ping;

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct RecipeBookUpdate {
    pub kind: RecipeBookKind,
    pub open: bool,
    pub filter: bool,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct DisplayedRecipe(pub Key);


#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct ItemName {
    pub name: String,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct ResourcePackResponse {
    pub result: ResourcePackResult,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct DisplayedAdvancementTab {
    #[options(varint = true)]
    pub action: i32,
    // pub tab_id: Option<Key> // TODO: weird Option<T> semantics
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct TradeSelection {
    #[options(varint = true)]
    pub slot: i32,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct BeaconEffectUpdate {
    #[options(varint = true)]
    pub primary: i32,
    #[options(varint = true)]
    pub secondary: i32,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct HeldItemChange {
    pub slot: i16, // mojang why is this different from serverside???
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct CommandBlockUpdate {
    #[options(compressed = true)]
    pub position: IVec3,
    pub command: String,
    pub mode: CommandBlockMode,
    pub flags: i8,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct CommandBlockMinecartUpdate {
    #[options(varint = true)]
    pub entity_id: i32,
    pub command: String,
    pub track_output: bool,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct CreativeInventoryUpdate {
    pub slot: i16,
    pub clicked: Slot,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct JigsawBlockUpdate {
    #[options(compressed = true)]
    pub position: IVec3,
    pub name: Key,
    pub target: Key,
    pub pool: Key,
    pub final_state: String,
    pub joint_type: String,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct StructureBlockUpdate {
    #[options(compressed = true)]
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
    #[options(varint = true)]
    pub seed: i64,
    pub flags: i8,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct SignUpdate {
    #[options(compressed = true)]
    pub position: IVec3,
    #[options(inner.max_len = 384)]
    pub lines: [String; 4],
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct PlayerAnimation {
    pub hand: Hand,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct Spectate {
    pub target_player: Uuid,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct PlayerBlockPlacement {
    pub hand: Hand,
    #[options(compressed = true)]
    pub position: IVec3,
    pub face: Face,
    pub cursor_position: Vec3,
    pub inside_block: bool,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct ItemUse {
    pub hand: Hand,
}
