// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{
    generator::{
        naming::method::QMethodName,
        rust::{fragment::RustFragmentPair, qobject::GeneratedRustQObject},
    },
    parser::method::ParsedMethod,
    syntax::attribute::attribute_take_path,
};
use quote::quote;
use syn::Result;

pub fn generate_rust_methods(invokables: &Vec<ParsedMethod>) -> Result<GeneratedRustQObject> {
    let mut generated = GeneratedRustQObject::default();

    for invokable in invokables {
        let idents = QMethodName::from(invokable);
        let wrapper_ident_cpp_str = idents.wrapper.cpp.to_string();

        // Remove any cxx_name attribute on the original method
        // As we need it to use the wrapper ident
        let original_method = {
            let mut original_method = invokable.method.clone();
            attribute_take_path(&mut original_method.attrs, &["cxx_name"]);
            original_method
        };

        let fragment = RustFragmentPair {
            cxx_bridge: vec![quote! {
                // Note: extern "Rust" block does not need to be unsafe
                extern "Rust" {
                    // Note that we are exposing a Rust method on the C++ type to C++
                    //
                    // CXX ends up generating the source, then we generate the matching header.
                    #[doc(hidden)]
                    #[cxx_name = #wrapper_ident_cpp_str]
                    #original_method
                }
            }],
            implementation: vec![],
        };

        generated
            .cxx_mod_contents
            .append(&mut fragment.cxx_bridge_as_items()?);
        generated
            .cxx_qt_mod_contents
            .append(&mut fragment.implementation_as_items()?);
    }

    Ok(generated)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::parser::parameter::ParsedFunctionParameter;
    use crate::tests::assert_tokens_eq;
    use quote::format_ident;
    use std::collections::HashSet;
    use syn::parse_quote;

    #[test]
    fn test_generate_rust_invokables() {
        let invokables = vec![
            ParsedMethod {
                method: parse_quote! {
                    #[cxx_name = "voidInvokable"]
                    fn void_invokable(self: &MyObject);
                },
                qobject_ident: format_ident!("MyObject"),
                mutable: false,
                safe: true,
                parameters: vec![],
                specifiers: HashSet::new(),
                is_qinvokable: true,
            },
            ParsedMethod {
                method: parse_quote! { fn trivial_invokable(self: &MyObject, param: i32) -> i32; },
                qobject_ident: format_ident!("MyObject"),
                mutable: false,
                safe: true,
                parameters: vec![ParsedFunctionParameter {
                    ident: format_ident!("param"),
                    ty: parse_quote! { i32 },
                }],
                specifiers: HashSet::new(),
                is_qinvokable: true,
            },
            ParsedMethod {
                method: parse_quote! { fn opaque_invokable(self: Pin<&mut MyObject>, param: &QColor) -> UniquePtr<QColor>; },
                qobject_ident: format_ident!("MyObject"),
                mutable: true,
                safe: true,
                parameters: vec![ParsedFunctionParameter {
                    ident: format_ident!("param"),
                    ty: parse_quote! { &QColor },
                }],
                specifiers: HashSet::new(),
                is_qinvokable: true,
            },
            ParsedMethod {
                method: parse_quote! { unsafe fn unsafe_invokable(self: &MyObject, param: *mut T) -> *mut T; },
                qobject_ident: format_ident!("MyObject"),
                mutable: false,
                safe: false,
                parameters: vec![ParsedFunctionParameter {
                    ident: format_ident!("param"),
                    ty: parse_quote! { *mut T },
                }],
                specifiers: HashSet::new(),
                is_qinvokable: true,
            },
        ];

        let generated = generate_rust_methods(&invokables).unwrap();

        assert_eq!(generated.cxx_mod_contents.len(), 4);
        assert_eq!(generated.cxx_qt_mod_contents.len(), 0);

        // void_invokable
        assert_tokens_eq(
            &generated.cxx_mod_contents[0],
            quote! {
                extern "Rust" {
                    #[doc(hidden)]
                    #[cxx_name = "voidInvokableWrapper"]
                    fn void_invokable(self: &MyObject);
                }
            },
        );

        // trivial_invokable
        assert_tokens_eq(
            &generated.cxx_mod_contents[1],
            quote! {
                extern "Rust" {
                    #[doc(hidden)]
                    #[cxx_name = "trivial_invokableWrapper"]
                    fn trivial_invokable(self: &MyObject, param: i32) -> i32;
                }
            },
        );

        // opaque_invokable
        assert_tokens_eq(
            &generated.cxx_mod_contents[2],
            quote! {
                extern "Rust" {
                    #[doc(hidden)]
                    #[cxx_name = "opaque_invokableWrapper"]
                    fn opaque_invokable(self: Pin<&mut MyObject>, param: &QColor) -> UniquePtr<QColor>;
                }
            },
        );

        // unsafe_invokable
        assert_tokens_eq(
            &generated.cxx_mod_contents[3],
            quote! {
                extern "Rust" {
                    #[doc(hidden)]
                    #[cxx_name = "unsafe_invokableWrapper"]
                    unsafe fn unsafe_invokable(self:&MyObject, param: *mut T) -> *mut T;
                }
            },
        );
    }
}
