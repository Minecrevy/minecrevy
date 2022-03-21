//! This module contains any packets sent by the server.

use minecrevy_io_str::{McRead, McWrite};
use minecrevy_protocol::{Packet, PacketCodec, ProtocolState};
use minecrevy_text::Text;

use crate::{client, server};

pub use self::login::*;
pub use self::play::*;
pub use self::status::*;

mod login;
mod play;
mod status;

/// Tells the client that they are being disconnected.
#[derive(Clone, PartialEq, Debug, McRead, McWrite, Packet)]
pub struct Disconnect(pub Text);

macro_rules! incoming_packets {
    ( $codec:ident in $state:expr => $( $id:expr => $packet:ty , )* ) => {
        $($codec.incoming.register::<$packet>($state, $id);)*
    };
}

macro_rules! outgoing_packets {
    ( $codec:ident in $state:expr => $( $id:expr => $packet:ty , )* ) => {
        $($codec.outgoing.register::<$packet>($state, $id);)*
    };
}

/// The [`PacketCodec`] to be used by servers.
pub fn codec() -> PacketCodec {
    use ProtocolState::*;

    let mut codec = PacketCodec::default();

    // Handshake
    incoming_packets!(codec in Handshake =>
        0x00 => client::Handshake,
    );

    // Status
    incoming_packets!(codec in Status =>
        0x00 => client::StatusRequest,
        0x01 => client::StatusPing,
    );
    outgoing_packets!(codec in Status =>
        0x00 => server::StatusResponse,
        0x01 => server::StatusPong,
    );

    // Login
    incoming_packets!(codec in Login =>
        0x00 => client::LoginStart,
        0x01 => client::EncryptionResponse,
        0x02 => client::LoginPluginResponse,
    );
    outgoing_packets!(codec in Login =>
        0x00 => server::Disconnect,
        0x01 => server::EncryptionRequest,
        0x02 => server::LoginSuccess,
        0x03 => server::Compression,
        0x04 => server::LoginPluginRequest,
    );

    // Play
    incoming_packets!(codec in Play =>
        0x00 => client::TeleportConfirm,
        0x01 => client::NBTQueryBlock,
        0x02 => client::DifficultyUpdate,
        0x03 => client::ChatMessage,
        0x04 => client::ClientStatusUpdate,
        0x05 => client::ClientSettings,
        0x06 => client::TabCompletion,
        0x07 => client::ClickWindowButton,
        0x08 => client::ClickWindowSlot,
        0x09 => client::CloseWindow,
        0x0A => client::PluginMessage,
        0x0B => client::BookEdit,
        0x0C => client::NBTQueryEntity,
        0x0D => client::EntityInteraction,
        0x0E => client::StructureGeneration,
        0x0F => client::KeepAlive,
        0x10 => client::LockDifficulty,
        0x11 => client::PlayerPosition,
        0x12 => client::PlayerPositionAndRotation,
        0x13 => client::PlayerRotation,
        0x14 => client::PlayerGrounded,
        0x15 => client::VehicleMovement,
        0x16 => client::BoatSteer,
        0x17 => client::PickItem,
        0x18 => client::CraftRequest,
        0x19 => client::PlayerAbilitiesUpdate,
        0x1A => client::BlockDig,
        0x1B => client::EntityAction,
        0x1C => client::VehicleSteer,
        0x1D => client::Pong,
        0x1E => client::RecipeBookUpdate,
        0x1F => client::DisplayedRecipe,
        0x20 => client::ItemName,
        0x21 => client::ResourcePackResponse,
        0x22 => client::DisplayedAdvancementTab,
        0x23 => client::TradeSelection,
        0x24 => client::BeaconEffectUpdate,
        0x25 => client::HeldItemChange,
        0x26 => client::CommandBlockUpdate,
        0x27 => client::CommandBlockMinecartUpdate,
        0x28 => client::CreativeInventoryUpdate,
        0x29 => client::JigsawBlockUpdate,
        0x2A => client::StructureBlockUpdate,
        0x2B => client::SignUpdate,
        0x2C => client::PlayerAnimation,
        0x2D => client::Spectate,
        0x2E => client::PlayerBlockPlacement,
        0x2F => client::ItemUse,
    );
    outgoing_packets!(codec in Play =>
        0x00 => server::SpawnEntity,
        0x01 => server::SpawnExpOrb,
        0x02 => server::SpawnLivingEntity,
        0x03 => server::SpawnPainting,
        0x04 => server::SpawnPlayer,
        0x05 => server::SpawnVibrationSignal,
        0x06 => server::EntityAnimation,
        0x07 => server::StatisticsUpdate,
        0x08 => server::AckBlockBreak,
        0x09 => server::BlockBreakAnimation,
        0x0A => server::SpawnBlockEntity,
        0x0B => server::BlockAction,
        0x0C => server::BlockUpdate,
        0x0D => server::BossBarUpdate,
        0x0E => server::DifficultyUpdate,
        0x0F => server::ChatMessage,
        0x10 => server::ClearTitles,
        0x11 => server::TabCompletion,
        0x12 => server::DeclareCommands,
        0x13 => server::CloseWindow,
        0x14 => server::WindowSlotsUpdate,
        0x15 => server::WindowPropertyUpdate,
        0x16 => server::WindowSlotUpdate,
        0x17 => server::ItemCooldown,
        0x18 => server::PluginMessage,
        0x19 => server::NamedSoundEffect,
        0x1A => server::Disconnect,
        0x1B => server::EntityStatus,
        0x1C => server::Explosion,
        0x1D => server::ChunkUnload,
        0x1E => server::GameStateUpdate,
        0x1F => server::OpenHorseWindow,
        0x20 => server::CreateWorldBorder,
        0x21 => server::KeepAlive,
        0x22 => server::ChunkDataAndLightUpdate,
        0x23 => server::GameEffect,
        0x24 => server::SpawnParticle,
        0x25 => server::LightUpdate,
        0x26 => server::JoinGame,
        0x27 => server::MapUpdate,
        0x28 => server::TradeList,
        0x29 => server::EntityPosition,
        0x2A => server::EntityPositionAndRotation,
        0x2B => server::EntityRotation,
        0x2C => server::VehicleMovement,
        0x2D => server::OpenBook,
        0x2E => server::OpenWindow,
        0x2F => server::OpenSignEditor,
        0x30 => server::Ping,
        0x31 => server::CraftResponse,
        0x32 => server::PlayerAbilitiesUpdate,
        0x33 => server::EndCombat,
        0x34 => server::EnterCombat,
        0x35 => server::EndCombatDeath,
        0x36 => server::TabListUpdate,
        0x37 => server::FacePlayer,
        0x38 => server::PlayerPositionAndRotation,
        0x39 => server::UnlockRecipes,
        0x3A => server::DestroyEntities,
        0x3B => server::RemoveEntityEffect,
        0x3C => server::ResourcePackRequest,
        0x3D => server::Respawn,
        0x3E => server::EntityHeadLook,
        0x3F => server::MultiBlockUpdate,
        0x40 => server::SelectAdvancementTab,
        0x41 => server::ActionBarMessage,
        0x42 => server::WorldBorderCenter,
        0x43 => server::WorldBorderLerpSize,
        0x44 => server::WorldBorderSize,
        0x45 => server::WorldBorderWarningDelay,
        0x46 => server::WorldBorderWarningReach,
        0x47 => server::CameraUpdate,
        0x48 => server::HeldItemUpdate,
        0x49 => server::ViewPositionUpdate,
        0x4A => server::ViewDistanceUpdate,
        0x4B => server::SpawnPosition,
        0x4C => server::ShowScoreboard,
        0x4D => server::EntityMetadata,
        0x4E => server::LeashEntity,
        0x4F => server::EntityVelocityUpdate,
        0x50 => server::EntityEquipmentUpdate,
        0x51 => server::ExperienceUpdate,
        0x52 => server::HealthUpdate,
        0x53 => server::ScoreboardObjectiveUpdate,
        0x54 => server::PassengersUpdate,
        0x55 => server::TeamUpdate,
        0x56 => server::ScoreUpdate,
        0x57 => server::SimulationDistanceUpdate,
        0x58 => server::SubTitle,
        0x59 => server::TimeUpdate,
        0x5A => server::Title,
        0x5B => server::TitleTimes,
        0x5C => server::EntitySoundEffect,
        0x5D => server::SoundEffect,
        0x5E => server::SoundStop,
        0x5F => server::TabListHeaderAndFooter,
        0x60 => server::NBTQueryResponse,
        0x61 => server::ItemPickup,
        0x62 => server::EntityTeleport,
        0x63 => server::AdvancementList,
        0x64 => server::EntityAttributes,
        0x65 => server::EntityPotionEffect,
        0x66 => server::DeclareRecipes,
        0x67 => server::Tags,
    );

    codec
}
