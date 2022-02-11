use std::collections::HashMap;

use darling::{ast, FromDeriveInput, FromField};
use proc_macro2::{Ident, Span, TokenStream};
use proc_macro_crate::{crate_name, FoundCrate};
use quote::quote;
use syn::{Field, Lit, LitBool, Meta, NestedMeta};
use syn::spanned::Spanned;

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(mcio), supports(struct_any))]
pub struct McIo {
    pub ident: syn::Ident,
    pub generics: syn::Generics,
    pub data: ast::Data::<(), McIoField>,
}

#[derive(Debug)]
pub struct McIoField {
    pub ident: Option<syn::Ident>,
    pub ty: syn::Type,
    pub options: HashMap<Vec<Ident>, NestedMeta>,
}

impl FromField for McIoField {
    fn from_field(field: &Field) -> darling::Result<Self> {
        let mut options = HashMap::new();

        // we only care about the 'mcio' attr
        let attr = field.attrs.iter()
            .find(|attr| attr.path.is_ident("mcio"));
        // parse the attribute for options if it exists
        if let Some(attr) = attr {
            Self::add_options(attr.parse_meta()?, &mut options, Vec::new());
        }

        Ok(Self {
            ident: field.ident.clone(),
            ty: field.ty.clone(),
            options,
        })
    }
}

impl McIoField {
    fn add_options(meta: Meta, options: &mut HashMap<Vec<Ident>, NestedMeta>, mut field_path: Vec<Ident>) {
        match meta {
            Meta::Path(path) => {
                // If we have #[mcio(field)] rather than #[mcio(field = "something")]
                // then default to it acting as #[mcio(field = true)]
                let value = NestedMeta::Lit(Lit::Bool(LitBool::new(true, path.span())));
                // Append the new field name to the path
                field_path.extend(path_to_idents(path));

                options.insert(field_path, value);
            }
            Meta::List(list) => {
                // Append the list's outer name to the path, like 'outer' in #[mcio(outer(field = true))]
                // but only when we aren't the most outer list
                if !list.path.is_ident("mcio") {
                    field_path.extend(path_to_idents(list.path));
                }
                // Recursively add options for nested fields
                for meta in list.nested {
                    match meta {
                        NestedMeta::Meta(meta) => {
                            Self::add_options(meta, options, field_path.clone());
                        }
                        NestedMeta::Lit(_lit) => unimplemented!("dont currently support #[mcio(\"some_text\")]"),
                    }
                }
            }
            Meta::NameValue(nv) => {
                // Append the new field name to the path
                field_path.extend(path_to_idents(nv.path));
                // Simply wrap our field value to pass it along
                let value = NestedMeta::Lit(nv.lit);

                options.insert(field_path, value);
            }
        }
    }
}

fn path_to_idents(path: syn::Path) -> impl Iterator<Item=Ident> {
    path.segments.into_iter()
        .map(|seg| seg.ident)
}

pub fn get_crate_ident(sub: &Ident) -> TokenStream {
    let found: FoundCrate = crate_name("minecrevy_io_str").expect("failed to find any crate name");

    match found {
        FoundCrate::Itself => quote! { crate::#sub },
        FoundCrate::Name(name) => {
            let ident = Ident::new(&name, Span::call_site());
            quote! { ::#ident::#sub }
        }
    }
}
