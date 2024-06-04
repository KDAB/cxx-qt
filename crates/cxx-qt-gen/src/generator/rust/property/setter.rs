// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{
    generator::{
        naming::{property::QPropertyNames, qobject::QObjectNames},
        rust::fragment::RustFragmentPair,
    },
    naming::rust::{syn_type_cxx_bridge_to_qualified, syn_type_is_cxx_bridge_unsafe},
    naming::TypeNames,
};
use quote::quote;
use syn::{Result, Type};

pub fn generate(
    idents: &QPropertyNames,
    qobject_idents: &QObjectNames,
    cxx_ty: &Type,
    type_names: &TypeNames,
) -> Result<RustFragmentPair> {
    let cpp_class_name_rust = &qobject_idents.name.rust_unqualified();
    let setter_wrapper_cpp = idents.setter_wrapper.cxx_unqualified();
    let setter_rust = &idents.setter.rust_unqualified();
    let ident = &idents.name.rust_unqualified();
    let ident_str = ident.to_string();
    let notify_ident = &idents.notify.rust_unqualified();
    let qualified_ty = syn_type_cxx_bridge_to_qualified(cxx_ty, type_names)?;
    let qualified_impl = type_names.rust_qualified(cpp_class_name_rust)?;

    // Determine if unsafe is required due to an unsafe type
    let has_unsafe = if syn_type_is_cxx_bridge_unsafe(cxx_ty) {
        quote! { unsafe }
    } else {
        quote! {}
    };

    Ok(RustFragmentPair {
        cxx_bridge: vec![quote! {
            extern "Rust" {
                #[cxx_name = #setter_wrapper_cpp]
                // TODO: Add #[namespace] of the QObject to the declaration
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
    })
}
