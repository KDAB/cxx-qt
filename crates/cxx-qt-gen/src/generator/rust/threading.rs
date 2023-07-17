// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::generator::{
    naming::{
        namespace::{namespace_combine_ident, NamespaceName},
        qobject::QObjectName,
    },
    rust::qobject::GeneratedRustQObjectBlocks,
};
use quote::quote;
use syn::Result;

use super::fragment::RustFragmentPair;

pub fn generate(
    qobject_ident: &QObjectName,
    namespace_ident: &NamespaceName,
) -> Result<GeneratedRustQObjectBlocks> {
    let mut blocks = GeneratedRustQObjectBlocks::default();

    let cpp_struct_ident = &qobject_ident.cpp_class.rust;
    let cxx_qt_thread_ident = &qobject_ident.cxx_qt_thread_class;
    let cxx_qt_thread_queued_fn_ident = &qobject_ident.cxx_qt_thread_queued_fn_struct;
    let cxx_qt_thread_queue_fn = qobject_ident.cxx_qt_thread_method("queue_boxed_fn");
    let cxx_qt_thread_clone = qobject_ident.cxx_qt_thread_method("threading_clone");
    let cxx_qt_thread_drop = qobject_ident.cxx_qt_thread_method("threading_drop");
    let namespace_internals = &namespace_ident.internal;
    let cxx_qt_thread_ident_type_id_str =
        namespace_combine_ident(&namespace_ident.namespace, cxx_qt_thread_ident);

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
                    type #cxx_qt_thread_ident = cxx_qt::CxxQtThread<#cpp_struct_ident>;
                    include!("cxx-qt-common/cxxqt_thread.h");

                    #[doc(hidden)]
                    #[cxx_name = "qtThread"]
                    fn cxx_qt_ffi_qt_thread(self: &#cpp_struct_ident) -> #cxx_qt_thread_ident;

                    // SAFETY:
                    // - Send + 'static: argument closure can be transferred to QObject thread.
                    // - FnOnce: QMetaObject::invokeMethod() should call the function at most once.
                    #[doc(hidden)]
                    #[namespace = "rust::cxxqtlib1"]
                    #[cxx_name = "cxxQtThreadQueue"]
                    fn #cxx_qt_thread_queue_fn(
                        cxx_qt_thread: &#cxx_qt_thread_ident,
                        func: fn(Pin<&mut #cpp_struct_ident>, Box<#cxx_qt_thread_queued_fn_ident>),
                        arg: Box<#cxx_qt_thread_queued_fn_ident>,
                    ) -> Result<()>;

                    #[doc(hidden)]
                    #[namespace = "rust::cxxqtlib1"]
                    #[cxx_name = "cxxQtThreadClone"]
                    fn #cxx_qt_thread_clone(cxx_qt_thread: &#cxx_qt_thread_ident) -> #cxx_qt_thread_ident;

                    #[doc(hidden)]
                    #[namespace = "rust::cxxqtlib1"]
                    #[cxx_name = "cxxQtThreadDrop"]
                    fn #cxx_qt_thread_drop(cxx_qt_thread: &mut #cxx_qt_thread_ident);
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
                impl cxx_qt::Threading for #cpp_struct_ident {
                    type BoxedQueuedFn = #cxx_qt_thread_queued_fn_ident;
                    type ThreadingTypeId = cxx::type_id!(#cxx_qt_thread_ident_type_id_str);

                    fn qt_thread(&self) -> #cxx_qt_thread_ident
                    {
                        self.cxx_qt_ffi_qt_thread()
                    }

                    #[doc(hidden)]
                    fn queue<F>(cxx_qt_thread: &#cxx_qt_thread_ident, f: F) -> std::result::Result<(), cxx::Exception>
                    where
                        F: FnOnce(std::pin::Pin<&mut #cpp_struct_ident>),
                        F: Send + 'static,
                    {
                        // Wrap the given closure and pass in to C++ function as an opaque type
                        // to work around the cxx limitation.
                        // https://github.com/dtolnay/cxx/issues/114
                        #[allow(clippy::boxed_local)]
                        #[doc(hidden)]
                        fn func(
                            obj: std::pin::Pin<&mut #cpp_struct_ident>,
                            arg: std::boxed::Box<#cxx_qt_thread_queued_fn_ident>,
                        ) {
                            (arg.inner)(obj)
                        }
                        let arg = #cxx_qt_thread_queued_fn_ident { inner: std::boxed::Box::new(f) };
                        #cxx_qt_thread_queue_fn(cxx_qt_thread, func, std::boxed::Box::new(arg))
                    }

                    #[doc(hidden)]
                    fn threading_clone(cxx_qt_thread: &#cxx_qt_thread_ident) -> #cxx_qt_thread_ident
                    {
                        #cxx_qt_thread_clone(cxx_qt_thread)
                    }

                    #[doc(hidden)]
                    fn threading_drop(cxx_qt_thread: &mut #cxx_qt_thread_ident)
                    {
                        #cxx_qt_thread_drop(cxx_qt_thread);
                    }
                }
            },
            quote! {
                #[doc(hidden)]
                pub struct #cxx_qt_thread_queued_fn_ident {
                    // An opaque Rust type is required to be Sized.
                    // https://github.com/dtolnay/cxx/issues/665
                    inner: std::boxed::Box<dyn FnOnce(std::pin::Pin<&mut #cpp_struct_ident>) + Send>,
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

    use crate::tests::assert_tokens_eq;

    use crate::parser::qobject::tests::create_parsed_qobject;

    #[test]
    fn test_generate_rust_threading() {
        let qobject = create_parsed_qobject();
        let qobject_idents = QObjectName::from(&qobject);
        let namespace_ident = NamespaceName::from(&qobject);

        let generated = generate(&qobject_idents, &namespace_ident).unwrap();

        assert_eq!(generated.cxx_mod_contents.len(), 2);
        assert_eq!(generated.cxx_qt_mod_contents.len(), 2);

        // CXX bridges

        assert_tokens_eq(
            &generated.cxx_mod_contents[0],
            quote! {
                unsafe extern "C++" {
                    #[doc(hidden)]
                    type MyObjectCxxQtThread = cxx_qt::CxxQtThread<MyObject>;
                    include!("cxx-qt-common/cxxqt_thread.h");

                    #[doc(hidden)]
                    #[cxx_name = "qtThread"]
                    fn cxx_qt_ffi_qt_thread(self: &MyObject) -> MyObjectCxxQtThread;

                    // SAFETY:
                    // - Send + 'static: argument closure can be transferred to QObject thread.
                    // - FnOnce: QMetaObject::invokeMethod() should call the function at most once.
                    #[doc(hidden)]
                    #[namespace = "rust::cxxqtlib1"]
                    #[cxx_name = "cxxQtThreadQueue"]
                    fn cxx_qt_ffi_my_object_queue_boxed_fn(
                        cxx_qt_thread: &MyObjectCxxQtThread,
                        func: fn(Pin<&mut MyObject>, Box<MyObjectCxxQtThreadQueuedFn>),
                        arg: Box<MyObjectCxxQtThreadQueuedFn>,
                    ) -> Result<()>;

                    #[doc(hidden)]
                    #[namespace = "rust::cxxqtlib1"]
                    #[cxx_name = "cxxQtThreadClone"]
                    fn cxx_qt_ffi_my_object_threading_clone(cxx_qt_thread: &MyObjectCxxQtThread) -> MyObjectCxxQtThread;

                    #[doc(hidden)]
                    #[namespace = "rust::cxxqtlib1"]
                    #[cxx_name = "cxxQtThreadDrop"]
                    fn cxx_qt_ffi_my_object_threading_drop(cxx_qt_thread: &mut MyObjectCxxQtThread);
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_mod_contents[1],
            quote! {
                extern "Rust" {
                    #[namespace = "cxx_qt_my_object"]
                    type MyObjectCxxQtThreadQueuedFn;
                }
            },
        );

        // CXX-Qt generated contents
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[0],
            quote! {
                impl cxx_qt::Threading for MyObject {
                    type BoxedQueuedFn = MyObjectCxxQtThreadQueuedFn;
                    type ThreadingTypeId = cxx::type_id!("MyObjectCxxQtThread");

                    fn qt_thread(&self) -> MyObjectCxxQtThread
                    {
                        self.cxx_qt_ffi_qt_thread()
                    }

                    #[doc(hidden)]
                    fn queue<F>(cxx_qt_thread: &MyObjectCxxQtThread, f: F) -> std::result::Result<(), cxx::Exception>
                    where
                        F: FnOnce(std::pin::Pin<&mut MyObject>),
                        F: Send + 'static,
                    {
                        // Wrap the given closure and pass in to C++ function as an opaque type
                        // to work around the cxx limitation.
                        // https://github.com/dtolnay/cxx/issues/114
                        #[allow(clippy::boxed_local)]
                        #[doc(hidden)]
                        fn func(
                            obj: std::pin::Pin<&mut MyObject>,
                            arg: std::boxed::Box<MyObjectCxxQtThreadQueuedFn>,
                        ) {
                            (arg.inner)(obj)
                        }
                        let arg = MyObjectCxxQtThreadQueuedFn { inner: std::boxed::Box::new(f) };
                        cxx_qt_ffi_my_object_queue_boxed_fn(cxx_qt_thread, func, std::boxed::Box::new(arg))
                    }

                    #[doc(hidden)]
                    fn threading_clone(cxx_qt_thread: &MyObjectCxxQtThread) -> MyObjectCxxQtThread
                    {
                        cxx_qt_ffi_my_object_threading_clone(cxx_qt_thread)
                    }

                    #[doc(hidden)]
                    fn threading_drop(cxx_qt_thread: &mut MyObjectCxxQtThread)
                    {
                        cxx_qt_ffi_my_object_threading_drop(cxx_qt_thread);
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
                    inner: std::boxed::Box<dyn FnOnce(std::pin::Pin<&mut MyObject>) + Send>,
                }
            },
        );
    }
}
