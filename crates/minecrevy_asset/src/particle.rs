use bevy::reflect::TypeUuid;
use derive_more::AsRef;
use minecrevy_core::key::Key;

/// A named particle type in Minecraft.
#[derive(TypeUuid, AsRef)]
#[uuid = "bff65951-1e8d-4200-9b9b-1a40f4879822"]
pub struct ParticleType {
    /// The key that uniquely identifies the particle type.
    #[as_ref]
    pub key: Key,
}
