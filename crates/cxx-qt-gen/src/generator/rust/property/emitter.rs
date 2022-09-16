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
    let emit_cpp = &idents.emit.cpp;
    let emit_rust = idents.emit.rust.to_string();

    RustFragmentPair {
        cxx_bridge: vec![quote! {
            unsafe extern "C++" {
                #[rust_name = #emit_rust]
                fn #emit_cpp(self: Pin<&mut #cpp_class_name_rust>);
            }
        }],
        implementation: vec![],
    }
}
