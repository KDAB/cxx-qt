// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx_qt::bridge]
pub mod qobject {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(QString, string)]
        type Sub2Object = super::Sub2ObjectRust;

        #[qinvokable]
        fn increment(self: Pin<&mut Sub2Object>);
    }
}

use core::pin::Pin;
use cxx_qt::CxxQtType;
use cxx_qt_lib::QString;

#[derive(Default)]
pub struct Sub2ObjectRust {
    string: QString,

    pub counter: u32,
}

impl qobject::Sub2Object {
    pub fn increment(mut self: Pin<&mut Self>) {
        self.as_mut().rust_mut().counter = crate::increment(self.rust().counter);

        let new_string = QString::from(&self.rust().counter.to_string());
        self.as_mut().set_string(new_string);
    }
}
