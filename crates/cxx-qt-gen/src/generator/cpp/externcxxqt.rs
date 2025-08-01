// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{
    generator::{cpp::signal::generate_cpp_signal, GeneratedOpt},
    naming::TypeNames,
    parser::externcxxqt::ParsedExternCxxQt,
    CppFragment,
};
use std::collections::BTreeSet;
use syn::Result;

#[derive(Default, Debug)]
pub struct GeneratedCppExternCxxQtBlocks {
    /// List of includes
    pub includes: BTreeSet<String>,
    /// List of forward declares before the class and include of the generated CXX header
    pub forward_declares: Vec<String>,
    /// Base class of the QObject
    pub base_classes: Vec<String>,
    /// List of fragments
    pub fragments: Vec<CppFragment>,
}

pub fn generate(
    blocks: &[ParsedExternCxxQt],
    type_names: &TypeNames,
    opt: &GeneratedOpt,
) -> Result<Vec<GeneratedCppExternCxxQtBlocks>> {
    let mut out = vec![];

    for block in blocks {
        for signal in &block.signals {
            let qobject_name = type_names.lookup(&signal.qobject_ident)?;
            let data = generate_cpp_signal(signal, qobject_name, type_names, opt)?;
            debug_assert!(data.methods.is_empty());
            let block = GeneratedCppExternCxxQtBlocks {
                includes: data.includes,
                forward_declares: data.forward_declares,
                fragments: data.fragments,
                ..Default::default()
            };
            out.push(block);
        }
        let mut generated = GeneratedCppExternCxxQtBlocks {
            base_classes: vec![],
            ..Default::default()
        };

        // Include casting header
        let mut result = GeneratedCppExternCxxQtBlocks::default();
        result.includes.insert("#include <cxx-qt/casting.h>".into());

        out.push(result);

        for qobject in &block.qobjects {
            let base_class = if let Some(ident) = &qobject.base_class {
                type_names.lookup(ident)?.cxx_qualified()
            } else {
                "QObject".to_owned()
            };
            generated.base_classes.push(base_class);
        }
        out.push(generated);
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use quote::format_ident;
    use syn::parse_quote;

    use super::*;

    #[test]
    fn test_generate_cpp_extern_qt() {
        let blocks = vec![ParsedExternCxxQt::parse(
            parse_quote! {
                unsafe extern "C++Qt" {
                    #[qobject]
                    type MyObject;

                    #[qsignal]
                    fn signal1(self: Pin<&mut MyObject>);

                    #[qsignal]
                    fn signal2(self: Pin<&mut MyObject>);
                }
            },
            &format_ident!("qobject"),
            None,
        )
        .unwrap()];

        // Unknown types
        let opt = GeneratedOpt::default();
        assert!(generate(&blocks, &TypeNames::default(), &opt).is_err());

        let generated = generate(&blocks, &TypeNames::mock(), &opt).unwrap();
        assert_eq!(generated.len(), 4);
    }

    #[test]
    fn test_generate_cpp_extern_qt_mapping() {
        let blocks = vec![ParsedExternCxxQt::parse(
            parse_quote! {
                unsafe extern "C++Qt" {
                    #[cxx_name = "ObjCpp"]
                    #[namespace = "mynamespace"]
                    #[qobject]
                    type ObjRust;

                    #[qsignal]
                    fn signal(self: Pin<&mut ObjRust>);
                }
            },
            &format_ident!("qobject"),
            None,
        )
        .unwrap()];
        let mut type_names = TypeNames::default();
        type_names.mock_insert("ObjRust", None, Some("ObjCpp"), Some("mynamespace"));

        let generated = generate(&blocks, &type_names, &GeneratedOpt::default()).unwrap();
        assert_eq!(generated.len(), 3);
    }
}
