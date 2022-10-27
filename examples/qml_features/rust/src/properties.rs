// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx_qt::bridge(cxx_file_stem = "rust_properties")]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
        include!("cxx-qt-lib/qurl.h");
        type QUrl = cxx_qt_lib::QUrl;
    }

    // ANCHOR: book_properties_struct
    #[cxx_qt::qobject]
    pub struct RustProperties {
        #[qproperty]
        connected: bool,

        #[qproperty]
        connected_url: QUrl,

        #[qproperty]
        previous_connected_url: QUrl,

        #[qproperty]
        status_message: QString,
    }
    // ANCHOR_END: book_properties_struct

    // ANCHOR: book_properties_default
    impl Default for RustProperties {
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
        #[qinvokable]
        pub fn connect(mut self: Pin<&mut Self>, mut url: QUrl) {
            // Check that the url starts with kdab
            if url.to_string().starts_with("https://kdab.com") {
                self.as_mut().set_connected(true);
                self.as_mut().set_status_message(QString::from("Connected"));

                // Safety:
                // We are directly modifying the Rust struct to avoid creating an extra QUrl.
                // But using rust_mut() is unsafe as this does not trigger a signal change for the property
                // So we need to manually call this ourselves.
                unsafe {
                    std::mem::swap(&mut self.as_mut().rust_mut().connected_url, &mut url);
                    self.as_mut().connected_url_changed();
                }
                // Then we can store the old url without having to temporarily store it
                self.set_previous_connected_url(url);
            } else {
                self.as_mut().set_connected(false);
                self.set_status_message(QString::from("URL does not start with https://kdab.com"));
            }
        }

        #[qinvokable]
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
}
