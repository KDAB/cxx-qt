// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{
    generator::naming::CombinedIdent,
    parser::parameter::ParsedFunctionParameter,
    syntax::{
        attribute::attribute_find_path, expr::expr_to_string, foreignmod, safety::Safety, types,
    },
};
use quote::format_ident;
use syn::{spanned::Spanned, Error, ForeignItemFn, Ident, Result};

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
    pub ident: CombinedIdent,
}

impl ParsedInheritedMethod {
    pub fn parse(mut method: ForeignItemFn, safety: Safety) -> Result<Self> {
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

        if let Some(index) = attribute_find_path(&method.attrs, &["cxx_name"]) {
            ident.cpp = format_ident!(
                "{}",
                expr_to_string(&method.attrs[index].meta.require_name_value()?.value)?
            );

            method.attrs.remove(index);
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
        // Missing qobject::
        assert_parse_error(parse_quote! {
            fn test(self: &T);
        });
        assert_parse_error(parse_quote! {
            fn test(self: &mut T);
        });
        assert_parse_error(parse_quote! {
            fn test(self: Pin<&mut T>);
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
            fn test(#[test] self: &qobject::T);
        });
        // Missing "unsafe"
        let function: ForeignItemFn = parse_quote! {
            fn test(self: &qobject::T);
        };
        assert!(ParsedInheritedMethod::parse(function, Safety::Unsafe).is_err());
    }

    #[test]
    fn test_parse_safe() {
        let function: ForeignItemFn = parse_quote! {
            #[cxx_name="testFunction"]
            fn test(self: Pin<&mut qobject::T>, a: i32, b: &str);
        };

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
