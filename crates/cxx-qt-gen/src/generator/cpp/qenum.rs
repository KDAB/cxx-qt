// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::collections::BTreeSet;

use indoc::formatdoc;
use syn::Result;

use crate::{
    generator::{cfg::try_eval_attributes, GeneratedOpt},
    naming::Name,
    parser::qenum::ParsedQEnum,
    writer::cpp::namespaced,
    CppFragment,
};

use super::{qobject::GeneratedCppQObjectBlocks, utils::Indent};

fn generate_definition(qenum: &ParsedQEnum) -> String {
    let enum_name = &qenum.name.cxx_unqualified();

    let enum_values = qenum
        .variants
        .iter()
        .enumerate()
        .map(|(index, variant)| format!("{variant} = {index}"))
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
    let is_standalone = qenum.qobject.is_none();
    // Skip if the cfg attributes are not resolved to true
    if !is_standalone || !try_eval_attributes(opt.cfg_evaluator.as_ref(), &qenum.cfgs)? {
        return Ok(String::new());
    }

    // required for Q_NAMESPACE and Q_ENUM_NS if we're not on a QObject
    includes.insert("#include <QtCore/QObject>".to_owned());

    let enum_definition = generate_definition(qenum).indented(2);
    let enum_name = &qenum.name.cxx_unqualified();
    Ok(namespaced(
        qenum.name.namespace().unwrap_or_default(),
        // The declaration must still include Q_NAMESPACE, as otherwise moc will complain.
        // This is redundant with `qnamespace!`, which is now only required if you want to specify
        // it as QML_ELEMENT.
        &formatdoc! {r#"
                Q_NAMESPACE
                {enum_definition}
                Q_ENUM_NS({enum_name}) "#
        },
    ))
}

pub fn generate_on_qobject<'a>(
    qenums: impl Iterator<Item = &'a ParsedQEnum>,
    qobject: &Name,
    opt: &GeneratedOpt,
) -> Result<GeneratedCppQObjectBlocks> {
    let mut generated = GeneratedCppQObjectBlocks::default();

    for qenum in qenums {
        // Skip if the cfg attributes are not resolved to true
        if !try_eval_attributes(opt.cfg_evaluator.as_ref(), &qenum.cfgs)? {
            continue;
        }

        let enum_name = qenum.name.cxx_unqualified();
        let mut qobject_name = qobject.cxx_qualified();
        // TODO: this is a workaround for cxx_qualified not returning a fully-qualified
        // identifier.
        // Once https://github.com/KDAB/cxx-qt/issues/619 is fixed, this can be removed.
        if !qobject_name.starts_with("::") {
            qobject_name.insert_str(0, "::");
        }

        generated.includes.insert("#include <cstdint>".to_owned());
        let enum_definition = generate_definition(qenum);
        generated.metaobjects.push(formatdoc! {r#"
            {enum_definition}
              Q_ENUM({enum_name})"#, enum_definition = enum_definition.indented(2)});

        generated
            .post_fragments
            .push(CppFragment::Header(namespaced(
                qenum.name.namespace().unwrap_or_default(),
                &format!("using {enum_name} = {qobject_name}::{enum_name};",),
            )));
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
        let generated = generate_on_qobject(qenums.iter(), &Name::mock("MyObject"), &opt).unwrap();

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

        let generated = generate_on_qobject(
            qenums.iter(),
            &Name::mock("MyObject"),
            &GeneratedOpt::default(),
        )
        .unwrap();
        assert_eq!(generated.includes.len(), 1);
        assert!(generated.includes.contains("#include <cstdint>"));
        assert_eq!(generated.metaobjects.len(), 1);
        assert_str_eq!(
            indoc! {r#"
              enum class MyEnum : ::std::int32_t {
                A = 0,
                B = 1,
                C = 2
              };
              Q_ENUM(MyEnum)
            "#}
            .indented(2),
            generated.metaobjects[0],
        );
        assert_eq!(
            CppFragment::Header("using MyEnum = ::MyObject::MyEnum;".to_owned()),
            generated.post_fragments[0]
        );
        assert_eq!(generated.forward_declares.len(), 0);
    }
}
