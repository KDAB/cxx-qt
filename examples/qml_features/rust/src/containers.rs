// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx_qt::bridge(cxx_file_stem = "rust_containers")]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qset.h");
        type QSet_i32 = cxx_qt_lib::QSet<i32>;
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    #[cxx_qt::qobject]
    #[derive(Default)]
    pub struct RustContainers {
        #[qproperty]
        string_set: QString,

        set: QSet_i32,
    }

    impl qobject::RustContainers {
        #[qinvokable]
        pub fn insert(mut self: Pin<&mut Self>, value: i32) {
            unsafe {
                self.as_mut().rust_mut().set.insert(value);
            }

            let set_items = self
                .as_ref()
                .set()
                .iter()
                .map(|value| value.to_string())
                .collect::<Vec<String>>()
                .join(", ");

            self.set_string_set(QString::from(&set_items));
        }
    }
}
