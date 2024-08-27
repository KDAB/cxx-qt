#[cfg(test)]
use syn::{parse_quote, ItemMod};

// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
pub mod cpp;
pub mod naming;
pub mod rust;
pub mod structuring;

#[cfg(test)]
/// Mocks a module containing a singleton type
pub fn mock_qml_singleton() -> ItemMod {
    parse_quote! {
        #[cxx_qt::bridge(namespace = "cxx_qt")]
        mod ffi {
            extern "RustQt" {
                #[qobject]
                #[qml_element]
                #[qml_singleton]
                type MyObject = super::MyObjectRust;
            }
        }
    }
}
