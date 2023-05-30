// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::qobject::GeneratedCppQObjectBlocks;
use crate::{
    generator::cpp::{types::CppType, GeneratedCppQObject},
    parser::{constructor::Constructor, cxxqtdata::ParsedCxxMappings},
    CppFragment,
};

use indoc::formatdoc;
use syn::{Result, Type};

fn default_constructor(
    qobject: &GeneratedCppQObject,
    initializers: String,
) -> GeneratedCppQObjectBlocks {
    GeneratedCppQObjectBlocks {
        methods: vec![CppFragment::Pair {
            header: format!(
                "explicit {class_name}(QObject* parent = nullptr);",
                class_name = qobject.ident
            ),
            source: formatdoc!(
                r#"
            {class_name}::{class_name}(QObject* parent)
              : {base_class}(parent)
              , m_rustObj({namespace_internals}::createRs())
              {initializers}
            {{
            }}
            "#,
                class_name = qobject.ident,
                base_class = qobject.base_class,
                namespace_internals = qobject.namespace_internals,
            ),
        }],
        ..Default::default()
    }
}

fn argument_names(arguments: &[Type]) -> Vec<String> {
    arguments
        .iter()
        .enumerate()
        .map(|(index, _)| format!("arg{index}"))
        .collect()
}

fn expand_arguments(arguments: &[Type], cxx_mappings: &ParsedCxxMappings) -> Result<String> {
    Ok(arguments
        .iter()
        .zip(argument_names(arguments).into_iter())
        .map(|(ty, name)| {
            CppType::from(ty, cxx_mappings).map(|ty| format!("{ty} {name}", ty = ty.as_cxx_ty()))
        })
        .collect::<Result<Vec<_>>>()?
        .join(", "))
}

pub fn generate(
    qobject: &GeneratedCppQObject,
    constructors: &Vec<Constructor>,
    member_initializers: &[String],
    cxx_mappings: &ParsedCxxMappings,
) -> Result<GeneratedCppQObjectBlocks> {
    let initializers = member_initializers
        .iter()
        .map(|initializer| format!(", {initializer}"))
        .collect::<Vec<_>>()
        .join("\n");

    if constructors.is_empty() {
        return Ok(default_constructor(qobject, initializers));
    }

    let mut generated = GeneratedCppQObjectBlocks::default();

    let class_name = qobject.ident.as_str();
    let namespace_internals = &qobject.namespace_internals;
    let base_class = &qobject.base_class;
    for (index, constructor) in constructors.iter().enumerate() {
        let argument_list = expand_arguments(&constructor.arguments, cxx_mappings)?;
        let constructor_argument_names = argument_names(&constructor.arguments);

        generated.methods.push(CppFragment::Pair {
            header: format!("explicit {class_name}({argument_list});"),
            source: formatdoc! {
                r#"
                {class_name}::{class_name}({argument_list})
                    : {class_name}({namespace_internals}::routeArguments{index}({move_arguments}))
                    {{ }}
                "#,
                move_arguments = constructor_argument_names.iter().map(|arg| format!("std::move({arg})")).collect::<Vec<_>>().join(", "),
            },
        });

        let base_args = if let Some(base_args) = &constructor.base_arguments {
            argument_names(base_args)
                .into_iter()
                .map(|arg| format!("std::move(args.baseArguments.{arg})"))
                .collect::<Vec<_>>()
                .join(", ")
        } else {
            "".to_string()
        };
        // For each constructor defined in CXX-Qt we need a pair of one public and one private
        // constructor.
        // The reason for this is that CXX-Qt needs to be able to route the list of raw arguments
        // provided in C++ to a Plain-Old-Data type that contains the arguments already routed
        // through Rust.
        // This second constructor which takes the routed arguments is private, so that only CXX-Qt
        // can use it.
        generated.private_methods.push(CppFragment::Pair {
            header: format!(
                "explicit {class_name}({namespace_internals}::CxxQtConstructorArguments{index}&& args);"
            ),
            source: formatdoc! {
                r#"
                {class_name}::{class_name}({namespace_internals}::CxxQtConstructorArguments{index}&& args)
                    : {base_class}({base_args})
                    , m_rustObj({namespace_internals}::newRs{index}(std::move(args.newArguments)))
                    {initializers}
                    {{
                        {namespace_internals}::initialize{index}(*this, std::move(args.initializeArguments));
                    }}
                "#,
            },
        })
    }

    Ok(generated)
}
