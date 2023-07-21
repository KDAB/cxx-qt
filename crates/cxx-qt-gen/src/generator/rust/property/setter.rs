// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::generator::{
    naming::{property::QPropertyName, qobject::QObjectName},
    rust::fragment::RustFragmentPair,
    utils::rust::{
        syn_ident_cxx_bridge_to_qualified_impl, syn_type_cxx_bridge_to_qualified,
        syn_type_is_cxx_bridge_unsafe,
    },
};
use quote::quote;
use std::collections::BTreeMap;
use syn::{Ident, Path, Type};

pub fn generate(
    idents: &QPropertyName,
    qobject_idents: &QObjectName,
    cxx_ty: &Type,
    qualified_mappings: &BTreeMap<Ident, Path>,
) -> RustFragmentPair {
    let cpp_class_name_rust = &qobject_idents.cpp_class.rust;
    let setter_wrapper_cpp = idents.setter_wrapper.cpp.to_string();
    let setter_rust = &idents.setter.rust;
    let ident = &idents.name.rust;
    let ident_str = ident.to_string();
    let notify_ident = &idents.notify.rust;
    let qualified_ty = syn_type_cxx_bridge_to_qualified(cxx_ty, qualified_mappings);
    let qualified_impl =
        syn_ident_cxx_bridge_to_qualified_impl(cpp_class_name_rust, qualified_mappings);

    // Determine if unsafe is required due to an unsafe type
    let has_unsafe = if syn_type_is_cxx_bridge_unsafe(cxx_ty) {
        quote! { unsafe }
    } else {
        quote! {}
    };

    RustFragmentPair {
        cxx_bridge: vec![quote! {
            extern "Rust" {
                #[cxx_name = #setter_wrapper_cpp]
                #has_unsafe fn #setter_rust(self: Pin<&mut #cpp_class_name_rust>, value: #cxx_ty);
            }
        }],
        implementation: vec![quote! {
            impl #qualified_impl {
                #[doc = "Setter for the Q_PROPERTY "]
                #[doc = #ident_str]
                pub fn #setter_rust(mut self: core::pin::Pin<&mut Self>, value: #qualified_ty) {
                    use cxx_qt::CxxQtType;
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
