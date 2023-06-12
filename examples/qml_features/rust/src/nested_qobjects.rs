// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! This example shows how a pointer from one Rust defined QObject to another Rust defined QObject can be used

/// A CXX-Qt bridge which shows how a pointer from one Rust defined QObject to another Rust defined QObject can be used
// ANCHOR: book_macro_code
#[cxx_qt::bridge(cxx_file_stem = "nested_qobjects")]
pub mod ffi {
    // ANCHOR: book_extern_block
    unsafe extern "C++" {
        #[cxx_name = "InnerObject"]
        /// The C++ part of the InnerObject so that it can be referred to
        type CxxInnerObject = super::qobject::InnerObject;
    }
    // ANCHOR_END: book_extern_block

    /// The inner QObject
    #[cxx_qt::qobject(qml_uri = "com.kdab.cxx_qt.demo", qml_version = "1.0")]
    #[derive(Default)]
    #[qproperty(i32, counter)]
    pub struct InnerObject {
        counter: i32,
    }

    extern "RustQt" {
        /// A signal showing how to refer to another QObject as an argument
        #[qsignal]
        unsafe fn called(self: Pin<&mut qobject::InnerObject>, inner: *mut CxxInnerObject);
    }

    /// The outer QObject which has a Q_PROPERTY pointing to the inner QObject
    #[cxx_qt::qobject(qml_uri = "com.kdab.cxx_qt.demo", qml_version = "1.0")]
    #[qproperty(*mut CxxInnerObject, inner)]
    pub struct OuterObject {
        inner: *mut CxxInnerObject,
    }

    impl Default for OuterObject {
        fn default() -> Self {
            Self {
                inner: std::ptr::null_mut(),
            }
        }
    }

    extern "RustQt" {
        /// A signal showing how to refer to another QObject as an argument
        #[qsignal]
        unsafe fn called(self: Pin<&mut qobject::OuterObject>, inner: *mut CxxInnerObject);
    }

    unsafe extern "RustQt" {
        /// Print the count of the given inner QObject
        //
        // This method needs to be unsafe otherwise clippy complains that the
        // public method might dereference the raw pointer.
        #[qinvokable]
        unsafe fn print_count(self: Pin<&mut qobject::OuterObject>, inner: *mut CxxInnerObject);

        /// Reset the counter of the inner QObject stored in the Q_PROPERTY
        #[qinvokable]
        fn reset(self: Pin<&mut qobject::OuterObject>);
    }

    impl cxx_qt::Constructor<()> for qobject::OuterObject {}
}

use core::pin::Pin;

// TODO: this will change to qobject::OuterObject once
// https://github.com/KDAB/cxx-qt/issues/559 is done
impl ffi::OuterObjectQt {
    /// Print the count of the given inner QObject
    //
    // This method needs to be unsafe otherwise clippy complains that the
    // public method might dereference the raw pointer.
    unsafe fn print_count(self: Pin<&mut Self>, inner: *mut ffi::CxxInnerObject) {
        if let Some(inner) = unsafe { inner.as_ref() } {
            println!("Inner object's counter property: {}", inner.counter());
        }

        unsafe {
            self.called(inner);
        }
    }

    /// Reset the counter of the inner QObject stored in the Q_PROPERTY
    fn reset(self: Pin<&mut Self>) {
        // We need to convert the *mut T to a Pin<&mut T> so that we can reach the methods
        if let Some(inner) = unsafe { self.inner().as_mut() } {
            let pinned_inner = unsafe { Pin::new_unchecked(inner) };
            // Now pinned inner can be used as normal
            pinned_inner.set_counter(10);
        }

        // Retrieve *mut T
        let inner = *self.inner();
        unsafe { self.called(inner) };
    }
}

impl cxx_qt::Constructor<()> for qobject::OuterObject {
    type InitializeArguments = ();
    type NewArguments = ();
    type BaseArguments = ();

    fn route_arguments(
        _: (),
    ) -> (
        Self::NewArguments,
        Self::BaseArguments,
        Self::InitializeArguments,
    ) {
        ((), (), ())
    }

    fn new(_: Self::NewArguments) -> <Self as cxx_qt::CxxQtType>::Rust {
        Default::default()
    }

    /// Initialise the QObject, creating a connection from one signal to another
    fn initialize(self: core::pin::Pin<&mut Self>, _: Self::InitializeArguments) {
        // Example of connecting a signal from one QObject to another QObject
        // this causes OuterObject::Called to trigger InnerObject::Called
        self.on_called(|qobject, obj| {
            // We need to convert the *mut T to a Pin<&mut T> so that we can reach the methods
            if let Some(inner) = unsafe { qobject.inner().as_mut() } {
                let pinned_inner = unsafe { Pin::new_unchecked(inner) };
                // Now pinned inner can be used as normal
                unsafe {
                    pinned_inner.called(obj);
                }
            }
        })
        .release();
    }
}

// ANCHOR_END: book_macro_code
