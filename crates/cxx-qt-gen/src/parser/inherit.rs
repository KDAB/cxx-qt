// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{
    naming::Name,
    parser::parameter::ParsedFunctionParameter,
    syntax::{
        attribute::attribute_take_path, expr::expr_to_string, foreignmod, safety::Safety, types,
    },
};
use quote::format_ident;
use syn::{spanned::Spanned, Attribute, Error, ForeignItemFn, Ident, Result};

/// Describes a method found in an extern "RustQt" with #[inherit]
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
    pub ident: Name,
    /// All the docs (each line) of the inherited method
    pub docs: Vec<Attribute>,
}

impl ParsedInheritedMethod {
    pub fn parse(mut method: ForeignItemFn, safety: Safety) -> Result<Self> {
        if safety == Safety::Unsafe && method.sig.unsafety.is_none() {
            return Err(Error::new(
                method.span(),
                "Inherited methods must be marked as unsafe or wrapped in an `unsafe extern \"RustQt\"` block!",
            ));
        }

        let self_receiver = foreignmod::self_type_from_foreign_fn(&method.sig)?;
        let (qobject_ident, mutability) = types::extract_qobject_ident(&self_receiver.ty)?;
        let mutable = mutability.is_some();

        let parameters = ParsedFunctionParameter::parse_all_ignoring_receiver(&method.sig)?;

        let mut ident =
            Name::from_rust_ident_and_attrs(&method.sig.ident, &method.attrs, None, None)?;

        if let Some(attr) = attribute_take_path(&mut method.attrs, &["cxx_name"]) {
            ident = ident.with_cxx_name(expr_to_string(&attr.meta.require_name_value()?.value)?);
        }

        let mut docs = vec![];
        while let Some(doc) = attribute_take_path(&mut method.attrs, &["doc"]) {
            docs.push(doc);
        }

        let safe = method.sig.unsafety.is_none();

        Ok(Self {
            method,
            qobject_ident,
            mutable,
            parameters,
            ident,
            safe,
            docs,
        })
    }

    /// the name of the wrapper function in C++
    pub fn wrapper_ident(&self) -> Ident {
        format_ident!("{}CxxQtInherit", self.ident.cxx_unqualified())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use syn::parse_quote;

    fn assert_parse_error(function: ForeignItemFn) {
        let result = ParsedInheritedMethod::parse(function, Safety::Safe);
        assert!(result.is_err());
    }

    #[test]
    fn test_parser_errors() {
        // Missing self type
        assert_parse_error(parse_quote! {
            fn test(&self);
        });
        assert_parse_error(parse_quote! {
            fn test(self: &mut T);
        });
        // Pointer types
        assert_parse_error(parse_quote! {
            fn test(self: *const T);
        });
        assert_parse_error(parse_quote! {
            fn test(self: *mut T);
        });
        // Invalid pin usage
        assert_parse_error(parse_quote! {
            fn test(self: Pin<&T>);
        });
        assert_parse_error(parse_quote! {
            fn test(self: &mut T);
        });
        // Attributes
        assert_parse_error(parse_quote! {
            fn test(#[test] self: &T);
        });
        // Missing "unsafe"
        let function: ForeignItemFn = parse_quote! {
            fn test(self: &T);
        };
        assert!(ParsedInheritedMethod::parse(function, Safety::Unsafe).is_err());
    }

    #[test]
    fn test_parse_ok() {
        // T by ref
        assert!(ParsedInheritedMethod::parse(
            parse_quote! {
                fn test(self: &T);
            },
            Safety::Safe
        )
        .is_ok());
        // T by Pin
        assert!(ParsedInheritedMethod::parse(
            parse_quote! {
                fn test(self: Pin<&mut T>);
            },
            Safety::Safe
        )
        .is_ok());
    }

    #[test]
    fn test_parse_safe() {
        let function: ForeignItemFn = parse_quote! {
            #[cxx_name="testFunction"]
            fn test(self: Pin<&mut T>, a: i32, b: &str);
        };

        let parsed = ParsedInheritedMethod::parse(function, Safety::Safe).unwrap();

        assert_eq!(parsed.qobject_ident, format_ident!("T"));
        assert_eq!(parsed.parameters.len(), 2);
        assert_eq!(
            parsed.ident.rust_unqualified().to_string(),
            String::from("test")
        );
        assert_eq!(parsed.ident.cxx_unqualified(), String::from("testFunction"));
        assert_eq!(
            parsed.wrapper_ident(),
            format_ident!("testFunctionCxxQtInherit")
        );
        assert!(parsed.mutable);
        assert!(parsed.safe);
    }
}
