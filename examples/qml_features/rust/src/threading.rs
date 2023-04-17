// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! This example shows how a threading can be used

/// A CXX-Qt bridge which shows how a threading can be used
// ANCHOR: book_macro_code
// ANCHOR: book_namespace_macro
#[cxx_qt::bridge(cxx_file_stem = "threading_website", namespace = "cxx_qt::website")]
pub mod ffi {
    // ANCHOR_END: book_namespace_macro
    #[namespace = ""]
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        /// QString from cxx_qt_lib
        type QString = cxx_qt_lib::QString;
        include!("cxx-qt-lib/qurl.h");
        /// QUrl from cxx_qt_lib
        type QUrl = cxx_qt_lib::QUrl;
    }

    /// A QObject which has threading
    #[cxx_qt::qobject(qml_uri = "com.kdab.cxx_qt.demo", qml_version = "1.0")]
    pub struct ThreadingWebsite {
        /// The title Q_PROPERTY
        #[qproperty]
        title: QString,
        /// The url Q_PROPERTY
        #[qproperty]
        url: QUrl,

        loading: std::sync::atomic::AtomicBool,
    }

    impl Default for ThreadingWebsite {
        fn default() -> Self {
            Self {
                url: QUrl::from("https://kdab.com"),
                title: QString::from("KDAB"),

                loading: std::sync::atomic::AtomicBool::new(false),
            }
        }
    }

    impl qobject::ThreadingWebsite {
        /// Swap the URL between kdab.com and github.com
        #[qinvokable]
        pub fn change_url(self: Pin<&mut Self>) {
            let new_url = if self.url().to_string() == "https://kdab.com" {
                "https://github.com/kdab/cxx-qt"
            } else {
                "https://kdab.com"
            };
            self.set_url(QUrl::from(new_url));
        }

        /// Simulate delay of a network request to retrieve the title of the website
        #[qinvokable]
        pub fn fetch_title(mut self: Pin<&mut Self>) {
            // Check that we aren't already retrieving a title
            if self
                .rust()
                .loading
                .compare_exchange(
                    false,
                    true,
                    std::sync::atomic::Ordering::SeqCst,
                    std::sync::atomic::Ordering::SeqCst,
                )
                .is_err()
            {
                println!("Already fetching a title.");
                return;
            }

            // Indicate that we are loading
            self.as_mut().set_title(QString::from("Loading..."));

            // Fetch items we need to move into the thread
            // ANCHOR: book_qt_thread
            let qt_thread = self.qt_thread();
            // ANCHOR_END: book_qt_thread
            let url = self.url().to_string();

            // Spawn a Rust thread to simulate the slow network request
            std::thread::spawn(move || {
                // Wait for 1 second
                std::thread::sleep(std::time::Duration::from_secs(1));

                // Build the new title
                let title = if url == "https://kdab.com" {
                    "KDAB".to_owned()
                } else {
                    "GitHub".to_owned()
                };

                // ANCHOR: book_qt_thread_queue
                // Queue a Rust closure to the Qt thread
                qt_thread
                    .queue(move |mut qobject_website| {
                        // Update the title property of the QObject
                        qobject_website.as_mut().set_title(QString::from(&title));

                        // Indicate that we have finished loading the title
                        qobject_website
                            .as_ref()
                            .rust()
                            .loading
                            .store(false, std::sync::atomic::Ordering::Relaxed);
                    })
                    .unwrap();
                // ANCHOR_END: book_qt_thread_queue
            });
        }
    }
}
// ANCHOR_END: book_macro_code
