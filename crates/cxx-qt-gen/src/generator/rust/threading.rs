// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{
    generator::{
        naming::{
            namespace::{namespace_combine_ident, NamespaceName},
            qobject::QObjectNames,
        },
        rust::fragment::GeneratedRustFragment,
    },
    naming::TypeNames,
};
use quote::quote;
use syn::Result;

use super::fragment::RustFragmentPair;

pub fn generate(
    qobject_names: &QObjectNames,
    namespace_ident: &NamespaceName,
    type_names: &TypeNames,
) -> Result<GeneratedRustFragment> {
    let mut blocks = GeneratedRustFragment::default();

    let module_ident = qobject_names.name.require_module()?;

    let cpp_struct_ident = qobject_names.name.rust_unqualified();
    let cxx_qt_thread_ident = &qobject_names.cxx_qt_thread_class;
    let cxx_qt_thread_queued_fn_ident = &qobject_names.cxx_qt_thread_queued_fn_struct;

    let (thread_queue_name, thread_queue_attrs, thread_queue_qualified) = qobject_names
        .cxx_qt_ffi_method("cxxQtThreadQueue")
        .into_cxx_parts();
    let (thread_clone_name, thread_clone_attrs, thread_clone_qualified) = qobject_names
        .cxx_qt_ffi_method("cxxQtThreadClone")
        .into_cxx_parts();
    let (thread_drop_name, thread_drop_attrs, thread_drop_qualified) = qobject_names
        .cxx_qt_ffi_method("cxxQtThreadDrop")
        .into_cxx_parts();
    let (thread_fn_name, thread_fn_attrs, thread_fn_qualified) =
        qobject_names.cxx_qt_ffi_method("qtThread").into_cxx_parts();
    let (thread_is_destroyed_name, thread_is_destroyed_attrs, thread_is_destroyed_qualified) =
        qobject_names
            .cxx_qt_ffi_method("cxxQtThreadIsDestroyed")
            .into_cxx_parts();

    let cxx_qt_thread_namespace = &namespace_ident.namespace;
    let namespace_internals = &namespace_ident.internal;
    let cxx_qt_thread_ident_type_id_str =
        namespace_combine_ident(&namespace_ident.namespace, cxx_qt_thread_ident);
    let qualified_impl = type_names.rust_qualified(cpp_struct_ident)?;

    let fragment = RustFragmentPair {
        cxx_bridge: vec![
            quote! {
                unsafe extern "C++" {
                    // Specialised version of CxxQtThread, which can be moved into other threads.
                    //
                    // CXX doesn't support having generic types in the function yet
                    // so we cannot have CxxQtThread<T> in cxx-qt-lib and then use that here
                    // For now we use a type alias in C++ then use it like a normal type here
                    // <https://github.com/dtolnay/cxx/issues/683>
                    #[doc(hidden)]
                    #[namespace = #cxx_qt_thread_namespace]
                    type #cxx_qt_thread_ident = cxx_qt::CxxQtThread<#cpp_struct_ident>;
                    include!("cxx-qt/thread.h");

                    #[doc(hidden)]
                    #(#thread_fn_attrs)*
                    fn #thread_fn_name(qobject: &#cpp_struct_ident) -> #cxx_qt_thread_ident;

                    // SAFETY:
                    // - Send + 'static: argument closure can be transferred to QObject thread.
                    // - FnOnce: QMetaObject::invokeMethod() should call the function at most once.
                    #[doc(hidden)]
                    #(#thread_queue_attrs)*
                    fn #thread_queue_name(
                        cxx_qt_thread: &#cxx_qt_thread_ident,
                        func: fn(Pin<&mut #cpp_struct_ident>, Box<#cxx_qt_thread_queued_fn_ident>),
                        arg: Box<#cxx_qt_thread_queued_fn_ident>,
                    ) -> u8;

                    #[doc(hidden)]
                    #(#thread_clone_attrs)*
                    fn #thread_clone_name(cxx_qt_thread: &#cxx_qt_thread_ident) -> #cxx_qt_thread_ident;

                    #[doc(hidden)]
                    #(#thread_drop_attrs)*
                    fn #thread_drop_name(cxx_qt_thread: &mut #cxx_qt_thread_ident);

                    #[doc(hidden)]
                    #(#thread_is_destroyed_attrs)*
                    fn #thread_is_destroyed_name(cxx_qt_thread: &#cxx_qt_thread_ident) -> bool;
                }
            },
            quote! {
                extern "Rust" {
                    #[namespace = #namespace_internals]
                    type #cxx_qt_thread_queued_fn_ident;
                }
            },
        ],
        implementation: vec![
            quote! {
                impl cxx_qt::Threading for #qualified_impl {
                    type BoxedQueuedFn = #cxx_qt_thread_queued_fn_ident;
                    type ThreadingTypeId = cxx::type_id!(#cxx_qt_thread_ident_type_id_str);

                    fn qt_thread(&self) -> #module_ident::#cxx_qt_thread_ident
                    {
                        #thread_fn_qualified(self)
                    }

                    #[doc(hidden)]
                    fn is_destroyed(cxx_qt_thread: &#module_ident::#cxx_qt_thread_ident) -> bool
                    {
                        #thread_is_destroyed_qualified(cxx_qt_thread)
                    }

                    #[doc(hidden)]
                    fn queue<F>(cxx_qt_thread: &#module_ident::#cxx_qt_thread_ident, f: F) -> std::result::Result<(), cxx_qt::ThreadingQueueError>
                    where
                        F: FnOnce(core::pin::Pin<&mut #qualified_impl>),
                        F: Send + 'static,
                    {
                        // Wrap the given closure and pass in to C++ function as an opaque type
                        // to work around the cxx limitation.
                        // https://github.com/dtolnay/cxx/issues/114
                        #[allow(clippy::boxed_local)]
                        #[doc(hidden)]
                        fn func(
                            obj: core::pin::Pin<&mut #qualified_impl>,
                            arg: std::boxed::Box<#cxx_qt_thread_queued_fn_ident>,
                        ) {
                            (arg.inner)(obj)
                        }
                        let arg = #cxx_qt_thread_queued_fn_ident { inner: std::boxed::Box::new(f) };
                        match #thread_queue_qualified(cxx_qt_thread, func, std::boxed::Box::new(arg)) {
                            0 => Ok(()),
                            others => Err(others.into()),
                        }
                    }

                    #[doc(hidden)]
                    fn threading_clone(cxx_qt_thread: &#module_ident::#cxx_qt_thread_ident) -> #module_ident::#cxx_qt_thread_ident
                    {
                        #thread_clone_qualified(cxx_qt_thread)
                    }

                    #[doc(hidden)]
                    fn threading_drop(cxx_qt_thread: &mut #module_ident::#cxx_qt_thread_ident)
                    {
                        #thread_drop_qualified(cxx_qt_thread);
                    }
                }
            },
            quote! {
                #[doc(hidden)]
                pub struct #cxx_qt_thread_queued_fn_ident {
                    // An opaque Rust type is required to be Sized.
                    // https://github.com/dtolnay/cxx/issues/665
                    inner: std::boxed::Box<dyn FnOnce(core::pin::Pin<&mut #qualified_impl>) + Send>,
                }
            },
        ],
    };

    blocks
        .cxx_mod_contents
        .append(&mut fragment.cxx_bridge_as_items()?);
    blocks
        .cxx_qt_mod_contents
        .append(&mut fragment.implementation_as_items()?);

    Ok(blocks)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::naming::TypeNames;
    use crate::tests::assert_tokens_eq;

    use crate::parser::qobject::tests::create_parsed_qobject;

    #[test]
    fn test_generate_rust_threading() {
        let qobject = create_parsed_qobject();
        let qobject_names = QObjectNames::from_qobject(&qobject, &TypeNames::mock()).unwrap();
        let namespace_ident = NamespaceName::from(&qobject);

        let generated = generate(&qobject_names, &namespace_ident, &TypeNames::mock()).unwrap();

        assert_eq!(generated.cxx_mod_contents.len(), 2);
        assert_eq!(generated.cxx_qt_mod_contents.len(), 2);

        // CXX bridges

        assert_tokens_eq(
            &generated.cxx_mod_contents[0],
            quote! {
                unsafe extern "C++" {
                    #[doc(hidden)]
                    #[namespace = ""]
                    type MyObjectCxxQtThread = cxx_qt::CxxQtThread<MyObject>;
                    include!("cxx-qt/thread.h");

                    #[doc(hidden)]
                    #[cxx_name = "qtThread"]
                    #[namespace = "rust::cxxqt1"]
                    fn cxx_qt_ffi_MyObject_qtThread(qobject: &MyObject) -> MyObjectCxxQtThread;

                    // SAFETY:
                    // - Send + 'static: argument closure can be transferred to QObject thread.
                    // - FnOnce: QMetaObject::invokeMethod() should call the function at most once.
                    #[doc(hidden)]
                    #[cxx_name = "cxxQtThreadQueue"]
                    #[namespace = "rust::cxxqt1"]
                    fn cxx_qt_ffi_MyObject_cxxQtThreadQueue(
                        cxx_qt_thread: &MyObjectCxxQtThread,
                        func: fn(Pin<&mut MyObject>, Box<MyObjectCxxQtThreadQueuedFn>),
                        arg: Box<MyObjectCxxQtThreadQueuedFn>,
                    ) -> u8;

                    #[doc(hidden)]
                    #[cxx_name = "cxxQtThreadClone"]
                    #[namespace = "rust::cxxqt1"]
                    fn cxx_qt_ffi_MyObject_cxxQtThreadClone(cxx_qt_thread: &MyObjectCxxQtThread) -> MyObjectCxxQtThread;

                    #[doc(hidden)]
                    #[cxx_name = "cxxQtThreadDrop"]
                    #[namespace = "rust::cxxqt1"]
                    fn cxx_qt_ffi_MyObject_cxxQtThreadDrop(cxx_qt_thread: &mut MyObjectCxxQtThread);

                    #[doc(hidden)]
                    #[cxx_name = "cxxQtThreadIsDestroyed"]
                    #[namespace = "rust::cxxqt1"]
                    fn cxx_qt_ffi_MyObject_cxxQtThreadIsDestroyed(cxx_qt_thread: &MyObjectCxxQtThread) -> bool;
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_mod_contents[1],
            quote! {
                extern "Rust" {
                    #[namespace = "cxx_qt_MyObject"]
                    type MyObjectCxxQtThreadQueuedFn;
                }
            },
        );

        // CXX-Qt generated contents
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[0],
            quote! {
                impl cxx_qt::Threading for qobject::MyObject {
                    type BoxedQueuedFn = MyObjectCxxQtThreadQueuedFn;
                    type ThreadingTypeId = cxx::type_id!("MyObjectCxxQtThread");

                    fn qt_thread(&self) -> qobject::MyObjectCxxQtThread
                    {
                        qobject::cxx_qt_ffi_MyObject_qtThread(self)
                    }

                    #[doc(hidden)]
                    fn is_destroyed(cxx_qt_thread: &qobject::MyObjectCxxQtThread) -> bool {
                        qobject::cxx_qt_ffi_MyObject_cxxQtThreadIsDestroyed(cxx_qt_thread)
                    }

                    #[doc(hidden)]
                    fn queue<F>(cxx_qt_thread: &qobject::MyObjectCxxQtThread, f: F) -> std::result::Result<(), cxx_qt::ThreadingQueueError>
                    where
                        F: FnOnce(core::pin::Pin<&mut qobject::MyObject>),
                        F: Send + 'static,
                    {
                        // Wrap the given closure and pass in to C++ function as an opaque type
                        // to work around the cxx limitation.
                        // https://github.com/dtolnay/cxx/issues/114
                        #[allow(clippy::boxed_local)]
                        #[doc(hidden)]
                        fn func(
                            obj: core::pin::Pin<&mut qobject::MyObject>,
                            arg: std::boxed::Box<MyObjectCxxQtThreadQueuedFn>,
                        ) {
                            (arg.inner)(obj)
                        }
                        let arg = MyObjectCxxQtThreadQueuedFn { inner: std::boxed::Box::new(f) };
                        match qobject::cxx_qt_ffi_MyObject_cxxQtThreadQueue(cxx_qt_thread, func, std::boxed::Box::new(arg)) {
                            0 => Ok(()),
                            others => Err(others.into()),
                        }
                    }

                    #[doc(hidden)]
                    fn threading_clone(cxx_qt_thread: &qobject::MyObjectCxxQtThread) -> qobject::MyObjectCxxQtThread
                    {
                        qobject::cxx_qt_ffi_MyObject_cxxQtThreadClone(cxx_qt_thread)
                    }

                    #[doc(hidden)]
                    fn threading_drop(cxx_qt_thread: &mut qobject::MyObjectCxxQtThread)
                    {
                        qobject::cxx_qt_ffi_MyObject_cxxQtThreadDrop(cxx_qt_thread);
                    }
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[1],
            quote! {
                #[doc(hidden)]
                pub struct MyObjectCxxQtThreadQueuedFn {
                    // An opaque Rust type is required to be Sized.
                    // https://github.com/dtolnay/cxx/issues/665
                    inner: std::boxed::Box<dyn FnOnce(core::pin::Pin<&mut qobject::MyObject>) + Send>,
                }
            },
        );
    }
}
