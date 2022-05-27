// SPDX-FileCopyrightText: 2021, 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use cxx_qt::make_qobject;

mod constants;
mod network;
mod workers;

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

    #[allow(clippy::enum_variant_names)]
    pub enum Signal {
        SensorAdded { uuid: String },
        SensorChanged { uuid: String },
        SensorRemoved { uuid: String },
    }

    pub struct Data {
        pub average_use: f64,
        pub sensors: u32,
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

    pub enum QtSync {
        DataChange(Data),
        SignalChange(Signal),
    }

    struct RustObj {
        qt_rx: Receiver<QtSync>,
        qt_tx: SyncSender<QtSync>,
        join_handles: Option<[JoinHandle<()>; 4]>,
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
        #[invokable]
        fn sensor_power(&self, uuid: &str) -> f64 {
            let sensors = SensorsWorker::read_sensors(&self.sensors);

            if let Ok(uuid) = Uuid::parse_str(uuid) {
                sensors.get(&uuid).map(|v| v.power).unwrap_or_default()
            } else {
                0.0
            }
        }

        #[invokable]
        fn start_server(&mut self, cpp: &mut CppObj) {
            if self.join_handles.is_some() {
                println!("Already running a server!");
                return;
            }

            let (network_tx, network_rx) = sync_channel(CHANNEL_NETWORK_COUNT);
            let sensors_changed = Arc::new(AtomicBool::new(false));

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
                // If a sensor is not seen for N seconds we remove it
                std::thread::spawn(move || {
                    block_on(TimeoutWorker::run(timeout_network_tx, timeout_sensors))
                }),
                // When values change this then requests an update to Qt
                std::thread::spawn(move || {
                    block_on(AccumulatorWorker::run(
                        accumulator_qt_tx,
                        accumulator_sensors,
                        accumulator_sensors_changed,
                        accumulator_update_requester,
                    ))
                }),
                // Prepare our sensors thread, which reads from the network channel
                // and collates the commands into a hashmap.
                std::thread::spawn(move || {
                    block_on(SensorsWorker::run(
                        network_rx,
                        sensors_qt_tx,
                        sensors,
                        sensors_changed,
                        sensors_update_requester,
                    ))
                }),
                // Start a TCP server which listens for requests
                std::thread::spawn(move || block_on(NetworkServer::listen(network_tx))),
            ]);
        }
    }

    impl UpdateRequestHandler<CppObj<'_>> for RustObj {
        fn handle_update_request(&mut self, cpp: &mut CppObj) {
            // Process the new data from the background thread
            for packet in self.qt_rx.try_iter() {
                match packet {
                    // Here we have constructed a new Data struct so can consume it's values
                    // for other uses we could have passed an Enum across the channel
                    // and then process the required action here
                    QtSync::DataChange(data) => cpp.grab_values_from_data(data),
                    QtSync::SignalChange(signal) => cpp.emit_queued(signal),
                }
            }
        }
    }
}
