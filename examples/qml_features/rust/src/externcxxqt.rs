// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! This example shows how an external QObject with signals can be used

/// A CXX-Qt bridge which shows how an external QObject with signals can be used
#[cxx_qt::bridge]
pub mod ffi {
    unsafe extern "C++Qt" {
        include!("external_qobject.h");
        /// ExternalQObject C++ class
        #[qobject]
        type ExternalQObject;

        // Since functions are just passed through the inlining isn't yet supported
        /// Trigger emitting the signal "amount" times
        fn trigger(self: Pin<&mut ExternalQObject>, amount: u32);

        /// Signal that is emitted when trigger is fired
        #[qsignal]
        fn triggered(self: Pin<&mut Self>);

        /// const signal that is emitted when trigger is fired
        #[qsignal]
        #[rust_name = "triggered_const_signal"]
        fn triggeredConstSignal(&self);

        /// Private signal that is emitted when trigger is fired
        #[qsignal]
        #[rust_name = "triggered_private_signal"]
        pub(self) fn triggeredPrivateSignal(self: Pin<&mut Self>);
    }

    extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(u32, count)]
        #[qproperty(u32, private_count, cxx_name = "privateCount")]
        type ExternalCxxQtHelper = super::ExternalCxxQtHelperRust;

        #[qinvokable]
        #[cxx_name = "connectToExternal"]
        unsafe fn connect_to_external(self: Pin<&mut Self>, external: *mut ExternalQObject);

        #[qinvokable]
        #[cxx_name = "triggerOnExternal"]
        unsafe fn trigger_on_external(
            self: Pin<&mut Self>,
            external: *mut ExternalQObject,
            amount: u32,
        );
    }

    impl cxx_qt::Threading for ExternalCxxQtHelper {}
}

use core::pin::Pin;
use cxx_qt::Threading;

/// Test struct
#[derive(Default)]
pub struct ExternalCxxQtHelperRust {
    count: u32,
    private_count: u32,
}

impl ffi::ExternalCxxQtHelper {
    unsafe fn connect_to_external(self: Pin<&mut Self>, external: *mut ffi::ExternalQObject) {
        if let Some(external) = external.as_mut() {
            let qt_thread = self.qt_thread();
            let mut pinned_external = Pin::new_unchecked(external);
            pinned_external
                .as_mut()
                .on_triggered(move |_| {
                    qt_thread
                        .queue(|mut qobject| {
                            let new_count = qobject.as_ref().count() + 1;
                            qobject.as_mut().set_count(new_count);
                        })
                        .unwrap();
                })
                .release();

            let qt_thread = self.qt_thread();
            pinned_external
                .as_mut()
                .on_triggered_private_signal(move |_| {
                    qt_thread
                        .queue(|mut qobject| {
                            let new_private_count = qobject.as_ref().private_count() + 1;
                            qobject.as_mut().set_private_count(new_private_count);
                        })
                        .unwrap();
                })
                .release();

            let qt_thread = self.qt_thread();
            pinned_external
                .as_mut()
                .on_triggered_const_signal(move |_| {
                    qt_thread
                        .queue(|qobject| {
                            let const_count = qobject.count();
                            println!("count from const signal: {}", const_count);
                        })
                        .unwrap();
                })
                .release();
        }
    }

    unsafe fn trigger_on_external(
        self: Pin<&mut Self>,
        external: *mut ffi::ExternalQObject,
        amount: u32,
    ) {
        if let Some(external) = external.as_mut() {
            let pinned_external = Pin::new_unchecked(external);
            pinned_external.trigger(amount);
        }
    }
}
