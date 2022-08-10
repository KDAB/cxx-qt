// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

// ANCHOR: book_macro_code
enum Event {
    TitleArrived(String),
}

// ANCHOR: book_namespace_macro
#[cxx_qt::bridge(namespace = "cxx_qt::website")]
mod ffi {
    // ANCHOR_END: book_namespace_macro

    use super::Event;
    use futures::{
        channel::mpsc::{UnboundedReceiver, UnboundedSender},
        executor::block_on,
        FutureExt, StreamExt,
    };
    use futures_timer::Delay;
    use std::{
        sync::atomic::{AtomicBool, Ordering},
        thread,
        time::Duration,
    };

    #[namespace = ""]
    unsafe extern "C++" {
        include!("cxx-qt-lib/include/qt_types.h");
        type QString = cxx_qt_lib::QString;
    }

    pub struct Data {
        url: UniquePtr<QString>,
        title: UniquePtr<QString>,
    }

    impl Default for Data {
        fn default() -> Self {
            Self {
                url: QString::from_str("known"),
                title: QString::from_str("Press refresh to get a title..."),
            }
        }
    }

    #[cxx_qt::qobject]
    pub struct Website {
        event_sender: UnboundedSender<Event>,
        event_queue: UnboundedReceiver<Event>,
        loading: AtomicBool,
    }

    impl Default for Website {
        fn default() -> Self {
            let (event_sender, event_queue) = futures::channel::mpsc::unbounded();

            Self {
                event_sender,
                event_queue,
                loading: AtomicBool::new(false),
            }
        }
    }

    impl cxx_qt::QObject<Website> {
        #[qinvokable]
        pub fn change_url(self: Pin<&mut Self>) {
            let url = self.as_ref().url().to_string();
            let new_url = if url == "known" { "unknown" } else { "known" };
            self.set_url(QString::from_str(new_url).as_ref().unwrap());
        }

        #[qinvokable]
        pub fn refresh_title(mut self: Pin<&mut Self>) {
            // TODO: SeqCst is probably not the most efficient solution
            let new_load = self.rust().loading.compare_exchange(
                false,
                true,
                Ordering::SeqCst,
                Ordering::SeqCst,
            );
            if new_load.is_err() {
                println!("Skipped refresh_title request, because already in progress.");
                return;
            }

            self.as_mut()
                .set_title(QString::from_str("Loading...").as_ref().unwrap());

            let url = self.as_ref().url().to_string();
            // ANCHOR: book_cpp_update_requester
            // Retrieve the update requester from the CppObj
            let update_requester = self.as_mut().update_requester();
            // ANCHOR_END: book_cpp_update_requester
            let event_sender = self.rust().event_sender.clone();

            let fetch_title = async move {
                // Simulate the delay of a network request with a simple timer
                Delay::new(Duration::from_secs(1)).await;

                let title = if url == "known" {
                    "Known website"
                } else {
                    "Unknown website"
                };

                event_sender
                    .unbounded_send(Event::TitleArrived(title.to_owned()))
                    .unwrap();
                // ANCHOR: book_request_update
                // Request an update from the background thread
                update_requester.request_update();
                // ANCHOR_END: book_request_update
            };
            thread::spawn(move || block_on(fetch_title));
        }

        #[qinvokable]
        pub fn new_title_value(self: Pin<&mut Self>) {
            println!("title changed");
        }

        #[qinvokable]
        pub fn new_url_value(self: Pin<&mut Self>) {
            self.refresh_title();
        }

        fn process_event(mut self: Pin<&mut Self>, event: &Event) {
            match event {
                Event::TitleArrived(title) => {
                    self.as_mut()
                        .set_title(QString::from_str(title).as_ref().unwrap());
                    self.rust().loading.store(false, Ordering::Relaxed);
                }
            }
        }
    }

    // ANCHOR: book_update_request_handler
    impl UpdateRequestHandler for cxx_qt::QObject<Website> {
        fn handle_update_request(mut self: Pin<&mut Self>) {
            while let Some(event) =
                unsafe { self.as_mut().rust_mut().event_queue.next().now_or_never() }
            {
                if let Some(event) = event {
                    self.as_mut().process_event(&event);
                }
            }
        }
    }
    // ANCHOR_END: book_update_request_handler
}
// ANCHOR_END: book_macro_code
