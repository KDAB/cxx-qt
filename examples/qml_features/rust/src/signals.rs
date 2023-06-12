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

    // ANCHOR: book_signals_block
    unsafe extern "RustQt" {
        /// A Q_SIGNAL emitted when a connection occurs
        #[qsignal]
        fn connected(self: Pin<&mut qobject::RustSignals>, url: &QUrl);

        /// A Q_SIGNAL emitted when a disconnect occurs
        #[qsignal]
        fn disconnected(self: Pin<&mut qobject::RustSignals>);

        /// A Q_SIGNAL emitted when an error occurs
        #[qsignal]
        fn error(self: Pin<&mut qobject::RustSignals>, message: QString);
    }
    // ANCHOR_END: book_signals_block

    /// A QObject which has Q_SIGNALs
    // ANCHOR: book_signals_struct
    #[cxx_qt::qobject(qml_uri = "com.kdab.cxx_qt.demo", qml_version = "1.0")]
    #[derive(Default)]
    #[qproperty(bool, logging_enabled)]
    pub struct RustSignals {
        pub(crate) connections: Option<[cxx_qt_lib::QMetaObjectConnection; 3]>,

        logging_enabled: bool,
    }

    // ANCHOR: book_rust_obj_impl
    unsafe extern "RustQt" {
        /// Connect to the given url
        #[qinvokable]
        fn connect(self: Pin<&mut qobject::RustSignals>, url: &QUrl);

        /// Disconnect
        #[qinvokable]
        fn disconnect(self: Pin<&mut qobject::RustSignals>);
    }
    // ANCHOR_END: book_rust_obj_impl

    impl cxx_qt::Constructor<()> for qobject::RustSignals {}
}

use core::pin::Pin;
use cxx_qt::CxxQtType;
use cxx_qt_lib::{ConnectionType, QString, QUrl};

// TODO: this will change to qobject::RustSignals once
// https://github.com/KDAB/cxx-qt/issues/559 is done
impl ffi::RustSignalsQt {
    /// Connect to the given url
    fn connect(self: Pin<&mut Self>, url: &QUrl) {
        // Check that the url starts with kdab
        if url.to_string().starts_with("https://kdab.com") {
            // Emit a signal to QML stating that we have connected
            self.connected(url);
        } else {
            // Emit a signal to QML stating that the url was incorrect
            self.error(QString::from("URL does not start with https://kdab.com"));
        }
    }

    /// Disconnect
    fn disconnect(self: Pin<&mut Self>) {
        // Emit a signal to QML stating that we have disconnected
        self.disconnected();
    }
}

impl cxx_qt::Constructor<()> for qobject::RustSignals {
    type BaseArguments = ();
    type NewArguments = ();
    type InitializeArguments = ();

    fn route_arguments(
        _: (),
    ) -> (
        Self::NewArguments,
        Self::BaseArguments,
        Self::InitializeArguments,
    ) {
        ((), (), ())
    }

    fn new(_: Self::NewArguments) -> <Self as CxxQtType>::Rust {
        Default::default()
    }

    /// Initialise the QObject, creating a connection reacting to the logging enabled property
    fn initialize(self: core::pin::Pin<&mut Self>, _: Self::InitializeArguments) {
        self.on_logging_enabled_changed(|mut qobject| {
            // Determine if logging is enabled
            if *qobject.as_ref().logging_enabled() {
                // If no connections have been made, then create them
                if qobject.as_ref().rust().connections.is_none() {
                    // ANCHOR: book_signals_connect
                    let connections = [
                        qobject.as_mut().on_connected(|_, url| {
                            println!("Connected: {}", url);
                        }),
                        qobject.as_mut().on_disconnected(|_| {
                            println!("Disconnected");
                        }),
                        // Demonstration of connecting with a different connection type
                        qobject.as_mut().connect_error(
                            |_, message| {
                                println!("Error: {}", message);
                            },
                            ConnectionType::QueuedConnection,
                        ),
                    ];
                    qobject.as_mut().rust_mut().connections = Some(connections);
                    // ANCHOR_END: book_signals_connect
                }
            } else {
                // Disabling logging so disconnect
                // ANCHOR: book_signals_disconnect
                // By making connections None, we trigger a drop on the connections
                // this then causes disconnections
                qobject.as_mut().rust_mut().connections = None;
                // ANCHOR_END: book_signals_disconnect
            }
        })
        .release();
    }
}
// ANCHOR_END: book_macro_code
