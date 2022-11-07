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
use syn::{Ident, Result};

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
        let original_method = &invokable.method;

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
        let has_return = if matches!(invokable.method.sig.output, syn::ReturnType::Default) {
            quote! {}
        } else {
            quote! { return }
        };
        let parameter_names = invokable
            .parameters
            .iter()
            .map(|parameter| parameter.ident.clone())
            .collect::<Vec<Ident>>();

        let fragment = RustFragmentPair {
            cxx_bridge: vec![quote! {
                extern "Rust" {
                    #[cxx_name = #wrapper_ident_cpp]
                    fn #wrapper_ident_rust(#parameter_signatures) #return_type;
                }
            }],
            implementation: vec![
                // TODO: not all methods have a wrapper
                quote! {
                    impl #rust_struct_name_rust {
                        pub fn #wrapper_ident_rust(#parameter_signatures) #return_type {
                            #has_return cpp.#invokable_ident_rust(#(#parameter_names),*);
                        }
                    }
                },
                quote! {
                    impl #cpp_class_name_rust {
                        #original_method
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
    use crate::tests::utils::{assert_tokens_eq, tokens_to_syn};
    use quote::format_ident;
    use std::collections::HashSet;

    #[test]
    fn test_generate_rust_invokables() {
        let invokables = vec![
            ParsedQInvokable {
                method: tokens_to_syn(quote! { fn void_invokable(&self) {} }),
                mutable: false,
                parameters: vec![],
                return_cxx_type: None,
                specifiers: HashSet::new(),
            },
            ParsedQInvokable {
                method: tokens_to_syn(quote! { fn trivial_invokable(&self, param: i32) -> i32 {} }),
                mutable: false,
                parameters: vec![ParsedFunctionParameter {
                    ident: format_ident!("param"),
                    ty: tokens_to_syn::<syn::Type>(quote! { i32 }),
                    cxx_type: None,
                }],
                return_cxx_type: None,
                specifiers: HashSet::new(),
            },
            ParsedQInvokable {
                method: tokens_to_syn(
                    quote! { fn opaque_invokable(self: Pin<&mut Self>, param: &QColor) -> UniquePtr<QColor> {} },
                ),
                mutable: true,
                parameters: vec![ParsedFunctionParameter {
                    ident: format_ident!("param"),
                    ty: tokens_to_syn::<syn::Type>(quote! { &QColor }),
                    cxx_type: None,
                }],
                return_cxx_type: Some("QColor".to_owned()),
                specifiers: HashSet::new(),
            },
        ];
        let qobject_idents = create_qobjectname();

        let generated = generate_rust_invokables(&invokables, &qobject_idents).unwrap();

        assert_eq!(generated.cxx_mod_contents.len(), 3);
        assert_eq!(generated.cxx_qt_mod_contents.len(), 6);

        // void_invokable
        assert_tokens_eq(
            &generated.cxx_mod_contents[0],
            quote! {
                extern "Rust" {
                    #[cxx_name = "voidInvokableWrapper"]
                    fn void_invokable_wrapper(self: &MyObject, cpp: &MyObjectQt);
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[0],
            quote! {
                impl MyObject {
                    pub fn void_invokable_wrapper(self: &MyObject, cpp: &MyObjectQt) {
                        cpp.void_invokable();
                    }
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[1],
            quote! {
                impl MyObjectQt {
                    fn void_invokable(&self) {}
                }
            },
        );

        // trivial_invokable
        assert_tokens_eq(
            &generated.cxx_mod_contents[1],
            quote! {
                extern "Rust" {
                    #[cxx_name = "trivialInvokableWrapper"]
                    fn trivial_invokable_wrapper(self: &MyObject, cpp: &MyObjectQt, param: i32) -> i32;
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[2],
            quote! {
                impl MyObject {
                    pub fn trivial_invokable_wrapper(self: &MyObject, cpp: &MyObjectQt, param: i32) -> i32 {
                        return cpp.trivial_invokable(param);
                    }
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[3],
            quote! {
                impl MyObjectQt {
                    fn trivial_invokable(&self, param: i32) -> i32 {}
                }
            },
        );

        // opaque_invokable
        assert_tokens_eq(
            &generated.cxx_mod_contents[2],
            quote! {
                extern "Rust" {
                    #[cxx_name = "opaqueInvokableWrapper"]
                    fn opaque_invokable_wrapper(self: &mut MyObject, cpp: Pin<&mut MyObjectQt>, param: &QColor) -> UniquePtr<QColor>;
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[4],
            quote! {
                impl MyObject {
                    pub fn opaque_invokable_wrapper(self: &mut MyObject, cpp: Pin<&mut MyObjectQt>, param: &QColor) -> UniquePtr<QColor> {
                        return cpp.opaque_invokable(param);
                    }
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[5],
            quote! {
                impl MyObjectQt {
                    fn opaque_invokable(self: Pin<&mut Self>, param: &QColor) -> UniquePtr<QColor> {}
                }
            },
        );
    }
}
