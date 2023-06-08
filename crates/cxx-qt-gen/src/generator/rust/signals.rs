// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{
    generator::{
        naming::{qobject::QObjectName, signals::QSignalName},
        rust::{fragment::RustFragmentPair, qobject::GeneratedRustQObjectBlocks},
    },
    parser::signals::ParsedSignal,
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::Result;

pub fn generate_rust_signals(
    signals: &Vec<ParsedSignal>,
    qobject_idents: &QObjectName,
) -> Result<GeneratedRustQObjectBlocks> {
    let mut generated = GeneratedRustQObjectBlocks::default();
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

        let parameters = signal
            .parameters
            .iter()
            .map(|parameter| {
                let ident = &parameter.ident;
                let ty = &parameter.ty;
                quote! { #ident: #ty }
            })
            .collect::<Vec<TokenStream>>();

        let self_type = if signal.mutable {
            quote! { Pin<&mut #qobject_name> }
        } else {
            quote! { &#qobject_name }
        };

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
                        #unsafe_call fn #signal_name_cpp(self: #self_type, #(#parameters),*);
                    }
                },
                quote! {
                    unsafe extern "C++" {
                        #[doc = "Connect the given function pointer to the signal "]
                        #[doc = #signal_name_cpp_str]
                        #[doc = ", so that when the signal is emitted the function pointer is executed."]
                        #[must_use]
                        #[rust_name = #connect_ident_rust_str]
                        fn #connect_ident_cpp(self: #self_type, func: #unsafe_call fn(#self_type, #(#parameters),*), conn_type: CxxQtConnectionType) -> CxxQtQMetaObjectConnection;
                    }
                },
            ],
            implementation: vec![quote! {
                impl #qobject_name {
                    #[doc = "Connect the given function pointer to the signal "]
                    #[doc = #signal_name_cpp_str]
                    #[doc = ", so that when the signal is emitted the function pointer is executed."]
                    #[doc = "\n"]
                    #[doc = "Note that this method uses a AutoConnection connection type."]
                    #[must_use]
                    pub fn #on_ident_rust(self: #self_type, func: fn(#self_type, #(#parameters),*)) -> CxxQtQMetaObjectConnection
                    {
                        self.#connect_ident_rust(func, CxxQtConnectionType::AutoConnection)
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
        };
        let qobject_idents = create_qobjectname();

        let generated = generate_rust_signals(&vec![qsignal], &qobject_idents).unwrap();

        assert_eq!(generated.cxx_mod_contents.len(), 2);
        assert_eq!(generated.cxx_qt_mod_contents.len(), 1);

        assert_tokens_eq(
            &generated.cxx_mod_contents[0],
            quote! {
                unsafe extern "C++" {
                    #[rust_name = "ready"]
                    fn ready(self: Pin<&mut MyObjectQt>, );
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
                    fn readyConnect(self: Pin<&mut MyObjectQt>, func: fn(Pin<&mut MyObjectQt>, ), conn_type : CxxQtConnectionType) -> CxxQtQMetaObjectConnection;
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[0],
            quote! {
                impl MyObjectQt {
                    #[doc = "Connect the given function pointer to the signal "]
                    #[doc = "ready"]
                    #[doc = ", so that when the signal is emitted the function pointer is executed."]
                    #[doc = "\n"]
                    #[doc = "Note that this method uses a AutoConnection connection type."]
                    #[must_use]
                    pub fn on_ready(self: Pin<&mut MyObjectQt>, func: fn(Pin<&mut MyObjectQt>, )) -> CxxQtQMetaObjectConnection
                    {
                        self.connect_ready(func, CxxQtConnectionType::AutoConnection)
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
        };
        let qobject_idents = create_qobjectname();

        let generated = generate_rust_signals(&vec![qsignal], &qobject_idents).unwrap();

        assert_eq!(generated.cxx_mod_contents.len(), 2);
        assert_eq!(generated.cxx_qt_mod_contents.len(), 1);

        assert_tokens_eq(
            &generated.cxx_mod_contents[0],
            quote! {
                unsafe extern "C++" {
                    #[attribute]
                    #[rust_name = "data_changed"]
                    fn dataChanged(self: Pin<&mut MyObjectQt>, trivial: i32, opaque: UniquePtr<QColor>);
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
                    fn dataChangedConnect(self: Pin<&mut MyObjectQt>, func: fn(Pin<&mut MyObjectQt>, trivial: i32, opaque: UniquePtr<QColor>), conn_type : CxxQtConnectionType) -> CxxQtQMetaObjectConnection;
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[0],
            quote! {
                impl MyObjectQt {
                    #[doc = "Connect the given function pointer to the signal "]
                    #[doc = "dataChanged"]
                    #[doc = ", so that when the signal is emitted the function pointer is executed."]
                    #[doc = "\n"]
                    #[doc = "Note that this method uses a AutoConnection connection type."]
                    #[must_use]
                    pub fn on_data_changed(self: Pin<&mut MyObjectQt>, func: fn(Pin<&mut MyObjectQt>, trivial: i32, opaque: UniquePtr<QColor>)) -> CxxQtQMetaObjectConnection
                    {
                        self.connect_data_changed(func, CxxQtConnectionType::AutoConnection)
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
        };
        let qobject_idents = create_qobjectname();

        let generated = generate_rust_signals(&vec![qsignal], &qobject_idents).unwrap();

        assert_eq!(generated.cxx_mod_contents.len(), 2);
        assert_eq!(generated.cxx_qt_mod_contents.len(), 1);

        assert_tokens_eq(
            &generated.cxx_mod_contents[0],
            quote! {
                extern "C++" {
                    #[rust_name = "unsafe_signal"]
                    unsafe fn unsafeSignal(self: Pin<&mut MyObjectQt>, param: *mut T);
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
                    fn unsafeSignalConnect(self: Pin <&mut MyObjectQt>, func: unsafe fn(Pin<&mut MyObjectQt>, param: *mut T), conn_type : CxxQtConnectionType) -> CxxQtQMetaObjectConnection;
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[0],
            quote! {
                impl MyObjectQt {
                    #[doc = "Connect the given function pointer to the signal "]
                    #[doc = "unsafeSignal"]
                    #[doc = ", so that when the signal is emitted the function pointer is executed."]
                    #[doc = "\n"]
                    #[doc = "Note that this method uses a AutoConnection connection type."]
                    #[must_use]
                    pub fn on_unsafe_signal(self: Pin<&mut MyObjectQt>, func: fn(Pin<&mut MyObjectQt>, param: *mut T)) -> CxxQtQMetaObjectConnection
                    {
                        self.connect_unsafe_signal(func, CxxQtConnectionType::AutoConnection)
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
        };
        let qobject_idents = create_qobjectname();

        let generated = generate_rust_signals(&vec![qsignal], &qobject_idents).unwrap();

        assert_eq!(generated.cxx_mod_contents.len(), 2);
        assert_eq!(generated.cxx_qt_mod_contents.len(), 1);

        assert_tokens_eq(
            &generated.cxx_mod_contents[0],
            quote! {
                unsafe extern "C++" {
                    #[inherit]
                    #[rust_name = "existing_signal"]
                    fn baseName(self: Pin<&mut MyObjectQt>, );
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
                    fn baseNameConnect(self: Pin<& mut MyObjectQt>, func: fn(Pin<&mut MyObjectQt>, ), conn_type : CxxQtConnectionType) -> CxxQtQMetaObjectConnection;
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[0],
            quote! {
                impl MyObjectQt {
                    #[doc = "Connect the given function pointer to the signal "]
                    #[doc = "baseName"]
                    #[doc = ", so that when the signal is emitted the function pointer is executed."]
                    #[doc = "\n"]
                    #[doc = "Note that this method uses a AutoConnection connection type."]
                    #[must_use]
                    pub fn on_existing_signal(self: Pin<&mut MyObjectQt>, func: fn(Pin<&mut MyObjectQt>, )) -> CxxQtQMetaObjectConnection
                    {
                        self.connect_existing_signal(func, CxxQtConnectionType::AutoConnection)
                    }
                }
            },
        );
    }
}
