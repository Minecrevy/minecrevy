#![allow(non_snake_case)]

use quote::quote_spanned;
use syn::{parse_macro_input, DeriveInput, Ident};

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

#[proc_macro_derive(Packet, attributes(meta))]
pub fn Packet(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ty = &input.ident;

    let mut meta = None;

    for attr in input.attrs {
        let path = attr.path().get_ident().map(|i| i.to_string());
        match path.as_ref().map(|s| s.as_str()) {
            Some("meta") => meta = Some(attr.parse_args::<Ident>().unwrap()),
            _ => { /* Unhandled attribute */ }
        }
    }

    let meta = meta.map(|ident| {
        quote_spanned! { ident.span() =>
            fn meta() -> Option<::minecrevy_io::PacketMeta> where Self: Sized { Some(::minecrevy_io::PacketMeta::#ident) }
        }
    });

    quote::quote! {
        #[automatically_derived]
        impl ::minecrevy_io::Packet for #ty {
            #meta
        }
    }
    .into()
}
