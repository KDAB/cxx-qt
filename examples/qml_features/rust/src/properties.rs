// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! This example shows how a Q_PROPERTY can be used

/// A CXX-Qt bridge which shows how a Q_PROPERTY can be used
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

    extern "RustQt" {
        // ANCHOR: book_properties_signature
        #[qobject]
        #[qml_element]
        #[qproperty(bool, connected, READ, NOTIFY = connected_state_changed)]
        #[qproperty(QUrl, connected_url, cxx_name = "connectedUrl", READ, WRITE = set_url, NOTIFY = connected_state_changed, RESET = reset_url)]
        #[qproperty(QUrl, previous_connected_url, cxx_name = "previousConnectedUrl", READ, NOTIFY = connected_state_changed)]
        #[qproperty(QString, status_message, cxx_name = "statusMessage", READ, NOTIFY = connected_state_changed)]
        type RustProperties = super::RustPropertiesRust;
        // ANCHOR_END: book_properties_signature

        /// Custom on changed signal, used for all the properties
        #[qsignal]
        #[cxx_name = "connectedStateChanged"]
        fn connected_state_changed(self: Pin<&mut RustProperties>);

        /// Custom setter for connected_url, which also handles setting the other qproperties
        #[cxx_name = "setUrl"]
        fn set_url(self: Pin<&mut RustProperties>, url: QUrl);

        /// Resets value of connected_url to empty, as well as calling the other disconnected logic
        #[cxx_name = "resetUrl"]
        fn reset_url(self: Pin<&mut RustProperties>);
    }

    impl cxx_qt::Initialize for RustProperties {}

    // Dummy constructor, added for an example in the book.
    // ANCHOR: book_constructor_new_decl
    impl<'a>
        cxx_qt::Constructor<
            (bool, &'a QUrl, &'a QUrl, &'a QString),
            NewArguments = (bool, &'a QUrl, &'a QUrl, &'a QString),
        > for RustProperties
    {
    }
    // ANCHOR_END: book_constructor_new_decl
}

use core::pin::Pin;
use cxx_qt::CxxQtType;
use cxx_qt_lib::{QString, QUrl};

// ANCHOR: book_properties_struct
/// A QObject which has Q_PROPERTYs
pub struct RustPropertiesRust {
    /// A connected Q_PROPERTY
    connected: bool,

    /// A connected_url Q_PROPERTY
    pub(crate) connected_url: QUrl,

    /// A previous_connected_url Q_PROPERTY
    previous_connected_url: QUrl,

    /// A status_message Q_PROPERTY
    status_message: QString,
}
// ANCHOR_END: book_properties_struct

// ANCHOR: book_properties_default
impl Default for RustPropertiesRust {
    fn default() -> Self {
        Self {
            connected: false,
            connected_url: QUrl::default(),
            previous_connected_url: QUrl::default(),
            status_message: QString::from("Disconnected"),
        }
    }
}
// ANCHOR_END: book_properties_default

impl qobject::RustProperties {
    /// Custom setter for RustProperties.connected_url
    pub fn set_url(mut self: Pin<&mut Self>, mut url: QUrl) {
        // Check that the url starts with kdab
        if url.to_string().starts_with("https://kdab.com") {
            self.as_mut().rust_mut().connected = true;
            self.as_mut().rust_mut().status_message = QString::from("Connected");

            std::mem::swap(&mut self.as_mut().rust_mut().connected_url, &mut url);

            // Then we can store the old url without having to temporarily store it
            self.as_mut().rust_mut().previous_connected_url = url;
        } else {
            self.as_mut().rust_mut().connected = false;
            self.as_mut().rust_mut().status_message =
                QString::from("URL does not start with https://kdab.com")
        }
        self.as_mut().connected_state_changed();
    }

    /// Reset the url to an empty state
    pub fn reset_url(mut self: Pin<&mut Self>) {
        self.as_mut().rust_mut().connected = false;
        self.as_mut().rust_mut().status_message = QString::from("Disconnected");
        let previous_url = self.as_ref().connected_url.clone();
        self.as_mut().rust_mut().previous_connected_url = previous_url;

        std::mem::swap(
            &mut self.as_mut().rust_mut().connected_url,
            &mut QUrl::default(),
        );
        self.as_mut().connected_state_changed();
    }
}

impl cxx_qt::Initialize for qobject::RustProperties {
    fn initialize(self: Pin<&mut Self>) {}
}

// Dummy constructor, added for an example in the book.
// ANCHOR: book_constructor_new_impl
impl<'a> cxx_qt::Constructor<(bool, &'a QUrl, &'a QUrl, &'a QString)> for qobject::RustProperties {
    type NewArguments = (bool, &'a QUrl, &'a QUrl, &'a QString);
    type InitializeArguments = ();
    type BaseArguments = ();

    fn route_arguments(
        arguments: (bool, &'a QUrl, &'a QUrl, &'a QString),
    ) -> (
        Self::NewArguments,
        Self::BaseArguments,
        Self::InitializeArguments,
    ) {
        // pass all arguments to the `new` function.
        (arguments, (), ())
    }

    fn new(
        (connected, connected_url, previous_connected_url, status_message): Self::NewArguments,
    ) -> Self::Rust {
        Self::Rust {
            connected,
            connected_url: connected_url.clone(),
            previous_connected_url: previous_connected_url.clone(),
            status_message: status_message.clone(),
        }
    }
}
// ANCHOR_END: book_constructor_new_impl
