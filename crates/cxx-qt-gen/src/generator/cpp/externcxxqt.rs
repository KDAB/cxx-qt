// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{
    generator::cpp::signal::generate_cpp_signal,
    parser::{externcxxqt::ParsedExternCxxQt, naming::TypeNames},
    CppFragment,
};
use std::collections::BTreeSet;
use syn::Result;

#[derive(Default)]
pub struct GeneratedCppExternCxxQtBlocks {
    /// List of includes
    pub includes: BTreeSet<String>,
    /// List of forward declares before the class and include of the generated CXX header
    pub forward_declares: Vec<String>,
    /// List of fragments
    pub fragments: Vec<CppFragment>,
}

pub fn generate(
    blocks: &[ParsedExternCxxQt],
    type_names: &TypeNames,
) -> Result<Vec<GeneratedCppExternCxxQtBlocks>> {
    let mut out = vec![];

    for block in blocks {
        for signal in &block.signals {
            let mut block = GeneratedCppExternCxxQtBlocks::default();
            let data = generate_cpp_signal(signal, &signal.qobject_ident, type_names)?;
            block.includes = data.includes;
            // Ensure that we include MaybeLockGuard<T> that is used in multiple places
            block
                .includes
                .insert("#include <cxx-qt-common/cxxqt_maybelockguard.h>".to_owned());
            block.forward_declares = data.forward_declares;
            block.fragments = data.fragments;
            debug_assert!(data.methods.is_empty());
            out.push(block);
        }
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use syn::parse_quote;

    use super::*;

    #[test]
    fn test_generate_cpp_extern_qt() {
        let blocks = vec![ParsedExternCxxQt::parse(parse_quote! {
            unsafe extern "C++Qt" {
                type MyObject;

                #[qsignal]
                fn signal1(self: Pin<&mut ObjRust>);

                #[qsignal]
                fn signal2(self: Pin<&mut ObjRust>);
            }
        })
        .unwrap()];
        let generated = generate(&blocks, &TypeNames::default()).unwrap();
        assert_eq!(generated.len(), 2);
    }

    #[test]
    fn test_generate_cpp_extern_qt_mapping() {
        let blocks = vec![ParsedExternCxxQt::parse(parse_quote! {
            unsafe extern "C++Qt" {
                #[cxx_name = "ObjCpp"]
                #[namespace = "mynamespace"]
                type ObjRust;

                #[qsignal]
                fn signal(self: Pin<&mut ObjRust>);
            }
        })
        .unwrap()];
        let mut type_names = TypeNames::default();
        type_names
            .cxx_names
            .insert("ObjRust".to_owned(), "ObjCpp".to_owned());
        type_names
            .namespaces
            .insert("ObjRust".to_owned(), "mynamespace".to_owned());

        let generated = generate(&blocks, &type_names).unwrap();
        assert_eq!(generated.len(), 1);
    }
}
