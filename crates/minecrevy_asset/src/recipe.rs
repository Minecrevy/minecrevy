use bevy::prelude::*;

use crate::{
    item::{Item, ItemStack},
    tag::Tag,
};

/// An ingredient used in a crafting recipe.
#[derive(Clone, PartialEq, Debug)]
pub struct Ingredient {
    /// The parts of the ingredient.
    pub parts: Vec<IngredientPart>,
}

/// A single part of an [`Ingredient`].
#[derive(Clone, PartialEq, Debug)]
pub enum IngredientPart {
    /// An [`ItemStack`].
    Item(ItemStack),
    /// An [`Item`] [`Tag`].
    Tag(Handle<Tag<Item>>),
}
