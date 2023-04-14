use std::iter::empty;

use proc_macro2::{Ident, Span, TokenStream};
use proc_macro_crate::{crate_name, FoundCrate};
use quote::{quote, quote_spanned};
use syn::{spanned::Spanned, Field, Fields, Index, Member, Type};

use crate::attr::{McIoOption, McIoOptions};

pub fn generate_options(
    ty: &Type,
    options: Option<&McIoOptions>,
    trait_ty: TokenStream,
) -> TokenStream {
    options
        .map(|options| {
            let options = options.0.iter().map(|McIoOption { path, eq, value }| {
                quote_spanned! { eq.span() => options.#(#path).* = (#value).into(); }
            });

            quote_spanned! { ty.span() =>
                {
                    let mut options = <#ty as #trait_ty>::Options::default();
                    #(#options)*
                    options
                }
            }
        })
        .unwrap_or_else(|| {
            quote_spanned! { ty.span() =>
                <#ty as #trait_ty>::Options::default()
            }
        })
}

pub fn iter_fields(fields: &Fields) -> Box<dyn Iterator<Item = (Member, &Field)> + '_> {
    match fields {
        Fields::Named(fields) => Box::new(
            fields
                .named
                .iter()
                .map(|field| (Member::Named(field.ident.clone().unwrap()), field)),
        ),
        Fields::Unnamed(fields) => {
            Box::new(fields.unnamed.iter().enumerate().map(|(idx, field)| {
                (
                    Member::Unnamed(Index {
                        index: idx as u32,
                        span: field.ty.span(),
                    }),
                    field,
                )
            }))
        }
        Fields::Unit => Box::new(empty()),
    }
}

pub fn crate_path<const N: usize>(crate_: &str, path: [&str; N]) -> TokenStream {
    let found = crate_name(crate_).expect("failed to find crate name");

    let mut crate_path = match found {
        FoundCrate::Itself => quote! { crate },
        FoundCrate::Name(name) => {
            let crate_ = Ident::new(&name, Span::mixed_site());
            quote! { ::#crate_ }
        }
    };

    crate_path.extend(path.into_iter().map(|item| {
        let ident = Ident::new(item, Span::mixed_site());
        quote! { ::#ident }
    }));

    crate_path
}
