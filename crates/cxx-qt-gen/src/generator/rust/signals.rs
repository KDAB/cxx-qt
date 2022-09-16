// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{
    generator::{
        naming::{qobject::QObjectName, signals::QSignalName},
        rust::{fragment::RustFragmentPair, qobject::GeneratedRustQObjectBlocks},
    },
    parser::signals::ParsedSignalsEnum,
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Result};

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
        let queued_ident_cpp = &idents.queued_name.cpp;
        let queued_ident_rust = &idents.queued_name.rust;
        let queued_ident_rust_str = idents.queued_name.rust.to_string();

        let parameter_signatures = if signal.parameters.is_empty() {
            quote! { self: Pin<&mut #cpp_class_name_rust> }
        } else {
            let parameters = signal
                .parameters
                .iter()
                .map(|parameter| {
                    let ident = &parameter.ident;
                    let ty = &parameter.ty;
                    quote! { #ident: #ty }
                })
                .collect::<Vec<TokenStream>>();
            quote! { self: Pin<&mut #cpp_class_name_rust>, #(#parameters),* }
        };
        let parameter_names = signal
            .parameters
            .iter()
            .map(|parameter| parameter.ident.clone())
            .collect::<Vec<Ident>>();

        let fragment = RustFragmentPair {
            cxx_bridge: vec![quote! {
                unsafe extern "C++" {
                    #[rust_name = #queued_ident_rust_str]
                    fn #queued_ident_cpp(#parameter_signatures);
                }
            }],
            implementation: vec![],
        };
        signal_matches.push(quote! {
            #signal_enum_ident::#signal_ident_rust { #(#parameter_names),* } => self.#queued_ident_rust(#(#parameter_names),*)
        });

        generated
            .cxx_mod_contents
            .append(&mut fragment.cxx_bridge_as_items()?);
        generated
            .cxx_qt_mod_contents
            .append(&mut fragment.implementation_as_items()?);
    }

    // Add the Rust method using the enum to call the methods
    generated.cxx_qt_mod_contents.push(syn::parse2(quote! {
        impl #cpp_class_name_rust {
            pub fn emit_queued(self: Pin<&mut Self>, signal: #signal_enum_ident) {
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

    use crate::tests::{assert_tokens_eq, tokens_to_syn};
    use crate::{
        generator::naming::qobject::tests::create_qobjectname, parser::signals::ParsedSignalsEnum,
    };
    use quote::quote;
    use syn::ItemEnum;

    #[test]
    fn test_generate_rust_signals() {
        let e: ItemEnum = tokens_to_syn(quote! {
            #[cxx_qt::signals(MyObject)]
            enum MySignals {
                Ready,
                DataChanged {
                    trivial: i32,
                    #[cxx_type = "QColor"]
                    opaque: UniquePtr<QColor>
                },
            }
        });
        let signals_enum = ParsedSignalsEnum::from(&e, 0).unwrap();
        let qobject_idents = create_qobjectname();

        let generated = generate_rust_signals(&signals_enum, &qobject_idents).unwrap();

        assert_eq!(generated.cxx_mod_contents.len(), 2);
        assert_eq!(generated.cxx_qt_mod_contents.len(), 2);

        // Ready
        assert_tokens_eq(
            &generated.cxx_mod_contents[0],
            quote! {
                unsafe extern "C++" {
                    #[rust_name = "emit_ready"]
                    fn emitReady(self: Pin<&mut MyObjectQt>);
                }
            },
        );

        // DataChanged
        assert_tokens_eq(
            &generated.cxx_mod_contents[1],
            quote! {
                unsafe extern "C++" {
                    #[rust_name = "emit_data_changed"]
                    fn emitDataChanged(self: Pin<&mut MyObjectQt>, trivial: i32, opaque: UniquePtr<QColor>);
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
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[1],
            quote! {
                impl MyObjectQt {
                    pub fn emit_queued(self: Pin<&mut Self>, signal: MySignals) {
                        match signal {
                            MySignals::Ready {} => self.emit_ready(),
                            MySignals::DataChanged { trivial, opaque } => self.emit_data_changed(trivial, opaque)
                        }
                    }
                }
            },
        );
    }
}
