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

        namespace {namespace} {{
        class {ident};
        }} // namespace {namespace}

        #include "cxx-qt-gen/include/{cxx_stem}.cxx.h"

        namespace {namespace} {{

        class {ident} : public QObject
        {{
          Q_OBJECT
          {metaobjects}

        public:
          explicit {ident}(QObject* parent = nullptr);
          ~{ident}();
          const {rust_ident}& unsafe_rust() const;
          {rust_ident}& unsafe_rust_mut();

        {methods}
        {slots}
        {signals}
        private:
          rust::Box<{rust_ident}> m_rustObj;
          std::mutex m_rustObjMutex;
          bool m_initialised = false;

          {members}
        }};

        typedef {ident} CppObj;

        std::unique_ptr<CppObj>
        newCppObject();

        }} // namespace {namespace}

        Q_DECLARE_METATYPE({namespace}::CppObj*)
    "#,
    cxx_stem = generated.cxx_stem,
    ident = generated.ident,
    namespace = generated.namespace,
    rust_ident = generated.rust_ident,
    metaobjects = generated.metaobjects.join("\n  "),
    methods = create_block("public", &generated.methods.iter().map(pair_as_header).collect::<Vec<&str>>()),
    slots = create_block("public Q_SLOTS", &generated.slots.iter().map(pair_as_header).collect::<Vec<&str>>()),
    signals = create_block("Q_SIGNALS", &generated.signals.iter().map(AsRef::as_ref).collect::<Vec<&str>>()),
    members = generated.members.join("\n  "),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::writer::cpp::tests::{create_generated_cpp, expected_header};
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
}
