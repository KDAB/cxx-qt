// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! This example shows how a Q_PROPERTY can be used

/// A CXX-Qt bridge which shows how a Q_PROPERTY can be used
#[cxx_qt::bridge(cxx_file_stem = "rust_properties")]
pub mod qobject {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        /// QString from cxx_qt_lib
        type QString = cxx_qt_lib::QString;
        include!("cxx-qt-lib/qurl.h");
        /// QUrl from cxx_qt_lib
        type QUrl = cxx_qt_lib::QUrl;
    }

    unsafe extern "RustQt" {
        // ANCHOR: book_properties_signature
        #[qobject]
        #[qml_element]
        #[qproperty(bool, connected)]
        #[qproperty(QUrl, connected_url)]
        #[qproperty(QUrl, previous_connected_url)]
        #[qproperty(QString, status_message)]
        type RustProperties = super::RustPropertiesRust;
        // ANCHOR_END: book_properties_signature

        /// Connect to the given url
        #[qinvokable]
        fn connect(self: Pin<&mut RustProperties>, mut url: QUrl);

        /// Disconnect from the stored url
        #[qinvokable]
        fn disconnect(self: Pin<&mut RustProperties>);
    }

    impl cxx_qt::Constructor<()> for RustProperties {}

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
    /// Connect to the given url
    pub fn connect(mut self: Pin<&mut Self>, mut url: QUrl) {
        // Check that the url starts with kdab
        if url.to_string().starts_with("https://kdab.com") {
            self.as_mut().set_connected(true);
            self.as_mut().set_status_message(QString::from("Connected"));

            // We are directly modifying the Rust struct to avoid creating an extra QUrl.
            // So we need to manually call the notify signal for the property ourselves.
            std::mem::swap(&mut self.as_mut().rust_mut().connected_url, &mut url);
            self.as_mut().connected_url_changed();

            // Then we can store the old url without having to temporarily store it
            self.set_previous_connected_url(url);
        } else {
            self.as_mut().set_connected(false);
            self.set_status_message(QString::from("URL does not start with https://kdab.com"));
        }
    }

    /// Disconnect from the stored url
    pub fn disconnect(mut self: Pin<&mut Self>) {
        self.as_mut().set_connected(false);
        self.as_mut()
            .set_status_message(QString::from("Disconnected"));
        // Here we show how data can be cloned instead of using the unsafe API to swap the values
        let previous_url = self.as_ref().connected_url().clone();
        self.as_mut().set_previous_connected_url(previous_url);
        self.set_connected_url(QUrl::default());
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
