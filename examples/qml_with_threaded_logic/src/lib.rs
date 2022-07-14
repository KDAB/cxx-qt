// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

// ANCHOR: book_macro_code
#[cxx_qt::bridge]
mod website {
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

    enum Event {
        TitleArrived(String),
    }

    pub struct Data {
        url: String,
        title: String,
    }

    impl Default for Data {
        fn default() -> Self {
            Self {
                url: "known".to_owned(),
                title: "Press refresh to get a title...".to_owned(),
            }
        }
    }

    struct RustObj {
        event_sender: UnboundedSender<Event>,
        event_queue: UnboundedReceiver<Event>,
        loading: AtomicBool,
    }

    impl Default for RustObj {
        fn default() -> Self {
            let (event_sender, event_queue) = futures::channel::mpsc::unbounded();

            Self {
                event_sender,
                event_queue,
                loading: AtomicBool::new(false),
            }
        }
    }

    impl RustObj {
        #[invokable]
        fn change_url(&self, cpp: &mut CppObj) {
            let url = cpp.url();
            let new_url = if url == "known" { "unknown" } else { "known" };
            cpp.set_url(new_url);
        }

        #[invokable]
        fn refresh_title(&self, cpp: &mut CppObj) {
            // TODO: SeqCst is probably not the most efficient solution
            let new_load =
                self.loading
                    .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst);
            if new_load.is_err() {
                println!("Skipped refresh_title request, because already in progress.");
                return;
            }

            cpp.set_title("Loading...");

            let url = cpp.url();
            // ANCHOR: book_cpp_update_requester
            // Retrieve the update requester from the CppObj
            let update_requester = cpp.update_requester();
            // ANCHOR_END: book_cpp_update_requester
            let event_sender = self.event_sender.clone();

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

        fn process_event(&mut self, event: &Event, cpp: &mut CppObj) {
            match event {
                Event::TitleArrived(title) => {
                    cpp.set_title(title);
                    self.loading.store(false, Ordering::Relaxed);
                }
            }
        }
    }

    // ANCHOR: book_update_request_handler
    impl UpdateRequestHandler<CppObj<'_>> for RustObj {
        fn handle_update_request(&mut self, cpp: &mut CppObj) {
            while let Some(event) = self.event_queue.next().now_or_never() {
                if let Some(event) = event {
                    self.process_event(&event, cpp);
                }
            }
        }
    }
    // ANCHOR_END: book_update_request_handler

    impl PropertyChangeHandler<CppObj<'_>, Property> for RustObj {
        fn handle_property_change(&mut self, cpp: &mut CppObj, property: Property) {
            match property {
                Property::Url => self.refresh_title(cpp),
                Property::Title => println!("title changed"),
                _ => unreachable!(),
            }
        }
    }
}
// ANCHOR_END: book_macro_code
