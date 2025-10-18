#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Error, Ident, Type, parse_macro_input, spanned::Spanned};

/// # Example
/// ```
/// use borrow_key::BorrowKey;
/// use core::hash::Hash;
///
/// #[derive(Debug, BorrowKey)]
/// struct Foo<T>
/// where
///     T: PartialEq + Eq + PartialOrd + Ord + Hash
/// {
///     #[key]
///     key: T,
///     value: u8
/// }
/// ```
#[proc_macro_derive(BorrowKey, attributes(key))]
pub fn derive_borrow_key(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let ident = input.ident;

    let mut key_ident = None::<Ident>;
    let mut key_type = None::<Type>;

    match input.data {
        Data::Struct(data_struct) => {
            for field in data_struct.fields {
                if let Some(attr) = field.attrs.iter().find(|a| a.meta.path().is_ident("key")) {
                    if key_ident.is_some() || key_type.is_some() {
                        return Error::new(attr.span(), "expect exact 1 key to be specified")
                            .to_compile_error()
                            .into();
                    }
                    key_ident = field.ident;
                    key_type = match attr.parse_args() {
                        Ok(r#type) => Some(r#type),
                        Err(_) => Some(field.ty),
                    }
                }
            }
        }
        Data::Enum(data_enum) => {
            return Error::new(data_enum.enum_token.span(), "enum type is not supported")
                .to_compile_error()
                .into();
        }
        Data::Union(data_union) => {
            return Error::new(data_union.union_token.span(), "union type is not supported")
                .to_compile_error()
                .into();
        }
    }

    if key_ident.is_none() || key_type.is_none() {
        return Error::new(
            ident.span(),
            "expect exact 1 key to be specified with #[key($type?: ty)]",
        )
        .to_compile_error()
        .into();
    }

    let expanded = quote! {
        impl #impl_generics ::core::borrow::Borrow<#key_type> for #ident #ty_generics #where_clause {
            fn borrow(&self) -> &#key_type {
                &self.#key_ident
            }
        }

        impl #impl_generics ::core::hash::Hash for #ident #ty_generics #where_clause {
            fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
                self.#key_ident.hash(state);
            }
        }

        impl #impl_generics ::core::cmp::PartialEq for #ident #ty_generics #where_clause {
            fn eq(&self, other: &Self) -> bool {
                self.key == other.key
            }
        }

        impl #impl_generics ::core::cmp::Eq for #ident #ty_generics #where_clause { }

        impl #impl_generics ::core::cmp::PartialOrd for #ident #ty_generics #where_clause {
            fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        impl #impl_generics ::core::cmp::Ord for #ident #ty_generics #where_clause {
            fn cmp(&self, other: &Self) -> ::core::cmp::Ordering {
                self.key.cmp(&other.key)
            }
        }

    };

    TokenStream::from(expanded)
}
