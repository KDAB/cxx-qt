// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::generator::cpp::GeneratedCppBlocks;
use crate::writer::cpp::{extract_extern_qt, namespaced, pair_as_source};
use indoc::formatdoc;

/// For a given GeneratedCppBlocks write the implementations
fn qobjects_source(generated: &GeneratedCppBlocks) -> Vec<String> {
    generated
        .qobjects
        .iter()
        .map(|qobject| {
            let methods = qobject
                .blocks
                .methods
                .iter()
                .chain(qobject.blocks.private_methods.iter())
                .filter_map(pair_as_source)
                .collect::<Vec<String>>()
                .join("\n");
            let namespaced = namespaced(qobject.name.namespace().unwrap_or_default(), &methods);

            qobject
                .blocks
                .fragments
                .iter()
                .filter_map(pair_as_source)
                .chain([namespaced])
                .collect::<Vec<String>>()
                .join("\n")
        })
        .collect::<Vec<String>>()
}

/// For a given GeneratedCppBlocks write this into a C++ source
pub fn write_cpp_source(generated: &GeneratedCppBlocks, include_path: &str) -> String {
    let extern_cxx_qt = extract_extern_qt(generated, pair_as_source);

    formatdoc! {r#"
        #include "{include_path}.cxxqt.h"

        {extern_cxx_qt}
        {qobjects}
    "#,
    qobjects = qobjects_source(generated).join("\n"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::writer::cpp::tests::{
        create_generated_cpp, create_generated_cpp_multi_qobjects,
        create_generated_cpp_no_namespace, expected_source, expected_source_multi_qobjects,
        expected_source_no_namespace,
    };
    use pretty_assertions::assert_str_eq;

    #[test]
    fn test_write_cpp_source() {
        let generated = create_generated_cpp();
        let output = write_cpp_source(&generated, "cxx-qt-gen/cxx_file_stem");
        assert_str_eq!(output, expected_source());
    }

    #[test]
    fn test_write_cpp_source_multi_qobjects() {
        let generated = create_generated_cpp_multi_qobjects();
        let output = write_cpp_source(&generated, "cxx-qt-gen/cxx_file_stem");
        assert_str_eq!(output, expected_source_multi_qobjects());
    }

    #[test]
    fn test_write_cpp_source_no_namespace() {
        let generated = create_generated_cpp_no_namespace();
        let output = write_cpp_source(&generated, "cxx-qt-gen/cxx_file_stem");
        assert_str_eq!(output, expected_source_no_namespace());
    }
}
