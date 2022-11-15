// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::generator::cpp::{fragment::CppFragment, GeneratedCppBlocks};
use crate::writer::cpp::namespace_pair;
use indoc::formatdoc;

/// Extract the header from a given CppFragment
fn pair_as_header(pair: &CppFragment) -> Option<&str> {
    match pair {
        CppFragment::Pair { header, source: _ } => Some(header),
        CppFragment::Header(header) => Some(header),
        CppFragment::Source(_) => None,
    }
}

/// With a given block name, join the given items and add them under the block
fn create_block(block: &str, items: &[&str]) -> String {
    if items.is_empty() {
        "".to_owned()
    } else {
        formatdoc! {r#"
        {block}:
          {items}
        "#,
            block = block,
            items = items
                .iter()
                // Remove any empty entries from the blocks
                .filter(|item| !item.is_empty())
                .cloned()
                .map(String::from)
                .collect::<Vec<String>>()
                .join("\n  "),
        }
    }
}

/// For a given GeneratedCppBlocks write the forward declare
fn forward_declare(generated: &GeneratedCppBlocks) -> Vec<String> {
    let (namespace_start, namespace_end) = namespace_pair(generated);

    generated
        .qobjects
        .iter()
        .map(|qobject| {
            formatdoc! { r#"
                {namespace_start}
                class {ident};
                using {cxx_qt_thread_ident} = rust::cxxqtlib1::CxxQtThread<{ident}>;
                {namespace_end}
            "#,
            ident = qobject.ident,
            cxx_qt_thread_ident = qobject.cxx_qt_thread_ident,
            namespace_start = namespace_start,
            namespace_end = namespace_end,
            }
        })
        .collect::<Vec<String>>()
}

/// For a given GeneratedCppBlocks write the classes
fn qobjects_header(generated: &GeneratedCppBlocks) -> Vec<String> {
    let (namespace_start, namespace_end) = namespace_pair(generated);

    generated.qobjects.iter().map(|qobject| {
        formatdoc! { r#"
            {namespace_start}
            class {ident} : public {base_class}
            {{
              Q_OBJECT
              {metaobjects}

            public:
              explicit {ident}(QObject* parent = nullptr);
              ~{ident}();
              const {rust_ident}& unsafeRust() const;
              {rust_ident}& unsafeRustMut();
              std::unique_ptr<{cxx_qt_thread_ident}> qtThread() const;

            {methods}
            private:
              rust::Box<{rust_ident}> m_rustObj;
              std::shared_ptr<std::recursive_mutex> m_rustObjMutex;
              std::shared_ptr<rust::cxxqtlib1::CxxQtGuardedPointer<{ident}>> m_cxxQtThreadObj;
            }};

            static_assert(std::is_base_of<QObject, {ident}>::value, "{ident} must inherit from QObject");
            {namespace_end}

            namespace {namespace_internals} {{
            std::unique_ptr<{ident}>
            newCppObject();
            }} // namespace {namespace_internals}

            Q_DECLARE_METATYPE({metatype}*)
        "#,
        ident = qobject.ident,
        cxx_qt_thread_ident = qobject.cxx_qt_thread_ident,
        namespace_start = namespace_start,
        namespace_end = namespace_end,
        namespace_internals = qobject.namespace_internals,
        rust_ident = qobject.rust_ident,
        base_class = qobject.base_class,
        metaobjects = qobject.blocks.metaobjects.join("\n  "),
        methods = create_block("public", &qobject.blocks.methods.iter().filter_map(pair_as_header).collect::<Vec<&str>>()),
        metatype = if generated.namespace.is_empty() {
            qobject.ident.clone()
        } else {
            format!("{namespace}::{ident}", namespace = generated.namespace, ident = qobject.ident)
        },
        }
    }).collect::<Vec<String>>()
}

/// For a given GeneratedCppBlocks write this into a C++ header
pub fn write_cpp_header(generated: &GeneratedCppBlocks) -> String {
    formatdoc! {r#"
        #pragma once

        #include <memory>
        #include <mutex>

        namespace rust::cxxqtlib1 {{
        template<typename T>
        class CxxQtThread;
        }}

        {forward_declare}
        #include "cxx-qt-gen/{cxx_file_stem}.cxx.h"

        {qobjects}
    "#,
    cxx_file_stem = generated.cxx_file_stem,
    forward_declare = forward_declare(generated).join("\n"),
    qobjects = qobjects_header(generated).join("\n"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::writer::cpp::tests::{
        create_generated_cpp, create_generated_cpp_multi_qobjects,
        create_generated_cpp_no_namespace, expected_header, expected_header_multi_qobjects,
        expected_header_no_namespace,
    };
    use indoc::indoc;
    use pretty_assertions::assert_str_eq;

    #[test]
    fn test_create_block() {
        let block = create_block("block", &["line1", "line2"]);
        let expected = indoc! {"
        block:
          line1
          line2
        "};
        assert_str_eq!(block, expected);
    }

    #[test]
    fn test_create_block_with_empty() {
        let block = create_block("block", &["line1", "", "line2"]);
        let expected = indoc! {"
        block:
          line1
          line2
        "};
        assert_str_eq!(block, expected);
    }

    #[test]
    fn test_write_cpp_header() {
        let generated = create_generated_cpp();
        let output = write_cpp_header(&generated);
        assert_str_eq!(output, expected_header());
    }

    #[test]
    fn test_write_cpp_header_multi_qobjects() {
        let generated = create_generated_cpp_multi_qobjects();
        let output = write_cpp_header(&generated);
        assert_str_eq!(output, expected_header_multi_qobjects());
    }

    #[test]
    fn test_write_cpp_header_no_namespace() {
        let generated = create_generated_cpp_no_namespace();
        let output = write_cpp_header(&generated);
        assert_str_eq!(output, expected_header_no_namespace());
    }
}
