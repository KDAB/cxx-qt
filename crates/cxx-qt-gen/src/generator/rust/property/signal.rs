// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::generator::{
    naming::{property::QPropertyName, qobject::QObjectName},
    rust::fragment::RustFragmentPair,
};
use quote::quote;

pub fn generate(idents: &QPropertyName, qobject_idents: &QObjectName) -> RustFragmentPair {
    let cpp_class_name_rust = &qobject_idents.cpp_class.rust;
    let notify_cpp = &idents.notify.cpp;
    let notify_rust = idents.notify.rust.to_string();
    let ident_str = idents.name.rust.to_string();
    let getter_mutable_rust_str = idents.getter_mutable.rust.to_string();

    RustFragmentPair {
        cxx_bridge: vec![quote! {
            unsafe extern "C++" {
                #[doc = "Notify signal for the Q_PROPERTY"]
                #[doc = #ident_str]
                #[doc = "\n"]
                #[doc = "This can be used to manually notify a change when the unsafe mutable getter,"]
                #[doc = #getter_mutable_rust_str]
                #[doc = ", is used."]
                #[rust_name = #notify_rust]
                fn #notify_cpp(self: Pin<&mut #cpp_class_name_rust>);
            }
        }],
        implementation: vec![],
    }
}
