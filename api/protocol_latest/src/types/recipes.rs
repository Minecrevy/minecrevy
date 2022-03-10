use minecrevy_io_str::{McRead, McWrite};
use minecrevy_key::Key;

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct InitRecipes {
    pub crafting_book_open: bool,
    pub crafting_book_filter: bool,
    pub smelting_book_open: bool,
    pub smelting_book_filter: bool,
    pub blast_furnace_book_open: bool,
    pub blast_furnace_book_filter: bool,
    pub smoker_book_open: bool,
    pub smoker_book_filter: bool,
    pub displayed_recipes: Vec<Key>,
    pub book_recipes: Vec<Key>,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct ChangeRecipes {
    pub crafting_book_open: bool,
    pub crafting_book_filter: bool,
    pub smelting_book_open: bool,
    pub smelting_book_filter: bool,
    pub blast_furnace_book_open: bool,
    pub blast_furnace_book_filter: bool,
    pub smoker_book_open: bool,
    pub smoker_book_filter: bool,
    pub book_recipes: Vec<Key>,
}
