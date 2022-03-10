use darling::ast;
use darling::ast::{Data, Style};
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::{Member, spanned::Spanned};

use crate::common::{EnumKind, McIo, McIoField, McIoVariant};

pub fn gen_impl(input: McIo) -> TokenStream {
    let McIo { ref ident, ref generics, data, kind } = input;
    let (imp, ty, where_clause) = generics.split_for_impl();

    let ast = gen_ast(data, kind);
    let mcread = crate::common::get_crate_ident(&Ident::new("McRead", ident.span()));

    quote! {
        #[automatically_derived]
        impl #imp #mcread for #ident #ty #where_clause {
            type Options = ();

            fn read<R: ::std::io::Read>(mut reader: R, _options: Self::Options) -> ::std::io::Result<Self> {
                #ast
            }
        }
    }
}

fn gen_ast(data: ast::Data<McIoVariant, McIoField>, kind: Option<EnumKind>) -> TokenStream {
    match data {
        Data::Enum(variants) => {
            let kind = kind.expect("must specify enum kind");

            let condition = gen_variant_condition(kind);
            let arms = variants.into_iter()
                .enumerate()
                .map(|(idx, variant)| gen_variant_arm(variant, idx, kind));

            quote! {
                match #condition {
                    #(#arms)*
                    v => Err(::std::io::Error::new(::std::io::ErrorKind::InvalidData, format!("invalid discriminator: {}", v))),
                }
            }
        }
        Data::Struct(fields) => {
            let fields = gen_struct(fields.into_iter());
            quote! {
                Ok(Self {
                    #(#fields)*
                })
            }
        }
    }
}

fn gen_variant_arm(variant: McIoVariant, idx: usize, kind: EnumKind) -> TokenStream {
    assert_eq!(Style::Tuple, variant.fields.style, "only tuple variants are allowed");
    assert_eq!(1, variant.fields.len(), "only newtype variants are allowed");

    let ident = variant.ident;
    let field = variant.fields.into_iter().next().unwrap();
    let field = gen_struct_field(field, Member::Unnamed(0.into()));

    let pattern = match kind {
        EnumKind::VarInt => {
            let idx = idx as i32;
            quote! { #idx }
        }
        EnumKind::I8 => {
            let idx = idx as i8;
            quote! { #idx }
        }
        EnumKind::U8 => {
            let idx = idx as u8;
            quote! { #idx }
        }
    };

    quote_spanned! { ident.span() =>
        #pattern => Ok(Self::#ident {
            #field
        }),
    }
}

fn gen_variant_condition(kind: EnumKind) -> TokenStream {
    let mcread = crate::common::get_crate_ident(&Ident::new("McRead", Span::call_site()));

    match kind {
        EnumKind::VarInt => {
            let options = crate::common::get_crate_ident(&Ident::new("IntOptions", Span::call_site()));
            quote! {
                <i32 as #mcread>::read(&mut reader, #options { varint: true })?
            }
        }
        EnumKind::I8 => {
            quote! {
                <i8 as #mcread>::read(&mut reader, ())?
            }
        }
        EnumKind::U8 => {
            quote! {
                <u8 as #mcread>::read(&mut reader, ())?
            }
        }
    }
}

fn gen_struct(fields: impl Iterator<Item=McIoField>) -> impl Iterator<Item=TokenStream> {
    fields
        .enumerate()
        .map(|(idx, field)| {
            (field.ident.as_ref()
                 .map(|id: &Ident| Member::Named(id.clone()))
                 .unwrap_or_else(|| Member::Unnamed(idx.into())),
             field)
        })
        .map(|(member, field)| gen_struct_field(field, member))
}

fn gen_struct_field(field: McIoField, ident: Member) -> TokenStream {
    let ty = &field.ty;
    let mcread = crate::common::get_crate_ident(&Ident::new("McRead", ty.span()));

    let option_fields = field.options.iter()
        .map(|(path, val)| {
            quote_spanned! { ty.span() => options.#(#path).* = #val.into(); }
        });

    let options = if option_fields.len() > 0 {
        quote_spanned! { ty.span() =>
            {
                let mut options = <#ty as #mcread>::Options::default();
                #(#option_fields)*
                options
            }
        }
    } else {
        quote_spanned! { ty.span() =>
            <#ty as #mcread>::Options::default()
        }
    };

    quote_spanned! { ty.span() =>
        #ident: #mcread::read(&mut reader, #options)?,
    }
}
