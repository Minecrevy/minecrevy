use darling::ast;
use darling::ast::{Data, Style};
use proc_macro2::{Ident, TokenStream};
use quote::{quote, quote_spanned};
use syn::Member;
use syn::spanned::Spanned;

use crate::common::{EnumKind, McIo, McIoField, McIoVariant};

pub fn gen_impl(input: McIo) -> TokenStream {
    let McIo { ref ident, ref generics, data, kind } = input;
    let (imp, ty, where_clause) = generics.split_for_impl();

    let ast = gen_ast(data, kind);
    let mcwrite = crate::common::get_crate_ident(&Ident::new("McWrite", ident.span()));

    quote! {
        #[automatically_derived]
        impl #imp #mcwrite for #ident #ty #where_clause {
            type Options = ();

            fn write<W: ::std::io::Write>(&self, mut writer: W, _options: Self::Options) -> ::std::io::Result<()> {
                #ast
                Ok(())
            }
        }
    }
}

fn gen_ast(data: ast::Data<McIoVariant, McIoField>, kind: Option<EnumKind>) -> TokenStream {
    match data {
        Data::Enum(variants) => {
            let kind = kind.expect("must specify enum kind");

            let arms = variants.into_iter()
                .enumerate()
                .map(|(idx, variant)| gen_variant_arm(variant, idx, kind));

            quote! {
                match self {
                    #(#arms)*
                }
            }
        }
        Data::Struct(fields) => {
            let fields = gen_struct(fields.into_iter());
            quote! {
                #(#fields)*
            }
        }
    }
}

fn gen_variant_arm(variant: McIoVariant, idx: usize, kind: EnumKind) -> TokenStream {
    assert_eq!(Style::Tuple, variant.fields.style, "only tuple variants are allowed");
    assert_eq!(1, variant.fields.len(), "only newtype variants are allowed");

    let ident = variant.ident;
    let field = variant.fields.into_iter().next().unwrap();

    let prefix = gen_variant_prefix(idx, &ident, kind);
    let field = gen_struct_field(field, Member::Named(Ident::new("v", ident.span())), None);

    quote_spanned! { ident.span() =>
        Self::#ident(v) => {
            #prefix
            #field
        },
    }
}

fn gen_variant_prefix(idx: usize, ident: &Ident, kind: EnumKind) -> TokenStream {
    let mcwrite = crate::common::get_crate_ident(&Ident::new("McWrite", ident.span()));

    match kind {
        EnumKind::VarInt => {
            let options = crate::common::get_crate_ident(&Ident::new("IntOptions", ident.span()));
            let idx = idx as i32;
            quote_spanned! { ident.span() =>
                <i32 as #mcwrite>::write(&#idx, &mut writer, #options { varint: true })?;
            }
        }
        EnumKind::I8 => {
            let idx = idx as i8;
            quote_spanned! { ident.span() =>
                <i8 as #mcwrite>::write(&#idx, &mut writer, ())?;
            }
        }
        EnumKind::U8 => {
            let idx = idx as u8;
            quote_spanned! { ident.span() =>
                <u8 as #mcwrite>::write(&#idx, &mut writer, ())?;
            }
        }
    }
}

fn gen_struct(fields: impl Iterator<Item=McIoField>) -> impl Iterator<Item=TokenStream> {
    fields
        .enumerate()
        .map(|(idx, field)| {
            (field.ident.clone()
                 .map(|id: Ident| Member::Named(id))
                 .unwrap_or_else(|| Member::Unnamed(idx.into())),
             field)
        })
        .map(|(member, field)| gen_struct_field(field, member, Some(quote! { self. })))
}

fn gen_struct_field(field: McIoField, ident: Member, prefix: Option<TokenStream>) -> TokenStream {
    let ty = field.ty;
    let mcwrite = crate::common::get_crate_ident(&Ident::new("McWrite", ty.span()));

    let option_fields = field.options.iter()
        .map(|(path, val)| {
            quote_spanned! { ty.span() => options.#(#path).* = #val.into(); }
        });

    let options = if option_fields.len() > 0 {
        quote_spanned! { ty.span() =>
            {
                let mut options = <#ty as #mcwrite>::Options::default();
                #(#option_fields)*
                options
            }
        }
    } else {
        quote_spanned! { ty.span() =>
            <#ty as #mcwrite>::Options::default()
        }
    };

    quote_spanned! { ty.span() =>
        <#ty as #mcwrite>::write(&#prefix #ident, &mut writer, #options)?;
    }
}
