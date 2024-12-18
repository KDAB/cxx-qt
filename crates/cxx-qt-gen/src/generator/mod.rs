// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx_gen::{CfgEvaluator, CfgResult};
#[cfg(test)]
use syn::{parse_quote, ItemMod};

pub mod cfg;
pub mod cpp;
pub mod naming;
pub mod rust;
pub mod structuring;

/// Options for C++ code generation.
#[non_exhaustive]
pub struct GeneratedOpt {
    /// Impl for handling conditional compilation attributes.
    pub cfg_evaluator: Box<dyn CfgEvaluator>,
}

impl Default for GeneratedOpt {
    fn default() -> Self {
        Self {
            cfg_evaluator: Box::new(UnsupportedCfgEvaluator),
        }
    }
}

pub(super) struct UnsupportedCfgEvaluator;

impl CfgEvaluator for UnsupportedCfgEvaluator {
    fn eval(&self, name: &str, value: Option<&str>) -> CfgResult {
        let _ = name;
        let _ = value;
        let msg = "cfg attribute is not supported".to_owned();
        CfgResult::Undetermined { msg }
    }
}

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
