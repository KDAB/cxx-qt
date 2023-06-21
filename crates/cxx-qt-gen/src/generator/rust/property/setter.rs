// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::generator::{
    naming::{property::QPropertyName, qobject::QObjectName},
    rust::fragment::RustFragmentPair,
    utils::rust::syn_type_is_cxx_bridge_unsafe,
};
use quote::quote;
use syn::Type;

pub fn generate(
    idents: &QPropertyName,
    qobject_idents: &QObjectName,
    ty: &Type,
) -> RustFragmentPair {
    let cpp_class_name_rust = &qobject_idents.cpp_class.rust;
    let setter_wrapper_cpp = idents.setter_wrapper.cpp.to_string();
    let setter_rust = &idents.setter.rust;
    let ident = &idents.name.rust;
    let ident_str = ident.to_string();
    let notify_ident = &idents.notify.rust;

    // Determine if unsafe is required due to an unsafe type
    let has_unsafe = if syn_type_is_cxx_bridge_unsafe(ty) {
        quote! { unsafe }
    } else {
        quote! {}
    };

    RustFragmentPair {
        cxx_bridge: vec![quote! {
            extern "Rust" {
                #[cxx_name = #setter_wrapper_cpp]
                #has_unsafe fn #setter_rust(self: Pin<&mut #cpp_class_name_rust>, value: #ty);
            }
        }],
        implementation: vec![quote! {
            impl #cpp_class_name_rust {
                #[doc = "Setter for the Q_PROPERTY "]
                #[doc = #ident_str]
                pub fn #setter_rust(mut self: core::pin::Pin<&mut Self>, value: #ty) {
                    if self.#ident == value {
                        // don't want to set the value again and reemit the signal,
                        // as this can cause binding loops
                        return;
                    }
                    self.as_mut().rust_mut().#ident = value;
                    self.as_mut().#notify_ident();
                }
            }
        }],
    }
}
