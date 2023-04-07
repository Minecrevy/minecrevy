use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned,
    Attribute, Expr, Path, Token,
};

use crate::util::{crate_ident, crate_path};

pub struct McIoAttrs {
    pub repr: Option<McIoEnum>,
    pub tag: Option<McIoTag>,
    pub options: Option<McIoOptions>,
}

impl McIoAttrs {
    pub fn parse<'a>(attrs: impl IntoIterator<Item = &'a Attribute>) -> syn::Result<Self> {
        let mut repr = None;
        let mut tag = None;
        let mut options = None;

        for attr in attrs {
            let path = attr.path.get_ident().map(|i| i.to_string());
            match path.as_ref().map(|s| s.as_str()) {
                Some("io_repr") => repr = Some(attr.parse_args::<McIoEnum>()?),
                Some("tag") => tag = Some(attr.parse_args::<McIoTag>()?),
                Some("options") => {
                    // filter: we don't care when no options are specified
                    options = Some(attr.parse_args::<McIoOptions>()?)
                        .filter(|options| !options.0.is_empty());
                }
                _ => { /* Unhandled attribute */ }
            }
        }

        Ok(Self { repr, tag, options })
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum McIoEnum {
    VarInt,
    I8,
    U8,
}

impl McIoEnum {
    pub fn as_condition(&self) -> TokenStream {
        let mcread = crate_ident("minecrevy_io", "McRead");

        match self {
            McIoEnum::VarInt => {
                let options = crate_path("minecrevy_io", ["options", "IntOptions"]);
                quote! {
                    <i32 as #mcread>::read(&mut reader, #options { varint: true })?
                }
            }
            McIoEnum::I8 => {
                quote! {
                    <i8 as #mcread>::read(&mut reader, ())?
                }
            }
            McIoEnum::U8 => {
                quote! {
                    <u8 as #mcread>::read(&mut reader, ())?
                }
            }
        }
    }

    pub fn as_prefix(&self, value: TokenStream) -> TokenStream {
        let mcwrite = crate_ident("minecrevy_io", "McWrite");

        match self {
            McIoEnum::VarInt => {
                let options = crate_path("minecrevy_io", ["options", "IntOptions"]);
                quote! {
                    <i32 as #mcwrite>::write(&#value, &mut writer, #options { varint: true })?;
                }
            }
            McIoEnum::I8 => {
                quote! {
                    <i8 as #mcwrite>::write(&#value, &mut writer, ())?;
                }
            }
            McIoEnum::U8 => {
                quote! {
                    <u8 as #mcwrite>::write(&#value, &mut writer, ())?;
                }
            }
        }
    }
}

impl Parse for McIoEnum {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let kind = input.parse::<Path>()?;

        match kind
            .get_ident()
            .map(|v| v.to_string())
            .as_ref()
            .map(|v| v.as_str())
        {
            Some("varint") => Ok(Self::VarInt),
            Some("i8") => Ok(Self::I8),
            Some("u8") => Ok(Self::U8),
            _ => Err(syn::Error::new(
                kind.span(),
                format!("invalid enum representation: {}", quote! { #kind }),
            )),
        }
    }
}

pub struct McIoTag {
    pub value: Expr,
}

impl Parse for McIoTag {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            value: input.parse::<Expr>()?,
        })
    }
}

pub struct McIoOptions(pub Vec<McIoOption>);

impl Parse for McIoOptions {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let values = Punctuated::<McIoOption, Token![,]>::parse_terminated(input)?
            .into_iter()
            .collect();
        Ok(McIoOptions(values))
    }
}

/// A single entry in a `#[options(x = 1, a.x = "hello")]` attribute, such as `x = 1` or `a.x = "hello"`.
pub struct McIoOption {
    /// Not a real [`Path`] (those use `::`, but we use `.` instead).
    pub path: Vec<Ident>,
    pub eq: Token![=],
    pub value: Expr,
}

impl Parse for McIoOption {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let path = Punctuated::<Ident, Token![.]>::parse_separated_nonempty(input)?
            .into_iter()
            .collect::<Vec<_>>();
        let eq = input.parse::<Token![=]>()?;
        let value = input.parse::<Expr>()?;
        Ok(Self { path, eq, value })
    }
}
