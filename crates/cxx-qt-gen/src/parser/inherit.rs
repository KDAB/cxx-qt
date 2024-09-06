// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::parser::method::MethodFields;
use crate::parser::{check_safety, has_invalid_attrs, separate_docs};
use crate::syntax::{attribute::attribute_take_path, safety::Safety};
use core::ops::Deref;
use quote::format_ident;
use syn::spanned::Spanned;
use syn::{Attribute, Error, ForeignItemFn, Ident, Result};

/// Describes a method found in an extern "RustQt" with #[inherit]
pub struct ParsedInheritedMethod {
    /// The common fields which are available on all callable types
    pub method_fields: MethodFields,
    /// All the docs (each line) of the inherited method
    pub docs: Vec<Attribute>,
}

impl ParsedInheritedMethod {
    const ALLOWED_ATTRS: [&'static str; 4] = ["cxx_name", "rust_name", "qinvokable", "doc"];

    pub fn parse(mut method: ForeignItemFn, safety: Safety) -> Result<Self> {
        check_safety(&method, &safety)?;

        if has_invalid_attrs(&method.attrs, &Self::ALLOWED_ATTRS) {
            return Err(Error::new(
                method.span(),
                "Only cxx_name and rust_name are allowed on inherited methods!",
            ));
        }

        let docs = separate_docs(&mut method);
        let mut fields = MethodFields::parse(method)?;

        // This block seems unnecessary but since attrs are passed through on generator/rust/inherit.rs a duplicate attr would occur without it
        attribute_take_path(&mut fields.method.attrs, &["cxx_name"]);

        Ok(Self {
            method_fields: fields,
            docs,
        })
    }

    /// the name of the wrapper function in C++
    pub fn wrapper_ident(&self) -> Ident {
        format_ident!("{}CxxQtInherit", self.name.cxx_unqualified())
    }
}

impl Deref for ParsedInheritedMethod {
    type Target = MethodFields;

    fn deref(&self) -> &Self::Target {
        &self.method_fields
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::assert_parse_errors;
    use syn::parse_quote;

    #[test]
    fn test_parser_errors() {
        // Missing "unsafe"
        let function: ForeignItemFn = parse_quote! {
            fn test(self: &T);
        };
        assert!(ParsedInheritedMethod::parse(function, Safety::Unsafe).is_err());

        assert_parse_errors! {
            |item| ParsedInheritedMethod::parse(item, Safety::Safe) =>

            // Missing self type
            { fn test(&self); }
            { fn test(self: &mut T); }
            // Pointer types
            { fn test(self: *const T); }
            { fn test(self: *mut T); }
            // Invalid pin usage
            { fn test(self: Pin<&T>); }
            { fn test(self: &mut T); }
            // Attributes
            { fn test(#[test] self: &T); }
        }
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
