use proc_macro2::{Literal, TokenStream};
use quote::{format_ident, quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{Attribute, Data, DataEnum, DataStruct, DeriveInput, Field, Member, Variant};

use crate::attr::{McIoAttrs, McIoEnum, McIoTag};
use crate::util::{crate_ident, generate_options, iter_fields};

pub fn generate_write_impl(input: DeriveInput) -> TokenStream {
    let ident = &input.ident;
    let (gen_impl, gen_ty, gen_where) = input.generics.split_for_impl();

    let mcwrite = crate_ident("minecrevy_io", "McWrite");
    let ast = generate_ast(&input.attrs, &input.data);

    quote! {
        #[automatically_derived]
        impl #gen_impl #mcwrite for #ident #gen_ty #gen_where {
            type Options = ();

            fn write<W: ::std::io::Write>(&self, mut writer: W, (): Self::Options) -> ::std::io::Result<()> {
                #ast
                Ok(())
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
    let fields = iter_fields(&data.fields)
        .map(|(ident, field)| generate_field(field, ident, Some(quote! { self. })));

    quote! {
        #(#fields)*
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
            return syn::Error::new(data.enum_token.span(), "must specify io_repr")
                .into_compile_error()
        }
    };

    let variants = data
        .variants
        .iter()
        .enumerate()
        .map(|(idx, variant)| generate_variant(repr, variant, idx));

    quote! {
        match self {
            #(#variants)*
        }
    }
}

fn generate_variant(repr: McIoEnum, variant: &Variant, idx: usize) -> TokenStream {
    let attrs = match McIoAttrs::parse(&variant.attrs) {
        Ok(attrs) => attrs,
        Err(e) => return e.into_compile_error(),
    };

    let field_names = iter_fields(&variant.fields).map(|(ident, _)| match ident {
        Member::Named(ident) => quote! { #ident },
        Member::Unnamed(idx) => {
            let ident = format_ident!("_{}", idx);
            quote! { #idx: #ident }
        }
    });

    let ident = &variant.ident;
    let pattern = quote! {
        Self::#ident { #(#field_names),* }
    };

    let discriminant = attrs
        .tag
        .map(|McIoTag { value }| quote_spanned! { value.span() => #value })
        .or_else(|| {
            variant
                .discriminant
                .as_ref()
                .map(|(_, expr)| quote_spanned! { expr.span() => #expr })
        })
        .unwrap_or_else(|| {
            let literal = Literal::usize_unsuffixed(idx);
            quote_spanned! { variant.ident.span() => #literal }
        });
    let prefix = repr.as_prefix(discriminant);

    let fields = iter_fields(&variant.fields).map(|(ident, field)| {
        let ident = match ident {
            Member::Named(ident) => Member::Named(ident),
            Member::Unnamed(idx) => Member::Named(format_ident!("_{}", idx)),
        };
        generate_field(field, ident, None)
    });

    quote_spanned! { ident.span() =>
        #pattern => {
            #prefix
            #(#fields)*
        }
    }
}

fn generate_field(field: &Field, ident: Member, this: Option<TokenStream>) -> TokenStream {
    let attrs = match McIoAttrs::parse(&field.attrs) {
        Ok(attrs) => attrs,
        Err(e) => return e.into_compile_error(),
    };

    let ty = &field.ty;
    let mcwrite = crate_ident("minecrevy_io", "McWrite");

    let options = generate_options(ty, attrs.options.as_ref(), quote! { #mcwrite });

    quote_spanned! { ty.span() =>
        <#ty as #mcwrite>::write(&#this #ident, &mut writer, #options)?;
    }
}
