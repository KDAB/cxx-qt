// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{
    generator::{
        naming::{invokable::QInvokableName, qobject::QObjectName},
        rust::{fragment::RustFragmentPair, qobject::GeneratedRustQObject},
    },
    parser::invokable::ParsedQInvokable,
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::Result;

pub fn generate_rust_invokables(
    invokables: &Vec<ParsedQInvokable>,
    qobject_idents: &QObjectName,
) -> Result<GeneratedRustQObject> {
    let mut generated = GeneratedRustQObject::default();
    let cpp_class_name_rust = &qobject_idents.cpp_class.rust;

    for invokable in invokables {
        let idents = QInvokableName::from(invokable);
        let wrapper_ident_cpp = idents.wrapper.cpp.to_string();
        let invokable_ident_rust = &idents.name.rust;

        // TODO: once we aren't using qobject::T in the extern "RustQt"
        // we can just pass through the original ExternFn block and add the attribute?
        let cpp_struct = if invokable.mutable {
            quote! { Pin<&mut #cpp_class_name_rust> }
        } else {
            quote! { &#cpp_class_name_rust }
        };
        let parameter_signatures = if invokable.parameters.is_empty() {
            quote! { self: #cpp_struct }
        } else {
            let parameters = invokable
                .parameters
                .iter()
                .map(|parameter| {
                    let ident = &parameter.ident;
                    let ty = &parameter.ty;
                    quote! { #ident: #ty }
                })
                .collect::<Vec<TokenStream>>();
            quote! { self: #cpp_struct, #(#parameters),* }
        };

        let return_type = &invokable.method.sig.output;

        let mut unsafe_block = None;
        let mut unsafe_call = Some(quote! { unsafe });
        if invokable.safe {
            std::mem::swap(&mut unsafe_call, &mut unsafe_block);
        }

        let fragment = RustFragmentPair {
            cxx_bridge: vec![quote! {
                // Note: extern "Rust" block does not need to be unsafe
                extern "Rust" {
                    // Note that we are exposing a Rust method on the C++ type to C++
                    //
                    // CXX ends up generating the source, then we generate the matching header.
                    #[doc(hidden)]
                    #[cxx_name = #wrapper_ident_cpp]
                    #unsafe_call fn #invokable_ident_rust(#parameter_signatures) #return_type;
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

    use crate::generator::naming::qobject::tests::create_qobjectname;
    use crate::parser::parameter::ParsedFunctionParameter;
    use crate::tests::assert_tokens_eq;
    use quote::format_ident;
    use std::collections::HashSet;
    use syn::parse_quote;

    #[test]
    fn test_generate_rust_invokables() {
        let invokables = vec![
            ParsedQInvokable {
                method: parse_quote! { fn void_invokable(self: &MyObject); },
                qobject_ident: format_ident!("MyObject"),
                mutable: false,
                safe: true,
                parameters: vec![],
                specifiers: HashSet::new(),
                is_qinvokable: true,
            },
            ParsedQInvokable {
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
            ParsedQInvokable {
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
            ParsedQInvokable {
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
        let qobject_idents = create_qobjectname();

        let generated = generate_rust_invokables(&invokables, &qobject_idents).unwrap();

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
                    #[cxx_name = "trivialInvokableWrapper"]
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
                    #[cxx_name = "opaqueInvokableWrapper"]
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
                    #[cxx_name = "unsafeInvokableWrapper"]
                    unsafe fn unsafe_invokable(self:&MyObject, param: *mut T) -> *mut T;
                }
            },
        );
    }
}
