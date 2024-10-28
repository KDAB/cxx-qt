// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Ben Ford <ben.ford@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! This example shows how cxx_name, rust_name and namespace can be used

/// A CXX-Qt bridge containing renamed and namespaced types
#[cxx_qt::bridge]
pub mod qobject {
    unsafe extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(i32, num, cxx_name = "numberProp")]
        #[cxx_name = "RenamedObject"]
        #[namespace = "my_namespace"]
        type NamedObject = super::NamedObjectRust;
    }

    unsafe extern "RustQt" {
        #[qinvokable]
        #[cxx_name = "increment"]
        #[rust_name = "plus_one"]
        fn increment_number(self: Pin<&mut NamedObject>);
    }

    #[auto_cxx_name]
    unsafe extern "RustQt" {
        #[qinvokable]
        fn get_num(self: &NamedObject) -> i32;
    }
}

use std::pin::Pin;

/// Simple counter struct which has been renamed in this example
#[derive(Default)]
pub struct NamedObjectRust {
    pub(crate) num: i32,
}

impl qobject::NamedObject {
    /// Increment method for the counter
    pub fn plus_one(self: Pin<&mut Self>) {
        let previous = *self.num();
        self.set_num(previous + 1);
    }

    /// Method to return a greeting
    pub fn get_num(&self) -> i32 {
        42
    }
}
