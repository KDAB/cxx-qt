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
    pub struct InnerObject {
        #[qproperty]
        counter: i32,
    }

    /// Signals for the inner QObject
    #[cxx_qt::qsignals(InnerObject)]
    pub enum InnerSignals {
        /// A signal showing how to refer to another QObject as an argument
        Called {
            /// Inner QObject being referred to
            inner: *mut CxxInnerObject,
        },
    }

    /// The outer QObject which has a Q_PROPERTY pointing to the inner QObject
    #[cxx_qt::qobject(qml_uri = "com.kdab.cxx_qt.demo", qml_version = "1.0")]
    pub struct OuterObject {
        #[qproperty]
        inner: *mut CxxInnerObject,
    }

    impl Default for OuterObject {
        fn default() -> Self {
            Self {
                inner: std::ptr::null_mut(),
            }
        }
    }

    /// Signals for the outer QObject
    #[cxx_qt::qsignals(OuterObject)]
    pub enum OuterSignals {
        /// A signal showing how to refer to another QObject as an argument
        Called {
            /// Inner QObject being referred to
            inner: *mut CxxInnerObject,
        },
    }

    impl qobject::OuterObject {
        /// Initialise the QObject, creating a connection from one signal to another
        #[qinvokable]
        pub fn initialise(self: Pin<&mut Self>) {
            // Example of connecting a signal from one QObject to another QObject
            // this causes OuterObject::Called to trigger InnerObject::Called
            self.on_called(
                |qobject, obj| {
                    // We need to convert the *mut T to a Pin<&mut T> so that we can reach the methods
                    if let Some(inner) = unsafe { qobject.inner().as_mut() } {
                        // TODO: Use `pin!` one it's stable so that this unsafe block can be removed
                        // https://doc.rust-lang.org/std/pin/macro.pin.html
                        let pinned_inner = unsafe { Pin::new_unchecked(inner) };
                        // Now pinned inner can be used as normal
                        pinned_inner.emit(InnerSignals::Called { inner: obj });
                    }
                },
                cxx_qt_lib::ConnectionType::AutoConnection,
            );
        }

        /// Print the count of the given inner QObject
        //
        // This method needs to be unsafe otherwise clippy complains that the
        // public method might dereference the raw pointer.
        #[qinvokable]
        pub unsafe fn print_count(self: Pin<&mut Self>, inner: *mut CxxInnerObject) {
            if let Some(inner) = unsafe { inner.as_ref() } {
                println!("Inner object's counter property: {}", inner.counter());
            }

            self.emit(OuterSignals::Called { inner });
        }

        /// Reset the counter of the inner QObject stored in the Q_PROPERTY
        #[qinvokable]
        pub fn reset(self: Pin<&mut Self>) {
            // We need to convert the *mut T to a Pin<&mut T> so that we can reach the methods
            if let Some(inner) = unsafe { self.inner().as_mut() } {
                // TODO: Use `pin!` one it's stable so that this unsafe block can be removed
                // https://doc.rust-lang.org/std/pin/macro.pin.html
                let pinned_inner = unsafe { Pin::new_unchecked(inner) };
                // Now pinned inner can be used as normal
                pinned_inner.set_counter(10);
            }

            // Retrieve *mut T
            let inner = *self.inner();
            self.emit(OuterSignals::Called { inner });
        }
    }
}
// ANCHOR_END: book_macro_code
