// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::collections::BTreeSet;

use indoc::formatdoc;
use syn::Result;

use crate::{
    generator::{cfg::try_eval_attributes, GeneratedOpt},
    parser::qenum::ParsedQEnum,
    writer::cpp::namespaced,
};

use super::{qobject::GeneratedCppQObjectBlocks, utils::Indent};

fn generate_definition(qenum: &ParsedQEnum) -> String {
    let enum_name = &qenum.name.cxx_unqualified();

    let enum_values = qenum
        .variants
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(",\n");

    formatdoc! { r#"
        enum class {enum_name} : ::std::int32_t {{
        {enum_values}
        }};
        "#, enum_values = enum_values.indented(2) }
}

pub fn generate_declaration(
    qenum: &ParsedQEnum,
    includes: &mut BTreeSet<String>,
    opt: &GeneratedOpt,
) -> Result<String> {
    // Skip if the cfg attributes are not resolved to true
    if !try_eval_attributes(opt.cfg_evaluator.as_ref(), &qenum.cfgs)? {
        return Ok(String::new());
    }

    let is_standalone = qenum.qobject.is_none();
    if is_standalone {
        // required for Q_NAMESPACE and Q_ENUM_NS if we're not on a QObject
        includes.insert("#include <QtCore/QObject>".to_owned());
    }

    let enum_definition = generate_definition(qenum).indented(2);
    let enum_name = &qenum.name.cxx_unqualified();
    Ok(namespaced(
        qenum.name.namespace().unwrap_or_default(),
        // The declaration must still include Q_NAMESPACE, as otherwise moc will complain.
        // This is redundant with `qnamespace!`, which is now only required if you want to specify
        // it as QML_ELEMENT.
        &if is_standalone {
            formatdoc! {r#"
                Q_NAMESPACE
                {enum_definition}
                Q_ENUM_NS({enum_name}) "# }
        } else {
            enum_definition
        },
    ))
}

pub fn generate_on_qobject<'a>(
    qenums: impl Iterator<Item = &'a ParsedQEnum>,
    opt: &GeneratedOpt,
) -> Result<GeneratedCppQObjectBlocks> {
    let mut generated = GeneratedCppQObjectBlocks::default();

    for qenum in qenums {
        // Skip if the cfg attributes are not resolved to true
        if !try_eval_attributes(opt.cfg_evaluator.as_ref(), &qenum.cfgs)? {
            continue;
        }

        let mut qualified_name = qenum.name.cxx_qualified();
        let enum_name = qenum.name.cxx_unqualified();
        // TODO: this is a workaround for cxx_qualified not returning a fully-qualified
        // identifier.
        // Once https://github.com/KDAB/cxx-qt/issues/619 is fixed, this can be removed.
        if !qualified_name.starts_with("::") {
            qualified_name.insert_str(0, "::");
        }

        generated.includes.insert("#include <cstdint>".to_owned());
        let enum_definition = generate_definition(qenum);
        generated.metaobjects.push(formatdoc! {r#"
            #ifdef Q_MOC_RUN
            {enum_definition}
              Q_ENUM({enum_name})
            #else
              using {enum_name} = {qualified_name};
              Q_ENUM({enum_name})
            #endif
        "#, enum_definition = enum_definition.indented(2)});
    }

    Ok(generated)
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use super::*;
    use crate::tests::CfgEvaluatorTest;
    use indoc::indoc;
    use pretty_assertions::assert_str_eq;
    use quote::format_ident;
    use syn::parse_quote;

    #[test]
    fn test_cfg() {
        let qenums = [ParsedQEnum::parse(
            parse_quote! {
                #[cfg(test_cfg_disabled)]
                enum MyEnum {
                    A, B, C
                }
            },
            Some(format_ident!("MyObject")),
            None,
            &format_ident!("qobject"),
        )
        .unwrap()];
        let opt = GeneratedOpt {
            cfg_evaluator: Box::new(CfgEvaluatorTest::default()),
        };
        let generated = generate_on_qobject(qenums.iter(), &opt).unwrap();

        assert!(generated.methods.is_empty());
    }

    #[test]
    fn generates() {
        let qenums = [ParsedQEnum::parse(
            parse_quote! {
                enum MyEnum {
                    A, B, C
                }
            },
            Some(format_ident!("MyObject")),
            None,
            &format_ident!("qobject"),
        )
        .unwrap()];

        let generated = generate_on_qobject(qenums.iter(), &GeneratedOpt::default()).unwrap();
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
        assert_eq!(generated.forward_declares.len(), 0);
    }
}
