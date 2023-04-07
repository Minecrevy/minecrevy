use darling::{ast::Data, FromDeriveInput, FromField};
use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{Ident, Type};

#[derive(FromDeriveInput)]
#[darling(attributes(extract), supports(struct_named))]
pub struct ExtractStruct {
    ident: Ident,
    data: Data<(), ExtractField>,
    #[darling(default)]
    namespace: Option<String>,
    asset: Type,
}

#[derive(FromField)]
pub struct ExtractField {
    ident: Option<Ident>,
}

pub fn generate_extract_impl(input: ExtractStruct) -> TokenStream {
    let ident = input.ident;
    let namespace = input.namespace.unwrap_or("minecraft".to_owned());
    let asset = input.asset;

    let fields = input.data.take_struct().unwrap().into_iter().map(|field| {
        let ident = field.ident.unwrap();
        let ident_str = ident.to_string();

        quote_spanned! { ident.span() =>
            #ident: index[&minecrevy_core::key::Key::new(#namespace, #ident_str).unwrap()].clone(),
        }
    });

    let generated = quote! {
        impl minecrevy_asset::index::ExtractIndexedAssets<minecrevy_core::key::Key> for #ident {
            type Asset = #asset;

            fn extract(index: &minecrevy_asset::index::AssetIndex<minecrevy_core::key::Key, Self::Asset>) -> Self {
                Self {
                    #(#fields)*
                }
            }
        }
    };

    generated.into()
}
