// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

// ANCHOR: book_macro_code
#[cxx_qt::bridge(cxx_file_stem = "rust_signals")]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
        include!("cxx-qt-lib/qurl.h");
        type QUrl = cxx_qt_lib::QUrl;
    }

    // ANCHOR: book_signals_enum
    #[cxx_qt::qsignals(RustSignals)]
    pub enum Connection<'a> {
        Connected { url: &'a QUrl },
        Disconnected,
        Error { message: QString },
    }
    // ANCHOR_END: book_signals_enum

    #[cxx_qt::qobject]
    #[derive(Default)]
    pub struct RustSignals;

    // ANCHOR: book_rust_obj_impl
    impl qobject::RustSignals {
        #[qinvokable]
        pub fn connect(mut self: Pin<&mut Self>, url: &QUrl) {
            // Check that the url starts with kdab
            if url.to_string().starts_with("https://kdab.com") {
                // Emit a signal to QML stating that we have connected
                self.as_mut().emit(Connection::Connected { url });
            } else {
                // Emit a signal to QML stating that the url was incorrect
                self.emit(Connection::Error {
                    message: QString::from("URL does not start with https://kdab.com"),
                });
            }
        }

        #[qinvokable]
        pub fn disconnect(mut self: Pin<&mut Self>) {
            // Emit a signal to QML stating that we have disconnected
            self.as_mut().emit(Connection::Disconnected);
        }
    }
    // ANCHOR_END: book_rust_obj_impl
}
// ANCHOR_END: book_macro_code
