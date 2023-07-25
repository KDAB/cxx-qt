// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx_qt::bridge(cxx_file_stem = "qapplication_cxx")]
mod ffi {
    unsafe extern "C++Qt" {
        include!(<QtWidgets/QApplication>);
        type QApplication;

        include!("cxx-qt-lib/qbytearray.h");
        type QByteArray = cxx_qt_lib::QByteArray;
        include!("cxx-qt-lib/qvector.h");
        type QVector_QByteArray = cxx_qt_lib::QVector<QByteArray>;
    }

    unsafe extern "C++Qt" {
        include!("qapplication.h");
        #[doc(hidden)]
        #[rust_name = "qapplication_new"]
        fn qapplicationNew(args: &QVector_QByteArray) -> UniquePtr<QApplication>;

        #[doc(hidden)]
        #[rust_name = "qapplication_exec"]
        fn qapplicationExec(app: Pin<&mut QApplication>) -> i32;
    }
}

impl ffi::QApplication {
    pub fn new() -> cxx::UniquePtr<Self> {
        let mut vector = cxx_qt_lib::QVector::<ffi::QByteArray>::default();

        // Construct an owned QVector of the args
        // as we need the args_os data to outlive this method
        // so we pass a QVector to C++ which is then stored
        for arg in std::env::args_os() {
            // Unix OsStrings can be directly converted to bytes.
            #[cfg(unix)]
            use std::os::unix::ffi::OsStrExt;

            // Windows OsStrings are WTF-8 encoded, so they need to be
            // converted to UTF-8 Strings before being converted to bytes.
            // https://simonsapin.github.io/wtf-8/
            #[cfg(windows)]
            let arg = arg.to_string_lossy();

            vector.append(ffi::QByteArray::from(arg.as_bytes()));
        }

        ffi::qapplication_new(&vector)
    }

    pub fn exec(self: core::pin::Pin<&mut Self>) -> i32 {
        ffi::qapplication_exec(self)
    }
}

pub use ffi::QApplication;
