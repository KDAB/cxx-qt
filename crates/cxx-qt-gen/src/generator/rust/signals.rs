// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{
    generator::{
        naming::{qobject::QObjectName, signals::QSignalName},
        rust::{
            fragment::RustFragmentPair, qobject::GeneratedRustQObjectBlocks,
            types::is_unsafe_cxx_type,
        },
    },
    parser::signals::ParsedSignalsEnum,
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Result, Type};

pub fn generate_rust_signals(
    signals_enum: &ParsedSignalsEnum,
    qobject_idents: &QObjectName,
) -> Result<GeneratedRustQObjectBlocks> {
    let mut generated = GeneratedRustQObjectBlocks::default();
    let cpp_class_name_rust = &qobject_idents.cpp_class.rust;
    let signal_enum_ident = &signals_enum.ident;
    let mut signal_matches = vec![];

    // Add the original enum into the implementation
    generated
        .cxx_qt_mod_contents
        .push(syn::Item::Enum(signals_enum.item.clone()));

    // Create the methods for the other signals
    for signal in &signals_enum.signals {
        let idents = QSignalName::from(signal);
        let signal_ident_rust = idents.enum_name;
        let signal_ident_cpp_str = idents.name.cpp.to_string();
        let emit_ident_cpp = &idents.emit_name.cpp;
        let emit_ident_rust = &idents.emit_name.rust;
        let emit_ident_rust_str = idents.emit_name.rust.to_string();
        let connect_ident_cpp = idents.connect_name.cpp;
        let connect_ident_rust = idents.connect_name.rust;
        let connect_ident_rust_str = connect_ident_rust.to_string();
        let on_ident_rust = idents.on_name;

        let mut parameters = signal
            .parameters
            .iter()
            .map(|parameter| {
                let ident = &parameter.ident;
                let mut ty = parameter.ty.clone();
                // Remove any lifetime from the signal, as this will be related
                // to the enum. For the CXX methods these can just be
                // normal references with inferred lifetimes.
                if let Type::Reference(ty) = &mut ty {
                    ty.lifetime = None;
                }
                quote! { #ident: #ty }
            })
            .collect::<Vec<TokenStream>>();
        let parameter_signatures = if signal.parameters.is_empty() {
            quote! { self: Pin<&mut #cpp_class_name_rust> }
        } else {
            quote! { self: Pin<&mut #cpp_class_name_rust>, #(#parameters),* }
        };
        let parameter_names = signal
            .parameters
            .iter()
            .map(|parameter| parameter.ident.clone())
            .collect::<Vec<Ident>>();

        // Determine if unsafe is required due to an unsafe parameter
        let has_unsafe = if signal
            .parameters
            .iter()
            .any(|parameter| is_unsafe_cxx_type(&parameter.ty))
        {
            quote! { unsafe }
        } else {
            quote! {}
        };

        // Add the self context to parameters as this is used for the connection function pointer
        parameters.insert(
            0,
            quote! {
                Pin<&mut #cpp_class_name_rust>
            },
        );

        let fragment = RustFragmentPair {
            cxx_bridge: vec![
                quote! {
                    unsafe extern "C++" {
                        #[doc(hidden)]
                        #[rust_name = #emit_ident_rust_str]
                        #has_unsafe fn #emit_ident_cpp(#parameter_signatures);
                    }
                },
                quote! {
                    unsafe extern "C++" {
                        #[doc = "Connect the given function pointer to the signal "]
                        #[doc = #signal_ident_cpp_str]
                        #[doc = ", so that when the signal is emitted the function pointer is executed."]
                        #[must_use]
                        #[rust_name = #connect_ident_rust_str]
                        fn #connect_ident_cpp(self: Pin<&mut #cpp_class_name_rust>, func: #has_unsafe fn(#(#parameters),*), conn_type: CxxQtConnectionType) -> CxxQtQMetaObjectConnection;
                    }
                },
            ],
            // Note we do not need the #has_unsafe here as this only needs to be in the CXX bridge
            // otherwise the function pointer itself needs to be unsafe
            implementation: vec![quote! {
                impl #cpp_class_name_rust {
                    #[doc = "Connect the given function pointer to the signal "]
                    #[doc = #signal_ident_cpp_str]
                    #[doc = ", so that when the signal is emitted the function pointer is executed."]
                    #[doc = "\n"]
                    #[doc = "Note that this method uses a AutoConnection connection type."]
                    #[must_use]
                    fn #on_ident_rust(self: Pin<&mut #cpp_class_name_rust>, func: fn(#(#parameters),*)) -> CxxQtQMetaObjectConnection
                    {
                        self.#connect_ident_rust(func, CxxQtConnectionType::AutoConnection)
                    }
                }
            }],
        };
        signal_matches.push(quote! {
            #signal_enum_ident::#signal_ident_rust { #(#parameter_names),* } => #has_unsafe { self.#emit_ident_rust(#(#parameter_names),*) }
        });

        generated
            .cxx_mod_contents
            .append(&mut fragment.cxx_bridge_as_items()?);
        generated
            .cxx_qt_mod_contents
            .append(&mut fragment.implementation_as_items()?);
    }

    // Add the Rust method using the enum to call the methods
    let qobject_ident_str = qobject_idents.rust_struct.rust.to_string();
    let signal_enum_ident_str = signal_enum_ident.to_string();
    generated.cxx_qt_mod_contents.push(syn::parse2(quote! {
        impl #cpp_class_name_rust {
            #[doc = "Emit the signal from the enum "]
            #[doc = #signal_enum_ident_str]
            #[doc = " on the QObject "]
            #[doc = #qobject_ident_str]
            pub fn emit(self: Pin<&mut Self>, signal: #signal_enum_ident) {
                match signal {
                    #(#signal_matches),*
                }
            }
        }
    })?);

    Ok(generated)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::tests::assert_tokens_eq;
    use crate::{
        generator::naming::qobject::tests::create_qobjectname, parser::signals::ParsedSignalsEnum,
    };
    use quote::quote;
    use syn::{parse_quote, ItemEnum};

    #[test]
    fn test_generate_rust_signals() {
        let e: ItemEnum = parse_quote! {
            #[cxx_qt::qsignals(MyObject)]
            enum MySignals {
                Ready,
                DataChanged {
                    trivial: i32,
                    #[cxx_type = "QColor"]
                    opaque: UniquePtr<QColor>
                },
                UnsafeSignal {
                    param: *mut T,
                },
                #[cxx_name = "baseName"]
                #[inherit]
                ExistingSignal,
            }
        };
        let signals_enum = ParsedSignalsEnum::from(&e, 0).unwrap();
        let qobject_idents = create_qobjectname();

        let generated = generate_rust_signals(&signals_enum, &qobject_idents).unwrap();

        assert_eq!(generated.cxx_mod_contents.len(), 8);
        assert_eq!(generated.cxx_qt_mod_contents.len(), 6);

        // Ready
        assert_tokens_eq(
            &generated.cxx_mod_contents[0],
            quote! {
                unsafe extern "C++" {
                    #[doc(hidden)]
                    #[rust_name = "emit_ready"]
                    fn emitReady(self: Pin<&mut MyObjectQt>);
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
                    fn readyConnect(self: Pin<&mut MyObjectQt>, func: fn(Pin<&mut MyObjectQt>), conn_type : CxxQtConnectionType) -> CxxQtQMetaObjectConnection;
                }
            },
        );

        // DataChanged
        assert_tokens_eq(
            &generated.cxx_mod_contents[2],
            quote! {
                unsafe extern "C++" {
                    #[doc(hidden)]
                    #[rust_name = "emit_data_changed"]
                    fn emitDataChanged(self: Pin<&mut MyObjectQt>, trivial: i32, opaque: UniquePtr<QColor>);
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_mod_contents[3],
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

        // UnsafeSignal
        assert_tokens_eq(
            &generated.cxx_mod_contents[4],
            quote! {
                unsafe extern "C++" {
                    #[doc(hidden)]
                    #[rust_name = "emit_unsafe_signal"]
                    unsafe fn emitUnsafeSignal(self: Pin<&mut MyObjectQt>, param: *mut T);
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_mod_contents[5],
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

        // ExistingSignal
        assert_tokens_eq(
            &generated.cxx_mod_contents[6],
            quote! {
                unsafe extern "C++" {
                    #[doc(hidden)]
                    #[rust_name = "emit_existing_signal"]
                    fn emitBaseName(self: Pin<&mut MyObjectQt>);
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_mod_contents[7],
            quote! {
                unsafe extern "C++" {
                    #[doc = "Connect the given function pointer to the signal "]
                    #[doc = "baseName"]
                    #[doc = ", so that when the signal is emitted the function pointer is executed."]
                    #[must_use]
                    #[rust_name = "connect_existing_signal"]
                    fn baseNameConnect(self: Pin<& mut MyObjectQt>, func: fn(Pin<&mut MyObjectQt>), conn_type : CxxQtConnectionType) -> CxxQtQMetaObjectConnection;
                }
            },
        );

        // enum
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[0],
            quote! {
                enum MySignals {
                    Ready,
                    DataChanged {
                        trivial: i32,
                        opaque: UniquePtr<QColor>
                    },
                    UnsafeSignal {
                        param: *mut T,
                    },
                    ExistingSignal,
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[1],
            quote! {
                impl MyObjectQt {
                    #[doc = "Connect the given function pointer to the signal "]
                    #[doc = "ready"]
                    #[doc = ", so that when the signal is emitted the function pointer is executed."]
                    #[doc = "\n"]
                    #[doc = "Note that this method uses a AutoConnection connection type."]
                    #[must_use]
                    fn on_ready(self: Pin<&mut MyObjectQt>, func: fn(Pin<&mut MyObjectQt>)) -> CxxQtQMetaObjectConnection
                    {
                        self.connect_ready(func, CxxQtConnectionType::AutoConnection)
                    }
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[2],
            quote! {
                impl MyObjectQt {
                    #[doc = "Connect the given function pointer to the signal "]
                    #[doc = "dataChanged"]
                    #[doc = ", so that when the signal is emitted the function pointer is executed."]
                    #[doc = "\n"]
                    #[doc = "Note that this method uses a AutoConnection connection type."]
                    #[must_use]
                    fn on_data_changed(self: Pin<&mut MyObjectQt>, func: fn(Pin<&mut MyObjectQt>, trivial: i32, opaque: UniquePtr<QColor>)) -> CxxQtQMetaObjectConnection
                    {
                        self.connect_data_changed(func, CxxQtConnectionType::AutoConnection)
                    }
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[3],
            quote! {
                impl MyObjectQt {
                    #[doc = "Connect the given function pointer to the signal "]
                    #[doc = "unsafeSignal"]
                    #[doc = ", so that when the signal is emitted the function pointer is executed."]
                    #[doc = "\n"]
                    #[doc = "Note that this method uses a AutoConnection connection type."]
                    #[must_use]
                    fn on_unsafe_signal(self: Pin<&mut MyObjectQt>, func: fn(Pin<&mut MyObjectQt>, param: *mut T)) -> CxxQtQMetaObjectConnection
                    {
                        self.connect_unsafe_signal(func, CxxQtConnectionType::AutoConnection)
                    }
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[4],
            quote! {
                impl MyObjectQt {
                    #[doc = "Connect the given function pointer to the signal "]
                    #[doc = "baseName"]
                    #[doc = ", so that when the signal is emitted the function pointer is executed."]
                    #[doc = "\n"]
                    #[doc = "Note that this method uses a AutoConnection connection type."]
                    #[must_use]
                    fn on_existing_signal(self: Pin<&mut MyObjectQt>, func: fn(Pin<&mut MyObjectQt>)) -> CxxQtQMetaObjectConnection
                    {
                        self.connect_existing_signal(func, CxxQtConnectionType::AutoConnection)
                    }
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[5],
            quote! {
                impl MyObjectQt {
                    #[doc = "Emit the signal from the enum "]
                    #[doc = "MySignals"]
                    #[doc = " on the QObject "]
                    #[doc = "MyObject"]
                    pub fn emit(self: Pin<&mut Self>, signal: MySignals) {
                        match signal {
                            MySignals::Ready {} => { self.emit_ready() },
                            MySignals::DataChanged { trivial, opaque } => { self.emit_data_changed(trivial, opaque) },
                            MySignals::UnsafeSignal { param } => unsafe { self.emit_unsafe_signal(param) },
                            MySignals::ExistingSignal {} => { self.emit_existing_signal() }
                        }
                    }
                }
            },
        );
    }
}
