#![allow(non_snake_case)]

use syn::{parse_macro_input, DeriveInput};

mod attr;
mod read;
mod util;
mod write;

#[proc_macro_derive(McRead, attributes(io_repr, options, tag))]
pub fn McRead(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    read::generate_read_impl(input).into()
}

#[proc_macro_derive(McWrite, attributes(io_repr, options, tag))]
pub fn McWrite(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    write::generate_write_impl(input).into()
}
