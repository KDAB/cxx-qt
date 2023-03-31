// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{
    generator::naming::CombinedIdent,
    parser::parameter::ParsedFunctionParameter,
    syntax::{
        attribute::{attribute_find_path, attribute_tokens_to_value},
        foreignmod,
        safety::Safety,
        types,
    },
};
use quote::format_ident;
use syn::{
    parse::{Parse, ParseStream},
    spanned::Spanned,
    Attribute, Error, ForeignItem, ForeignItemFn, Ident, Item, ItemForeignMod, LitStr, Result,
    Token,
};

/// Used when parsing a syn::Item::Verbatim, that we suspect may be a `#[cxx_qt::inherit]` block,
/// but we don't yet know whether this is actually the case.
/// This is the case if `#[cxx_qt::inherit]` is used with `unsafe extern "C++"`.
pub enum MaybeInheritMethods {
    /// We found a `#[cxx_qt::inherit]` block
    Found(InheritMethods),
    /// `#[cxx_qt::inherit]` block not found, pass this Item through to outside code!
    PassThrough(Item),
}

impl Parse for MaybeInheritMethods {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.fork();
        if let Ok(attribute) = lookahead.call(Attribute::parse_outer) {
            if attribute_find_path(attribute.as_slice(), &["cxx_qt", "inherit"]).is_some() {
                input.call(Attribute::parse_outer)?;
                let methods = input.parse::<InheritMethods>()?;
                return Ok(Self::Found(methods));
            }
        }

        Ok(Self::PassThrough(input.parse()?))
    }
}

/// This type is used when parsing the `#[cxx_qt::inherit]` macro contents into raw ForeignItemFn items
pub struct InheritMethods {
    pub safety: Safety,
    pub base_functions: Vec<ForeignItemFn>,
}

impl Parse for InheritMethods {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut base_functions = Vec::new();

        // This looks somewhat counter-intuitive, but if we add `unsafe`
        // to the `extern "C++"` block, the contained functions will be safe to call.
        let safety = if input.peek(Token![unsafe]) {
            Safety::Safe
        } else {
            Safety::Unsafe
        };
        if safety == Safety::Safe {
            input.parse::<Token![unsafe]>()?;
        }

        let extern_block = input.parse::<ItemForeignMod>()?;
        if extern_block.abi.name != Some(LitStr::new("C++", extern_block.abi.span())) {
            return Err(Error::new(
                extern_block.abi.span(),
                "Inherit blocks must be marked with `extern \"C++\"`",
            ));
        }

        for item in extern_block.items {
            match item {
                ForeignItem::Fn(function) => {
                    base_functions.push(function);
                }
                _ => {
                    return Err(Error::new(
                        item.span(),
                        "Only functions are allowed in #[cxx_qt::inherit] blocks",
                    ))
                }
            }
        }

        Ok(InheritMethods {
            safety,
            base_functions,
        })
    }
}

/// Describes a method found in #[cxx_qt::inherit]
pub struct ParsedInheritedMethod {
    /// The original [syn::ForeignItemFn] of the inherited method declaration
    pub method: ForeignItemFn,
    /// The type of the self argument
    pub qobject_ident: Ident,
    /// whether the inherited method is marked as mutable
    pub mutable: bool,
    /// Whether the method is safe to call.
    pub safe: bool,
    /// the parameters of the method, without the `self` argument
    pub parameters: Vec<ParsedFunctionParameter>,
    /// the name of the function in Rust, as well as C++
    pub ident: CombinedIdent,
}

impl ParsedInheritedMethod {
    pub fn parse(method: ForeignItemFn, safety: Safety) -> Result<Self> {
        if safety == Safety::Unsafe && method.sig.unsafety.is_none() {
            return Err(Error::new(
                method.span(),
                "Inherited methods must be marked as unsafe or wrapped in an `unsafe extern \"C++\"` block!",
            ));
        }

        let self_receiver = foreignmod::self_type_from_foreign_fn(&method.sig)?;
        let (qobject_ident, mutability) = types::extract_qobject_ident(&self_receiver.ty)?;
        let mutable = mutability.is_some();

        let parameters = ParsedFunctionParameter::parse_all_ignoring_receiver(&method.sig)?;

        let mut ident = CombinedIdent::from_rust_function(method.sig.ident.clone());
        for attribute in &method.attrs {
            if !attribute.meta.path().is_ident(&format_ident!("cxx_name")) {
                return Err(Error::new(
                    attribute.span(),
                    "Unsupported attribute in #[cxx_qt::inherit]",
                ));
            }

            let name = attribute_tokens_to_value::<LitStr>(attribute)?;

            ident.cpp = format_ident!("{}", name.value());
        }
        let safe = method.sig.unsafety.is_none();

        Ok(Self {
            method,
            qobject_ident,
            mutable,
            parameters,
            ident,
            safe,
        })
    }

    /// the name of the wrapper function in C++
    pub fn wrapper_ident(&self) -> Ident {
        format_ident!("{}CxxQtInherit", self.ident.cpp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::tokens_to_syn;
    use quote::quote;

    #[test]
    fn test_parse_unsafe_mod() {
        let module = quote! {
            extern "C++" {
                unsafe fn test(self: &qobject::T);
            }
        };
        let parsed: InheritMethods = syn::parse2(module).unwrap();
        assert_eq!(parsed.base_functions.len(), 1);
        assert_eq!(parsed.safety, Safety::Unsafe);
    }

    #[test]
    fn test_parse_safe_mod() {
        let module = quote! {
            #[cxx_qt::inherit]
            unsafe extern "C++" {
                fn test(self: &qobject::T);
                unsafe fn test2(self: &qobject::T);
            }
        };
        let parsed: MaybeInheritMethods = syn::parse2(module).unwrap();
        match parsed {
            MaybeInheritMethods::Found(inherit) => {
                assert_eq!(inherit.base_functions.len(), 2);
                assert_eq!(inherit.safety, Safety::Safe);
            }
            MaybeInheritMethods::PassThrough(item) => {
                panic!("Expected InheritMethods, got {item:?}");
            }
        }
    }

    fn assert_parse_error(tokens: proc_macro2::TokenStream) {
        let function: ForeignItemFn = tokens_to_syn(tokens);

        let result = ParsedInheritedMethod::parse(function, Safety::Safe);
        assert!(result.is_err());
    }

    #[test]
    fn test_parser_errors() {
        // Missing self type
        assert_parse_error(quote! {
            fn test(&self);
        });
        // Missing qobject::
        assert_parse_error(quote! {
            fn test(self: &T);
        });
        assert_parse_error(quote! {
            fn test(self: &mut T);
        });
        assert_parse_error(quote! {
            fn test(self: Pin<&mut T>);
        });
        // Pointer types
        assert_parse_error(quote! {
            fn test(self: *const T);
        });
        assert_parse_error(quote! {
            fn test(self: *mut T);
        });
        // Invalid pin usage
        assert_parse_error(quote! {
            fn test(self: Pin<&T>);
        });
        assert_parse_error(quote! {
            fn test(self: &mut T);
        });
        // Attributes
        assert_parse_error(quote! {
            #[myattribute]
            fn test(self: &qobject::T);
        });
        assert_parse_error(quote! {
            fn test(#[test] self: &qobject::T);
        });
        // Missing "unsafe"
        let function: ForeignItemFn = tokens_to_syn(quote! {
            fn test(self: &qobject::T);
        });
        assert!(ParsedInheritedMethod::parse(function, Safety::Unsafe).is_err());
    }

    #[test]
    fn test_parse_safe() {
        let function: ForeignItemFn = tokens_to_syn(quote! {
            #[cxx_name="testFunction"]
            fn test(self: Pin<&mut qobject::T>, a: i32, b: &str);
        });

        let parsed = ParsedInheritedMethod::parse(function, Safety::Safe).unwrap();

        assert_eq!(parsed.qobject_ident, format_ident!("T"));
        assert_eq!(parsed.parameters.len(), 2);
        assert_eq!(parsed.ident.rust, format_ident!("test"));
        assert_eq!(parsed.ident.cpp, format_ident!("testFunction"));
        assert_eq!(
            parsed.wrapper_ident(),
            format_ident!("testFunctionCxxQtInherit")
        );
        assert!(parsed.mutable);
        assert!(parsed.safe);
    }
}
