// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

// ANCHOR: book_macro_code
// ANCHOR: book_namespace_macro
#[cxx_qt::bridge(namespace = "cxx_qt::website")]
mod ffi {
    // ANCHOR_END: book_namespace_macro
    #[namespace = ""]
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        include!("cxx-qt-lib/qurl.h");
        type QString = cxx_qt_lib::QString;
        type QUrl = cxx_qt_lib::QUrl;
    }

    #[cxx_qt::qobject]
    pub struct ThreadingWebsite {
        #[qproperty]
        title: QString,
        #[qproperty]
        url: QUrl,

        tx: std::sync::mpsc::SyncSender<String>,
        rx: std::sync::mpsc::Receiver<String>,
        loading: std::sync::atomic::AtomicBool,
    }

    impl Default for ThreadingWebsite {
        fn default() -> Self {
            let (tx, rx) = std::sync::mpsc::sync_channel(32);
            Self {
                url: QUrl::from("https://kdab.com"),
                title: QString::from("KDAB"),

                tx,
                rx,
                loading: std::sync::atomic::AtomicBool::new(false),
            }
        }
    }

    impl cxx_qt::QObject<ThreadingWebsite> {
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
            let tx = self.rust().tx.clone();
            let url = self.url().to_string();

            // Spawn a Rust thread to simulate the slow network request
            std::thread::spawn(move || {
                // Wait for 1 second
                std::thread::sleep(std::time::Duration::from_secs(1));

                // Build the new title and add to the Rust channel
                let title = if url == "https://kdab.com" {
                    "KDAB"
                } else {
                    "GitHub"
                };
                tx.send(title.to_owned()).unwrap();

                // ANCHOR: book_qt_thread_queue
                // Queue a Rust function pointer o the Qt thread
                qt_thread
                    .queue(|mut qobject_website| {
                        // Retrieve the latest item in the Rust channel
                        if let Some(title) = qobject_website.as_ref().rust().rx.try_iter().last() {
                            // Update the title property of the QObject
                            qobject_website.as_mut().set_title(QString::from(&title));

                            // Indicate that we have finished loading the title
                            qobject_website
                                .as_ref()
                                .rust()
                                .loading
                                .store(false, std::sync::atomic::Ordering::Relaxed);
                        }
                    })
                    .unwrap();
                // ANCHOR_END: book_qt_thread_queue
            });
        }
    }
}
// ANCHOR_END: book_macro_code
