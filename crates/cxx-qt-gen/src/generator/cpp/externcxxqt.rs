// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{
    generator::{
        cpp::signal::generate_cpp_free_signal,
        naming::namespace::namespace_externcxxqt_with_qobject_namespace,
    },
    parser::{externcxxqt::ParsedExternCxxQt, mappings::ParsedCxxMappings},
    CppFragment,
};
use syn::Result;

#[derive(Default)]
pub struct GeneratedCppExternCxxQtBlocks {
    /// List of methods
    pub method: CppFragment,
    /// Namespace of the method block
    pub namespace: String,
}

pub fn generate(
    blocks: &[ParsedExternCxxQt],
    cxx_mappings: &ParsedCxxMappings,
) -> Result<Vec<GeneratedCppExternCxxQtBlocks>> {
    let mut out = vec![];

    for block in blocks {
        for signal in &block.signals {
            // Build a namespace that includes any namespace for the T
            let namespace = namespace_externcxxqt_with_qobject_namespace(
                cxx_mappings
                    .namespaces
                    .get(&signal.qobject_ident.to_string()),
            );

            out.push(GeneratedCppExternCxxQtBlocks {
                method: generate_cpp_free_signal(signal, cxx_mappings)?,
                namespace,
            });
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
        let generated = generate(&blocks, &ParsedCxxMappings::default()).unwrap();
        assert_eq!(generated.len(), 2);

        assert_eq!(generated[0].namespace, "rust::cxxqtgen1::externcxxqt");
        assert_eq!(generated[1].namespace, "rust::cxxqtgen1::externcxxqt");
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
        let mut cxx_mappings = ParsedCxxMappings::default();
        cxx_mappings
            .cxx_names
            .insert("ObjRust".to_owned(), "ObjCpp".to_owned());
        cxx_mappings
            .namespaces
            .insert("ObjRust".to_owned(), "mynamespace".to_owned());

        let generated = generate(&blocks, &cxx_mappings).unwrap();
        assert_eq!(generated.len(), 1);

        assert_eq!(
            generated[0].namespace,
            "rust::cxxqtgen1::externcxxqt::mynamespace"
        );
    }
}
