use cxx_qt::CxxQtType;
// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Ben Ford <ben.ford@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use cxx_qt_lib::QString;

#[cxx_qt::bridge(namespace = "bridge_namespace")]
mod qobject {
    #[namespace = ""]
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    unsafe extern "RustQt" {
        #[qobject]
        #[qproperty(i32, number)]
        #[qproperty(QString, string)]
        type NamedObject = super::NamedObjectRust;

        #[qobject]
        #[qproperty(i32, number)]
        #[cxx_name = "RenamedObject"]
        #[namespace = "object_namespace"]
        type NamedObject2 = super::NamedObject2Rust;
    }

    // Note that we are only testing with C++ here so we don't need qinvokable
    unsafe extern "RustQt" {
        #[cxx_name = "sayHi"]
        fn say_hi(self: &NamedObject, string: &QString, number: i32);

        #[cxx_name = "getDouble"]
        fn get_double_num(self: &NamedObject2) -> i32;
    }
}

pub struct NamedObjectRust {
    number: i32,
    string: QString,
}

impl Default for NamedObjectRust {
    fn default() -> Self {
        Self {
            number: 0,
            string: QString::from(""),
        }
    }
}

pub struct NamedObject2Rust {
    number: i32,
}

impl Default for NamedObject2Rust {
    fn default() -> Self {
        Self { number: 25 }
    }
}

impl qobject::NamedObject {
    fn say_hi(&self, string: &QString, number: i32) {
        println!("Hi from Rust! String is {string} and number is {number}");
    }
}

impl qobject::NamedObject2 {
    fn get_double_num(&self) -> i32 {
        self.rust().number * 2
    }
}
