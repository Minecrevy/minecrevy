use std::ops::Index;

use bevy::{ecs::system::SystemParam, prelude::*};

use crate::{
    block::{Block, BlockState},
    item::Item,
};

pub mod block;
pub mod item;
pub mod test;

#[derive(SystemParam)]
pub struct GameAssets<'w> {
    pub blocks: Res<'w, Assets<Block>>,
    pub block_states: Res<'w, Assets<BlockState>>,
    pub items: Res<'w, Assets<Item>>,
}

impl Index<&Handle<Block>> for GameAssets<'_> {
    type Output = Block;

    fn index(&self, handle: &Handle<Block>) -> &Self::Output {
        self.blocks.get(handle).unwrap()
    }
}

impl Index<&Handle<BlockState>> for GameAssets<'_> {
    type Output = BlockState;

    fn index(&self, handle: &Handle<BlockState>) -> &Self::Output {
        self.block_states.get(handle).unwrap()
    }
}

impl Index<&Handle<Item>> for GameAssets<'_> {
    type Output = Item;

    fn index(&self, handle: &Handle<Item>) -> &Self::Output {
        self.items.get(handle).unwrap()
    }
}
