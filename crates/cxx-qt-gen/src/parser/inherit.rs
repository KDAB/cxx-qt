// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::parser::attribute::AttributeConstraint;
use crate::parser::{method::MethodFields, CaseConversion};
use core::ops::Deref;
use quote::format_ident;
use std::ops::DerefMut;
use syn::{Attribute, ForeignItemFn, Ident, Result};

/// Describes a method found in an extern "RustQt" with #[inherit]
pub struct ParsedInheritedMethod {
    /// The common fields which are available on all callable types
    pub method_fields: MethodFields,
    /// All the docs (each line) of the inherited method
    pub docs: Vec<Attribute>,
    /// Cfgs for the inherited method
    pub cfgs: Vec<Attribute>,
}

impl ParsedInheritedMethod {
    const ALLOWED_ATTRS: [(AttributeConstraint, &'static str); 6] = [
        (AttributeConstraint::Unique, "cxx_name"),
        (AttributeConstraint::Unique, "rust_name"),
        (AttributeConstraint::Unique, "qinvokable"),
        (AttributeConstraint::Duplicate, "doc"),
        (AttributeConstraint::Unique, "inherit"),
        (AttributeConstraint::Duplicate, "cfg"),
    ];

    pub fn parse(method: ForeignItemFn, auto_case: CaseConversion) -> Result<Self> {
        let method_fields = MethodFields::parse(method, auto_case, &Self::ALLOWED_ATTRS)?;

        let docs = method_fields.attrs.extract_docs();
        let cfgs = method_fields.attrs.extract_cfgs();
        Ok(Self {
            method_fields,
            docs,
            cfgs,
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

impl DerefMut for ParsedInheritedMethod {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.method_fields
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::assert_parse_errors;
    use syn::parse_quote;

    #[test]
    fn test_parser_errors() {
        assert_parse_errors! {
            |item| ParsedInheritedMethod::parse(item, CaseConversion::none()) =>

            // Missing self type
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
            CaseConversion::none()
        )
        .is_ok());
        // T by ref is ok in this shorthand (provided the block has one QObject)
        assert!(ParsedInheritedMethod::parse(
            parse_quote! {
                fn test(&self);
            },
            CaseConversion::none()
        )
        .is_ok());
        // T by Pin
        assert!(ParsedInheritedMethod::parse(
            parse_quote! {
                fn test(self: Pin<&mut T>);
            },
            CaseConversion::none()
        )
        .is_ok());
    }

    #[test]
    fn test_parse_safe() {
        let function: ForeignItemFn = parse_quote! {
            #[cxx_name="testFunction"]
            fn test(self: Pin<&mut T>, a: i32, b: &str);
        };

        let parsed = ParsedInheritedMethod::parse(function, CaseConversion::none()).unwrap();

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
