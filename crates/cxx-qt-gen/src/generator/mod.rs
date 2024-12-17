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
pub(super) struct TestCfgEvaluator {
    // CfgResult cannot be cloned so emulate with Option<bool>
    pub result: Option<bool>,
}

#[cfg(test)]
impl CfgEvaluator for TestCfgEvaluator {
    fn eval(&self, _name: &str, _value: Option<&str>) -> CfgResult {
        match self.result {
            Some(true) => CfgResult::True,
            Some(false) => CfgResult::False,
            None => CfgResult::Undetermined {
                msg: "Undetermined".to_owned(),
            },
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cfg_unsupported() {
        let evaluator = UnsupportedCfgEvaluator {};
        let result = evaluator.eval("test", Some("test"));
        assert!(matches!(result, CfgResult::Undetermined { .. }));
    }

    #[test]
    fn test_cfg_test() {
        let mut evaluator = TestCfgEvaluator { result: None };
        let result_none = evaluator.eval("test", Some("test"));
        assert!(matches!(result_none, CfgResult::Undetermined { .. }));

        evaluator.result = Some(true);
        let result_true = evaluator.eval("test", Some("test"));
        assert!(matches!(result_true, CfgResult::True));

        evaluator.result = Some(false);
        let result_false = evaluator.eval("test", Some("test"));
        assert!(matches!(result_false, CfgResult::False));
    }
}
