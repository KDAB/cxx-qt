// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{
    generator::{
        cpp::{fragment::CppFragment, qobject::GeneratedCppQObjectBlocks, types::CppType},
        naming::{qobject::QObjectName, signals::QSignalName},
    },
    parser::{cxxqtdata::ParsedCxxMappings, signals::ParsedSignal},
};
use indoc::formatdoc;
use syn::Result;

pub fn generate_cpp_signals(
    signals: &Vec<ParsedSignal>,
    qobject_idents: &QObjectName,
    cxx_mappings: &ParsedCxxMappings,
    lock_guard: Option<&str>,
) -> Result<GeneratedCppQObjectBlocks> {
    let mut generated = GeneratedCppQObjectBlocks::default();
    let qobject_ident = qobject_idents.cpp_class.cpp.to_string();

    for signal in signals {
        // Generate the parameters
        let mut parameter_types_cpp = vec![];
        let mut parameter_values_emitter = vec![];

        for parameter in &signal.parameters {
            let cxx_ty = CppType::from(&parameter.ty, cxx_mappings)?;
            let ident_str = parameter.ident.to_string();
            parameter_types_cpp.push(format!(
                "{cxx_ty} {ident}",
                ident = parameter.ident,
                cxx_ty = cxx_ty.as_cxx_ty(),
            ));
            parameter_values_emitter.push(format!("::std::move({ident})", ident = ident_str,));
        }

        // Prepare the idents
        let idents = QSignalName::from(signal);
        let emit_ident = idents.emit_name.cpp.to_string();
        let signal_ident = idents.name.cpp.to_string();
        let connect_ident = idents.connect_name.cpp.to_string();

        // Generate the Q_SIGNAL if this is not an existing signal
        if !signal.inherit {
            generated.methods.push(CppFragment::Header(format!(
                "Q_SIGNAL void {ident}({parameters});",
                ident = signal_ident,
                parameters = parameter_types_cpp.join(", "),
            )));
        }

        // Generate the emitters
        generated.methods.push(CppFragment::Pair {
            header: format!(
                "void {ident}({parameters});",
                ident = emit_ident,
                parameters = parameter_types_cpp.join(", "),
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
                parameters = parameter_types_cpp.join(", "),
                parameter_values = parameter_values_emitter.join(", "),
                emit_ident = emit_ident,
                qobject_ident = qobject_ident,
            },
        });

        // Generate connection
        let mut parameter_types_rust = parameter_types_cpp.clone();
        let mut parameter_values_connection = parameter_values_emitter.clone();
        parameter_types_rust.insert(0, format!("{qobject_ident}&"));
        parameter_values_connection.insert(0, "*this".to_owned());

        generated.methods.push(CppFragment::Pair {
            header: format!(
                "::QMetaObject::Connection {connect_ident}(::rust::Fn<void({parameters})> func, ::Qt::ConnectionType type);",
                parameters = parameter_types_rust.join(", ")
            ),
            source: formatdoc! {
                r#"
                ::QMetaObject::Connection
                {qobject_ident}::{connect_ident}(::rust::Fn<void({parameters_rust})> func, ::Qt::ConnectionType type)
                {{
                    return ::QObject::connect(this,
                            &{qobject_ident}::{signal_ident},
                            this,
                            [&, func = ::std::move(func)]({parameters_cpp}) {{
                              {rust_obj_guard}
                              func({parameter_values});
                            }}, type);
                }}
                "#,
                connect_ident = connect_ident,
                parameters_cpp = parameter_types_cpp.join(", "),
                parameters_rust = parameter_types_rust.join(", "),
                parameter_values = parameter_values_connection.join(", "),
                rust_obj_guard = lock_guard.unwrap_or_default(),
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
    use indoc::indoc;
    use pretty_assertions::assert_str_eq;
    use quote::format_ident;
    use syn::parse_quote;

    #[test]
    fn test_generate_cpp_signals() {
        let signals = vec![ParsedSignal {
            ident: format_ident!("data_changed"),
            cxx_name: None,
            inherit: false,
            parameters: vec![
                ParsedFunctionParameter {
                    ident: format_ident!("trivial"),
                    ty: parse_quote! { i32 },
                },
                ParsedFunctionParameter {
                    ident: format_ident!("opaque"),
                    ty: parse_quote! { UniquePtr<QColor> },
                },
            ],
        }];
        let qobject_idents = create_qobjectname();

        let generated = generate_cpp_signals(
            &signals,
            &qobject_idents,
            &ParsedCxxMappings::default(),
            Some("// ::std::lock_guard"),
        )
        .unwrap();

        assert_eq!(generated.methods.len(), 3);
        let header = if let CppFragment::Header(header) = &generated.methods[0] {
            header
        } else {
            panic!("Expected header")
        };
        assert_str_eq!(
            header,
            "Q_SIGNAL void dataChanged(::std::int32_t trivial, ::std::unique_ptr<QColor> opaque);"
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
                Q_EMIT dataChanged(::std::move(trivial), ::std::move(opaque));
            }
            "#}
        );

        let (header, source) = if let CppFragment::Pair { header, source } = &generated.methods[2] {
            (header, source)
        } else {
            panic!("Expected Pair")
        };
        assert_str_eq!(
            header,
            "::QMetaObject::Connection dataChangedConnect(::rust::Fn<void(MyObject&, ::std::int32_t trivial, ::std::unique_ptr<QColor> opaque)> func, ::Qt::ConnectionType type);"
        );
        assert_str_eq!(
            source,
            indoc! {r#"
            ::QMetaObject::Connection
            MyObject::dataChangedConnect(::rust::Fn<void(MyObject&, ::std::int32_t trivial, ::std::unique_ptr<QColor> opaque)> func, ::Qt::ConnectionType type)
            {
                return ::QObject::connect(this,
                        &MyObject::dataChanged,
                        this,
                        [&, func = ::std::move(func)](::std::int32_t trivial, ::std::unique_ptr<QColor> opaque) {
                          // ::std::lock_guard
                          func(*this, ::std::move(trivial), ::std::move(opaque));
                        }, type);
            }
            "#}
        );
    }

    #[test]
    fn test_generate_cpp_signals_mapped_cxx_name() {
        let signals = vec![ParsedSignal {
            ident: format_ident!("data_changed"),
            cxx_name: None,
            inherit: false,
            parameters: vec![ParsedFunctionParameter {
                ident: format_ident!("mapped"),
                ty: parse_quote! { A1 },
            }],
        }];
        let qobject_idents = create_qobjectname();

        let mut cxx_mappings = ParsedCxxMappings::default();
        cxx_mappings
            .cxx_names
            .insert("A".to_owned(), "A1".to_owned());

        let generated = generate_cpp_signals(
            &signals,
            &qobject_idents,
            &cxx_mappings,
            Some("// ::std::lock_guard"),
        )
        .unwrap();

        assert_eq!(generated.methods.len(), 3);
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
                Q_EMIT dataChanged(::std::move(mapped));
            }
            "#}
        );

        let (header, source) = if let CppFragment::Pair { header, source } = &generated.methods[2] {
            (header, source)
        } else {
            panic!("Expected Pair")
        };
        assert_str_eq!(
            header,
            "::QMetaObject::Connection dataChangedConnect(::rust::Fn<void(MyObject&, A1 mapped)> func, ::Qt::ConnectionType type);"
        );
        assert_str_eq!(
            source,
            indoc! {r#"
            ::QMetaObject::Connection
            MyObject::dataChangedConnect(::rust::Fn<void(MyObject&, A1 mapped)> func, ::Qt::ConnectionType type)
            {
                return ::QObject::connect(this,
                        &MyObject::dataChanged,
                        this,
                        [&, func = ::std::move(func)](A1 mapped) {
                          // ::std::lock_guard
                          func(*this, ::std::move(mapped));
                        }, type);
            }
            "#}
        );
    }

    #[test]
    fn test_generate_cpp_signals_existing_cxx_name() {
        let signals = vec![ParsedSignal {
            ident: format_ident!("ExistingSignal"),
            cxx_name: Some("baseName".to_owned()),
            inherit: true,
            parameters: vec![],
        }];
        let qobject_idents = create_qobjectname();

        let generated = generate_cpp_signals(
            &signals,
            &qobject_idents,
            &ParsedCxxMappings::default(),
            Some("// ::std::lock_guard"),
        )
        .unwrap();

        assert_eq!(generated.methods.len(), 2);
        let (header, source) = if let CppFragment::Pair { header, source } = &generated.methods[0] {
            (header, source)
        } else {
            panic!("Expected Pair")
        };
        assert_str_eq!(header, "void emitBaseName();");
        assert_str_eq!(
            source,
            indoc! {r#"
            void
            MyObject::emitBaseName()
            {
                Q_EMIT baseName();
            }
            "#}
        );

        let (header, source) = if let CppFragment::Pair { header, source } = &generated.methods[1] {
            (header, source)
        } else {
            panic!("Expected Pair")
        };
        assert_str_eq!(header, "::QMetaObject::Connection baseNameConnect(::rust::Fn<void(MyObject&)> func, ::Qt::ConnectionType type);");
        assert_str_eq!(
            source,
            indoc! {r#"
            ::QMetaObject::Connection
            MyObject::baseNameConnect(::rust::Fn<void(MyObject&)> func, ::Qt::ConnectionType type)
            {
                return ::QObject::connect(this,
                        &MyObject::baseName,
                        this,
                        [&, func = ::std::move(func)]() {
                          // ::std::lock_guard
                          func(*this);
                        }, type);
            }
            "#}
        );
    }
}
