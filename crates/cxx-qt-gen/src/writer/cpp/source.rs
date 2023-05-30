// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::generator::cpp::{fragment::CppFragment, GeneratedCppBlocks};
use crate::writer::cpp::namespace_pair;
use indoc::formatdoc;

/// Extract the source from a given CppFragment
fn pair_as_source(pair: &CppFragment) -> Option<String> {
    match pair {
        CppFragment::Pair { header: _, source } => Some(source.clone()),
        CppFragment::Header(_) => None,
        CppFragment::Source(source) => Some(source.clone()),
    }
}

/// For a given GeneratedCppBlocks write the implementations
fn qobjects_source(generated: &GeneratedCppBlocks) -> Vec<String> {
    let (namespace_start, namespace_end) = namespace_pair(generated);

    generated.qobjects.iter().map(|qobject| {
        formatdoc! { r#"
            {namespace_start}

            {ident}::{ident}(QObject* parent)
              : {base_class}(parent)
              {members}
            {{
            }}

            {ident}::~{ident}()
            {{
            {deconstructors}
            }}

            {rust_ident} const&
            {ident}::unsafeRust() const
            {{
              return *m_rustObj;
            }}

            {rust_ident}&
            {ident}::unsafeRustMut()
            {{
              return *m_rustObj;
            }}

            {methods}
            {namespace_end}

            namespace {namespace_internals} {{
            ::std::unique_ptr<{ident}>
            newCppObject()
            {{
              return ::std::make_unique<{ident}>();
            }}
            }} // namespace {namespace_internals}
        "#,
        ident = qobject.ident,
        namespace_start = namespace_start,
        namespace_end = namespace_end,
        namespace_internals = qobject.namespace_internals,
        base_class = qobject.base_class,
        rust_ident = qobject.rust_ident,
        methods = qobject.blocks.methods.iter().filter_map(pair_as_source).collect::<Vec<String>>().join("\n"),
        members = {
            let mut members = vec![
                format!(", m_rustObj({namespace_internals}::createRs())", namespace_internals = qobject.namespace_internals),
                ", m_rustObjMutex(::std::make_shared<::std::recursive_mutex>())".to_string(),
            ];

            members.extend(qobject.blocks.members.iter().filter_map(pair_as_source).collect::<Vec<String>>());
            members.join("\n  ")
        },
        deconstructors = qobject.blocks.deconstructors.join("\n  "),
        }
  }).collect::<Vec<String>>()
}

/// For a given GeneratedCppBlocks write this into a C++ source
pub fn write_cpp_source(generated: &GeneratedCppBlocks) -> String {
    formatdoc! {r#"
        #include "cxx-qt-gen/{cxx_file_stem}.cxxqt.h"

        {qobjects}
    "#,
    cxx_file_stem = generated.cxx_file_stem,
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
        let output = write_cpp_source(&generated);
        assert_str_eq!(output, expected_source());
    }

    #[test]
    fn test_write_cpp_source_multi_qobjects() {
        let generated = create_generated_cpp_multi_qobjects();
        let output = write_cpp_source(&generated);
        assert_str_eq!(output, expected_source_multi_qobjects());
    }

    #[test]
    fn test_write_cpp_source_no_namespace() {
        let generated = create_generated_cpp_no_namespace();
        let output = write_cpp_source(&generated);
        assert_str_eq!(output, expected_source_no_namespace());
    }
}
