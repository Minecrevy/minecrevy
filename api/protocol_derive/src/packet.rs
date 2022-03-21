use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn generate_packet_impl(input: DeriveInput) -> TokenStream {
    let ident = &input.ident;
    let (gen_impl, gen_ty, gen_where) = input.generics.split_for_impl();

    quote! {
        #[automatically_derived]
        impl #gen_impl ::minecrevy_protocol::Packet for #ident #gen_ty #gen_where {}
    }
}
