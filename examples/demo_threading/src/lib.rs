// SPDX-FileCopyrightText: 2021, 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

mod constants;
mod network;
mod workers;

#[cxx_qt::bridge(namespace = "cxx_qt::energy_usage")]
pub mod ffi {
    use super::{
        constants::{CHANNEL_NETWORK_COUNT, CHANNEL_QT_COUNT, SENSOR_MAXIMUM_COUNT},
        network::NetworkServer,
        workers::{AccumulatorWorker, SensorChanged, SensorHashMap, SensorsWorker, TimeoutWorker},
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

    #[namespace = ""]
    unsafe extern "C++" {
        include!("cxx-qt-lib/include/qt_types.h");
        type QString = cxx_qt_lib::QString;
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

    #[cxx_qt::qobject]
    pub struct EnergyUsage {
        pub qt_data_rx: Receiver<Data>,
        qt_data_tx: SyncSender<Data>,
        pub qt_signals_rx: Receiver<SensorChanged>,
        qt_signals_tx: SyncSender<SensorChanged>,
        join_handles: Option<[JoinHandle<()>; 4]>,
        sensors: Arc<Mutex<Arc<SensorHashMap>>>,
    }

    impl Default for EnergyUsage {
        fn default() -> Self {
            let (qt_data_tx, qt_data_rx) = sync_channel(CHANNEL_QT_COUNT);
            let (qt_signals_tx, qt_signals_rx) = sync_channel(CHANNEL_QT_COUNT);
            Self {
                qt_data_rx,
                qt_data_tx,
                qt_signals_rx,
                qt_signals_tx,
                join_handles: None,
                sensors: Arc::new(Mutex::new(Arc::new(SensorHashMap::with_capacity(
                    SENSOR_MAXIMUM_COUNT,
                )))),
            }
        }
    }

    #[cxx_qt::signals(EnergyUsage)]
    #[allow(clippy::enum_variant_names)]
    pub enum EnergySignals {
        SensorAdded { uuid: UniquePtr<QString> },
        SensorChanged { uuid: UniquePtr<QString> },
        SensorRemoved { uuid: UniquePtr<QString> },
    }

    impl cxx_qt::QObject<EnergyUsage> {
        #[qinvokable]
        pub fn sensor_power(self: Pin<&mut Self>, uuid: &QString) -> f64 {
            // TODO: for now we use the unsafe rust_mut() API
            // later there will be getters and setters for the properties
            unsafe {
                let sensors = SensorsWorker::read_sensors(&self.rust_mut().sensors);

                if let Ok(uuid) = Uuid::parse_str(&uuid.to_string()) {
                    sensors.get(&uuid).map(|v| v.power).unwrap_or_default()
                } else {
                    0.0
                }
            }
        }

        #[qinvokable]
        pub fn start_server(mut self: Pin<&mut Self>) {
            if self.rust().join_handles.is_some() {
                println!("Already running a server!");
                return;
            }

            let (network_tx, network_rx) = sync_channel(CHANNEL_NETWORK_COUNT);
            let sensors_changed = Arc::new(AtomicBool::new(false));

            // TODO: for now we use the unsafe rust_mut() API
            // later there will be getters and setters for the properties
            let accumulator_sensors = Arc::clone(unsafe { &self.as_mut().rust_mut().sensors });
            let accumulator_sensors_changed = Arc::clone(&sensors_changed);
            let accumulator_qt_data_tx = unsafe { self.as_mut().rust_mut().qt_data_tx.clone() };
            let accumulator_qt_thread = self.qt_thread();
            let sensors = Arc::clone(unsafe { &self.as_mut().rust_mut().sensors });
            let sensors_qt_signals_tx = unsafe { self.as_mut().rust_mut().qt_signals_tx.clone() };
            let sensors_qt_thread = self.qt_thread();
            let timeout_sensors = Arc::clone(unsafe { &self.as_mut().rust_mut().sensors });
            let timeout_network_tx = network_tx.clone();

            // Start our threads
            unsafe {
                self.rust_mut().join_handles = Some([
                    // If a sensor is not seen for N seconds we remove it
                    std::thread::spawn(move || {
                        block_on(TimeoutWorker::run(timeout_network_tx, timeout_sensors))
                    }),
                    // When values change this then requests an update to Qt
                    std::thread::spawn(move || {
                        block_on(AccumulatorWorker::run(
                            accumulator_qt_data_tx,
                            accumulator_sensors,
                            accumulator_sensors_changed,
                            accumulator_qt_thread,
                        ))
                    }),
                    // Prepare our sensors thread, which reads from the network channel
                    // and collates the commands into a hashmap.
                    std::thread::spawn(move || {
                        block_on(SensorsWorker::run(
                            network_rx,
                            sensors_qt_signals_tx,
                            sensors,
                            sensors_changed,
                            sensors_qt_thread,
                        ))
                    }),
                    // Start a TCP server which listens for requests
                    std::thread::spawn(move || block_on(NetworkServer::listen(network_tx))),
                ]);
            }
        }
    }
}
