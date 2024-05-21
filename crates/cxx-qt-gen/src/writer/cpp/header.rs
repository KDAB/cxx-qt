// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::collections::BTreeSet;

use crate::generator::cpp::{fragment::CppFragment, GeneratedCppBlocks};
use crate::writer::cpp::namespaced;
use indoc::formatdoc;

/// Extract the header from a given CppFragment
fn pair_as_header(pair: &CppFragment) -> Option<String> {
    match pair {
        CppFragment::Pair { header, source: _ } => Some(header.clone()),
        CppFragment::Header(header) => Some(header.clone()),
        CppFragment::Source(_) => None,
    }
}

/// With a given block name, join the given items and add them under the block
fn create_block(block: &str, items: &[String]) -> String {
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
                .collect::<Vec<String>>()
                .join("\n  "),
        }
    }
}

/// For a given GeneratedCppBlocks write the forward declare
//
// Note that this is needed incase ObjectA refers to ObjectB in it's class
fn forward_declare(generated: &GeneratedCppBlocks) -> Vec<String> {
    generated
        .qobjects
        .iter()
        .map(|qobject| {
            let forward_declares = namespaced(
                qobject.name.namespace().unwrap_or_default(),
                &formatdoc! {r#"
                    class {ident};
                    {forward_declares}
                "#,
                ident = &qobject.name.cxx_unqualified(),
                forward_declares = qobject.blocks.forward_declares.join("\n"),
                },
            );
            let forward_declares_namespaced = qobject.blocks.forward_declares_namespaced.join("\n");
            formatdoc! {r#"
                {forward_declares}
                {forward_declares_namespaced}
            "#}
        })
        .chain(generated.forward_declares.iter().cloned())
        .chain(
            generated
                .extern_cxx_qt
                .iter()
                .map(|external| external.forward_declares.join("\n")),
        )
        .collect::<Vec<String>>()
}

/// For a given GeneratedCppBlocks write the classes
fn qobjects_header(generated: &GeneratedCppBlocks) -> Vec<String> {
    generated.qobjects.iter().map(|qobject| {
        let ident = &qobject.name.cxx_unqualified();
        let qobject_macro = if qobject.has_qobject_macro {
            "Q_OBJECT"
        } else {
            ""
        };
        let qobject_assert = if qobject.has_qobject_macro {
            format!("static_assert(::std::is_base_of<QObject, {ident}>::value, \"{ident} must inherit from QObject\");")
        } else {
            "".to_owned()
        };
        let class_definition = namespaced(
            qobject.name.namespace().unwrap_or_default(),
            &formatdoc! { r#"
                class {ident} : {base_classes}
                {{
                  {qobject_macro}
                public:
                  {metaobjects}

                  virtual ~{ident}() = default;

                {public_methods}
                {private_methods}
                }};

                {qobject_assert}"#,
            // Note that there is always a base class as we always have CxxQtType
            base_classes = qobject.blocks.base_classes.iter().map(|base| format!("public {}", base)).collect::<Vec<String>>().join(", "),
            metaobjects = qobject.blocks.metaobjects.join("\n  "),
            public_methods = create_block("public", &qobject.blocks.methods.iter().filter_map(pair_as_header).collect::<Vec<String>>()),
            private_methods = create_block("private", &qobject.blocks.private_methods.iter().filter_map(pair_as_header).collect::<Vec<String>>()),
        });

        let fragments = qobject
            .blocks
            .fragments
            .iter()
            .filter_map(pair_as_header)
            .collect::<Vec<String>>()
            .join("\n");

        let declare_metatype = if qobject.has_qobject_macro {
            let ty = qobject.name.cxx_qualified();
            format!("Q_DECLARE_METATYPE({ty}*)")
        } else {
            "".to_owned()
        };

        formatdoc! {r#"
            {fragments}
            {class_definition}

            {declare_metatype}
            "#
        }
    }).collect::<Vec<String>>()
}

/// For a given GeneratedCppBlocks write this into a C++ header
pub fn write_cpp_header(generated: &GeneratedCppBlocks) -> String {
    let includes = {
        let mut include_set = BTreeSet::new();
        include_set.extend(
            generated
                .includes
                .iter()
                .chain(
                    generated
                        .qobjects
                        .iter()
                        .flat_map(|qobject| &qobject.blocks.includes),
                )
                .chain(
                    generated
                        .extern_cxx_qt
                        .iter()
                        .flat_map(|block| &block.includes),
                ),
        );
        include_set
            .into_iter()
            .cloned()
            .collect::<Vec<String>>()
            .join("\n")
    };
    let extern_cxx_qt = generated
        .extern_cxx_qt
        .iter()
        .flat_map(|block| {
            block
                .fragments
                .iter()
                .filter_map(pair_as_header)
                .collect::<Vec<String>>()
        })
        .collect::<Vec<String>>()
        .join("\n");

    formatdoc! {r#"
        #pragma once

        {includes}

        {forward_declare}
        #include "cxx-qt-gen/{cxx_file_stem}.cxx.h"

        {extern_cxx_qt}
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
        let block = create_block("block", &["line1".to_string(), "line2".to_string()]);
        let expected = indoc! {"
        block:
          line1
          line2
        "};
        assert_str_eq!(block, expected);
    }

    #[test]
    fn test_create_block_with_empty() {
        let block = create_block(
            "block",
            &["line1".to_string(), "".to_string(), "line2".to_string()],
        );
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
