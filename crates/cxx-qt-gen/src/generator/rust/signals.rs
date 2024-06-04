// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{
    generator::{
        naming::{
            qobject::QObjectNames,
            signals::{QSignalHelperName, QSignalName},
        },
        rust::fragment::{GeneratedRustFragment, RustFragmentPair},
    },
    naming::{rust::syn_type_cxx_bridge_to_qualified, Name, TypeNames},
    parser::signals::ParsedSignal,
    syntax::attribute::attribute_find_path,
};
use quote::quote;
use syn::{parse_quote, FnArg, Ident, Result, Type};

pub fn generate_rust_signal(
    signal: &ParsedSignal,
    qobject_name: &Name,
    type_names: &TypeNames,
    module_ident: &Ident,
) -> Result<GeneratedRustFragment> {
    let idents = QSignalName::from(signal);
    let idents_helper = QSignalHelperName::new(&idents, qobject_name)?;

    let qobject_name_rust = qobject_name.rust_unqualified();

    let signal_name_cpp = idents.name.cxx_unqualified();
    let connect_ident_rust = idents.connect_name.rust;
    let on_ident_rust = idents.on_name;
    let original_method = &signal.method;

    let free_connect_ident_cpp = idents_helper.connect_name.cpp;
    let free_connect_ident_rust = idents_helper.connect_name.rust;
    let free_connect_ident_rust_str = free_connect_ident_rust.to_string();

    let parameters_cxx: Vec<FnArg> = signal
        .parameters
        .iter()
        .map(|parameter| {
            let ident = &parameter.ident;
            let ty = &parameter.ty;
            parse_quote! { #ident: #ty }
        })
        .collect();
    let parameters_qualified_arg: Vec<FnArg> = parameters_cxx
        .iter()
        .cloned()
        .map(|mut parameter| -> Result<_> {
            if let FnArg::Typed(pat_type) = &mut parameter {
                *pat_type.ty = syn_type_cxx_bridge_to_qualified(&pat_type.ty, type_names)?;
            }
            Ok(parameter)
        })
        .collect::<Result<_>>()?;
    let parameters_name: Vec<Ident> = signal
        .parameters
        .iter()
        .map(|parameter| parameter.ident.clone())
        .collect();
    let parameters_qualified_type: Vec<Type> = parameters_cxx
        .iter()
        .cloned()
        .map(|parameter| match parameter {
            FnArg::Typed(pat_type) => syn_type_cxx_bridge_to_qualified(&pat_type.ty, type_names),
            _ => unreachable!("should only have typed no receiver"),
        })
        .collect::<Result<_>>()?;

    let self_type_cxx = if signal.mutable {
        parse_quote! { Pin<&mut #qobject_name_rust> }
    } else {
        parse_quote! { &#qobject_name_rust }
    };
    let self_type_qualified = syn_type_cxx_bridge_to_qualified(&self_type_cxx, type_names)?;
    let qualified_impl = qobject_name.rust_qualified();

    let mut unsafe_block = None;
    let mut unsafe_call = Some(quote! { unsafe });
    if signal.safe {
        std::mem::swap(&mut unsafe_call, &mut unsafe_block);
    }

    let mut cxx_bridge = vec![];

    // TODO: what happens with RustQt signals, can they be private yet?
    if !signal.private {
        cxx_bridge.push(quote! {
            #unsafe_block extern "C++" {
                #original_method
            }
        });
    }

    let closure_struct = idents_helper.struct_closure;
    let signal_handler_alias = idents_helper.handler_alias;
    let signal_handler_alias_namespaced_str = idents_helper.handler_alias_namespaced.to_string();
    let signal_handler_call = idents_helper.function_call;
    let signal_handler_drop = idents_helper.function_drop;
    let namespace_str = idents_helper.namespace.to_string();

    cxx_bridge.push(quote! {
        unsafe extern "C++" {
            #[doc(hidden)]
            #[namespace = #namespace_str]
            type #signal_handler_alias = cxx_qt::signalhandler::CxxQtSignalHandler<super::#closure_struct>;

            #[doc(hidden)]
            #[namespace = #namespace_str]
            #[rust_name = #free_connect_ident_rust_str]
            fn #free_connect_ident_cpp(self_value: #self_type_cxx, signal_handler: #signal_handler_alias, conn_type: CxxQtConnectionType) -> CxxQtQMetaObjectConnection;
        }
    });

    cxx_bridge.push(quote! {
        #[namespace = #namespace_str]
        extern "Rust" {
            #[doc(hidden)]
            fn #signal_handler_drop(handler: #signal_handler_alias);

            #[doc(hidden)]
            #unsafe_call fn #signal_handler_call(handler: &mut #signal_handler_alias, self_value: #self_type_cxx, #(#parameters_cxx),*);
        }
    });

    let fragment = RustFragmentPair {
        cxx_bridge,
        implementation: vec![
            quote! {
                impl #qualified_impl {
                    #[doc = "Connect the given function pointer to the signal "]
                    #[doc = #signal_name_cpp]
                    #[doc = ", so that when the signal is emitted the function pointer is executed."]
                    pub fn #connect_ident_rust<F: FnMut(#self_type_qualified, #(#parameters_qualified_type),*) + 'static>(self: #self_type_qualified, mut closure: F, conn_type: cxx_qt::ConnectionType) -> cxx_qt::QMetaObjectConnectionGuard
                    {
                        cxx_qt::QMetaObjectConnectionGuard::from(#module_ident::#free_connect_ident_rust(
                            self,
                            cxx_qt::signalhandler::CxxQtSignalHandler::<#closure_struct>::new(Box::new(closure)),
                            conn_type,
                        ))
                    }
                }
            },
            quote! {
                impl #qualified_impl {
                    #[doc = "Connect the given function pointer to the signal "]
                    #[doc = #signal_name_cpp]
                    #[doc = ", so that when the signal is emitted the function pointer is executed."]
                    #[doc = "\n"]
                    #[doc = "Note that this method uses a AutoConnection connection type."]
                    pub fn #on_ident_rust<F: FnMut(#self_type_qualified, #(#parameters_qualified_type),*) + 'static>(self: #self_type_qualified, mut closure: F) -> cxx_qt::QMetaObjectConnectionGuard
                    {
                        cxx_qt::QMetaObjectConnectionGuard::from(#module_ident::#free_connect_ident_rust(
                            self,
                            cxx_qt::signalhandler::CxxQtSignalHandler::<#closure_struct>::new(Box::new(closure)),
                            cxx_qt::ConnectionType::AutoConnection,
                        ))
                    }
                }
            },
            quote! {
                #[doc(hidden)]
                pub struct #closure_struct {}
            },
            quote! {
                impl cxx_qt::signalhandler::CxxQtSignalHandlerClosure for #closure_struct {
                    type Id = cxx::type_id!(#signal_handler_alias_namespaced_str);
                    type FnType = dyn FnMut(#self_type_qualified, #(#parameters_qualified_type),*);
                }
            },
            quote! {
                use core::mem::drop as #signal_handler_drop;
            },
            quote! {
                fn #signal_handler_call(
                    handler: &mut cxx_qt::signalhandler::CxxQtSignalHandler<#closure_struct>,
                    self_value: #self_type_qualified,
                    #(#parameters_qualified_arg),*
                ) {
                    handler.closure()(self_value, #(#parameters_name),*);
                }
            },
            quote! {
                cxx_qt::static_assertions::assert_eq_align!(cxx_qt::signalhandler::CxxQtSignalHandler<#closure_struct>, usize);
            },
            quote! {
                cxx_qt::static_assertions::assert_eq_size!(cxx_qt::signalhandler::CxxQtSignalHandler<#closure_struct>, [usize; 2]);
            },
        ],
    };

    let mut generated = GeneratedRustFragment::default();
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
    qobject_idents: &QObjectNames,
    type_names: &TypeNames,
    module_ident: &Ident,
) -> Result<GeneratedRustFragment> {
    let mut generated = GeneratedRustFragment::default();

    // Create the methods for the other signals
    for signal in signals {
        let signal = {
            let mut signal = signal.clone();

            // Inject a cxx_name if there isn't any custom naming as we automatically rename RustQt signals
            if attribute_find_path(&signal.method.attrs, &["cxx_name"]).is_none()
                && attribute_find_path(&signal.method.attrs, &["rust_name"]).is_none()
            {
                let idents = QSignalName::from(&signal);
                let signal_name_cpp = idents.name.cxx_unqualified();
                signal
                    .method
                    .attrs
                    .push(parse_quote!(#[cxx_name = #signal_name_cpp]));
                signal
            } else {
                signal
            }
        };
        generated.append(&mut generate_rust_signal(
            &signal,
            &qobject_idents.name,
            type_names,
            module_ident,
        )?);
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
            name: Name::new(format_ident!("ready")),
            safe: true,
            inherit: false,
            private: false,
        };
        let qobject_idents = create_qobjectname();

        let generated = generate_rust_signals(
            &vec![qsignal],
            &qobject_idents,
            &TypeNames::mock(),
            &format_ident!("ffi"),
        )
        .unwrap();

        assert_eq!(generated.cxx_mod_contents.len(), 3);
        assert_eq!(generated.cxx_qt_mod_contents.len(), 8);

        assert_tokens_eq(
            &generated.cxx_mod_contents[0],
            quote! {
                unsafe extern "C++" {
                    #[cxx_name = "ready"]
                    fn ready(self: Pin<&mut MyObject>);
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_mod_contents[1],
            quote! {
                unsafe extern "C++" {
                    #[doc(hidden)]
                    #[namespace = "rust::cxxqtgen1"]
                    type MyObjectCxxQtSignalHandlerready = cxx_qt::signalhandler::CxxQtSignalHandler<super::MyObjectCxxQtSignalClosureready>;

                    #[doc(hidden)]
                    #[namespace = "rust::cxxqtgen1"]
                    #[rust_name = "MyObject_connect_ready"]
                    fn MyObject_readyConnect(self_value: Pin<&mut MyObject>, signal_handler: MyObjectCxxQtSignalHandlerready, conn_type: CxxQtConnectionType) -> CxxQtQMetaObjectConnection;
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_mod_contents[2],
            quote! {
                #[namespace = "rust::cxxqtgen1"]
                extern "Rust" {
                    #[doc(hidden)]
                    fn drop_MyObject_signal_handler_ready(handler: MyObjectCxxQtSignalHandlerready);

                    #[doc(hidden)]
                    fn call_MyObject_signal_handler_ready(handler: &mut MyObjectCxxQtSignalHandlerready, self_value: Pin<&mut MyObject>, );
                }
            },
        );

        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[0],
            quote! {
                impl qobject::MyObject {
                    #[doc = "Connect the given function pointer to the signal "]
                    #[doc = "ready"]
                    #[doc = ", so that when the signal is emitted the function pointer is executed."]
                    pub fn connect_ready<F: FnMut(core::pin::Pin<&mut qobject::MyObject>, ) + 'static>(self: core::pin::Pin<&mut qobject::MyObject>, mut closure: F, conn_type: cxx_qt::ConnectionType) -> cxx_qt::QMetaObjectConnectionGuard
                    {
                        cxx_qt::QMetaObjectConnectionGuard::from(ffi::MyObject_connect_ready(
                            self,
                            cxx_qt::signalhandler::CxxQtSignalHandler::<MyObjectCxxQtSignalClosureready>::new(Box::new(closure)),
                            conn_type,
                        ))
                    }
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[1],
            quote! {
                impl qobject::MyObject {
                    #[doc = "Connect the given function pointer to the signal "]
                    #[doc = "ready"]
                    #[doc = ", so that when the signal is emitted the function pointer is executed."]
                    #[doc = "\n"]
                    #[doc = "Note that this method uses a AutoConnection connection type."]
                    pub fn on_ready<F: FnMut(core::pin::Pin<&mut qobject::MyObject>, ) + 'static>(self: core::pin::Pin<&mut qobject::MyObject>, mut closure: F) -> cxx_qt::QMetaObjectConnectionGuard
                    {
                        cxx_qt::QMetaObjectConnectionGuard::from(ffi::MyObject_connect_ready(
                            self,
                            cxx_qt::signalhandler::CxxQtSignalHandler::<MyObjectCxxQtSignalClosureready>::new(Box::new(closure)),
                            cxx_qt::ConnectionType::AutoConnection,
                        ))
                    }
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[2],
            quote! {
                #[doc(hidden)]
                pub struct MyObjectCxxQtSignalClosureready {}
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[3],
            quote! {
                impl cxx_qt::signalhandler::CxxQtSignalHandlerClosure for MyObjectCxxQtSignalClosureready {
                    type Id = cxx::type_id!("::rust::cxxqtgen1::MyObjectCxxQtSignalHandlerready");
                    type FnType = dyn FnMut(core::pin::Pin<&mut qobject::MyObject>, );
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[4],
            quote! {
                use core::mem::drop as drop_MyObject_signal_handler_ready;
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[5],
            quote! {
                fn call_MyObject_signal_handler_ready(
                    handler: &mut cxx_qt::signalhandler::CxxQtSignalHandler<MyObjectCxxQtSignalClosureready>,
                    self_value: core::pin::Pin<&mut qobject::MyObject>,
                ) {
                    handler.closure()(self_value, );
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[6],
            quote! {
                cxx_qt::static_assertions::assert_eq_align!(cxx_qt::signalhandler::CxxQtSignalHandler<MyObjectCxxQtSignalClosureready>, usize);
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[7],
            quote! {
                cxx_qt::static_assertions::assert_eq_size!(cxx_qt::signalhandler::CxxQtSignalHandler<MyObjectCxxQtSignalClosureready>, [usize; 2]);
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
            name: Name::new(format_ident!("data_changed")).with_cxx_name("dataChanged".to_owned()),
            safe: true,
            inherit: false,
            private: false,
        };
        let qobject_idents = create_qobjectname();

        let mut type_names = TypeNames::mock();
        type_names.mock_insert("QColor", None, None, None);
        let generated = generate_rust_signals(
            &vec![qsignal],
            &qobject_idents,
            &type_names,
            &format_ident!("ffi"),
        )
        .unwrap();

        assert_eq!(generated.cxx_mod_contents.len(), 3);
        assert_eq!(generated.cxx_qt_mod_contents.len(), 8);

        assert_tokens_eq(
            &generated.cxx_mod_contents[0],
            quote! {
                unsafe extern "C++" {
                    #[attribute]
                    #[cxx_name = "dataChanged"]
                    fn data_changed(self: Pin<&mut MyObject>, trivial: i32, opaque: UniquePtr<QColor>);
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_mod_contents[1],
            quote! {
                unsafe extern "C++" {
                    #[doc(hidden)]
                    #[namespace = "rust::cxxqtgen1"]
                    type MyObjectCxxQtSignalHandlerdataChanged = cxx_qt::signalhandler::CxxQtSignalHandler<super::MyObjectCxxQtSignalClosuredataChanged>;

                    #[doc(hidden)]
                    #[namespace = "rust::cxxqtgen1"]
                    #[rust_name = "MyObject_connect_data_changed"]
                    fn MyObject_dataChangedConnect(self_value: Pin<&mut MyObject>, signal_handler: MyObjectCxxQtSignalHandlerdataChanged, conn_type: CxxQtConnectionType) -> CxxQtQMetaObjectConnection;
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_mod_contents[2],
            quote! {
                #[namespace = "rust::cxxqtgen1"]
                extern "Rust" {
                    #[doc(hidden)]
                    fn drop_MyObject_signal_handler_dataChanged(handler: MyObjectCxxQtSignalHandlerdataChanged);

                    #[doc(hidden)]
                    fn call_MyObject_signal_handler_dataChanged(handler: &mut MyObjectCxxQtSignalHandlerdataChanged, self_value: Pin<&mut MyObject>, trivial: i32, opaque: UniquePtr<QColor>);
                }
            },
        );

        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[0],
            quote! {
                impl qobject::MyObject {
                    #[doc = "Connect the given function pointer to the signal "]
                    #[doc = "dataChanged"]
                    #[doc = ", so that when the signal is emitted the function pointer is executed."]
                    pub fn connect_data_changed<F: FnMut(core::pin::Pin<&mut qobject::MyObject>, i32, cxx::UniquePtr<QColor>) + 'static>(self: core::pin::Pin<&mut qobject::MyObject>, mut closure: F, conn_type: cxx_qt::ConnectionType) -> cxx_qt::QMetaObjectConnectionGuard
                    {
                        cxx_qt::QMetaObjectConnectionGuard::from(ffi::MyObject_connect_data_changed(
                            self,
                            cxx_qt::signalhandler::CxxQtSignalHandler::<MyObjectCxxQtSignalClosuredataChanged>::new(Box::new(closure)),
                            conn_type,
                        ))
                    }
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[1],
            quote! {
                impl qobject::MyObject {
                    #[doc = "Connect the given function pointer to the signal "]
                    #[doc = "dataChanged"]
                    #[doc = ", so that when the signal is emitted the function pointer is executed."]
                    #[doc = "\n"]
                    #[doc = "Note that this method uses a AutoConnection connection type."]
                    pub fn on_data_changed<F: FnMut(core::pin::Pin<&mut qobject::MyObject>, i32, cxx::UniquePtr<QColor>) + 'static>(self: core::pin::Pin<&mut qobject::MyObject>, mut closure: F) -> cxx_qt::QMetaObjectConnectionGuard
                    {
                        cxx_qt::QMetaObjectConnectionGuard::from(ffi::MyObject_connect_data_changed(
                            self,
                            cxx_qt::signalhandler::CxxQtSignalHandler::<MyObjectCxxQtSignalClosuredataChanged>::new(Box::new(closure)),
                            cxx_qt::ConnectionType::AutoConnection,
                        ))
                    }
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[2],
            quote! {
                #[doc(hidden)]
                pub struct MyObjectCxxQtSignalClosuredataChanged {}
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[3],
            quote! {
                impl cxx_qt::signalhandler::CxxQtSignalHandlerClosure for MyObjectCxxQtSignalClosuredataChanged {
                    type Id = cxx::type_id!("::rust::cxxqtgen1::MyObjectCxxQtSignalHandlerdataChanged");
                    type FnType = dyn FnMut(core::pin::Pin<&mut qobject::MyObject>, i32, cxx::UniquePtr<QColor>);
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[4],
            quote! {
                use core::mem::drop as drop_MyObject_signal_handler_dataChanged;
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[5],
            quote! {
                fn call_MyObject_signal_handler_dataChanged(
                    handler: &mut cxx_qt::signalhandler::CxxQtSignalHandler<MyObjectCxxQtSignalClosuredataChanged>,
                    self_value: core::pin::Pin<&mut qobject::MyObject>,
                    trivial: i32,
                    opaque: cxx::UniquePtr<QColor>
                ) {
                    handler.closure()(self_value, trivial, opaque);
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[6],
            quote! {
                cxx_qt::static_assertions::assert_eq_align!(cxx_qt::signalhandler::CxxQtSignalHandler<MyObjectCxxQtSignalClosuredataChanged>, usize);
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[7],
            quote! {
                cxx_qt::static_assertions::assert_eq_size!(cxx_qt::signalhandler::CxxQtSignalHandler<MyObjectCxxQtSignalClosuredataChanged>, [usize; 2]);
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
            name: Name::new(format_ident!("unsafe_signal"))
                .with_cxx_name("unsafeSignal".to_owned()),
            safe: false,
            inherit: false,
            private: false,
        };
        let qobject_idents = create_qobjectname();

        let mut type_names = TypeNames::mock();
        type_names.mock_insert("T", None, None, None);
        let generated = generate_rust_signals(
            &vec![qsignal],
            &qobject_idents,
            &type_names,
            &format_ident!("ffi"),
        )
        .unwrap();

        assert_eq!(generated.cxx_mod_contents.len(), 3);
        assert_eq!(generated.cxx_qt_mod_contents.len(), 8);

        assert_tokens_eq(
            &generated.cxx_mod_contents[0],
            quote! {
                extern "C++" {
                    #[cxx_name = "unsafeSignal"]
                    unsafe fn unsafe_signal(self: Pin<&mut MyObject>, param: *mut T);
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_mod_contents[1],
            quote! {
                unsafe extern "C++" {
                    #[doc(hidden)]
                    #[namespace = "rust::cxxqtgen1"]
                    type MyObjectCxxQtSignalHandlerunsafeSignal = cxx_qt::signalhandler::CxxQtSignalHandler<super::MyObjectCxxQtSignalClosureunsafeSignal>;

                    #[doc(hidden)]
                    #[namespace = "rust::cxxqtgen1"]
                    #[rust_name = "MyObject_connect_unsafe_signal"]
                    fn MyObject_unsafeSignalConnect(self_value: Pin<&mut MyObject>, signal_handler: MyObjectCxxQtSignalHandlerunsafeSignal, conn_type: CxxQtConnectionType) -> CxxQtQMetaObjectConnection;
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_mod_contents[2],
            quote! {
                #[namespace = "rust::cxxqtgen1"]
                extern "Rust" {
                    #[doc(hidden)]
                    fn drop_MyObject_signal_handler_unsafeSignal(handler: MyObjectCxxQtSignalHandlerunsafeSignal);

                    #[doc(hidden)]
                    unsafe fn call_MyObject_signal_handler_unsafeSignal(handler: &mut MyObjectCxxQtSignalHandlerunsafeSignal, self_value: Pin<&mut MyObject>, param: *mut T);
                }
            },
        );

        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[0],
            quote! {
                impl qobject::MyObject {
                    #[doc = "Connect the given function pointer to the signal "]
                    #[doc = "unsafeSignal"]
                    #[doc = ", so that when the signal is emitted the function pointer is executed."]
                    pub fn connect_unsafe_signal<F: FnMut(core::pin::Pin<&mut qobject::MyObject>, *mut T) + 'static>(self: core::pin::Pin<&mut qobject::MyObject>, mut closure: F, conn_type: cxx_qt::ConnectionType) -> cxx_qt::QMetaObjectConnectionGuard
                    {
                        cxx_qt::QMetaObjectConnectionGuard::from(ffi::MyObject_connect_unsafe_signal(
                            self,
                            cxx_qt::signalhandler::CxxQtSignalHandler::<MyObjectCxxQtSignalClosureunsafeSignal>::new(Box::new(closure)),
                            conn_type,
                        ))
                    }
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[1],
            quote! {
                impl qobject::MyObject {
                    #[doc = "Connect the given function pointer to the signal "]
                    #[doc = "unsafeSignal"]
                    #[doc = ", so that when the signal is emitted the function pointer is executed."]
                    #[doc = "\n"]
                    #[doc = "Note that this method uses a AutoConnection connection type."]
                    pub fn on_unsafe_signal<F: FnMut(core::pin::Pin<&mut qobject::MyObject>, *mut T) + 'static>(self: core::pin::Pin<&mut qobject::MyObject>, mut closure: F) -> cxx_qt::QMetaObjectConnectionGuard
                    {
                        cxx_qt::QMetaObjectConnectionGuard::from(ffi::MyObject_connect_unsafe_signal(
                            self,
                            cxx_qt::signalhandler::CxxQtSignalHandler::<MyObjectCxxQtSignalClosureunsafeSignal>::new(Box::new(closure)),
                            cxx_qt::ConnectionType::AutoConnection,
                        ))
                    }
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[2],
            quote! {
                #[doc(hidden)]
                pub struct MyObjectCxxQtSignalClosureunsafeSignal {}
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[3],
            quote! {
                impl cxx_qt::signalhandler::CxxQtSignalHandlerClosure for MyObjectCxxQtSignalClosureunsafeSignal {
                    type Id = cxx::type_id!("::rust::cxxqtgen1::MyObjectCxxQtSignalHandlerunsafeSignal");
                    type FnType = dyn FnMut(core::pin::Pin<&mut qobject::MyObject>, *mut T);
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[4],
            quote! {
                use core::mem::drop as drop_MyObject_signal_handler_unsafeSignal;
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[5],
            quote! {
                fn call_MyObject_signal_handler_unsafeSignal(
                    handler: &mut cxx_qt::signalhandler::CxxQtSignalHandler<MyObjectCxxQtSignalClosureunsafeSignal>,
                    self_value: core::pin::Pin<&mut qobject::MyObject>,
                    param: *mut T
                ) {
                    handler.closure()(self_value, param);
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[6],
            quote! {
                cxx_qt::static_assertions::assert_eq_align!(cxx_qt::signalhandler::CxxQtSignalHandler<MyObjectCxxQtSignalClosureunsafeSignal>, usize);
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[7],
            quote! {
                cxx_qt::static_assertions::assert_eq_size!(cxx_qt::signalhandler::CxxQtSignalHandler<MyObjectCxxQtSignalClosureunsafeSignal>, [usize; 2]);
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
            name: Name::new(format_ident!("existing_signal")).with_cxx_name("baseName".to_owned()),
            safe: true,
            inherit: true,
            private: false,
        };
        let qobject_idents = create_qobjectname();

        let generated = generate_rust_signals(
            &vec![qsignal],
            &qobject_idents,
            &TypeNames::mock(),
            &format_ident!("ffi"),
        )
        .unwrap();

        assert_eq!(generated.cxx_mod_contents.len(), 3);
        assert_eq!(generated.cxx_qt_mod_contents.len(), 8);

        assert_tokens_eq(
            &generated.cxx_mod_contents[0],
            quote! {
                unsafe extern "C++" {
                    #[inherit]
                    #[cxx_name = "baseName"]
                    fn existing_signal(self: Pin<&mut MyObject>, );
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_mod_contents[1],
            quote! {
                unsafe extern "C++" {
                    #[doc(hidden)]
                    #[namespace = "rust::cxxqtgen1"]
                    type MyObjectCxxQtSignalHandlerbaseName = cxx_qt::signalhandler::CxxQtSignalHandler<super::MyObjectCxxQtSignalClosurebaseName>;

                    #[doc(hidden)]
                    #[namespace = "rust::cxxqtgen1"]
                    #[rust_name = "MyObject_connect_existing_signal"]
                    fn MyObject_baseNameConnect(self_value: Pin<&mut MyObject>, signal_handler: MyObjectCxxQtSignalHandlerbaseName, conn_type: CxxQtConnectionType) -> CxxQtQMetaObjectConnection;
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_mod_contents[2],
            quote! {
                #[namespace = "rust::cxxqtgen1"]
                extern "Rust" {
                    #[doc(hidden)]
                    fn drop_MyObject_signal_handler_baseName(handler: MyObjectCxxQtSignalHandlerbaseName);

                    #[doc(hidden)]
                    fn call_MyObject_signal_handler_baseName(handler: &mut MyObjectCxxQtSignalHandlerbaseName, self_value: Pin<&mut MyObject>, );
                }
            },
        );

        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[0],
            quote! {
                impl qobject::MyObject {
                    #[doc = "Connect the given function pointer to the signal "]
                    #[doc = "baseName"]
                    #[doc = ", so that when the signal is emitted the function pointer is executed."]
                    pub fn connect_existing_signal<F: FnMut(core::pin::Pin<&mut qobject::MyObject>, ) + 'static>(self: core::pin::Pin<&mut qobject::MyObject>, mut closure: F, conn_type: cxx_qt::ConnectionType) -> cxx_qt::QMetaObjectConnectionGuard
                    {
                        cxx_qt::QMetaObjectConnectionGuard::from(ffi::MyObject_connect_existing_signal(
                            self,
                            cxx_qt::signalhandler::CxxQtSignalHandler::<MyObjectCxxQtSignalClosurebaseName>::new(Box::new(closure)),
                            conn_type,
                        ))
                    }
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[1],
            quote! {
                impl qobject::MyObject {
                    #[doc = "Connect the given function pointer to the signal "]
                    #[doc = "baseName"]
                    #[doc = ", so that when the signal is emitted the function pointer is executed."]
                    #[doc = "\n"]
                    #[doc = "Note that this method uses a AutoConnection connection type."]
                    pub fn on_existing_signal<F: FnMut(core::pin::Pin<&mut qobject::MyObject>, ) + 'static>(self: core::pin::Pin<&mut qobject::MyObject>, mut closure: F) -> cxx_qt::QMetaObjectConnectionGuard
                    {
                        cxx_qt::QMetaObjectConnectionGuard::from(ffi::MyObject_connect_existing_signal(
                            self,
                            cxx_qt::signalhandler::CxxQtSignalHandler::<MyObjectCxxQtSignalClosurebaseName>::new(Box::new(closure)),
                            cxx_qt::ConnectionType::AutoConnection,
                        ))
                    }
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[2],
            quote! {
                #[doc(hidden)]
                pub struct MyObjectCxxQtSignalClosurebaseName {}
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[3],
            quote! {
                impl cxx_qt::signalhandler::CxxQtSignalHandlerClosure for MyObjectCxxQtSignalClosurebaseName {
                    type Id = cxx::type_id!("::rust::cxxqtgen1::MyObjectCxxQtSignalHandlerbaseName");
                    type FnType = dyn FnMut(core::pin::Pin<&mut qobject::MyObject>, );
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[4],
            quote! {
                use core::mem::drop as drop_MyObject_signal_handler_baseName;
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[5],
            quote! {
                fn call_MyObject_signal_handler_baseName(
                    handler: &mut cxx_qt::signalhandler::CxxQtSignalHandler<MyObjectCxxQtSignalClosurebaseName>,
                    self_value: core::pin::Pin<&mut qobject::MyObject>,
                ) {
                    handler.closure()(self_value, );
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[6],
            quote! {
                cxx_qt::static_assertions::assert_eq_align!(cxx_qt::signalhandler::CxxQtSignalHandler<MyObjectCxxQtSignalClosurebaseName>, usize);
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[7],
            quote! {
                cxx_qt::static_assertions::assert_eq_size!(cxx_qt::signalhandler::CxxQtSignalHandler<MyObjectCxxQtSignalClosurebaseName>, [usize; 2]);
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
            name: Name::new(format_ident!("ready")),
            safe: true,
            inherit: false,
            private: false,
        };

        let qobject_name = TypeNames::mock()
            .lookup(&qsignal.qobject_ident)
            .unwrap()
            .clone();
        let generated = generate_rust_signal(
            &qsignal,
            &qobject_name,
            &TypeNames::mock(),
            &format_ident!("ffi"),
        )
        .unwrap();

        assert_eq!(generated.cxx_mod_contents.len(), 3);
        assert_eq!(generated.cxx_qt_mod_contents.len(), 8);

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
                    #[doc(hidden)]
                    #[namespace = "rust::cxxqtgen1"]
                    type MyObjectCxxQtSignalHandlerready = cxx_qt::signalhandler::CxxQtSignalHandler<super::MyObjectCxxQtSignalClosureready>;

                    #[doc(hidden)]
                    #[namespace = "rust::cxxqtgen1"]
                    #[rust_name = "MyObject_connect_ready"]
                    fn MyObject_readyConnect(self_value: Pin<&mut MyObject>, signal_handler: MyObjectCxxQtSignalHandlerready, conn_type: CxxQtConnectionType) -> CxxQtQMetaObjectConnection;
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_mod_contents[2],
            quote! {
                #[namespace = "rust::cxxqtgen1"]
                extern "Rust" {
                    #[doc(hidden)]
                    fn drop_MyObject_signal_handler_ready(handler: MyObjectCxxQtSignalHandlerready);

                    #[doc(hidden)]
                    fn call_MyObject_signal_handler_ready(handler: &mut MyObjectCxxQtSignalHandlerready, self_value: Pin<&mut MyObject>, );
                }
            },
        );

        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[0],
            quote! {
                impl qobject::MyObject {
                    #[doc = "Connect the given function pointer to the signal "]
                    #[doc = "ready"]
                    #[doc = ", so that when the signal is emitted the function pointer is executed."]
                    pub fn connect_ready<F: FnMut(core::pin::Pin<&mut qobject::MyObject>, ) + 'static>(self: core::pin::Pin<&mut qobject::MyObject>, mut closure: F, conn_type: cxx_qt::ConnectionType) -> cxx_qt::QMetaObjectConnectionGuard
                    {
                        cxx_qt::QMetaObjectConnectionGuard::from(ffi::MyObject_connect_ready(
                            self,
                            cxx_qt::signalhandler::CxxQtSignalHandler::<MyObjectCxxQtSignalClosureready>::new(Box::new(closure)),
                            conn_type,
                        ))
                    }
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[1],
            quote! {
                impl qobject::MyObject {
                    #[doc = "Connect the given function pointer to the signal "]
                    #[doc = "ready"]
                    #[doc = ", so that when the signal is emitted the function pointer is executed."]
                    #[doc = "\n"]
                    #[doc = "Note that this method uses a AutoConnection connection type."]
                    pub fn on_ready<F: FnMut(core::pin::Pin<&mut qobject::MyObject>, ) + 'static>(self: core::pin::Pin<&mut qobject::MyObject>, mut closure: F) -> cxx_qt::QMetaObjectConnectionGuard
                    {
                        cxx_qt::QMetaObjectConnectionGuard::from(ffi::MyObject_connect_ready(
                            self,
                            cxx_qt::signalhandler::CxxQtSignalHandler::<MyObjectCxxQtSignalClosureready>::new(Box::new(closure)),
                            cxx_qt::ConnectionType::AutoConnection,
                        ))
                    }
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[2],
            quote! {
                #[doc(hidden)]
                pub struct MyObjectCxxQtSignalClosureready {}
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[3],
            quote! {
                impl cxx_qt::signalhandler::CxxQtSignalHandlerClosure for MyObjectCxxQtSignalClosureready {
                    type Id = cxx::type_id!("::rust::cxxqtgen1::MyObjectCxxQtSignalHandlerready");
                    type FnType = dyn FnMut(core::pin::Pin<&mut qobject::MyObject>, );
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[4],
            quote! {
                use core::mem::drop as drop_MyObject_signal_handler_ready;
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[5],
            quote! {
                fn call_MyObject_signal_handler_ready(
                    handler: &mut cxx_qt::signalhandler::CxxQtSignalHandler<MyObjectCxxQtSignalClosureready>,
                    self_value: core::pin::Pin<&mut qobject::MyObject>,
                ) {
                    handler.closure()(self_value, );
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[6],
            quote! {
                cxx_qt::static_assertions::assert_eq_align!(cxx_qt::signalhandler::CxxQtSignalHandler<MyObjectCxxQtSignalClosureready>, usize);
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[7],
            quote! {
                cxx_qt::static_assertions::assert_eq_size!(cxx_qt::signalhandler::CxxQtSignalHandler<MyObjectCxxQtSignalClosureready>, [usize; 2]);
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
            name: Name::new(format_ident!("ready")),
            safe: true,
            inherit: false,
            private: true,
        };

        let qobject_name = TypeNames::mock()
            .lookup(&qsignal.qobject_ident)
            .unwrap()
            .clone();
        let generated = generate_rust_signal(
            &qsignal,
            &qobject_name,
            &TypeNames::mock(),
            &format_ident!("ffi"),
        )
        .unwrap();

        assert_eq!(generated.cxx_mod_contents.len(), 2);
        assert_eq!(generated.cxx_qt_mod_contents.len(), 8);

        assert_tokens_eq(
            &generated.cxx_mod_contents[0],
            quote! {
                unsafe extern "C++" {
                    #[doc(hidden)]
                    #[namespace = "rust::cxxqtgen1"]
                    type MyObjectCxxQtSignalHandlerready = cxx_qt::signalhandler::CxxQtSignalHandler<super::MyObjectCxxQtSignalClosureready>;

                    #[doc(hidden)]
                    #[namespace = "rust::cxxqtgen1"]
                    #[rust_name = "MyObject_connect_ready"]
                    fn MyObject_readyConnect(self_value: Pin<&mut MyObject>, signal_handler: MyObjectCxxQtSignalHandlerready, conn_type: CxxQtConnectionType) -> CxxQtQMetaObjectConnection;
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_mod_contents[1],
            quote! {
                #[namespace = "rust::cxxqtgen1"]
                extern "Rust" {
                    #[doc(hidden)]
                    fn drop_MyObject_signal_handler_ready(handler: MyObjectCxxQtSignalHandlerready);

                    #[doc(hidden)]
                    fn call_MyObject_signal_handler_ready(handler: &mut MyObjectCxxQtSignalHandlerready, self_value: Pin<&mut MyObject>, );
                }
            },
        );

        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[0],
            quote! {
                impl qobject::MyObject {
                    #[doc = "Connect the given function pointer to the signal "]
                    #[doc = "ready"]
                    #[doc = ", so that when the signal is emitted the function pointer is executed."]
                    pub fn connect_ready<F: FnMut(core::pin::Pin<&mut qobject::MyObject>, ) + 'static>(self: core::pin::Pin<&mut qobject::MyObject>, mut closure: F, conn_type: cxx_qt::ConnectionType) -> cxx_qt::QMetaObjectConnectionGuard
                    {
                        cxx_qt::QMetaObjectConnectionGuard::from(ffi::MyObject_connect_ready(
                            self,
                            cxx_qt::signalhandler::CxxQtSignalHandler::<MyObjectCxxQtSignalClosureready>::new(Box::new(closure)),
                            conn_type,
                        ))
                    }
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[1],
            quote! {
                impl qobject::MyObject {
                    #[doc = "Connect the given function pointer to the signal "]
                    #[doc = "ready"]
                    #[doc = ", so that when the signal is emitted the function pointer is executed."]
                    #[doc = "\n"]
                    #[doc = "Note that this method uses a AutoConnection connection type."]
                    pub fn on_ready<F: FnMut(core::pin::Pin<&mut qobject::MyObject>, ) + 'static>(self: core::pin::Pin<&mut qobject::MyObject>, mut closure: F) -> cxx_qt::QMetaObjectConnectionGuard
                    {
                        cxx_qt::QMetaObjectConnectionGuard::from(ffi::MyObject_connect_ready(
                            self,
                            cxx_qt::signalhandler::CxxQtSignalHandler::<MyObjectCxxQtSignalClosureready>::new(Box::new(closure)),
                            cxx_qt::ConnectionType::AutoConnection,
                        ))
                    }
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[2],
            quote! {
                #[doc(hidden)]
                pub struct MyObjectCxxQtSignalClosureready {}
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[3],
            quote! {
                impl cxx_qt::signalhandler::CxxQtSignalHandlerClosure for MyObjectCxxQtSignalClosureready {
                    type Id = cxx::type_id!("::rust::cxxqtgen1::MyObjectCxxQtSignalHandlerready");
                    type FnType = dyn FnMut(core::pin::Pin<&mut qobject::MyObject>, );
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[4],
            quote! {
                use core::mem::drop as drop_MyObject_signal_handler_ready;
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[5],
            quote! {
                fn call_MyObject_signal_handler_ready(
                    handler: &mut cxx_qt::signalhandler::CxxQtSignalHandler<MyObjectCxxQtSignalClosureready>,
                    self_value: core::pin::Pin<&mut qobject::MyObject>,
                ) {
                    handler.closure()(self_value, );
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[6],
            quote! {
                cxx_qt::static_assertions::assert_eq_align!(cxx_qt::signalhandler::CxxQtSignalHandler<MyObjectCxxQtSignalClosureready>, usize);
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[7],
            quote! {
                cxx_qt::static_assertions::assert_eq_size!(cxx_qt::signalhandler::CxxQtSignalHandler<MyObjectCxxQtSignalClosureready>, [usize; 2]);
            },
        );
    }
}
