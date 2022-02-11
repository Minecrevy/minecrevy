#![allow(non_snake_case)]

use darling::FromDeriveInput;
use syn::{DeriveInput, parse_macro_input};
use crate::util::McIo;

mod read;
mod write;
pub(crate) mod util;

#[proc_macro_derive(McRead, attributes(mcio))]
pub fn McRead(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let input = McIo::from_derive_input(&input).unwrap();
    read::gen_impl(input).into()
}

#[proc_macro_derive(McWrite, attributes(mcio))]
pub fn McWrite(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let input = McIo::from_derive_input(&input).unwrap();
    write::gen_impl(input).into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
