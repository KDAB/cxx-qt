// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! This example shows how a pointer from one Rust defined QObject to another Rust defined QObject can be used

/// A CXX-Qt bridge which shows how a pointer from one Rust defined QObject to another Rust defined QObject can be used
// ANCHOR: book_macro_code
#[cxx_qt::bridge(cxx_file_stem = "nested_qobjects")]
pub mod qobject {
    // ANCHOR: book_extern_block
    extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(i32, counter)]
        type InnerObject = super::InnerObjectRust;
    }
    // ANCHOR_END: book_extern_block

    extern "RustQt" {
        /// A signal showing how to refer to another QObject as an argument
        #[qsignal]
        unsafe fn called(self: Pin<&mut InnerObject>, inner: *mut InnerObject);
    }

    extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(*mut InnerObject, inner)]
        type OuterObject = super::OuterObjectRust;

        /// A signal showing how to refer to another QObject as an argument
        #[qsignal]
        unsafe fn called(self: Pin<&mut OuterObject>, inner: *mut InnerObject);
    }

    unsafe extern "RustQt" {
        /// Print the count of the given inner QObject
        #[qinvokable]
        #[cxx_name = "printCount"]
        unsafe fn print_count(self: Pin<&mut OuterObject>, inner: *mut InnerObject);

        /// Reset the counter of the inner QObject stored in the Q_PROPERTY
        #[qinvokable]
        fn reset(self: Pin<&mut OuterObject>);
    }

    impl cxx_qt::Constructor<()> for OuterObject {}
}

use core::pin::Pin;

/// The inner QObject
#[derive(Default)]
pub struct InnerObjectRust {
    counter: i32,
}

/// The outer QObject which has a Q_PROPERTY pointing to the inner QObject
pub struct OuterObjectRust {
    inner: *mut qobject::InnerObject,
}

impl Default for OuterObjectRust {
    fn default() -> Self {
        Self {
            inner: std::ptr::null_mut(),
        }
    }
}

impl qobject::OuterObject {
    /// Print the count of the given inner QObject
    ///
    /// # Safety
    ///
    /// As we deref a pointer in a public method this needs to be marked as unsafe
    pub unsafe fn print_count(self: Pin<&mut Self>, inner: *mut qobject::InnerObject) {
        if let Some(inner) = inner.as_ref() {
            println!("Inner object's counter property: {}", inner.counter());
        }

        self.called(inner);
    }

    /// Reset the counter of the inner QObject stored in the Q_PROPERTY
    pub fn reset(self: Pin<&mut Self>) {
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

impl cxx_qt::Initialize for qobject::OuterObject {
    /// Initialize the QObject, creating a connection from one signal to another
    fn initialize(self: core::pin::Pin<&mut Self>) {
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
