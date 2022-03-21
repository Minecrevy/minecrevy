#![allow(non_snake_case)]

use syn::{DeriveInput, parse_macro_input};

mod packet;

#[proc_macro_derive(Packet)]
pub fn Packet(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    packet::generate_packet_impl(input).into()
}
