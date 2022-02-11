use darling::ast;
use proc_macro2::{Ident, TokenStream};
use quote::{quote, quote_spanned};
use syn::Member;
use syn::spanned::Spanned;

use crate::util::{McIo, McIoField};

pub fn gen_impl(input: McIo) -> TokenStream {
    let McIo { ref ident, ref generics, ref data } = input;
    let (imp, ty, where_clause) = generics.split_for_impl();

    let fields = gen_fields(data);
    let mcwrite = crate::util::get_crate_ident(&Ident::new("McWrite", ident.span()));

    quote! {
        #[automatically_derived]
        impl #imp #mcwrite for #ident #ty #where_clause {
            type Options = ();

            fn write<W: ::std::io::Write>(&self, mut writer: W, _options: Self::Options) -> ::std::io::Result<()> {
                #(#fields)*
                Ok(())
            }
        }
    }
}

fn gen_fields(data: &ast::Data<(), McIoField>) -> impl Iterator<Item=TokenStream> + '_ {
    data.as_ref()
        .take_struct()
        .expect("enum unsupported")
        .fields
        .into_iter()
        .enumerate()
        .map(|(idx, field)| {
            (field, field.ident.as_ref()
                .map(|id: &Ident| Member::Named(id.clone()))
                .unwrap_or_else(|| Member::Unnamed(idx.into())))
        })
        .map(|(field, member)| gen_field(field, member))
}

fn gen_field(field: &McIoField, ident: Member) -> TokenStream {
    let ty = &field.ty;
    let mcwrite = crate::util::get_crate_ident(&Ident::new("McWrite", ty.span()));

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
        <#ty as #mcwrite>::write(&self.#ident, &mut writer, #options)?;
    }
}
