// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{
    generator::{
        naming::{property::QPropertyNames, qobject::QObjectNames},
        rust::fragment::RustFragmentPair,
    },
    naming::rust::syn_type_cxx_bridge_to_qualified,
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
    let getter_wrapper_cpp = idents.getter_wrapper.cxx_unqualified();
    let getter_rust = idents.getter.rust_unqualified();
    let ident = idents.name.rust_unqualified();
    let ident_str = ident.to_string();
    let qualified_ty = syn_type_cxx_bridge_to_qualified(cxx_ty, type_names)?;
    let qualified_impl = type_names.rust_qualified(cpp_class_name_rust)?;

    Ok(RustFragmentPair {
        cxx_bridge: vec![quote! {
            extern "Rust" {
                #[cxx_name = #getter_wrapper_cpp]
                // TODO: Add #[namespace] of the QObject to the declaration
                unsafe fn #getter_rust<'a>(self: &'a #cpp_class_name_rust) -> &'a #cxx_ty;
            }
        }],
        implementation: vec![quote! {
            impl #qualified_impl {
                #[doc = "Getter for the Q_PROPERTY "]
                #[doc = #ident_str]
                pub fn #getter_rust(&self) -> &#qualified_ty {
                    &self.#ident
                }
            }
        }],
    })
}
