// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{
    generator::{
        naming::{
            property::{NameState, QPropertyNames},
            qobject::QObjectNames,
        },
        rust::fragment::RustFragmentPair,
    },
    naming::rust::syn_type_cxx_bridge_to_qualified,
    naming::TypeNames,
};
use quote::quote;
use syn::{Result, Type};

pub fn generate(
    idents: &QPropertyNames,
    qobject_names: &QObjectNames,
    cxx_ty: &Type,
    type_names: &TypeNames,
) -> Result<Option<RustFragmentPair>> {
    if let (NameState::Auto(getter), Some(getter_wrapper)) =
        (&idents.getter, &idents.getter_wrapper)
    {
        let cpp_class_name_rust = qobject_names.name.rust_unqualified();
        let getter_wrapper_cpp = getter_wrapper.cxx_unqualified();
        let getter_rust = getter.rust_unqualified();
        let ident = idents.name.rust_unqualified();
        let ident_str = ident.to_string();
        let qualified_ty = syn_type_cxx_bridge_to_qualified(cxx_ty, type_names)?;
        let qualified_impl = type_names.rust_qualified(cpp_class_name_rust)?;

        Ok(Some(RustFragmentPair {
            cxx_bridge: vec![quote! {
                extern "Rust" {
                    #[cxx_name = #getter_wrapper_cpp]
                    // Namespace is not needed here
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
        }))
    } else {
        Ok(None)
    }
}
