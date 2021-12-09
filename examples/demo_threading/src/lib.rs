// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use cxx_qt::make_qobject;

#[make_qobject]
mod energy_usage {
    use std::{
        sync::{
            atomic::{AtomicBool, Ordering},
            mpsc::{channel, Receiver, Sender},
            Arc,
        },
        thread::JoinHandle,
    };

    enum EventArrived {
        AverageUse(f64),
        Connected(bool),
        Sensors(u32),
    }

    pub struct Data {
        average_use: f64,
        // FIXME: we don't have ability to mark properties readonly to QML yet
        // https://github.com/KDAB/cxx-qt/issues/42
        is_connected: bool,
        sensors: u32,
    }

    impl Default for Data {
        fn default() -> Self {
            Self {
                average_use: 0.0,
                is_connected: false,
                sensors: 0,
            }
        }
    }

    struct RustObj {
        event_sender: Sender<EventArrived>,
        event_receiver: Receiver<EventArrived>,
        join_handle: Option<JoinHandle<()>>,
        thread_running: Arc<AtomicBool>,
    }

    impl Default for RustObj {
        fn default() -> Self {
            let (event_sender, event_receiver) = channel();

            Self {
                event_sender,
                event_receiver,
                join_handle: None,
                thread_running: Arc::new(AtomicBool::new(false)),
            }
        }
    }

    impl RustObj {
        #[invokable]
        fn connect(&mut self, cpp: &mut CppObj) {
            // Mark that we are starting the thread
            // if there is one already then stop
            if self.thread_running.swap(true, Ordering::SeqCst) {
                println!("Already running a thread!");
                return;
            }

            // Prepare for moving into the thread
            let update_requester = cpp.update_requester();
            let event_sender = self.event_sender.clone();
            let thread_running = self.thread_running.clone();

            // Start our thread
            self.join_handle = Some(std::thread::spawn(move || {
                // Setup an initial set of sensors and state we have connected
                event_sender.send(EventArrived::Sensors(100)).unwrap();
                event_sender.send(EventArrived::Connected(true)).unwrap();
                update_requester.request_update();

                let mut usage = 0.0;
                let mut counter = 0;

                // Loop until we are told to disconnect
                while thread_running.load(Ordering::SeqCst) {
                    // TODO: do something interesting here with multiple sensors?

                    // For now every 5th loop, bump the usage between 0 -> 100
                    if counter == 0 {
                        usage += 1.0;
                        usage %= 100.0;
                    }

                    counter += 1;
                    counter %= 5;

                    // Indicate that the average use has changed
                    event_sender.send(EventArrived::AverageUse(usage)).unwrap();
                    update_requester.request_update();

                    std::thread::sleep(std::time::Duration::from_millis(16));
                }

                // End of thread so set values back to defaults
                event_sender.send(EventArrived::AverageUse(0.0)).unwrap();
                event_sender.send(EventArrived::Sensors(0)).unwrap();
                event_sender.send(EventArrived::Connected(false)).unwrap();
                update_requester.request_update();
            }));
        }

        #[invokable]
        fn disconnect(&mut self) {
            // Tell the thread to stop, if it was running then join, and wait until it stops
            if self.thread_running.swap(false, Ordering::SeqCst) {
                self.join_handle.take().map(JoinHandle::join);
            }
        }
    }

    impl UpdateRequestHandler<CppObj<'_>> for RustObj {
        fn handle_update_request(&mut self, cpp: &mut CppObj) {
            // Process each of the update requests from the background thread
            while let Ok(event) = self.event_receiver.try_recv() {
                match event {
                    EventArrived::AverageUse(usage) => cpp.set_average_use(usage),
                    EventArrived::Connected(connected) => cpp.set_is_connected(connected),
                    EventArrived::Sensors(sensors) => cpp.set_sensors(sensors),
                }
            }
        }
    }
}
