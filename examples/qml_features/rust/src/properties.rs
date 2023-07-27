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
        // ANCHOR: book_properties_struct
        #[qobject]
        #[qml_element]
        #[qproperty(bool, connected)]
        #[qproperty(QUrl, connected_url)]
        #[qproperty(QUrl, previous_connected_url)]
        #[qproperty(QString, status_message)]
        type RustProperties = super::RustPropertiesRust;
        // ANCHOR_END: book_properties_struct

        /// Connect to the given url
        #[qinvokable]
        fn connect(self: Pin<&mut RustProperties>, mut url: QUrl);

        /// Disconnect from the stored url
        #[qinvokable]
        fn disconnect(self: Pin<&mut RustProperties>);
    }
}

use core::pin::Pin;
use cxx_qt::CxxQtType;
use cxx_qt_lib::{QString, QUrl};

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
