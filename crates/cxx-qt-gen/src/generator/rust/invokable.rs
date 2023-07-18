// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{
    generator::{
        naming::{invokable::QInvokableName, qobject::QObjectName},
        rust::{fragment::RustFragmentPair, qobject::GeneratedRustQObjectBlocks},
    },
    parser::invokable::ParsedQInvokable,
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Result, ReturnType};

pub fn generate_rust_invokables(
    invokables: &Vec<ParsedQInvokable>,
    qobject_idents: &QObjectName,
) -> Result<GeneratedRustQObjectBlocks> {
    let mut generated = GeneratedRustQObjectBlocks::default();
    let cpp_class_name_rust = &qobject_idents.cpp_class.rust;
    let rust_struct_name_rust = &qobject_idents.rust_struct.rust;

    for invokable in invokables {
        let idents = QInvokableName::from(invokable);
        let wrapper_ident_cpp = idents.wrapper.cpp.to_string();
        let wrapper_ident_rust = &idents.wrapper.rust;
        let invokable_ident_rust = &idents.name.rust;

        let cpp_struct = if invokable.mutable {
            quote! {  Pin<&mut #cpp_class_name_rust> }
        } else {
            quote! { &#cpp_class_name_rust }
        };
        let rust_struct = if invokable.mutable {
            quote! {  &mut #rust_struct_name_rust }
        } else {
            quote! {  &#rust_struct_name_rust }
        };
        let parameter_signatures = if invokable.parameters.is_empty() {
            quote! { self: #rust_struct, cpp: #cpp_struct }
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
            quote! { self: #rust_struct, cpp: #cpp_struct, #(#parameters),* }
        };
        let return_type = &invokable.method.sig.output;
        let has_return = if matches!(invokable.method.sig.output, ReturnType::Default) {
            quote! {}
        } else {
            quote! { return }
        };

        let mut unsafe_block = None;
        let mut unsafe_call = Some(quote! { unsafe });
        if invokable.safe {
            std::mem::swap(&mut unsafe_call, &mut unsafe_block);
        }

        let parameter_names = invokable
            .parameters
            .iter()
            .map(|parameter| parameter.ident.clone())
            .collect::<Vec<Ident>>();

        let fragment = RustFragmentPair {
            cxx_bridge: vec![quote! {
                // TODO: is an unsafe block valid?
                extern "Rust" {
                    #[cxx_name = #wrapper_ident_cpp]
                    #unsafe_call fn #wrapper_ident_rust(#parameter_signatures) #return_type;
                }
            }],
            implementation: vec![
                // TODO: not all methods have a wrapper
                quote! {
                    impl #rust_struct_name_rust {
                        #[doc(hidden)]
                        pub #unsafe_call fn #wrapper_ident_rust(#parameter_signatures) #return_type {
                            #has_return cpp.#invokable_ident_rust(#(#parameter_names),*);
                        }
                    }
                },
            ],
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
            },
        ];
        let qobject_idents = create_qobjectname();

        let generated = generate_rust_invokables(&invokables, &qobject_idents).unwrap();

        assert_eq!(generated.cxx_mod_contents.len(), 4);
        assert_eq!(generated.cxx_qt_mod_contents.len(), 4);

        // void_invokable
        assert_tokens_eq(
            &generated.cxx_mod_contents[0],
            quote! {
                extern "Rust" {
                    #[cxx_name = "voidInvokableWrapper"]
                    fn void_invokable_wrapper(self: &MyObjectRust, cpp: &MyObject);
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[0],
            quote! {
                impl MyObjectRust {
                    #[doc(hidden)]
                    pub fn void_invokable_wrapper(self: &MyObjectRust, cpp: &MyObject) {
                        cpp.void_invokable();
                    }
                }
            },
        );

        // trivial_invokable
        assert_tokens_eq(
            &generated.cxx_mod_contents[1],
            quote! {
                extern "Rust" {
                    #[cxx_name = "trivialInvokableWrapper"]
                    fn trivial_invokable_wrapper(self: &MyObjectRust, cpp: &MyObject, param: i32) -> i32;
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[1],
            quote! {
                impl MyObjectRust {
                    #[doc(hidden)]
                    pub fn trivial_invokable_wrapper(self: &MyObjectRust, cpp: &MyObject, param: i32) -> i32 {
                        return cpp.trivial_invokable(param);
                    }
                }
            },
        );

        // opaque_invokable
        assert_tokens_eq(
            &generated.cxx_mod_contents[2],
            quote! {
                extern "Rust" {
                    #[cxx_name = "opaqueInvokableWrapper"]
                    fn opaque_invokable_wrapper(self: &mut MyObjectRust, cpp: Pin<&mut MyObject>, param: &QColor) -> UniquePtr<QColor>;
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[2],
            quote! {
                impl MyObjectRust {
                    #[doc(hidden)]
                    pub fn opaque_invokable_wrapper(self: &mut MyObjectRust, cpp: Pin<&mut MyObject>, param: &QColor) -> UniquePtr<QColor> {
                        return cpp.opaque_invokable(param);
                    }
                }
            },
        );

        // unsafe_invokable
        assert_tokens_eq(
            &generated.cxx_mod_contents[3],
            quote! {
                extern "Rust" {
                    #[cxx_name = "unsafeInvokableWrapper"]
                    unsafe fn unsafe_invokable_wrapper(self: &MyObjectRust, cpp: &MyObject, param: *mut T) -> *mut T;
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[3],
            quote! {
                impl MyObjectRust {
                    #[doc(hidden)]
                    pub unsafe fn unsafe_invokable_wrapper(self: &MyObjectRust, cpp: &MyObject, param: *mut T) -> *mut T {
                        return cpp.unsafe_invokable(param);
                    }
                }
            },
        );
    }
}
