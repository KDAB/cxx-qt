// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::generator::{
    naming::{property::QPropertyName, qobject::QObjectName},
    rust::fragment::RustFragmentPair,
};
use quote::quote;
use syn::Type;

pub fn generate(
    idents: &QPropertyName,
    qobject_idents: &QObjectName,
    ty: &Type,
) -> RustFragmentPair {
    let cpp_class_name_rust = &qobject_idents.cpp_class.rust;
    let getter_wrapper_cpp = idents.getter_wrapper.cpp.to_string();
    let getter_rust = &idents.getter.rust;
    let ident = &idents.name.rust;
    let ident_str = ident.to_string();

    RustFragmentPair {
        cxx_bridge: vec![quote! {
            extern "Rust" {
                #[cxx_name = #getter_wrapper_cpp]
                unsafe fn #getter_rust<'a>(self: &'a #cpp_class_name_rust) -> &'a #ty;
            }
        }],
        implementation: vec![quote! {
            impl #cpp_class_name_rust {
                #[doc = "Getter for the Q_PROPERTY "]
                #[doc = #ident_str]
                pub fn #getter_rust(&self) -> &#ty {
                    &self.#ident
                }
            }
        }],
    }
}
