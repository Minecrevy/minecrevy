//! A storage-agnostic Minecraft chunk API.

use bevy::prelude::*;

/// [`Bundle`] for all required [`Component`]s that make up a Minecraft "chunk".
#[derive(Bundle)]
pub struct ChunkBundle {
    pub chunk: ChunkPos,
    pub blocks: Blocks,
    pub lights: Lights,
}

/// A [`Component`] that represents a "chunk" of blocks at the specified chunk position.
#[derive(Component, Deref, DerefMut)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Default)]
pub struct ChunkPos(pub IVec2);

/// A [`Component`] that stores the blocks that make up a [`Chunk`].
#[derive(Component, Debug)]
pub struct Blocks(); // TODO

/// A [`Component`] that stores the light levels that make up a [`Chunk`].
#[derive(Component, Debug)]
pub struct Lights(); // TODO
