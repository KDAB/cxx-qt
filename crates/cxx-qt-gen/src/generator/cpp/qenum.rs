// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use indoc::formatdoc;
use syn::Result;

use crate::{
    generator::utils::cpp::Indent,
    parser::{mappings::ParsedCxxMappings, qenum::ParsedQEnum},
};

use super::qobject::GeneratedCppQObjectBlocks;

pub fn generate(
    qenums: &[ParsedQEnum],
    cxx_mappings: &ParsedCxxMappings,
) -> Result<GeneratedCppQObjectBlocks> {
    let mut generated = GeneratedCppQObjectBlocks::default();

    for qenum in qenums {
        let enum_name = &qenum.ident.to_string();

        let mut qualified_name = cxx_mappings.cxx(enum_name);
        // TODO: this is a workaround for cxx_mappings.cxx not always returning a fully-qualified
        // identifier.
        // Once https://github.com/KDAB/cxx-qt/issues/619 is fixed, this can be removed.
        if !qualified_name.starts_with("::") {
            qualified_name.insert_str(0, "::");
        }

        let enum_values = qenum
            .variants
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(",\n");

        generated.includes.insert("#include <cstdint>".to_string());
        let enum_definition = formatdoc! { r#"
            enum class {enum_name} : ::std::int32_t {{
            {enum_values}
            }};
        "#, enum_values = enum_values.indented(2) };
        generated.forward_declares.push(enum_definition.clone());
        generated.metaobjects.push(formatdoc! {r#"
            #ifdef Q_MOC_RUN
            {enum_definition}
              Q_ENUM({enum_name})
            #else
              using {enum_name} = {qualified_name};
              Q_ENUM({enum_name})
            #endif
        "#, enum_definition = enum_definition.indented(2) });
    }

    Ok(generated)
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_str_eq;
    use syn::parse_quote;

    #[test]
    fn generates() {
        let qenums = [ParsedQEnum::parse(parse_quote! {
            enum MyEnum {
                A, B, C
            }
        })
        .unwrap()];

        let generated = generate(&qenums, &ParsedCxxMappings::default()).unwrap();
        assert_eq!(generated.includes.len(), 1);
        assert!(generated.includes.contains("#include <cstdint>"));
        assert_eq!(generated.metaobjects.len(), 1);
        assert_str_eq!(
            indoc! {r#"
                #ifdef Q_MOC_RUN
                  enum class MyEnum : ::std::int32_t {
                    A,
                    B,
                    C
                  };
                  Q_ENUM(MyEnum)
                #else
                  using MyEnum = ::MyEnum;
                  Q_ENUM(MyEnum)
                #endif
            "#},
            generated.metaobjects[0],
        );
        assert_eq!(generated.forward_declares.len(), 1);
        assert_str_eq!(
            indoc! { r#"
                enum class MyEnum : ::std::int32_t {
                  A,
                  B,
                  C
                };
            "# },
            generated.forward_declares[0],
        );
    }
}
