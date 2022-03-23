use proc_macro::TokenStream;
use syn::parse_macro_input;

mod key;

/// Macro for const-validated `Key`s.
///
/// # Usage
/// ```rust
/// use minecrevy_key::{Key, key};
///
/// pub const STONE: Key = key!("minecraft:stone");
/// ```
#[proc_macro]
pub fn key(input: TokenStream) -> TokenStream {
    let key = parse_macro_input!(input as key::Key);
    key::generate_key(key).into()
}
