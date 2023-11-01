// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Be Wilson <be.wilson@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

// ANCHOR: book_cxx_qt_module
// ANCHOR: book_bridge_macro

/// The bridge definition for our QObject
#[cxx_qt::bridge]
pub mod qobject {
    // ANCHOR_END: book_bridge_macro

    // ANCHOR: book_qstring_import
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        /// An alias to the QString type
        type QString = cxx_qt_lib::QString;
    }
    // ANCHOR_END: book_qstring_import

    // ANCHOR: book_rustobj_struct_signature
    unsafe extern "RustQt" {
        // The QObject definition
        // We tell CXX-Qt that we want a QObject class with the name MyObject
        // based on the Rust struct MyObjectRust.
        #[qobject]
        #[qml_element]
        #[qproperty(i32, number)]
        #[qproperty(QString, string)]
        type MyObject = super::MyObjectRust;
    }
    // ANCHOR_END: book_rustobj_struct_signature

    // ANCHOR: book_rustobj_invokable_signature
    unsafe extern "RustQt" {
        // Declare the invokable methods we want to expose on the QObject
        #[qinvokable]
        fn increment_number(self: Pin<&mut MyObject>);

        #[qinvokable]
        fn say_hi(self: &MyObject, string: &QString, number: i32);
    }
    // ANCHOR_END: book_rustobj_invokable_signature
}

// ANCHOR: book_use
use core::pin::Pin;
use cxx_qt_lib::QString;
// ANCHOR_END: book_use

/// The Rust struct for the QObject
// ANCHOR: book_rustobj_struct
#[derive(Default)]
pub struct MyObjectRust {
    number: i32,
    string: QString,
}
// ANCHOR_END: book_rustobj_struct

// ANCHOR: book_rustobj_invokable_impl
impl qobject::MyObject {
    /// Increment the number Q_PROPERTY
    pub fn increment_number(self: Pin<&mut Self>) {
        let previous = *self.number();
        self.set_number(previous + 1);
    }

    /// Print a log message with the given string and number
    pub fn say_hi(&self, string: &QString, number: i32) {
        println!("Hi from Rust! String is '{string}' and number is {number}");
    }
}
// ANCHOR_END: book_rustobj_invokable_impl

// ANCHOR_END: book_cxx_qt_module
