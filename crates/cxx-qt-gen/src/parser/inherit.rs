// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::parser::{check_safety, extract_common_fields, separate_docs, Invokable, MethodFields};
use crate::{
    naming::Name,
    parser::parameter::ParsedFunctionParameter,
    syntax::{attribute::attribute_take_path, safety::Safety},
};
use quote::format_ident;
use syn::{Attribute, ForeignItemFn, Ident, Result};

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
    pub name: Name,
    /// All the docs (each line) of the inherited method
    pub docs: Vec<Attribute>,
}

impl Invokable for &ParsedInheritedMethod {
    fn name(&self) -> &Name {
        &self.name
    }
}

impl ParsedInheritedMethod {
    pub fn parse(mut method: ForeignItemFn, safety: Safety) -> Result<Self> {
        check_safety(&method, &safety)?;

        let docs = separate_docs(&mut method);
        let invokable_fields = extract_common_fields(&method, docs)?;

        // This block seems unnecessary but since attrs are passed through on generator/rust/inherit.rs a duplicate attr would occur without it
        attribute_take_path(&mut method.attrs, &["cxx_name"]);

        Ok(Self::from_invokable_fields(invokable_fields, method))
    }

    fn from_invokable_fields(fields: MethodFields, method: ForeignItemFn) -> Self {
        Self {
            method,
            qobject_ident: fields.qobject_ident,
            mutable: fields.mutable,
            safe: fields.safe,
            parameters: fields.parameters,
            name: fields.name,
            docs: fields.docs,
        }
    }

    /// the name of the wrapper function in C++
    pub fn wrapper_ident(&self) -> Ident {
        format_ident!("{}CxxQtInherit", self.name.cxx_unqualified())
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
            parsed.name.rust_unqualified().to_string(),
            String::from("test")
        );
        assert_eq!(parsed.name.cxx_unqualified(), String::from("testFunction"));
        assert_eq!(
            parsed.wrapper_ident(),
            format_ident!("testFunctionCxxQtInherit")
        );
        assert!(parsed.mutable);
        assert!(parsed.safe);
    }
}
