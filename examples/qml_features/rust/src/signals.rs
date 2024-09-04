// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! This example shows how a Q_SIGNAL can be used

/// A CXX-Qt bridge which shows how a Q_SIGNAL can be used
// ANCHOR: book_macro_code
#[cxx_qt::bridge]
pub mod qobject {
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
        fn connected(self: Pin<&mut RustSignals>, url: &QUrl);

        /// A Q_SIGNAL emitted when a disconnect occurs
        #[qsignal]
        fn disconnected(self: Pin<&mut RustSignals>);

        /// A Q_SIGNAL emitted when an error occurs
        #[qsignal]
        fn error(self: Pin<&mut RustSignals>, message: QString);
    }
    // ANCHOR_END: book_signals_block

    // ANCHOR: book_signals_struct
    unsafe extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(bool, logging_enabled)]
        type RustSignals = super::RustSignalsRust;
    }
    // ANCHOR_END: book_signals_struct

    // ANCHOR: book_rust_obj_impl
    unsafe extern "RustQt" {
        /// Connect to the given url
        #[qinvokable]
        fn connect(self: Pin<&mut RustSignals>, url: &QUrl);

        /// Disconnect
        #[qinvokable]
        fn disconnect(self: Pin<&mut RustSignals>);
    }
    // ANCHOR_END: book_rust_obj_impl

    // ANCHOR: book_initialize_decl
    impl cxx_qt::Constructor<()> for RustSignals {}
    // ANCHOR_END: book_initialize_decl

    // ANCHOR: book_constructor_decl
    impl<'a> cxx_qt::Constructor<(&'a QUrl,), InitializeArguments = (&'a QUrl,)> for RustSignals {}
    // ANCHOR_END: book_constructor_decl
}

use core::pin::Pin;
use cxx_qt::CxxQtType;
use cxx_qt_lib::{ConnectionType, QMetaObjectConnectionGuard, QString, QUrl};

/// A QObject which has Q_SIGNALs
#[derive(Default)]
pub struct RustSignalsRust {
    pub(crate) connections: Option<[QMetaObjectConnectionGuard; 3]>,

    logging_enabled: bool,
}

impl qobject::RustSignals {
    /// Connect to the given url
    pub fn connect(self: Pin<&mut Self>, url: &QUrl) {
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
    pub fn disconnect(self: Pin<&mut Self>) {
        // Emit a signal to QML stating that we have disconnected
        self.disconnected();
    }
}

// ANCHOR: book_initialize_impl
impl cxx_qt::Initialize for qobject::RustSignals {
    /// Initialize the QObject, creating a connection reacting to the logging enabled property
    fn initialize(self: core::pin::Pin<&mut Self>) {
        // ANCHOR_END: book_initialize_impl
        self.on_logging_enabled_changed(|mut qobject| {
            // Determine if logging is enabled
            if *qobject.as_ref().logging_enabled() {
                // If no connections have been made, then create them
                if qobject.as_ref().connections.is_none() {
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

impl<'a> cxx_qt::Constructor<(&'a QUrl,)> for qobject::RustSignals {
    type NewArguments = ();
    type BaseArguments = ();
    type InitializeArguments = (&'a QUrl,);

    fn route_arguments(
        (url,): (&'a QUrl,),
    ) -> (
        Self::NewArguments,
        Self::BaseArguments,
        Self::InitializeArguments,
    ) {
        ((), (), (url,))
    }

    fn new(_arguments: Self::NewArguments) -> <Self as CxxQtType>::Rust {
        Default::default()
    }

    fn initialize(mut self: core::pin::Pin<&mut Self>, (url,): Self::InitializeArguments) {
        <Self as cxx_qt::Constructor<()>>::initialize(self.as_mut(), ());

        self.connect(url);
    }
}
// ANCHOR_END: book_macro_code
