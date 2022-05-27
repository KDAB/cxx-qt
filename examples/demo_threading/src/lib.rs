// SPDX-FileCopyrightText: 2021, 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use cxx_qt::make_qobject;

mod constants;
mod network;
mod workers;

// This mod defines our QObject called EnergyUsage
#[make_qobject]
pub mod energy_usage {
    use super::{
        constants::{CHANNEL_NETWORK_COUNT, CHANNEL_QT_COUNT, SENSOR_MAXIMUM_COUNT},
        network::NetworkServer,
        workers::{AccumulatorWorker, SensorHashMap, SensorsWorker, TimeoutWorker},
    };
    use futures::executor::block_on;
    use std::{
        sync::{
            atomic::AtomicBool,
            mpsc::{sync_channel, Receiver, SyncSender},
            Arc, Mutex,
        },
        thread::JoinHandle,
    };
    use uuid::Uuid;

    /// Define Q_SIGNALS that are created on the QObject
    #[allow(clippy::enum_variant_names)]
    pub enum Signal {
        /// A new sensor has been detected
        SensorAdded { uuid: String },
        /// A value on an existing sensor has changed
        SensorChanged { uuid: String },
        /// An existing sensor has been removed
        SensorRemoved { uuid: String },
    }

    /// Define the Q_PROPERTYs that are created on the QObject
    pub struct Data {
        /// The average power usage of the connected sensors
        pub average_use: f64,
        /// The count of connected sensors
        pub sensors: u32,
        /// The total power usage of the connected sensors
        pub total_use: f64,
    }

    impl Default for Data {
        fn default() -> Self {
            Self {
                average_use: 0.0,
                sensors: 0,
                total_use: 0.0,
            }
        }
    }

    /// Define an enum used for communication between Rust background threads
    /// and the Qt update thread
    pub enum QtSync {
        /// The Q_PROPERTYs have changed and have new data
        DataChange(Data),
        /// One of the  Q_SIGNALS should be emitted
        SignalChange(Signal),
    }

    struct RustObj {
        /// The sender and receiver for syncing the Qt state from Rust background threads
        qt_rx: Receiver<QtSync>,
        qt_tx: SyncSender<QtSync>,
        /// The join handles of the running threads
        join_handles: Option<[JoinHandle<()>; 4]>,
        /// A HashMap of the currently connected sensors
        ///
        /// This uses an Arc inside the Mutex as well as outside so that the HashMap is only
        /// cloned when required. By using Arc::make_mut on the inner HashMap data is only cloned
        /// when mutating if another thread is still holding onto reference to the data.
        /// https://doc.rust-lang.org/std/sync/struct.Arc.html#method.make_mut
        sensors: Arc<Mutex<Arc<SensorHashMap>>>,
    }

    impl Default for RustObj {
        fn default() -> Self {
            let (qt_tx, qt_rx) = sync_channel(CHANNEL_QT_COUNT);
            Self {
                qt_rx,
                qt_tx,
                join_handles: None,
                sensors: Arc::new(Mutex::new(Arc::new(SensorHashMap::with_capacity(
                    SENSOR_MAXIMUM_COUNT,
                )))),
            }
        }
    }

    impl RustObj {
        /// A Q_INVOKABLE that returns the current power usage for a given uuid
        #[invokable]
        fn sensor_power(&self, uuid: &str) -> f64 {
            let sensors = SensorsWorker::read_sensors(&self.sensors);

            if let Ok(uuid) = Uuid::parse_str(uuid) {
                sensors.get(&uuid).map(|v| v.power).unwrap_or_default()
            } else {
                0.0
            }
        }

        /// A Q_INVOKABLE which starts the TCP server
        #[invokable]
        fn start_server(&mut self, cpp: &mut CppObj) {
            if self.join_handles.is_some() {
                println!("Already running a server!");
                return;
            }

            // Create a channel which is used for passing valid network requests
            // from the NetworkServer to the SensorsWorker
            let (network_tx, network_rx) = sync_channel(CHANNEL_NETWORK_COUNT);
            // Create an AtomicBool which the SensorsWorker uses to tell
            // the AccumulatorWorker that the sensors have changed
            let sensors_changed = Arc::new(AtomicBool::new(false));

            // Make relevent clones so that we can pass them to the threads
            let accumulator_sensors = Arc::clone(&self.sensors);
            let accumulator_sensors_changed = Arc::clone(&sensors_changed);
            let accumulator_qt_tx = self.qt_tx.clone();
            let accumulator_update_requester = cpp.update_requester();
            let sensors = Arc::clone(&self.sensors);
            let sensors_qt_tx = self.qt_tx.clone();
            let sensors_update_requester = cpp.update_requester();
            let timeout_sensors = Arc::clone(&self.sensors);
            let timeout_network_tx = network_tx.clone();

            // Start our threads
            self.join_handles = Some([
                // Create a TimeoutWorker
                // If a sensor is not seen for N seconds then a disconnect is requested
                std::thread::spawn(move || {
                    block_on(TimeoutWorker::run(timeout_network_tx, timeout_sensors))
                }),
                // Create a AccumulatorWorker
                // When sensor values change this creates accumulations of the data
                // (such as total, average etc) and then requests an update to Qt
                std::thread::spawn(move || {
                    block_on(AccumulatorWorker::run(
                        accumulator_qt_tx,
                        accumulator_sensors,
                        accumulator_sensors_changed,
                        accumulator_update_requester,
                    ))
                }),
                // Create a SensorsWorker
                // Reads network requests from the NetworkServer, collates the commands
                // by mutating the sensors hashmap, and requests signal changes to Qt
                std::thread::spawn(move || {
                    block_on(SensorsWorker::run(
                        network_rx,
                        sensors_qt_tx,
                        sensors,
                        sensors_changed,
                        sensors_update_requester,
                    ))
                }),
                // Create a NetworkServer
                // Starts a TCP server which listens for requests and sends valid
                // network requests to the SensorsWorker
                std::thread::spawn(move || {
                    block_on(NetworkServer::listen("127.0.0.1:8080", network_tx))
                }),
            ]);
        }
    }

    impl UpdateRequestHandler<CppObj<'_>> for RustObj {
        /// When an update to Qt has been requested, this method is called from
        /// the Qt event loop
        fn handle_update_request(&mut self, cpp: &mut CppObj) {
            // Process packets from the background thread
            for packet in self.qt_rx.try_iter() {
                match packet {
                    // The Q_PROPERTYs have changed, so load all the values from Data again
                    //
                    // Note that all the Q_PROPERTYs always change in this example, we could
                    // also have an enum per Q_PROPERTY and use the individual setters
                    QtSync::DataChange(data) => cpp.grab_values_from_data(data),
                    // A Q_SIGNALS has been requested so emit it
                    QtSync::SignalChange(signal) => cpp.emit_queued(signal),
                }
            }
        }
    }
}
