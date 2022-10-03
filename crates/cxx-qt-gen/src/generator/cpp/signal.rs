// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{
    generator::{
        cpp::{
            fragment::CppFragment, qobject::GeneratedCppQObjectBlocks, types::CppType,
            CXX_QT_CONVERT,
        },
        naming::{qobject::QObjectName, signals::QSignalName},
    },
    parser::signals::ParsedSignal,
};
use indoc::formatdoc;
use std::collections::BTreeMap;
use syn::Result;

pub fn generate_cpp_signals(
    signals: &Vec<ParsedSignal>,
    qobject_idents: &QObjectName,
    cxx_names_map: &BTreeMap<String, String>,
) -> Result<GeneratedCppQObjectBlocks> {
    let mut generated = GeneratedCppQObjectBlocks::default();
    let qobject_ident = qobject_idents.cpp_class.cpp.to_string();

    for signal in signals {
        // Generate the parameters
        let mut parameter_types = vec![];
        let mut parameter_types_emitter = vec![];
        let mut parameter_values = vec![];

        for parameter in &signal.parameters {
            let cxx_ty = CppType::from(&parameter.ty, &parameter.cxx_type, cxx_names_map)?;
            let ident_str = parameter.ident.to_string();
            parameter_types.push(format!(
                "{cxx_ty} {ident}",
                ident = parameter.ident,
                cxx_ty = cxx_ty.as_cxx_ty(),
            ));
            parameter_types_emitter.push(format!(
                "{rust_ty} {ident}",
                ident = parameter.ident,
                rust_ty = cxx_ty.as_rust_ty(),
            ));
            parameter_values.push(format!(
                "{convert}<{cxx_ty}, {rust_ty}>{{}}(std::move({ident}))",
                convert = CXX_QT_CONVERT,
                cxx_ty = cxx_ty.as_cxx_ty(),
                ident = ident_str,
                rust_ty = cxx_ty.as_rust_ty(),
            ));
        }

        // Prepare the idents
        let idents = QSignalName::from(signal);
        let emit_ident = idents.emit_name.cpp.to_string();
        let signal_ident = idents.name.cpp.to_string();

        // Generate the Q_SIGNAL
        generated.methods.push(CppFragment::Header(format!(
            "Q_SIGNAL void {ident}({parameters});",
            ident = signal_ident,
            parameters = parameter_types.join(", "),
        )));

        // Generate the emitters
        generated.methods.push(CppFragment::Pair {
            header: format!(
                "void {ident}({parameters});",
                ident = emit_ident,
                parameters = parameter_types_emitter.join(", "),
            ),
            source: formatdoc! {
                r#"
                void
                {qobject_ident}::{emit_ident}({parameters})
                {{
                    Q_EMIT {ident}({parameter_values});
                }}
                "#,
                ident = signal_ident,
                parameters = parameter_types_emitter.join(", "),
                parameter_values = parameter_values.join(", "),
                emit_ident = emit_ident,
                qobject_ident = qobject_ident,
            },
        });
    }

    Ok(generated)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::generator::naming::qobject::tests::create_qobjectname;
    use crate::parser::parameter::ParsedFunctionParameter;
    use crate::tests::tokens_to_syn;
    use indoc::indoc;
    use pretty_assertions::assert_str_eq;
    use quote::{format_ident, quote};

    #[test]
    fn test_generate_cpp_signals() {
        let signals = vec![ParsedSignal {
            ident: format_ident!("data_changed"),
            parameters: vec![
                ParsedFunctionParameter {
                    ident: format_ident!("trivial"),
                    ty: tokens_to_syn(quote! { i32 }),
                    cxx_type: None,
                },
                ParsedFunctionParameter {
                    ident: format_ident!("opaque"),
                    ty: tokens_to_syn(quote! { UniquePtr<QColor> }),
                    cxx_type: Some("QColor".to_owned()),
                },
            ],
        }];
        let qobject_idents = create_qobjectname();

        let generated =
            generate_cpp_signals(&signals, &qobject_idents, &BTreeMap::default()).unwrap();

        assert_eq!(generated.methods.len(), 2);
        let header = if let CppFragment::Header(header) = &generated.methods[0] {
            header
        } else {
            panic!("Expected header")
        };
        assert_str_eq!(
            header,
            "Q_SIGNAL void dataChanged(::std::int32_t trivial, QColor opaque);"
        );

        let (header, source) = if let CppFragment::Pair { header, source } = &generated.methods[1] {
            (header, source)
        } else {
            panic!("Expected Pair")
        };
        assert_str_eq!(
            header,
            "void emitDataChanged(::std::int32_t trivial, ::std::unique_ptr<QColor> opaque);"
        );
        assert_str_eq!(
            source,
            indoc! {r#"
            void
            MyObject::emitDataChanged(::std::int32_t trivial, ::std::unique_ptr<QColor> opaque)
            {
                Q_EMIT dataChanged(rust::cxxqtlib1::cxx_qt_convert<::std::int32_t, ::std::int32_t>{}(std::move(trivial)), rust::cxxqtlib1::cxx_qt_convert<QColor, ::std::unique_ptr<QColor>>{}(std::move(opaque)));
            }
            "#}
        );
    }

    #[test]
    fn test_generate_cpp_signals_cxx_names_mapped() {
        let signals = vec![ParsedSignal {
            ident: format_ident!("data_changed"),
            parameters: vec![ParsedFunctionParameter {
                ident: format_ident!("mapped"),
                ty: tokens_to_syn(quote! { A1 }),
                cxx_type: None,
            }],
        }];
        let qobject_idents = create_qobjectname();

        let mut cxx_names_map = BTreeMap::new();
        cxx_names_map.insert("A".to_owned(), "A1".to_owned());

        let generated = generate_cpp_signals(&signals, &qobject_idents, &cxx_names_map).unwrap();

        assert_eq!(generated.methods.len(), 2);
        let header = if let CppFragment::Header(header) = &generated.methods[0] {
            header
        } else {
            panic!("Expected header")
        };
        assert_str_eq!(header, "Q_SIGNAL void dataChanged(A1 mapped);");

        let (header, source) = if let CppFragment::Pair { header, source } = &generated.methods[1] {
            (header, source)
        } else {
            panic!("Expected Pair")
        };
        assert_str_eq!(header, "void emitDataChanged(A1 mapped);");
        assert_str_eq!(
            source,
            indoc! {r#"
            void
            MyObject::emitDataChanged(A1 mapped)
            {
                Q_EMIT dataChanged(rust::cxxqtlib1::cxx_qt_convert<A1, A1>{}(std::move(mapped)));
            }
            "#}
        );
    }
}
