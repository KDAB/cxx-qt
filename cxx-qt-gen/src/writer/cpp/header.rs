// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::generator::cpp::{fragment::CppFragmentPair, GeneratedCppBlocks};
use indoc::formatdoc;

/// Extract the header from a given CppFragmentPair
fn pair_as_header(pair: &CppFragmentPair) -> &str {
    &pair.header
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

        {namespace_start}
        class {ident};
        using {cxx_qt_thread_ident} = rust::cxxqtlib1::CxxQtThread<{ident}>;
        {namespace_end}

        #include "cxx-qt-gen/include/{cxx_stem}.cxx.h"

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
        {slots}
        {signals}
        private:
          rust::Box<{rust_ident}> m_rustObj;
          std::shared_ptr<std::mutex> m_rustObjMutex;
          std::shared_ptr<rust::cxxqtlib1::CxxQtGuardedPointer<{ident}>> m_cxxQtThreadObj;

          {members}
        }};

        static_assert(std::is_base_of<QObject, {ident}>::value, "{ident} must inherit from QObject");
        {namespace_end}

        namespace {namespace_internals} {{
        std::unique_ptr<{ident}>
        newCppObject();
        }} // namespace {namespace_internals}

        Q_DECLARE_METATYPE({metatype}*)
    "#,
    cxx_stem = generated.cxx_stem,
    ident = generated.ident,
    cxx_qt_thread_ident = generated.cxx_qt_thread_ident,
    namespace_start = if generated.namespace.is_empty() {
        "".to_owned()
    } else {
        format!("namespace {namespace} {{", namespace = generated.namespace)
    },
    namespace_end = if generated.namespace.is_empty() {
        "".to_owned()
    } else {
        format!("}} // namespace {namespace}", namespace = generated.namespace)
    },
    namespace_internals = generated.namespace_internals,
    rust_ident = generated.rust_ident,
    base_class = generated.base_class,
    metaobjects = generated.metaobjects.join("\n  "),
    methods = create_block("public", &generated.methods.iter().map(pair_as_header).collect::<Vec<&str>>()),
    slots = create_block("public Q_SLOTS", &generated.slots.iter().map(pair_as_header).collect::<Vec<&str>>()),
    signals = create_block("Q_SIGNALS", &generated.signals.iter().map(AsRef::as_ref).collect::<Vec<&str>>()),
    members = generated.members.join("\n  "),
    metatype = if generated.namespace.is_empty() {
        generated.ident.clone()
    } else {
        format!("{namespace}::{ident}", namespace = generated.namespace, ident = generated.ident)
    },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::writer::cpp::tests::{
        create_generated_cpp, create_generated_cpp_no_namespace, expected_header,
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
    fn test_write_cpp_header_no_namespace() {
        let generated = create_generated_cpp_no_namespace();
        let output = write_cpp_header(&generated);
        assert_str_eq!(output, expected_header_no_namespace());
    }
}
