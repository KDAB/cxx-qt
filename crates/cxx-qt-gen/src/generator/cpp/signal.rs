// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{
    generator::{
        cpp::{
            fragment::CppFragmentPair, qobject::GeneratedCppQObjectBlocks, types::CppType,
            CXX_QT_CONVERT,
        },
        naming::{qobject::QObjectName, signals::QSignalName},
    },
    parser::signals::ParsedSignal,
};
use indoc::formatdoc;
use syn::Result;

pub fn generate_cpp_signals(
    generated: &mut GeneratedCppQObjectBlocks,
    signals: &Vec<ParsedSignal>,
    qobject_idents: &QObjectName,
) -> Result<()> {
    let qobject_ident = qobject_idents.cpp_class.cpp.to_string();

    for signal in signals {
        // Generate the parameters
        let mut parameter_types = vec![];
        let mut parameter_types_queued = vec![];
        let mut parameter_values = vec![];
        let mut captures = vec!["this".to_owned()];

        for parameter in &signal.parameters {
            let cxx_ty = CppType::from(&parameter.ty, &parameter.cxx_type)?;
            let ident_str = parameter.ident.to_string();
            captures.push(format!("{ident} = std::move({ident})", ident = ident_str));
            parameter_types.push(format!(
                "{cxx_ty} {ident}",
                ident = parameter.ident,
                cxx_ty = cxx_ty.as_cxx_ty(),
            ));
            parameter_types_queued.push(format!(
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
        let queued_ident = idents.queued_name.cpp.to_string();
        let signal_ident = idents.name.cpp.to_string();

        // Generate the Q_SIGNAL
        generated.signals.push(format!(
            "void {ident}({parameters});",
            ident = signal_ident,
            parameters = parameter_types.join(", "),
        ));

        // Generate the queued emitters
        generated.methods.push(CppFragmentPair {
            header: format!(
                "void {ident}({parameters});",
                ident = queued_ident,
                parameters = parameter_types_queued.join(", "),
            ),
            source: formatdoc! {
                r#"
                void
                {qobject_ident}::{queued_ident}({parameters})
                {{
                    const auto signalSuccess = QMetaObject::invokeMethod(
                        this, [{captures}]() mutable {{
                            Q_EMIT {ident}({parameter_values});
                        }}, Qt::QueuedConnection);
                    Q_ASSERT(signalSuccess);
                }}
                "#,
                captures = captures.join(", "),
                ident = signal_ident,
                parameters = parameter_types_queued.join(", "),
                parameter_values = parameter_values.join(", "),
                queued_ident = queued_ident,
                qobject_ident = qobject_ident,
            },
        });
    }

    Ok(())
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
        let mut generated = GeneratedCppQObjectBlocks::default();
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

        assert!(generate_cpp_signals(&mut generated, &signals, &qobject_idents).is_ok());

        assert_eq!(generated.signals.len(), 1);
        assert_str_eq!(
            generated.signals[0],
            "void dataChanged(qint32 trivial, QColor opaque);"
        );

        assert_eq!(generated.methods.len(), 1);
        assert_str_eq!(
            generated.methods[0].header,
            "void emitDataChanged(qint32 trivial, ::std::unique_ptr<QColor> opaque);"
        );
        assert_str_eq!(
            generated.methods[0].source,
            indoc! {r#"
            void
            MyObject::emitDataChanged(qint32 trivial, ::std::unique_ptr<QColor> opaque)
            {
                const auto signalSuccess = QMetaObject::invokeMethod(
                    this, [this, trivial = std::move(trivial), opaque = std::move(opaque)]() mutable {
                        Q_EMIT dataChanged(rust::cxxqtlib1::cxx_qt_convert<qint32, qint32>{}(std::move(trivial)), rust::cxxqtlib1::cxx_qt_convert<QColor, ::std::unique_ptr<QColor>>{}(std::move(opaque)));
                    }, Qt::QueuedConnection);
                Q_ASSERT(signalSuccess);
            }
            "#}
        );
    }
}
