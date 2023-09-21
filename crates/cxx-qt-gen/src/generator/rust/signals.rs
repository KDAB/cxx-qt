// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{
    generator::{
        naming::{
            namespace::namespace_externcxxqt_with_qobject_namespace, qobject::QObjectName,
            signals::QSignalName,
        },
        rust::{
            externcxxqt::GeneratedExternCxxQt, fragment::RustFragmentPair,
            qobject::GeneratedRustQObject,
        },
        utils::rust::{syn_ident_cxx_bridge_to_qualified_impl, syn_type_cxx_bridge_to_qualified},
    },
    parser::{mappings::ParsedCxxMappings, signals::ParsedSignal},
};
use quote::{format_ident, quote};
use syn::{parse_quote, FnArg, Ident, Result};

pub fn generate_rust_free_signal(
    signal: &ParsedSignal,
    cxx_mappings: &ParsedCxxMappings,
    module_ident: &Ident,
) -> Result<GeneratedExternCxxQt> {
    let qobject_name = &signal.qobject_ident;
    let idents = QSignalName::from(signal);
    let signal_name_cpp = idents.name.cpp;
    let signal_name_cpp_str = signal_name_cpp.to_string();
    let free_connect_ident_cpp =
        format_ident!("{}_{}", signal.qobject_ident, idents.connect_name.cpp);
    let free_connect_ident_rust =
        format_ident!("{}_{}", signal.qobject_ident, idents.connect_name.rust);
    let free_connect_ident_rust_str =
        format_ident!("{}_{}", signal.qobject_ident, idents.connect_name.rust).to_string();
    let connect_ident_rust = idents.connect_name.rust;
    let on_ident_rust = idents.on_name;
    let original_method = &signal.method;

    // Build a namespace that includes any namespace for the T
    let connect_namespace = namespace_externcxxqt_with_qobject_namespace(
        cxx_mappings
            .namespaces
            .get(&signal.qobject_ident.to_string()),
    );

    let parameters_cxx: Vec<FnArg> = signal
        .parameters
        .iter()
        .map(|parameter| {
            let ident = &parameter.ident;
            let ty = &parameter.ty;
            parse_quote! { #ident: #ty }
        })
        .collect();
    let parameters_qualified: Vec<FnArg> = parameters_cxx
        .iter()
        .cloned()
        .map(|mut parameter| {
            if let FnArg::Typed(pat_type) = &mut parameter {
                *pat_type.ty =
                    syn_type_cxx_bridge_to_qualified(&pat_type.ty, &cxx_mappings.qualified);
            }
            parameter
        })
        .collect();

    let self_type_cxx = if signal.mutable {
        parse_quote! { Pin<&mut #qobject_name> }
    } else {
        parse_quote! { &#qobject_name }
    };
    let self_type_qualified =
        syn_type_cxx_bridge_to_qualified(&self_type_cxx, &cxx_mappings.qualified);
    let qualified_impl =
        syn_ident_cxx_bridge_to_qualified_impl(qobject_name, &cxx_mappings.qualified);

    let mut unsafe_block = None;
    let mut unsafe_call = Some(quote! { unsafe });
    if signal.safe {
        std::mem::swap(&mut unsafe_call, &mut unsafe_block);
    }

    let mut cxx_bridge = vec![];

    if !signal.private {
        cxx_bridge.push(quote! {
            #unsafe_block extern "C++" {
                #original_method
            }
        });
    }

    cxx_bridge.push(quote! {
        unsafe extern "C++" {
            #[doc(hidden)]
            #[namespace = #connect_namespace]
            #[must_use]
            #[rust_name = #free_connect_ident_rust_str]
            fn #free_connect_ident_cpp(self_value: #self_type_cxx, func: #unsafe_call fn(#self_type_cxx, #(#parameters_cxx),*), conn_type: CxxQtConnectionType) -> CxxQtQMetaObjectConnection;
        }
    });

    let fragment = RustFragmentPair {
        cxx_bridge,
        implementation: vec![
            quote! {
                impl #qualified_impl {
                    #[doc = "Connect the given function pointer to the signal "]
                    #[doc = #signal_name_cpp_str]
                    #[doc = ", so that when the signal is emitted the function pointer is executed."]
                    #[doc = "\n"]
                    #[doc = "Note that this method uses a AutoConnection connection type."]
                    #[must_use]
                    pub fn #on_ident_rust(self: #self_type_qualified, func: fn(#self_type_qualified, #(#parameters_qualified),*)) -> cxx_qt_lib::QMetaObjectConnection
                    {
                        #module_ident::#free_connect_ident_rust(self, func, cxx_qt_lib::ConnectionType::AutoConnection)
                    }
                }
            },
            quote! {
                impl #qualified_impl {
                    #[doc = "Connect the given function pointer to the signal "]
                    #[doc = #signal_name_cpp_str]
                    #[doc = ", so that when the signal is emitted the function pointer is executed."]
                    #[must_use]
                    pub fn #connect_ident_rust(self: #self_type_qualified, func: fn(#self_type_qualified, #(#parameters_qualified),*), conn_type: cxx_qt_lib::ConnectionType) -> cxx_qt_lib::QMetaObjectConnection
                    {
                        #module_ident::#free_connect_ident_rust(self, func, conn_type)
                    }
                }
            },
        ],
    };

    let mut generated = GeneratedExternCxxQt::default();
    generated
        .cxx_mod_contents
        .append(&mut fragment.cxx_bridge_as_items()?);
    generated
        .cxx_qt_mod_contents
        .append(&mut fragment.implementation_as_items()?);

    Ok(generated)
}

pub fn generate_rust_signals(
    signals: &Vec<ParsedSignal>,
    qobject_idents: &QObjectName,
    cxx_mappings: &ParsedCxxMappings,
) -> Result<GeneratedRustQObject> {
    let mut generated = GeneratedRustQObject::default();
    let qobject_name = &qobject_idents.cpp_class.rust;

    // Create the methods for the other signals
    for signal in signals {
        let idents = QSignalName::from(signal);
        let signal_name_rust = idents.name.rust;
        let signal_name_rust_str = signal_name_rust.to_string();
        let signal_name_cpp = idents.name.cpp;
        let signal_name_cpp_str = signal_name_cpp.to_string();
        let connect_ident_cpp = idents.connect_name.cpp;
        let connect_ident_rust = idents.connect_name.rust;
        let connect_ident_rust_str = connect_ident_rust.to_string();
        let on_ident_rust = idents.on_name;

        let parameters_cxx: Vec<FnArg> = signal
            .parameters
            .iter()
            .map(|parameter| {
                let ident = &parameter.ident;
                let ty = &parameter.ty;
                parse_quote! { #ident: #ty }
            })
            .collect();
        let parameters_qualified: Vec<FnArg> = parameters_cxx
            .iter()
            .cloned()
            .map(|mut parameter| {
                if let FnArg::Typed(pat_type) = &mut parameter {
                    *pat_type.ty =
                        syn_type_cxx_bridge_to_qualified(&pat_type.ty, &cxx_mappings.qualified);
                }
                parameter
            })
            .collect();

        let self_type_cxx = if signal.mutable {
            parse_quote! { Pin<&mut #qobject_name> }
        } else {
            parse_quote! { &#qobject_name }
        };
        let self_type_qualified =
            syn_type_cxx_bridge_to_qualified(&self_type_cxx, &cxx_mappings.qualified);
        let qualified_impl =
            syn_ident_cxx_bridge_to_qualified_impl(qobject_name, &cxx_mappings.qualified);

        let mut unsafe_block = None;
        let mut unsafe_call = Some(quote! { unsafe });
        if signal.safe {
            std::mem::swap(&mut unsafe_call, &mut unsafe_block);
        }

        let attrs = &signal.method.attrs;

        let fragment = RustFragmentPair {
            cxx_bridge: vec![
                quote! {
                    #unsafe_block extern "C++" {
                        #(#attrs)*
                        #[rust_name = #signal_name_rust_str]
                        #unsafe_call fn #signal_name_cpp(self: #self_type_cxx, #(#parameters_cxx),*);
                    }
                },
                quote! {
                    unsafe extern "C++" {
                        #[doc = "Connect the given function pointer to the signal "]
                        #[doc = #signal_name_cpp_str]
                        #[doc = ", so that when the signal is emitted the function pointer is executed."]
                        #[must_use]
                        #[rust_name = #connect_ident_rust_str]
                        fn #connect_ident_cpp(self: #self_type_cxx, func: #unsafe_call fn(#self_type_cxx, #(#parameters_cxx),*), conn_type: CxxQtConnectionType) -> CxxQtQMetaObjectConnection;
                    }
                },
            ],
            implementation: vec![quote! {
                impl #qualified_impl {
                    #[doc = "Connect the given function pointer to the signal "]
                    #[doc = #signal_name_cpp_str]
                    #[doc = ", so that when the signal is emitted the function pointer is executed."]
                    #[doc = "\n"]
                    #[doc = "Note that this method uses a AutoConnection connection type."]
                    #[must_use]
                    pub fn #on_ident_rust(self: #self_type_qualified, func: fn(#self_type_qualified, #(#parameters_qualified),*)) -> cxx_qt_lib::QMetaObjectConnection
                    {
                        self.#connect_ident_rust(func, cxx_qt_lib::ConnectionType::AutoConnection)
                    }
                }
            }],
        };

        generated
            .cxx_mod_contents
            .append(&mut fragment.cxx_bridge_as_items()?);
        generated
            .cxx_qt_mod_contents
            .append(&mut fragment.implementation_as_items()?);
    }

    Ok(generated)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::generator::naming::{qobject::tests::create_qobjectname, CombinedIdent};
    use crate::parser::parameter::ParsedFunctionParameter;
    use crate::tests::assert_tokens_eq;
    use quote::{format_ident, quote};
    use syn::parse_quote;

    #[test]
    fn test_generate_rust_signal() {
        let qsignal = ParsedSignal {
            method: parse_quote! {
                fn ready(self: Pin<&mut MyObject>);
            },
            qobject_ident: format_ident!("MyObject"),
            mutable: true,
            parameters: vec![],
            ident: CombinedIdent {
                cpp: format_ident!("ready"),
                rust: format_ident!("ready"),
            },
            safe: true,
            inherit: false,
            private: false,
        };
        let qobject_idents = create_qobjectname();

        let generated = generate_rust_signals(
            &vec![qsignal],
            &qobject_idents,
            &ParsedCxxMappings::default(),
        )
        .unwrap();

        assert_eq!(generated.cxx_mod_contents.len(), 2);
        assert_eq!(generated.cxx_qt_mod_contents.len(), 1);

        assert_tokens_eq(
            &generated.cxx_mod_contents[0],
            quote! {
                unsafe extern "C++" {
                    #[rust_name = "ready"]
                    fn ready(self: Pin<&mut MyObject>, );
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_mod_contents[1],
            quote! {
                unsafe extern "C++" {
                    #[doc = "Connect the given function pointer to the signal "]
                    #[doc = "ready"]
                    #[doc = ", so that when the signal is emitted the function pointer is executed."]
                    #[must_use]
                    #[rust_name = "connect_ready"]
                    fn readyConnect(self: Pin<&mut MyObject>, func: fn(Pin<&mut MyObject>, ), conn_type : CxxQtConnectionType) -> CxxQtQMetaObjectConnection;
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[0],
            quote! {
                impl MyObject {
                    #[doc = "Connect the given function pointer to the signal "]
                    #[doc = "ready"]
                    #[doc = ", so that when the signal is emitted the function pointer is executed."]
                    #[doc = "\n"]
                    #[doc = "Note that this method uses a AutoConnection connection type."]
                    #[must_use]
                    pub fn on_ready(self: core::pin::Pin<&mut MyObject>, func: fn(core::pin::Pin<&mut MyObject>, )) -> cxx_qt_lib::QMetaObjectConnection
                    {
                        self.connect_ready(func, cxx_qt_lib::ConnectionType::AutoConnection)
                    }
                }
            },
        );
    }

    #[test]
    fn test_generate_rust_signal_parameters() {
        let qsignal = ParsedSignal {
            method: parse_quote! {
                #[attribute]
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
        };
        let qobject_idents = create_qobjectname();

        let generated = generate_rust_signals(
            &vec![qsignal],
            &qobject_idents,
            &ParsedCxxMappings::default(),
        )
        .unwrap();

        assert_eq!(generated.cxx_mod_contents.len(), 2);
        assert_eq!(generated.cxx_qt_mod_contents.len(), 1);

        assert_tokens_eq(
            &generated.cxx_mod_contents[0],
            quote! {
                unsafe extern "C++" {
                    #[attribute]
                    #[rust_name = "data_changed"]
                    fn dataChanged(self: Pin<&mut MyObject>, trivial: i32, opaque: UniquePtr<QColor>);
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_mod_contents[1],
            quote! {
                unsafe extern "C++" {
                    #[doc = "Connect the given function pointer to the signal "]
                    #[doc = "dataChanged"]
                    #[doc = ", so that when the signal is emitted the function pointer is executed."]
                    #[must_use]
                    #[rust_name = "connect_data_changed"]
                    fn dataChangedConnect(self: Pin<&mut MyObject>, func: fn(Pin<&mut MyObject>, trivial: i32, opaque: UniquePtr<QColor>), conn_type : CxxQtConnectionType) -> CxxQtQMetaObjectConnection;
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[0],
            quote! {
                impl MyObject {
                    #[doc = "Connect the given function pointer to the signal "]
                    #[doc = "dataChanged"]
                    #[doc = ", so that when the signal is emitted the function pointer is executed."]
                    #[doc = "\n"]
                    #[doc = "Note that this method uses a AutoConnection connection type."]
                    #[must_use]
                    pub fn on_data_changed(self: core::pin::Pin<&mut MyObject>, func: fn(core::pin::Pin<&mut MyObject>, trivial: i32, opaque: cxx::UniquePtr<QColor>)) -> cxx_qt_lib::QMetaObjectConnection
                    {
                        self.connect_data_changed(func, cxx_qt_lib::ConnectionType::AutoConnection)
                    }
                }
            },
        );
    }

    #[test]
    fn test_generate_rust_signal_unsafe() {
        let qsignal = ParsedSignal {
            method: parse_quote! {
                unsafe fn unsafe_signal(self: Pin<&mut MyObject>, param: *mut T);
            },
            qobject_ident: format_ident!("MyObject"),
            mutable: true,
            parameters: vec![ParsedFunctionParameter {
                ident: format_ident!("param"),
                ty: parse_quote! { *mut T },
            }],
            ident: CombinedIdent {
                cpp: format_ident!("unsafeSignal"),
                rust: format_ident!("unsafe_signal"),
            },
            safe: false,
            inherit: false,
            private: false,
        };
        let qobject_idents = create_qobjectname();

        let generated = generate_rust_signals(
            &vec![qsignal],
            &qobject_idents,
            &ParsedCxxMappings::default(),
        )
        .unwrap();

        assert_eq!(generated.cxx_mod_contents.len(), 2);
        assert_eq!(generated.cxx_qt_mod_contents.len(), 1);

        assert_tokens_eq(
            &generated.cxx_mod_contents[0],
            quote! {
                extern "C++" {
                    #[rust_name = "unsafe_signal"]
                    unsafe fn unsafeSignal(self: Pin<&mut MyObject>, param: *mut T);
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_mod_contents[1],
            quote! {
                unsafe extern "C++" {
                    #[doc = "Connect the given function pointer to the signal "]
                    #[doc = "unsafeSignal"]
                    #[doc = ", so that when the signal is emitted the function pointer is executed."]
                    #[must_use]
                    #[rust_name = "connect_unsafe_signal"]
                    fn unsafeSignalConnect(self: Pin <&mut MyObject>, func: unsafe fn(Pin<&mut MyObject>, param: *mut T), conn_type : CxxQtConnectionType) -> CxxQtQMetaObjectConnection;
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[0],
            quote! {
                impl MyObject {
                    #[doc = "Connect the given function pointer to the signal "]
                    #[doc = "unsafeSignal"]
                    #[doc = ", so that when the signal is emitted the function pointer is executed."]
                    #[doc = "\n"]
                    #[doc = "Note that this method uses a AutoConnection connection type."]
                    #[must_use]
                    pub fn on_unsafe_signal(self: core::pin::Pin<&mut MyObject>, func: fn(core::pin::Pin<&mut MyObject>, param: *mut T)) -> cxx_qt_lib::QMetaObjectConnection
                    {
                        self.connect_unsafe_signal(func, cxx_qt_lib::ConnectionType::AutoConnection)
                    }
                }
            },
        );
    }

    #[test]
    fn test_generate_rust_signal_existing() {
        let qsignal = ParsedSignal {
            method: parse_quote! {
                #[inherit]
                fn existing_signal(self: Pin<&mut MyObject>, );
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
        };
        let qobject_idents = create_qobjectname();

        let generated = generate_rust_signals(
            &vec![qsignal],
            &qobject_idents,
            &ParsedCxxMappings::default(),
        )
        .unwrap();

        assert_eq!(generated.cxx_mod_contents.len(), 2);
        assert_eq!(generated.cxx_qt_mod_contents.len(), 1);

        assert_tokens_eq(
            &generated.cxx_mod_contents[0],
            quote! {
                unsafe extern "C++" {
                    #[inherit]
                    #[rust_name = "existing_signal"]
                    fn baseName(self: Pin<&mut MyObject>, );
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_mod_contents[1],
            quote! {
                unsafe extern "C++" {
                    #[doc = "Connect the given function pointer to the signal "]
                    #[doc = "baseName"]
                    #[doc = ", so that when the signal is emitted the function pointer is executed."]
                    #[must_use]
                    #[rust_name = "connect_existing_signal"]
                    fn baseNameConnect(self: Pin<& mut MyObject>, func: fn(Pin<&mut MyObject>, ), conn_type : CxxQtConnectionType) -> CxxQtQMetaObjectConnection;
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[0],
            quote! {
                impl MyObject {
                    #[doc = "Connect the given function pointer to the signal "]
                    #[doc = "baseName"]
                    #[doc = ", so that when the signal is emitted the function pointer is executed."]
                    #[doc = "\n"]
                    #[doc = "Note that this method uses a AutoConnection connection type."]
                    #[must_use]
                    pub fn on_existing_signal(self: core::pin::Pin<&mut MyObject>, func: fn(core::pin::Pin<&mut MyObject>, )) -> cxx_qt_lib::QMetaObjectConnection
                    {
                        self.connect_existing_signal(func, cxx_qt_lib::ConnectionType::AutoConnection)
                    }
                }
            },
        );
    }

    #[test]
    fn test_generate_rust_signal_free() {
        let qsignal = ParsedSignal {
            method: parse_quote! {
                fn ready(self: Pin<&mut MyObject>);
            },
            qobject_ident: format_ident!("MyObject"),
            mutable: true,
            parameters: vec![],
            ident: CombinedIdent {
                cpp: format_ident!("ready"),
                rust: format_ident!("ready"),
            },
            safe: true,
            inherit: false,
            private: false,
        };

        let generated = generate_rust_free_signal(
            &qsignal,
            &ParsedCxxMappings::default(),
            &format_ident!("ffi"),
        )
        .unwrap();

        assert_eq!(generated.cxx_mod_contents.len(), 2);
        assert_eq!(generated.cxx_qt_mod_contents.len(), 2);

        assert_tokens_eq(
            &generated.cxx_mod_contents[0],
            quote! {
                unsafe extern "C++" {
                    fn ready(self: Pin<&mut MyObject>);
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_mod_contents[1],
            quote! {
                unsafe extern "C++" {
                    #[doc (hidden)]
                    #[namespace = "rust::cxxqtgen1::externcxxqt"]
                    #[must_use]
                    #[rust_name = "MyObject_connect_ready"]
                    fn MyObject_readyConnect(self_value: Pin<&mut MyObject>, func: fn(Pin<&mut MyObject>, ), conn_type : CxxQtConnectionType) -> CxxQtQMetaObjectConnection;
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[0],
            quote! {
                impl MyObject {
                    #[doc = "Connect the given function pointer to the signal "]
                    #[doc = "ready"]
                    #[doc = ", so that when the signal is emitted the function pointer is executed."]
                    #[doc = "\n"]
                    #[doc = "Note that this method uses a AutoConnection connection type."]
                    #[must_use]
                    pub fn on_ready(self: core::pin::Pin<&mut MyObject>, func: fn(core::pin::Pin<&mut MyObject>, )) -> cxx_qt_lib::QMetaObjectConnection
                    {
                        ffi::MyObject_connect_ready(self, func, cxx_qt_lib::ConnectionType::AutoConnection)
                    }
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[1],
            quote! {
                impl MyObject {
                    #[doc = "Connect the given function pointer to the signal "]
                    #[doc = "ready"]
                    #[doc = ", so that when the signal is emitted the function pointer is executed."]
                    #[must_use]
                    pub fn connect_ready(self: core::pin::Pin<&mut MyObject>, func: fn(core::pin::Pin<&mut MyObject>, ), conn_type: cxx_qt_lib::ConnectionType) -> cxx_qt_lib::QMetaObjectConnection
                    {
                        ffi::MyObject_connect_ready(self, func, conn_type)
                    }
                }
            },
        );
    }

    #[test]
    fn test_generate_rust_signal_free_private() {
        let qsignal = ParsedSignal {
            method: parse_quote! {
                fn ready(self: Pin<&mut MyObject>);
            },
            qobject_ident: format_ident!("MyObject"),
            mutable: true,
            parameters: vec![],
            ident: CombinedIdent {
                cpp: format_ident!("ready"),
                rust: format_ident!("ready"),
            },
            safe: true,
            inherit: false,
            private: true,
        };

        let generated = generate_rust_free_signal(
            &qsignal,
            &ParsedCxxMappings::default(),
            &format_ident!("ffi"),
        )
        .unwrap();

        assert_eq!(generated.cxx_mod_contents.len(), 1);
        assert_eq!(generated.cxx_qt_mod_contents.len(), 2);

        assert_tokens_eq(
            &generated.cxx_mod_contents[0],
            quote! {
                unsafe extern "C++" {
                    #[doc (hidden)]
                    #[namespace = "rust::cxxqtgen1::externcxxqt"]
                    #[must_use]
                    #[rust_name = "MyObject_connect_ready"]
                    fn MyObject_readyConnect(self_value: Pin<&mut MyObject>, func: fn(Pin<&mut MyObject>, ), conn_type : CxxQtConnectionType) -> CxxQtQMetaObjectConnection;
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[0],
            quote! {
                impl MyObject {
                    #[doc = "Connect the given function pointer to the signal "]
                    #[doc = "ready"]
                    #[doc = ", so that when the signal is emitted the function pointer is executed."]
                    #[doc = "\n"]
                    #[doc = "Note that this method uses a AutoConnection connection type."]
                    #[must_use]
                    pub fn on_ready(self: core::pin::Pin<&mut MyObject>, func: fn(core::pin::Pin<&mut MyObject>, )) -> cxx_qt_lib::QMetaObjectConnection
                    {
                        ffi::MyObject_connect_ready(self, func, cxx_qt_lib::ConnectionType::AutoConnection)
                    }
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[1],
            quote! {
                impl MyObject {
                    #[doc = "Connect the given function pointer to the signal "]
                    #[doc = "ready"]
                    #[doc = ", so that when the signal is emitted the function pointer is executed."]
                    #[must_use]
                    pub fn connect_ready(self: core::pin::Pin<&mut MyObject>, func: fn(core::pin::Pin<&mut MyObject>, ), conn_type: cxx_qt_lib::ConnectionType) -> cxx_qt_lib::QMetaObjectConnection
                    {
                        ffi::MyObject_connect_ready(self, func, conn_type)
                    }
                }
            },
        );
    }
}
