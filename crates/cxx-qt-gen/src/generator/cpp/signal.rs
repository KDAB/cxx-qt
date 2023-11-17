// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{
    generator::{
        cpp::{fragment::CppFragment, qobject::GeneratedCppQObjectBlocks},
        naming::{
            qobject::QObjectName,
            signals::{QSignalHelperName, QSignalName},
        },
        utils::cpp::syn_type_to_cpp_type,
    },
    parser::{
        mappings::ParsedCxxMappings, parameter::ParsedFunctionParameter, signals::ParsedSignal,
    },
};
use indoc::formatdoc;
use std::collections::BTreeSet;
use syn::{Ident, Result};

#[derive(Default)]
pub struct CppSignalFragment {
    /// List of includes
    pub includes: BTreeSet<String>,
    /// List of forward declares that go before the CXX include
    pub forward_declares: Vec<String>,
    /// List of fragments that go at the top of the header or source
    ///
    /// Note that these should include their namespace
    pub fragments: Vec<CppFragment>,
    /// Any methods for the class
    pub methods: Vec<CppFragment>,
}

/// Combined output of possible parameter lines to be used
struct Parameters {
    /// name with type of parameters
    named_types: String,
    /// name with type of parameters including self
    named_types_with_self: String,
    /// Raw types of the parameters including self
    types_with_self: String,
    /// Raw ::std::move values of the parameters including self
    values_with_self: String,
}

/// From given parameters, mappings, and self value constructor the combined parameter lines
fn parameter_types_and_values(
    parameters: &[ParsedFunctionParameter],
    cxx_mappings: &ParsedCxxMappings,
    self_ty: &str,
) -> Result<Parameters> {
    let mut parameter_named_types_with_self = vec![];
    let mut parameter_types_with_self = vec![];
    let mut parameter_values_with_self = vec![];

    for parameter in parameters {
        let cxx_ty = syn_type_to_cpp_type(&parameter.ty, cxx_mappings)?;
        let ident_str = parameter.ident.to_string();
        parameter_named_types_with_self.push(format!("{cxx_ty} {ident_str}",));
        parameter_types_with_self.push(cxx_ty.clone());
        parameter_values_with_self.push(format!("::std::move({ident_str})"));
    }

    let parameter_named_types = parameter_named_types_with_self.join(", ");

    // Insert the extra argument into the closure
    let self_ty = cxx_mappings.cxx(self_ty);
    parameter_named_types_with_self.insert(0, format!("{self_ty}& self"));
    parameter_types_with_self.insert(0, format!("{self_ty}&"));
    parameter_values_with_self.insert(0, "self".to_owned());

    Ok(Parameters {
        named_types: parameter_named_types,
        named_types_with_self: parameter_named_types_with_self.join(", "),
        types_with_self: parameter_types_with_self.join(", "),
        values_with_self: parameter_values_with_self.join(", "),
    })
}

pub fn generate_cpp_signal(
    signal: &ParsedSignal,
    qobject_ident: &Ident,
    cxx_mappings: &ParsedCxxMappings,
) -> Result<CppSignalFragment> {
    let mut generated = CppSignalFragment::default();

    // Add the include we need
    generated
        .includes
        .insert("#include <cxx-qt-common/cxxqt_signalhandler.h>".to_owned());

    // Build a namespace that includes any namespace for the T
    let qobject_ident_str = qobject_ident.to_string();
    let qobject_ident_namespaced = cxx_mappings.cxx(&qobject_ident_str);

    // Prepare the idents
    let idents = QSignalName::from(signal);
    let idents_helper = QSignalHelperName::new(&idents, qobject_ident, cxx_mappings);

    let signal_ident = idents.name.cpp;
    let free_connect_ident_cpp = idents_helper.connect_name.cpp;

    // Retrieve the parameters for the signal
    let parameters =
        parameter_types_and_values(&signal.parameters, cxx_mappings, &qobject_ident_namespaced)?;
    let parameters_named_types = parameters.named_types;
    let parameters_named_types_with_self = parameters.named_types_with_self;
    let parameter_types_with_self = parameters.types_with_self;
    let parameter_values_with_self = parameters.values_with_self;

    let param_struct = idents_helper.struct_param;
    let signal_handler_alias = idents_helper.handler_alias;
    let signal_handler_alias_namespaced = idents_helper.handler_alias_namespaced;
    let signal_handler_call = idents_helper.function_call;
    let signal_handler_drop = idents_helper.function_drop;
    let namespace = idents_helper.namespace;

    let signal_handler_type = format!("SignalHandler<::{namespace}::{param_struct} *>");

    generated.forward_declares.push(formatdoc! {
        r#"
        namespace {namespace} {{
        using {signal_handler_alias} = ::rust::cxxqtlib1::SignalHandler<struct {param_struct} *>;
        }} // namespace {namespace}
        "#
    });

    // Generate the Q_SIGNAL if this is not an existing signal
    if !signal.inherit {
        generated.methods.push(CppFragment::Header(format!(
            "Q_SIGNAL void {signal_ident}({parameters_named_types});"
        )));
    }

    generated.fragments.push(CppFragment::Pair {
        header: formatdoc! {
        r#"
            namespace {namespace} {{
            ::QMetaObject::Connection
            {free_connect_ident_cpp}({qobject_ident_namespaced}& self, {signal_handler_alias_namespaced} closure, ::Qt::ConnectionType type);
            }} // namespace {namespace}
            "#
        },
        source: formatdoc! {
            r#"
            // Define namespace otherwise we hit a GCC bug
            // https://gcc.gnu.org/bugzilla/show_bug.cgi?id=56480
            namespace rust::cxxqtlib1 {{
            template <>
            {signal_handler_type}::~SignalHandler() noexcept
            {{
                if (data[0] == nullptr && data[1] == nullptr)
                {{
                    return;
                }}

                {signal_handler_drop}(::std::move(*this));
            }}

            template <>
            template <>
            void {signal_handler_type}::operator()<{parameter_types_with_self}>({parameters_named_types_with_self})
            {{
                {signal_handler_call}(*this, {parameter_values_with_self});
            }}

            static_assert(alignof({signal_handler_type}) <= alignof(::std::size_t), "unexpected aligment");
            static_assert(sizeof({signal_handler_type}) == sizeof(::std::size_t[2]), "unexpected size");
            }} // namespace rust::cxxqtlib1

            namespace {namespace} {{
            ::QMetaObject::Connection
            {free_connect_ident_cpp}({qobject_ident_namespaced}& self, {signal_handler_alias_namespaced} closure, ::Qt::ConnectionType type)
            {{
                return ::QObject::connect(
                    &self,
                    &{qobject_ident_namespaced}::{signal_ident},
                    &self,
                    [&, closure = ::std::move(closure)]({parameters_named_types}) mutable {{
                        const ::rust::cxxqtlib1::MaybeLockGuard<{qobject_ident_namespaced}> guard(self);
                        closure.template operator()<{parameter_types_with_self}>({parameter_values_with_self});
                    }},
                    type);
            }}
            }} // namespace {namespace}
        "#,
        }
    });

    Ok(generated)
}

pub fn generate_cpp_signals(
    signals: &Vec<ParsedSignal>,
    qobject_idents: &QObjectName,
    cxx_mappings: &ParsedCxxMappings,
) -> Result<GeneratedCppQObjectBlocks> {
    let mut generated = GeneratedCppQObjectBlocks::default();
    let qobject_ident = &qobject_idents.cpp_class.cpp;

    for signal in signals {
        let mut block = GeneratedCppQObjectBlocks::default();
        let data = generate_cpp_signal(signal, qobject_ident, cxx_mappings)?;
        block.includes = data.includes;
        block.forward_declares_namespaced = data.forward_declares;
        block.fragments = data.fragments;
        block.methods = data.methods;
        generated.append(&mut block);
    }

    Ok(generated)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::generator::naming::{qobject::tests::create_qobjectname, CombinedIdent};
    use crate::parser::parameter::ParsedFunctionParameter;
    use indoc::indoc;
    use pretty_assertions::assert_str_eq;
    use quote::format_ident;
    use syn::parse_quote;

    #[test]
    fn test_generate_cpp_signals() {
        let signals = vec![ParsedSignal {
            method: parse_quote! {
                fn data_changed(self: Pin<&mut MyObject>, trivial: i32, opaque: UniquePtr<QColor>);
            },
            qobject_ident: format_ident!("MyObject"),
            mutable: true,
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
            ident: CombinedIdent {
                cpp: format_ident!("dataChanged"),
                rust: format_ident!("data_changed"),
            },
            safe: true,
            inherit: false,
            private: false,
        }];
        let qobject_idents = create_qobjectname();

        let generated =
            generate_cpp_signals(&signals, &qobject_idents, &ParsedCxxMappings::default()).unwrap();

        assert_eq!(generated.methods.len(), 1);
        let header = if let CppFragment::Header(header) = &generated.methods[0] {
            header
        } else {
            panic!("Expected header")
        };
        assert_str_eq!(
            header,
            "Q_SIGNAL void dataChanged(::std::int32_t trivial, ::std::unique_ptr<QColor> opaque);"
        );

        assert_eq!(generated.fragments.len(), 1);
        let (header, source) = if let CppFragment::Pair { header, source } = &generated.fragments[0]
        {
            (header, source)
        } else {
            panic!("Expected Pair")
        };
        assert_str_eq!(
            header,
            indoc! {r#"
            namespace rust::cxxqtgen1 {
            ::QMetaObject::Connection
            MyObject_dataChangedConnect(MyObject& self, ::rust::cxxqtgen1::MyObjectCxxQtSignalHandlerdataChanged closure, ::Qt::ConnectionType type);
            } // namespace rust::cxxqtgen1
            "#}
        );
        assert_str_eq!(
            source,
            indoc! {r#"
            // Define namespace otherwise we hit a GCC bug
            // https://gcc.gnu.org/bugzilla/show_bug.cgi?id=56480
            namespace rust::cxxqtlib1 {
            template <>
            SignalHandler<::rust::cxxqtgen1::MyObjectCxxQtSignalParamsdataChanged *>::~SignalHandler() noexcept
            {
                if (data[0] == nullptr && data[1] == nullptr)
                {
                    return;
                }

                drop_MyObject_signal_handler_dataChanged(::std::move(*this));
            }

            template <>
            template <>
            void SignalHandler<::rust::cxxqtgen1::MyObjectCxxQtSignalParamsdataChanged *>::operator()<MyObject&, ::std::int32_t, ::std::unique_ptr<QColor>>(MyObject& self, ::std::int32_t trivial, ::std::unique_ptr<QColor> opaque)
            {
                call_MyObject_signal_handler_dataChanged(*this, self, ::std::move(trivial), ::std::move(opaque));
            }

            static_assert(alignof(SignalHandler<::rust::cxxqtgen1::MyObjectCxxQtSignalParamsdataChanged *>) <= alignof(::std::size_t), "unexpected aligment");
            static_assert(sizeof(SignalHandler<::rust::cxxqtgen1::MyObjectCxxQtSignalParamsdataChanged *>) == sizeof(::std::size_t[2]), "unexpected size");
            } // namespace rust::cxxqtlib1

            namespace rust::cxxqtgen1 {
            ::QMetaObject::Connection
            MyObject_dataChangedConnect(MyObject& self, ::rust::cxxqtgen1::MyObjectCxxQtSignalHandlerdataChanged closure, ::Qt::ConnectionType type)
            {
                return ::QObject::connect(
                    &self,
                    &MyObject::dataChanged,
                    &self,
                    [&, closure = ::std::move(closure)](::std::int32_t trivial, ::std::unique_ptr<QColor> opaque) mutable {
                        const ::rust::cxxqtlib1::MaybeLockGuard<MyObject> guard(self);
                        closure.template operator()<MyObject&, ::std::int32_t, ::std::unique_ptr<QColor>>(self, ::std::move(trivial), ::std::move(opaque));
                    },
                    type);
            }
            } // namespace rust::cxxqtgen1
            "#}
        );
    }

    #[test]
    fn test_generate_cpp_signals_mapped_cxx_name() {
        let signals = vec![ParsedSignal {
            method: parse_quote! {
                fn data_changed(self: Pin<&mut MyObject>, mapped: A1);
            },
            qobject_ident: format_ident!("MyObject"),
            mutable: true,
            parameters: vec![ParsedFunctionParameter {
                ident: format_ident!("mapped"),
                ty: parse_quote! { A1 },
            }],
            ident: CombinedIdent {
                cpp: format_ident!("dataChanged"),
                rust: format_ident!("data_changed"),
            },
            safe: true,
            inherit: false,
            private: false,
        }];
        let qobject_idents = create_qobjectname();

        let mut cxx_mappings = ParsedCxxMappings::default();
        cxx_mappings
            .cxx_names
            .insert("A".to_owned(), "A1".to_owned());

        let generated = generate_cpp_signals(&signals, &qobject_idents, &cxx_mappings).unwrap();

        assert_eq!(generated.methods.len(), 1);
        let header = if let CppFragment::Header(header) = &generated.methods[0] {
            header
        } else {
            panic!("Expected header")
        };
        assert_str_eq!(header, "Q_SIGNAL void dataChanged(A1 mapped);");

        assert_eq!(generated.fragments.len(), 1);
        let (header, source) = if let CppFragment::Pair { header, source } = &generated.fragments[0]
        {
            (header, source)
        } else {
            panic!("Expected Pair")
        };
        assert_str_eq!(
            header,
            indoc! {r#"
            namespace rust::cxxqtgen1 {
            ::QMetaObject::Connection
            MyObject_dataChangedConnect(MyObject& self, ::rust::cxxqtgen1::MyObjectCxxQtSignalHandlerdataChanged closure, ::Qt::ConnectionType type);
            } // namespace rust::cxxqtgen1
            "#}
        );
        assert_str_eq!(
            source,
            indoc! {r#"
            // Define namespace otherwise we hit a GCC bug
            // https://gcc.gnu.org/bugzilla/show_bug.cgi?id=56480
            namespace rust::cxxqtlib1 {
            template <>
            SignalHandler<::rust::cxxqtgen1::MyObjectCxxQtSignalParamsdataChanged *>::~SignalHandler() noexcept
            {
                if (data[0] == nullptr && data[1] == nullptr)
                {
                    return;
                }

                drop_MyObject_signal_handler_dataChanged(::std::move(*this));
            }

            template <>
            template <>
            void SignalHandler<::rust::cxxqtgen1::MyObjectCxxQtSignalParamsdataChanged *>::operator()<MyObject&, A1>(MyObject& self, A1 mapped)
            {
                call_MyObject_signal_handler_dataChanged(*this, self, ::std::move(mapped));
            }

            static_assert(alignof(SignalHandler<::rust::cxxqtgen1::MyObjectCxxQtSignalParamsdataChanged *>) <= alignof(::std::size_t), "unexpected aligment");
            static_assert(sizeof(SignalHandler<::rust::cxxqtgen1::MyObjectCxxQtSignalParamsdataChanged *>) == sizeof(::std::size_t[2]), "unexpected size");
            } // namespace rust::cxxqtlib1

            namespace rust::cxxqtgen1 {
            ::QMetaObject::Connection
            MyObject_dataChangedConnect(MyObject& self, ::rust::cxxqtgen1::MyObjectCxxQtSignalHandlerdataChanged closure, ::Qt::ConnectionType type)
            {
                return ::QObject::connect(
                    &self,
                    &MyObject::dataChanged,
                    &self,
                    [&, closure = ::std::move(closure)](A1 mapped) mutable {
                        const ::rust::cxxqtlib1::MaybeLockGuard<MyObject> guard(self);
                        closure.template operator()<MyObject&, A1>(self, ::std::move(mapped));
                    },
                    type);
            }
            } // namespace rust::cxxqtgen1
            "#}
        );
    }

    #[test]
    fn test_generate_cpp_signals_existing_cxx_name() {
        let signals = vec![ParsedSignal {
            method: parse_quote! {
                #[cxx_name = "baseName"]
                fn existing_signal(self: Pin<&mut MyObject>);
            },
            qobject_ident: format_ident!("MyObject"),
            mutable: true,
            parameters: vec![],
            ident: CombinedIdent {
                cpp: format_ident!("baseName"),
                rust: format_ident!("existing_signal"),
            },
            safe: true,
            inherit: true,
            private: false,
        }];
        let qobject_idents = create_qobjectname();

        let generated =
            generate_cpp_signals(&signals, &qobject_idents, &ParsedCxxMappings::default()).unwrap();

        assert_eq!(generated.methods.len(), 0);
        assert_eq!(generated.fragments.len(), 1);

        let (header, source) = if let CppFragment::Pair { header, source } = &generated.fragments[0]
        {
            (header, source)
        } else {
            panic!("Expected Pair")
        };
        assert_str_eq!(
            header,
            indoc! {r#"
            namespace rust::cxxqtgen1 {
            ::QMetaObject::Connection
            MyObject_baseNameConnect(MyObject& self, ::rust::cxxqtgen1::MyObjectCxxQtSignalHandlerbaseName closure, ::Qt::ConnectionType type);
            } // namespace rust::cxxqtgen1
            "#}
        );
        assert_str_eq!(
            source,
            indoc! {r#"
            // Define namespace otherwise we hit a GCC bug
            // https://gcc.gnu.org/bugzilla/show_bug.cgi?id=56480
            namespace rust::cxxqtlib1 {
            template <>
            SignalHandler<::rust::cxxqtgen1::MyObjectCxxQtSignalParamsbaseName *>::~SignalHandler() noexcept
            {
                if (data[0] == nullptr && data[1] == nullptr)
                {
                    return;
                }

                drop_MyObject_signal_handler_baseName(::std::move(*this));
            }

            template <>
            template <>
            void SignalHandler<::rust::cxxqtgen1::MyObjectCxxQtSignalParamsbaseName *>::operator()<MyObject&>(MyObject& self)
            {
                call_MyObject_signal_handler_baseName(*this, self);
            }

            static_assert(alignof(SignalHandler<::rust::cxxqtgen1::MyObjectCxxQtSignalParamsbaseName *>) <= alignof(::std::size_t), "unexpected aligment");
            static_assert(sizeof(SignalHandler<::rust::cxxqtgen1::MyObjectCxxQtSignalParamsbaseName *>) == sizeof(::std::size_t[2]), "unexpected size");
            } // namespace rust::cxxqtlib1

            namespace rust::cxxqtgen1 {
            ::QMetaObject::Connection
            MyObject_baseNameConnect(MyObject& self, ::rust::cxxqtgen1::MyObjectCxxQtSignalHandlerbaseName closure, ::Qt::ConnectionType type)
            {
                return ::QObject::connect(
                    &self,
                    &MyObject::baseName,
                    &self,
                    [&, closure = ::std::move(closure)]() mutable {
                        const ::rust::cxxqtlib1::MaybeLockGuard<MyObject> guard(self);
                        closure.template operator()<MyObject&>(self);
                    },
                    type);
            }
            } // namespace rust::cxxqtgen1
            "#}
        );
    }

    #[test]
    fn test_generate_cpp_signal_free() {
        let signal = ParsedSignal {
            method: parse_quote! {
                fn signal_rust_name(self: Pin<&mut ObjRust>);
            },
            qobject_ident: format_ident!("ObjRust"),
            mutable: true,
            parameters: vec![],
            ident: CombinedIdent {
                cpp: format_ident!("signalRustName"),
                rust: format_ident!("signal_rust_name"),
            },
            safe: true,
            inherit: true,
            private: false,
        };

        let generated = generate_cpp_signal(
            &signal,
            &signal.qobject_ident,
            &ParsedCxxMappings::default(),
        )
        .unwrap();

        assert_eq!(generated.methods.len(), 0);

        assert_eq!(generated.fragments.len(), 1);
        let (header, source) = if let CppFragment::Pair { header, source } = &generated.fragments[0]
        {
            (header, source)
        } else {
            panic!("Expected Pair")
        };

        assert_str_eq!(
            header,
            indoc! {
            r#"
            namespace rust::cxxqtgen1 {
            ::QMetaObject::Connection
            ObjRust_signalRustNameConnect(ObjRust& self, ::rust::cxxqtgen1::ObjRustCxxQtSignalHandlersignalRustName closure, ::Qt::ConnectionType type);
            } // namespace rust::cxxqtgen1
            "#}
        );
        assert_str_eq!(
            source,
            indoc! {r#"
            // Define namespace otherwise we hit a GCC bug
            // https://gcc.gnu.org/bugzilla/show_bug.cgi?id=56480
            namespace rust::cxxqtlib1 {
            template <>
            SignalHandler<::rust::cxxqtgen1::ObjRustCxxQtSignalParamssignalRustName *>::~SignalHandler() noexcept
            {
                if (data[0] == nullptr && data[1] == nullptr)
                {
                    return;
                }

                drop_ObjRust_signal_handler_signalRustName(::std::move(*this));
            }

            template <>
            template <>
            void SignalHandler<::rust::cxxqtgen1::ObjRustCxxQtSignalParamssignalRustName *>::operator()<ObjRust&>(ObjRust& self)
            {
                call_ObjRust_signal_handler_signalRustName(*this, self);
            }

            static_assert(alignof(SignalHandler<::rust::cxxqtgen1::ObjRustCxxQtSignalParamssignalRustName *>) <= alignof(::std::size_t), "unexpected aligment");
            static_assert(sizeof(SignalHandler<::rust::cxxqtgen1::ObjRustCxxQtSignalParamssignalRustName *>) == sizeof(::std::size_t[2]), "unexpected size");
            } // namespace rust::cxxqtlib1

            namespace rust::cxxqtgen1 {
            ::QMetaObject::Connection
            ObjRust_signalRustNameConnect(ObjRust& self, ::rust::cxxqtgen1::ObjRustCxxQtSignalHandlersignalRustName closure, ::Qt::ConnectionType type)
            {
                return ::QObject::connect(
                    &self,
                    &ObjRust::signalRustName,
                    &self,
                    [&, closure = ::std::move(closure)]() mutable {
                        const ::rust::cxxqtlib1::MaybeLockGuard<ObjRust> guard(self);
                        closure.template operator()<ObjRust&>(self);
                    },
                    type);
            }
            } // namespace rust::cxxqtgen1
            "#}
        );
    }

    #[test]
    fn test_generate_cpp_signal_free_mapped() {
        let signal = ParsedSignal {
            method: parse_quote! {
                #[cxx_name = "signalCxxName"]
                fn signal_rust_name(self: Pin<&mut ObjRust>);
            },
            qobject_ident: format_ident!("ObjRust"),
            mutable: true,
            parameters: vec![],
            ident: CombinedIdent {
                cpp: format_ident!("signalCxxName"),
                rust: format_ident!("signal_rust_name"),
            },
            safe: true,
            inherit: true,
            private: false,
        };

        let mut cxx_mappings = ParsedCxxMappings::default();
        cxx_mappings
            .cxx_names
            .insert("ObjRust".to_owned(), "ObjCpp".to_owned());
        cxx_mappings
            .namespaces
            .insert("ObjRust".to_owned(), "mynamespace".to_owned());

        let generated = generate_cpp_signal(&signal, &signal.qobject_ident, &cxx_mappings).unwrap();

        assert_eq!(generated.methods.len(), 0);

        assert_eq!(generated.fragments.len(), 1);
        let (header, source) = if let CppFragment::Pair { header, source } = &generated.fragments[0]
        {
            (header, source)
        } else {
            panic!("Expected Pair")
        };

        assert_str_eq!(
            header,
            indoc! {
            r#"
            namespace rust::cxxqtgen1::mynamespace {
            ::QMetaObject::Connection
            ObjRust_signalCxxNameConnect(::mynamespace::ObjCpp& self, ::rust::cxxqtgen1::mynamespace::ObjRustCxxQtSignalHandlersignalCxxName closure, ::Qt::ConnectionType type);
            } // namespace rust::cxxqtgen1::mynamespace
            "#}
        );
        assert_str_eq!(
            source,
            indoc! {r#"
            // Define namespace otherwise we hit a GCC bug
            // https://gcc.gnu.org/bugzilla/show_bug.cgi?id=56480
            namespace rust::cxxqtlib1 {
            template <>
            SignalHandler<::rust::cxxqtgen1::mynamespace::ObjRustCxxQtSignalParamssignalCxxName *>::~SignalHandler() noexcept
            {
                if (data[0] == nullptr && data[1] == nullptr)
                {
                    return;
                }

                drop_ObjRust_signal_handler_signalCxxName(::std::move(*this));
            }

            template <>
            template <>
            void SignalHandler<::rust::cxxqtgen1::mynamespace::ObjRustCxxQtSignalParamssignalCxxName *>::operator()<::mynamespace::ObjCpp&>(::mynamespace::ObjCpp& self)
            {
                call_ObjRust_signal_handler_signalCxxName(*this, self);
            }

            static_assert(alignof(SignalHandler<::rust::cxxqtgen1::mynamespace::ObjRustCxxQtSignalParamssignalCxxName *>) <= alignof(::std::size_t), "unexpected aligment");
            static_assert(sizeof(SignalHandler<::rust::cxxqtgen1::mynamespace::ObjRustCxxQtSignalParamssignalCxxName *>) == sizeof(::std::size_t[2]), "unexpected size");
            } // namespace rust::cxxqtlib1

            namespace rust::cxxqtgen1::mynamespace {
            ::QMetaObject::Connection
            ObjRust_signalCxxNameConnect(::mynamespace::ObjCpp& self, ::rust::cxxqtgen1::mynamespace::ObjRustCxxQtSignalHandlersignalCxxName closure, ::Qt::ConnectionType type)
            {
                return ::QObject::connect(
                    &self,
                    &::mynamespace::ObjCpp::signalCxxName,
                    &self,
                    [&, closure = ::std::move(closure)]() mutable {
                        const ::rust::cxxqtlib1::MaybeLockGuard<::mynamespace::ObjCpp> guard(self);
                        closure.template operator()<::mynamespace::ObjCpp&>(self);
                    },
                    type);
            }
            } // namespace rust::cxxqtgen1::mynamespace
            "#}
        );
    }
}
