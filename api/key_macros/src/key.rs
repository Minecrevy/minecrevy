use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{LitStr, Token};
use syn::parse::{Parse, ParseStream};

pub fn generate_key(key: Key) -> TokenStream {
    key.validate_and_split()
        .map(|(namespace, path)| generate_impl(key.is_ref, &namespace, &path))
        .unwrap_or_else(|e| e.into_compile_error())
}

fn generate_impl(is_ref: bool, namespace: &str, path: &str) -> TokenStream {
    if is_ref {
        quote! {
            unsafe { ::minecrevy_key::KeyRef::static_unchecked(#namespace, #path) }
        }
    } else {
        quote! {
            unsafe { ::minecrevy_key::Key::static_unchecked(#namespace, #path) }
        }
    }
}

pub struct Key {
    is_ref: bool,
    formatted: LitStr,
}

impl Parse for Key {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            is_ref: if input.peek(Token![ref]) { input.parse::<Token![ref]>().is_ok() } else { false },
            formatted: input.parse()?,
        })
    }
}

impl Key {
    fn validate_and_split(&self) -> syn::Result<(String, String)> {
        let formatted = self.formatted.value();

        let (namespace, path) = if let Some((namespace, path)) = formatted.split_once(':') {
            (namespace.to_owned(), path.to_owned())
        } else {
            ("minecraft".to_owned(), formatted)
        };

        Ok((Self::validate_namespace(namespace)?, Self::validate_path(path)?))
    }

    fn validate_namespace(namespace: String) -> syn::Result<String> {
        fn is_namespace_char(c: char) -> bool {
            matches!(c, 'a'..='z' | '0'..='9' | '_' | '.' | '-')
        }

        if namespace.is_empty() {
            Ok("minecraft".to_owned())
        } else if namespace.chars().all(is_namespace_char) {
            Ok(namespace)
        } else {
            Err(syn::Error::new(Span::call_site(), format!("non [a-z0-9_.-] character in namespace: {}", namespace)))
        }
    }

    fn validate_path(path: String) -> syn::Result<String> {
        fn is_path_char(c: char) -> bool {
            matches!(c, 'a'..='z' | '0'..='9' | '_' | '.' | '-' | '/')
        }

        if path.chars().all(is_path_char) {
            Ok(path)
        } else {
            Err(syn::Error::new(Span::call_site(), format!("non [a-z0-9_.-/] character in path: {}", path)))
        }
    }
}
