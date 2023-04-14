use proc_macro2::{Literal, TokenStream};
use quote::{quote, quote_spanned};
use syn::{
    spanned::Spanned, Attribute, Data, DataEnum, DataStruct, DeriveInput, Field, Member, Variant,
};

use crate::{
    attr::{McIoAttrs, McIoTag},
    util::{crate_path, generate_options, iter_fields},
};

pub fn generate_read_impl(input: DeriveInput) -> TokenStream {
    let ident = &input.ident;
    let (gen_impl, gen_ty, gen_where) = input.generics.split_for_impl();

    let mcread = crate_path("minecrevy_io", ["McRead"]);
    let version = crate_path("minecrevy_io", ["ProtocolVersion"]);
    let ast = generate_ast(&input.attrs, &input.data);

    quote! {
        #[automatically_derived]
        impl #gen_impl #mcread for #ident #gen_ty #gen_where {
            type Options = ();

            fn read<R: ::std::io::Read>(mut reader: R, (): Self::Options, version: #version) -> ::std::io::Result<Self> {
                #ast
            }
        }
    }
}

fn generate_ast(attrs: &Vec<Attribute>, data: &Data) -> TokenStream {
    match data {
        Data::Struct(data) => generate_struct(data),
        Data::Enum(data) => generate_enum(attrs, data),
        Data::Union(_) => panic!("cannot derive unions"),
    }
}

fn generate_struct(data: &DataStruct) -> TokenStream {
    let fields = iter_fields(&data.fields).map(|(ident, field)| generate_field(field, ident));

    quote! {
        Ok(Self {
            #(#fields)*
        })
    }
}

fn generate_enum(attrs: &Vec<Attribute>, data: &DataEnum) -> TokenStream {
    let attrs = match McIoAttrs::parse(attrs) {
        Ok(attrs) => attrs,
        Err(e) => return e.into_compile_error(),
    };

    let repr = match attrs.repr {
        Some(repr) => repr,
        None => {
            return syn::Error::new(
                data.enum_token.span(),
                "must specify #[io_repr(<type>)] for enums",
            )
            .into_compile_error()
        }
    };

    let condition = repr.as_condition();

    let variants = data
        .variants
        .iter()
        .enumerate()
        .map(|(idx, variant)| generate_variant(variant, idx));

    quote! {
        match #condition {
            #(#variants)*
            v => Err(::std::io::Error::new(::std::io::ErrorKind::InvalidData, format!("invalid tag: {}", v))),
        }
    }
}

fn generate_variant(variant: &Variant, idx: usize) -> TokenStream {
    let attrs = match McIoAttrs::parse(&variant.attrs) {
        Ok(attrs) => attrs,
        Err(e) => return e.into_compile_error(),
    };

    let pattern = attrs
        .tag
        .map(|McIoTag { value }| quote_spanned! { value.span() => #value })
        .or_else(|| {
            variant
                .discriminant
                .as_ref()
                .map(|(_, value)| quote_spanned! { value.span() => #value })
        })
        .unwrap_or_else(|| {
            let literal = Literal::usize_unsuffixed(idx);
            quote_spanned! { variant.ident.span() => #literal }
        });

    let ident = &variant.ident;
    let fields = iter_fields(&variant.fields).map(|(ident, field)| generate_field(field, ident));

    quote_spanned! { variant.ident.span() =>
        #pattern => Ok(Self::#ident {
            #(#fields)*
        }),
    }
}

fn generate_field(field: &Field, ident: Member) -> TokenStream {
    let attrs = match McIoAttrs::parse(&field.attrs) {
        Ok(attrs) => attrs,
        Err(e) => return e.into_compile_error(),
    };

    let ty = &field.ty;
    let mcread = crate_path("minecrevy_io", ["McRead"]);

    let options = generate_options(ty, attrs.options.as_ref(), quote! { #mcread });

    quote_spanned! { ty.span() =>
        #ident: #mcread::read(&mut reader, #options, version)?,
    }
}
