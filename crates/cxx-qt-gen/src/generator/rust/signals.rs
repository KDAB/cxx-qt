// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::generator::rust::get_params_tokens;
use crate::{
    generator::{
        naming::{
            qobject::QObjectNames,
            signals::{QSignalHelperNames, QSignalNames},
        },
        rust::fragment::GeneratedRustFragment,
    },
    naming::{rust::syn_type_cxx_bridge_to_qualified, Name, TypeNames},
    parser::signals::ParsedSignal,
};
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{parse_quote, parse_quote_spanned, FnArg, Ident, Result, Type};

pub fn generate_rust_signal(
    signal: &ParsedSignal,
    qobject_name: &Name,
    type_names: &TypeNames,
    unsafety_block: Option<TokenStream>,
) -> Result<GeneratedRustFragment> {
    let span = signal.method.span();
    let idents = QSignalNames::from(signal);
    let idents_helper = QSignalHelperNames::new(&idents, qobject_name)?;

    let qobject_name_rust = qobject_name.rust_unqualified();

    let module_ident = qobject_name.require_module()?;

    let signal_name_cpp = idents.name.cxx_unqualified();
    let connect_ident_rust = idents.connect_name.rust_unqualified();
    let on_ident_rust = idents.on_name;

    let free_connect_ident_cpp = idents_helper.connect_name.cxx_unqualified();
    let free_connect_ident_rust = idents_helper.connect_name.rust_unqualified();

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
            } else {
                // CODECOV_EXCLUDE_START
                unreachable!("ParsedSignal strips the self parameter off already so this should be unreachable")
                // CODECOV_EXCLUDE_STOP
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
            _ => {
                // CODECOV_EXCLUDE_START
                unreachable!("should only have typed no receiver")
                // CODECOV_EXCLUDE_STOP
            }
        })
        .collect::<Result<_>>()?;

    let self_type_cxx = if signal.mutable {
        parse_quote_spanned! {span => Pin<&mut #qobject_name_rust> }
    } else {
        // CODECOV_EXCLUDE_START
        unreachable!("Signals cannot be immutable right now so this cannot be reached")
        // CODECOV_EXCLUDE_STOP
    };
    let self_type_qualified = syn_type_cxx_bridge_to_qualified(&self_type_cxx, type_names)?;
    let qualified_impl = qobject_name.rust_qualified();

    let rust_class_name = qobject_name.rust_unqualified();
    let cpp_ident = idents.name.cxx_unqualified();

    let unsafe_call = if signal.safe {
        None
    } else {
        Some(quote! { unsafe })
    };
    let doc_comments = &signal.docs;
    let cfgs = &signal.cfgs;
    let namespace = if let Some(namespace) = qobject_name.namespace() {
        quote_spanned! { span=> #[namespace = #namespace ] }
    } else {
        quote! {}
    };

    let signal_ident_cpp = idents.name.rust_unqualified();
    let parameter_signatures =
        get_params_tokens(signal.mutable, &signal.parameters, rust_class_name);

    let return_type = &signal.method.sig.output;

    let closure_struct = idents_helper.struct_closure;
    let signal_handler_alias = idents_helper.handler_alias;
    let signal_handler_alias_namespaced_str = idents_helper.handler_alias_namespaced.to_string();
    let signal_handler_call = idents_helper.function_call;
    let signal_handler_drop = idents_helper.function_drop;
    let namespace_str = idents_helper.namespace.to_string();

    let mut cxx_mod_contents = vec![];

    // TODO: what happens with RustQt signals, can they be private yet?
    if !signal.private {
        cxx_mod_contents.push(parse_quote_spanned! {
            span=>
            #unsafety_block extern "C++" {
                #[cxx_name = #cpp_ident]
                #(#cfgs)*
                #(#doc_comments)*
                #namespace
                #unsafe_call fn #signal_ident_cpp(#parameter_signatures) #return_type;
            }
        });
    }

    cxx_mod_contents.extend(vec![
        parse_quote_spanned! {
            span=>
            #(#cfgs)*
            unsafe extern "C++" {
                #[doc(hidden)]
                #[namespace = #namespace_str]
                type #signal_handler_alias = cxx_qt::signalhandler::CxxQtSignalHandler<super::#closure_struct>;

                #[doc(hidden)]
                #[namespace = #namespace_str]
                #[cxx_name = #free_connect_ident_cpp]
                fn #free_connect_ident_rust(self_value: #self_type_cxx, signal_handler: #signal_handler_alias, conn_type: CxxQtConnectionType) -> CxxQtQMetaObjectConnection;
            }
        },
        parse_quote_spanned! {
            span=>
            #[namespace = #namespace_str]
            extern "Rust" {
                #[doc(hidden)]
                fn #signal_handler_drop(handler: #signal_handler_alias);

                #[doc(hidden)]
                #unsafe_call fn #signal_handler_call(handler: &mut #signal_handler_alias, self_value: #self_type_cxx, #(#parameters_cxx),*);
            }
        }]);

    Ok(GeneratedRustFragment {
        cxx_mod_contents,
        cxx_qt_mod_contents: vec![
            parse_quote_spanned! {
                span=>
                #(#cfgs)*
                impl #qualified_impl {
                    #[doc = "Connect the given function pointer to the signal "]
                    #[doc = #signal_name_cpp]
                    #[doc = ", so that when the signal is emitted the function pointer is executed."]
                    pub fn #connect_ident_rust<F: FnMut(#self_type_qualified, #(#parameters_qualified_type),*) + 'static + Send>(self: #self_type_qualified, closure: F, conn_type: cxx_qt::ConnectionType) -> cxx_qt::QMetaObjectConnectionGuard
                    {
                        cxx_qt::QMetaObjectConnectionGuard::from(#module_ident::#free_connect_ident_rust(
                            self,
                            cxx_qt::signalhandler::CxxQtSignalHandler::<#closure_struct>::new(Box::new(closure)),
                            conn_type,
                        ))
                    }
                }
            },
            parse_quote_spanned! {
                span=>
                #(#cfgs)*
                impl #qualified_impl {
                    #[doc = "Connect the given function pointer to the signal "]
                    #[doc = #signal_name_cpp]
                    #[doc = ", so that when the signal is emitted the function pointer is executed."]
                    #[doc = "\n"]
                    #[doc = "Note that this method uses a AutoConnection connection type."]
                    pub fn #on_ident_rust<F: FnMut(#self_type_qualified, #(#parameters_qualified_type),*) + 'static + Send>(self: #self_type_qualified, closure: F) -> cxx_qt::QMetaObjectConnectionGuard
                    {
                        cxx_qt::QMetaObjectConnectionGuard::from(#module_ident::#free_connect_ident_rust(
                            self,
                            cxx_qt::signalhandler::CxxQtSignalHandler::<#closure_struct>::new(Box::new(closure)),
                            cxx_qt::ConnectionType::AutoConnection,
                        ))
                    }
                }
            },
            parse_quote_spanned! {
                span =>
                #(#cfgs)*
                #[doc(hidden)]
                pub struct #closure_struct {}
            },
            parse_quote_spanned! {
                span =>
                #(#cfgs)*
                impl cxx_qt::signalhandler::CxxQtSignalHandlerClosure for #closure_struct {
                    type Id = cxx::type_id!(#signal_handler_alias_namespaced_str);
                    type FnType = dyn FnMut(#self_type_qualified, #(#parameters_qualified_type),*) + Send;
                }
            },
            parse_quote_spanned! {
                span =>
                #(#cfgs)*
                use core::mem::drop as #signal_handler_drop;
            },
            parse_quote_spanned! {
                span =>
                #(#cfgs)*
                fn #signal_handler_call(
                    handler: &mut cxx_qt::signalhandler::CxxQtSignalHandler<#closure_struct>,
                    self_value: #self_type_qualified,
                    #(#parameters_qualified_arg),*
                ) {
                    handler.closure()(self_value, #(#parameters_name),*);
                }
            },
            parse_quote_spanned! {
                span =>
                #(#cfgs)*
                cxx_qt::static_assertions::assert_eq_align!(cxx_qt::signalhandler::CxxQtSignalHandler<#closure_struct>, usize);
            },
            parse_quote_spanned! {
                span =>
                #(#cfgs)*
                cxx_qt::static_assertions::assert_eq_size!(cxx_qt::signalhandler::CxxQtSignalHandler<#closure_struct>, [usize; 2]);
            },
        ],
    })
}

pub fn generate_rust_signals(
    signals: &[&ParsedSignal],
    qobject_names: &QObjectNames,
    type_names: &TypeNames,
) -> Result<GeneratedRustFragment> {
    let generated = signals
        .iter()
        .map(|signal| {
            generate_rust_signal(
                signal,
                &qobject_names.name,
                type_names,
                // When generating from a RustQt block we always use unsafe extern C++
                Some(quote! { unsafe }),
            )
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(GeneratedRustFragment::flatten(generated))
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::generator::naming::qobject::tests::create_qobjectname;
    use crate::parser::method::MethodFields;
    use crate::tests::assert_tokens_eq;
    use quote::{format_ident, quote};
    use syn::{parse_quote, ForeignItemFn, Item};

    fn common_asserts(cxx_mod_contents: &Vec<Item>, cxx_qt_mod_contents: &Vec<Item>) {
        assert_eq!(cxx_mod_contents.len(), 2);
        assert_eq!(cxx_qt_mod_contents.len(), 8);

        assert_tokens_eq(
            &cxx_mod_contents[0],
            quote! {
                unsafe extern "C++" {
                    #[doc(hidden)]
                    #[namespace = "rust::cxxqtgen1"]
                    type MyObjectCxxQtSignalHandlerready = cxx_qt::signalhandler::CxxQtSignalHandler<super::MyObjectCxxQtSignalClosureready>;

                    #[doc(hidden)]
                    #[namespace = "rust::cxxqtgen1"]
                    #[cxx_name = "MyObject_readyConnect"]
                    fn MyObject_connect_ready(self_value: Pin<&mut MyObject>, signal_handler: MyObjectCxxQtSignalHandlerready, conn_type: CxxQtConnectionType) -> CxxQtQMetaObjectConnection;
                }
            },
        );
        assert_tokens_eq(
            &cxx_mod_contents[1],
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
            &cxx_qt_mod_contents[0],
            quote! {
                impl qobject::MyObject {
                    #[doc = "Connect the given function pointer to the signal "]
                    #[doc = "ready"]
                    #[doc = ", so that when the signal is emitted the function pointer is executed."]
                    pub fn connect_ready<F: FnMut(core::pin::Pin<&mut qobject::MyObject>, ) + 'static + Send>(self: core::pin::Pin<&mut qobject::MyObject>, closure: F, conn_type: cxx_qt::ConnectionType) -> cxx_qt::QMetaObjectConnectionGuard
                    {
                        cxx_qt::QMetaObjectConnectionGuard::from(qobject::MyObject_connect_ready(
                            self,
                            cxx_qt::signalhandler::CxxQtSignalHandler::<MyObjectCxxQtSignalClosureready>::new(Box::new(closure)),
                            conn_type,
                        ))
                    }
                }
            },
        );
        assert_tokens_eq(
            &cxx_qt_mod_contents[1],
            quote! {
                impl qobject::MyObject {
                    #[doc = "Connect the given function pointer to the signal "]
                    #[doc = "ready"]
                    #[doc = ", so that when the signal is emitted the function pointer is executed."]
                    #[doc = "\n"]
                    #[doc = "Note that this method uses a AutoConnection connection type."]
                    pub fn on_ready<F: FnMut(core::pin::Pin<&mut qobject::MyObject>, ) + 'static + Send>(self: core::pin::Pin<&mut qobject::MyObject>, closure: F) -> cxx_qt::QMetaObjectConnectionGuard
                    {
                        cxx_qt::QMetaObjectConnectionGuard::from(qobject::MyObject_connect_ready(
                            self,
                            cxx_qt::signalhandler::CxxQtSignalHandler::<MyObjectCxxQtSignalClosureready>::new(Box::new(closure)),
                            cxx_qt::ConnectionType::AutoConnection,
                        ))
                    }
                }
            },
        );
        assert_tokens_eq(
            &cxx_qt_mod_contents[2],
            quote! {
                #[doc(hidden)]
                pub struct MyObjectCxxQtSignalClosureready {}
            },
        );
        assert_tokens_eq(
            &cxx_qt_mod_contents[3],
            quote! {
                impl cxx_qt::signalhandler::CxxQtSignalHandlerClosure for MyObjectCxxQtSignalClosureready {
                    type Id = cxx::type_id!("::rust::cxxqtgen1::MyObjectCxxQtSignalHandlerready");
                    type FnType = dyn FnMut(core::pin::Pin<&mut qobject::MyObject>, ) + Send;
                }
            },
        );
        assert_tokens_eq(
            &cxx_qt_mod_contents[4],
            quote! {
                use core::mem::drop as drop_MyObject_signal_handler_ready;
            },
        );
        assert_tokens_eq(
            &cxx_qt_mod_contents[5],
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
            &cxx_qt_mod_contents[6],
            quote! {
                cxx_qt::static_assertions::assert_eq_align!(cxx_qt::signalhandler::CxxQtSignalHandler<MyObjectCxxQtSignalClosureready>, usize);
            },
        );
        assert_tokens_eq(
            &cxx_qt_mod_contents[7],
            quote! {
                cxx_qt::static_assertions::assert_eq_size!(cxx_qt::signalhandler::CxxQtSignalHandler<MyObjectCxxQtSignalClosureready>, [usize; 2]);
            },
        );
    }

    #[test]
    fn test_generate_rust_signal() {
        let method: ForeignItemFn = parse_quote! {
            fn ready(self: Pin<&mut MyObject>);
        };
        let qsignal = ParsedSignal::mock(&method);

        let type_names = TypeNames::mock();

        let qobject_names = create_qobjectname();
        let generated =
            generate_rust_signals(&vec![&qsignal], &qobject_names, &type_names).unwrap();

        let qobject_name = type_names.lookup(&qsignal.qobject_ident).unwrap().clone();
        let other_generated = generate_rust_signal(
            &qsignal,
            &qobject_name,
            &type_names,
            Some(quote! { unsafe }),
        )
        .unwrap();

        assert_eq!(generated, other_generated);

        assert_tokens_eq(
            &generated.cxx_mod_contents[0],
            quote! {
                unsafe extern "C++" {
                    #[cxx_name = "ready"]
                    fn ready(self: Pin<&mut MyObject>);
                }
            },
        );

        common_asserts(
            &generated.cxx_mod_contents[1..].into(),
            &generated.cxx_qt_mod_contents,
        );
    }

    #[test]
    fn test_generate_rust_signal_parameters() {
        let method: ForeignItemFn = parse_quote! {
            #[cxx_name = "dataChanged"]
            fn data_changed(self: Pin<&mut MyObject>, trivial: i32, opaque: UniquePtr<QColor>);
        };
        let qsignal = ParsedSignal::mock(&method);
        let qobject_names = create_qobjectname();

        let mut type_names = TypeNames::mock();
        type_names.mock_insert("QColor", None, None, None);
        let generated =
            generate_rust_signals(&vec![&qsignal], &qobject_names, &type_names).unwrap();

        assert_eq!(generated.cxx_mod_contents.len(), 3);
        assert_eq!(generated.cxx_qt_mod_contents.len(), 8);

        assert_tokens_eq(
            &generated.cxx_mod_contents[0],
            quote! {
                unsafe extern "C++" {
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
                    #[cxx_name = "MyObject_dataChangedConnect"]
                    fn MyObject_connect_data_changed(self_value: Pin<&mut MyObject>, signal_handler: MyObjectCxxQtSignalHandlerdataChanged, conn_type: CxxQtConnectionType) -> CxxQtQMetaObjectConnection;
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
                    pub fn connect_data_changed<F: FnMut(core::pin::Pin<&mut qobject::MyObject>, i32, cxx::UniquePtr<QColor>) + 'static + Send>(self: core::pin::Pin<&mut qobject::MyObject>, closure: F, conn_type: cxx_qt::ConnectionType) -> cxx_qt::QMetaObjectConnectionGuard
                    {
                        cxx_qt::QMetaObjectConnectionGuard::from(qobject::MyObject_connect_data_changed(
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
                    pub fn on_data_changed<F: FnMut(core::pin::Pin<&mut qobject::MyObject>, i32, cxx::UniquePtr<QColor>) + 'static + Send>(self: core::pin::Pin<&mut qobject::MyObject>, closure: F) -> cxx_qt::QMetaObjectConnectionGuard
                    {
                        cxx_qt::QMetaObjectConnectionGuard::from(qobject::MyObject_connect_data_changed(
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
                    type FnType = dyn FnMut(core::pin::Pin<&mut qobject::MyObject>, i32, cxx::UniquePtr<QColor>) + Send;
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
        let method = parse_quote! {
            #[cxx_name = "unsafeSignal"]
            unsafe fn unsafe_signal(self: Pin<&mut MyObject>, param: *mut T);
        };
        let qsignal = ParsedSignal::mock(&method);
        let qobject_names = create_qobjectname();

        let mut type_names = TypeNames::mock();
        type_names.mock_insert("T", None, None, None);
        let generated =
            generate_rust_signals(&vec![&qsignal], &qobject_names, &type_names).unwrap();

        assert_eq!(generated.cxx_mod_contents.len(), 3);
        assert_eq!(generated.cxx_qt_mod_contents.len(), 8);

        assert_tokens_eq(
            &generated.cxx_mod_contents[0],
            quote! {
                unsafe extern "C++" {
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
                    #[cxx_name = "MyObject_unsafeSignalConnect"]
                    fn MyObject_connect_unsafe_signal(self_value: Pin<&mut MyObject>, signal_handler: MyObjectCxxQtSignalHandlerunsafeSignal, conn_type: CxxQtConnectionType) -> CxxQtQMetaObjectConnection;
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
                    pub fn connect_unsafe_signal<F: FnMut(core::pin::Pin<&mut qobject::MyObject>, *mut T) + 'static +Send>(self: core::pin::Pin<&mut qobject::MyObject>, closure: F, conn_type: cxx_qt::ConnectionType) -> cxx_qt::QMetaObjectConnectionGuard
                    {
                        cxx_qt::QMetaObjectConnectionGuard::from(qobject::MyObject_connect_unsafe_signal(
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
                    pub fn on_unsafe_signal<F: FnMut(core::pin::Pin<&mut qobject::MyObject>, *mut T) + 'static + Send>(self: core::pin::Pin<&mut qobject::MyObject>, closure: F) -> cxx_qt::QMetaObjectConnectionGuard
                    {
                        cxx_qt::QMetaObjectConnectionGuard::from(qobject::MyObject_connect_unsafe_signal(
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
                    type FnType = dyn FnMut(core::pin::Pin<&mut qobject::MyObject>, *mut T) + Send;
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
        let method = parse_quote! {
            #[inherit]
            #[cxx_name = "baseName"]
            fn existing_signal(self: Pin<&mut MyObject>, );
        };
        let qsignal = ParsedSignal {
            inherit: true,
            ..ParsedSignal::mock(&method)
        };
        let qobject_names = create_qobjectname();

        let generated =
            generate_rust_signals(&vec![&qsignal], &qobject_names, &TypeNames::mock()).unwrap();

        assert_eq!(generated.cxx_mod_contents.len(), 3);
        assert_eq!(generated.cxx_qt_mod_contents.len(), 8);

        assert_tokens_eq(
            &generated.cxx_mod_contents[0],
            quote! {
                unsafe extern "C++" {
                    #[cxx_name = "baseName"]
                    fn existing_signal(self: Pin<&mut MyObject>);
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
                    #[cxx_name = "MyObject_baseNameConnect"]
                    fn MyObject_connect_existing_signal(self_value: Pin<&mut MyObject>, signal_handler: MyObjectCxxQtSignalHandlerbaseName, conn_type: CxxQtConnectionType) -> CxxQtQMetaObjectConnection;
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
                    pub fn connect_existing_signal<F: FnMut(core::pin::Pin<&mut qobject::MyObject>, ) + 'static + Send>(self: core::pin::Pin<&mut qobject::MyObject>, closure: F, conn_type: cxx_qt::ConnectionType) -> cxx_qt::QMetaObjectConnectionGuard
                    {
                        cxx_qt::QMetaObjectConnectionGuard::from(qobject::MyObject_connect_existing_signal(
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
                    pub fn on_existing_signal<F: FnMut(core::pin::Pin<&mut qobject::MyObject>, ) + 'static + Send>(self: core::pin::Pin<&mut qobject::MyObject>, closure: F) -> cxx_qt::QMetaObjectConnectionGuard
                    {
                        cxx_qt::QMetaObjectConnectionGuard::from(qobject::MyObject_connect_existing_signal(
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
                    type FnType = dyn FnMut(core::pin::Pin<&mut qobject::MyObject>, ) + Send;
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
    fn test_generate_rust_signal_free_private() {
        let method: ForeignItemFn = parse_quote! {
            fn ready(self: Pin<&mut MyObject>);
        };
        let mock = ParsedSignal::mock(&method);
        let qsignal = ParsedSignal {
            method_fields: MethodFields {
                name: Name::new(format_ident!("ready")),
                ..mock.method_fields
            },
            private: true,
            ..mock
        };

        let type_names = TypeNames::mock();

        let qobject_name = type_names.lookup(&qsignal.qobject_ident).unwrap().clone();
        let generated = generate_rust_signal(
            &qsignal,
            &qobject_name,
            &type_names,
            Some(quote! { unsafe }),
        )
        .unwrap();

        common_asserts(&generated.cxx_mod_contents, &generated.cxx_qt_mod_contents);
    }
}
