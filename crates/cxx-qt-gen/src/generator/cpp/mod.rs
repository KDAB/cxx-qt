// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

pub mod fragment;
pub mod invokable;
pub mod property;
pub mod qobject;
pub mod signal;
pub mod types;

use crate::generator::naming;
use crate::parser::Parser;
use qobject::GeneratedCppQObject;
use syn::{spanned::Spanned, Error, Result};

pub const RUST_OBJ_MUTEX_LOCK_GUARD: &str =
    "const std::lock_guard<std::recursive_mutex> guard(*m_rustObjMutex);";
pub const CXX_QT_CONVERT: &str = "rust::cxxqtlib1::cxx_qt_convert";

/// Representation of the generated C++ code for a group of QObjects
pub struct GeneratedCppBlocks {
    /// Stem of the CXX header to include
    pub cxx_stem: String,
    /// Ident of the common namespace of the QObjects
    pub namespace: String,
    /// Generated QObjects
    pub qobjects: Vec<GeneratedCppQObject>,
}

impl GeneratedCppBlocks {
    pub fn from(parser: &Parser) -> Result<GeneratedCppBlocks> {
        // TODO: for now we use the name of the first and only QObject
        // later this needs to come from elsewhere
        if parser.cxx_qt_data.qobjects.len() != 1 {
            return Err(Error::new(
                parser.passthrough_module.span(),
                "Expected one QObject in the ItemMod.",
            ));
        }
        let qt_ident = parser.cxx_qt_data.qobjects.keys().take(1).next().unwrap();

        Ok(GeneratedCppBlocks {
            cxx_stem: naming::module::cxx_stem_from_ident(qt_ident).to_string(),
            namespace: parser.cxx_qt_data.namespace.clone(),
            qobjects: parser
                .cxx_qt_data
                .qobjects
                .values()
                .map(GeneratedCppQObject::from)
                .collect::<Result<Vec<GeneratedCppQObject>>>()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::parser::Parser;
    use crate::tests::tokens_to_syn;
    use quote::quote;
    use syn::ItemMod;

    #[test]
    fn test_generated_cpp_blocks() {
        let module: ItemMod = tokens_to_syn(quote! {
            #[cxx_qt::bridge]
            mod ffi {
                #[cxx_qt::qobject]
                struct MyObject;
            }
        });
        let parser = Parser::from(module).unwrap();

        let cpp = GeneratedCppBlocks::from(&parser).unwrap();
        assert_eq!(cpp.cxx_stem, "my_object");
        assert_eq!(cpp.namespace, "");
        assert_eq!(cpp.qobjects.len(), 1);
    }

    #[test]
    fn test_generated_cpp_blocks_namespace() {
        let module: ItemMod = tokens_to_syn(quote! {
            #[cxx_qt::bridge(namespace = "cxx_qt")]
            mod ffi {
                #[cxx_qt::qobject]
                struct MyObject;
            }
        });
        let parser = Parser::from(module).unwrap();

        let cpp = GeneratedCppBlocks::from(&parser).unwrap();
        assert_eq!(cpp.namespace, "cxx_qt");
    }
}
