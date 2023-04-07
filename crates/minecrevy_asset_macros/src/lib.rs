//! Macros for the `minecrevy_asset` crate.

use darling::FromDeriveInput;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

use crate::extract::{generate_extract_impl, ExtractStruct};

mod extract;

/// Implements `ExtractIndexedAssets` for structs with named fields. This is especially useful for
/// generating helper structs to easily fetch Minecraft registry data by field name.
///
/// # Example
///
/// ```
/// # use minecrevy_asset_macros::ExtractIndexedAssets;
/// # use minecrevy_core::key::Key;
/// # use bevy::reflect::TypeUuid;
/// #
/// #[derive(TypeUuid)]
/// #[uuid = "83971243-ad96-444d-b19e-53bc7b4837fc"]
/// pub struct MyAsset {
///     pub key: Key,
/// }
///
/// impl AsRef<Key> for MyAsset {
///     fn as_ref(&self) -> &Key {
///         &self.key
///     }
/// }
///
/// #[derive(ExtractIndexedAssets)]
/// #[extract(asset = "MyAsset")]
/// pub struct MyAssets {
///     pub foo: Handle<MyAsset>,
///     pub bar: Handle<MyAsset>,
///     pub tree: Handle<MyAsset>,
/// }
/// ```
#[proc_macro_derive(ExtractIndexedAssets, attributes(extract))]
#[allow(non_snake_case)]
pub fn ExtractIndexedAssets(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    let extract = ExtractStruct::from_derive_input(&derive_input).unwrap();

    generate_extract_impl(extract)
}
