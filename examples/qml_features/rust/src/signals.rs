// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! This example shows how a Q_SIGNAL can be used

/// A CXX-Qt bridge which shows how a Q_SIGNAL can be used
// ANCHOR: book_macro_code
#[cxx_qt::bridge(cxx_file_stem = "rust_signals")]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        /// QString from cxx_qt_lib
        type QString = cxx_qt_lib::QString;
        include!("cxx-qt-lib/qurl.h");
        /// QUrl from cxx_qt_lib
        type QUrl = cxx_qt_lib::QUrl;
    }

    /// Q_SIGNALs for the QObject
    // ANCHOR: book_signals_enum
    #[cxx_qt::qsignals(RustSignals)]
    pub enum Connection<'a> {
        /// A Q_SIGNAL emitted when a connection occurs
        Connected {
            /// The url for the connection
            url: &'a QUrl,
        },
        /// A Q_SIGNAL emitted when a disconnect occurs
        Disconnected,
        /// A Q_SIGNAL emitted when an error occurs
        Error {
            /// The message of the error
            message: QString,
        },
    }
    // ANCHOR_END: book_signals_enum

    /// A QObject which has Q_SIGNALs
    // ANCHOR: book_signals_struct
    #[cxx_qt::qobject(qml_uri = "com.kdab.cxx_qt.demo", qml_version = "1.0")]
    #[derive(Default)]
    pub struct RustSignals {
        connections: Option<[cxx::UniquePtr<cxx_qt_lib::QMetaObjectConnection>; 3]>,
    }
    // ANCHOR: book_signals_struct

    // ANCHOR: book_rust_obj_impl
    impl qobject::RustSignals {
        /// Connect to the given url
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

        /// Disconnect
        #[qinvokable]
        pub fn disconnect(mut self: Pin<&mut Self>) {
            // Emit a signal to QML stating that we have disconnected
            self.as_mut().emit(Connection::Disconnected);
        }

        #[qinvokable]
        pub fn toggle_logging(mut self: Pin<&mut Self>) {
            if let Some(connections) = self.as_mut().connections_mut().take() {
                for conn in connections {
                    conn.disconnect();
                }
            } else {
                let connections = [
                    self.as_mut().on_connected(|_, url| {
                        println!("Connected: {}", url);
                    }),
                    self.as_mut().on_error(|_, message| {
                        println!("Error: {}", message);
                    }),
                    self.as_mut().on_disconnected(|_| {
                        println!("Disconnected");
                    }),
                ];
                self.as_mut().set_connections(Some(connections));
            }
        }
    }
    // ANCHOR_END: book_rust_obj_impl
}
// ANCHOR_END: book_macro_code
