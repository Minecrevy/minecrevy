use bevy::{prelude::Handle, reflect::TypeUuid};
use derive_more::AsRef;
use minecrevy_core::key::Key;

/// A named sound file in Minecraft.
#[derive(TypeUuid, AsRef, Clone, PartialEq, Debug)]
#[uuid = "d4d36a32-8ab9-497a-b87f-9a594f845561"]
pub struct Sound {
    /// The key that uniquely identifies the sound.
    #[as_ref]
    pub key: Key,
}

/// A group of [`Sound`]s used by [`Block`](crate::block::Block)s.
#[derive(TypeUuid, AsRef)]
#[uuid = "fef8c763-dc49-4b05-9dea-d4dc84ef1e2a"]
pub struct SoundGroup {
    /// The key that uniquely identifies the group.
    #[as_ref]
    pub key: Key,
    /// The base volume of the sounds.
    pub volume: f32,
    /// The base pitch of the sounds.
    pub pitch: f32,
    /// The sound made when a block is broken.
    pub break_sound: Handle<Sound>,
    /// The sound made when a block is stepped on.
    pub step_sound: Handle<Sound>,
    /// The sound made when a block is placed.
    pub place_sound: Handle<Sound>,
    /// The sound made when a block is attacked.
    pub hit_sound: Handle<Sound>,
    /// The sound made when a block is fallen upon.
    pub fall_sound: Handle<Sound>,
}
