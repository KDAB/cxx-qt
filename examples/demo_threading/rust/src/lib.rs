// SPDX-FileCopyrightText: 2021, 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

mod constants;
mod network;
mod workers;

// This mod defines our QObject called EnergyUsage
#[cxx_qt::bridge(namespace = "cxx_qt::energy_usage")]
pub mod qobject {
    #[namespace = ""]
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(f64, average_use)]
        #[qproperty(u32, sensors)]
        #[qproperty(f64, total_use)]
        type EnergyUsage = super::EnergyUsageRust;
    }

    // Enabling threading on the qobject
    impl cxx_qt::Threading for EnergyUsage {}

    extern "RustQt" {
        /// A new sensor has been detected
        #[qsignal]
        #[cxx_name = "sensorAdded"]
        fn sensor_added(self: Pin<&mut EnergyUsage>, uuid: QString);
        /// A value on an existing sensor has changed
        #[qsignal]
        #[cxx_name = "sensorChanged"]
        fn sensor_changed(self: Pin<&mut EnergyUsage>, uuid: QString);
        /// An existing sensor has been removed
        #[qsignal]
        #[cxx_name = "sensorRemoved"]
        fn sensor_removed(self: Pin<&mut EnergyUsage>, uuid: QString);
    }

    extern "RustQt" {
        /// A Q_INVOKABLE that returns the current power usage for a given uuid
        #[qinvokable]
        #[cxx_name = "sensorPower"]
        fn sensor_power(self: Pin<&mut EnergyUsage>, uuid: &QString) -> f64;
    }

    impl cxx_qt::Initialize for EnergyUsage {}
}

use crate::{
    constants::{CHANNEL_NETWORK_COUNT, SENSOR_MAXIMUM_COUNT},
    network::NetworkServer,
    workers::{AccumulatorWorker, SensorHashMap, SensorsWorker, TimeoutWorker},
};

use core::pin::Pin;
use cxx_qt::{CxxQtType, Threading};
use cxx_qt_lib::QString;
use futures::executor::block_on;
use std::{
    sync::{atomic::AtomicBool, mpsc::sync_channel, Arc, Mutex},
    thread::JoinHandle,
};
use uuid::Uuid;

pub struct EnergyUsageRust {
    /// The average power usage of the connected sensors
    average_use: f64,
    /// The count of connected sensors
    sensors: u32,
    /// The total power usage of the connected sensors
    total_use: f64,

    /// The join handles of the running threads
    pub(crate) join_handles: Option<[JoinHandle<()>; 4]>,
    /// A HashMap of the currently connected sensors
    ///
    /// This uses an Arc inside the Mutex as well as outside so that the HashMap is only
    /// cloned when required. By using Arc::make_mut on the inner HashMap data is only cloned
    /// when mutating if another thread is still holding onto reference to the data.
    /// <https://doc.rust-lang.org/std/sync/struct.Arc.html#method.make_mut>
    pub(crate) sensors_map: Arc<Mutex<Arc<SensorHashMap>>>,
}

impl Default for EnergyUsageRust {
    fn default() -> Self {
        Self {
            average_use: 0.0,
            sensors: 0,
            total_use: 0.0,

            join_handles: None,
            sensors_map: Arc::new(Mutex::new(Arc::new(SensorHashMap::with_capacity(
                SENSOR_MAXIMUM_COUNT,
            )))),
        }
    }
}

impl qobject::EnergyUsage {
    /// A Q_INVOKABLE that returns the current power usage for a given uuid
    pub fn sensor_power(self: Pin<&mut Self>, uuid: &QString) -> f64 {
        let sensors = SensorsWorker::read_sensors(&self.rust_mut().sensors_map);

        if let Ok(uuid) = Uuid::parse_str(&uuid.to_string()) {
            sensors.get(&uuid).map(|v| v.power).unwrap_or_default()
        } else {
            0.0
        }
    }
}

impl cxx_qt::Constructor<()> for qobject::EnergyUsage {
    type NewArguments = ();
    type BaseArguments = ();
    type InitializeArguments = ();

    fn route_arguments(
        _args: (),
    ) -> (
        Self::NewArguments,
        Self::BaseArguments,
        Self::InitializeArguments,
    ) {
        ((), (), ())
    }

    fn new((): ()) -> EnergyUsageRust {
        EnergyUsageRust::default()
    }

    fn initialize(mut self: core::pin::Pin<&mut Self>, _arguments: Self::InitializeArguments) {
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
        let accumulator_sensors = Arc::clone(&self.as_mut().rust_mut().sensors_map);
        let accumulator_sensors_changed = Arc::clone(&sensors_changed);
        let accumulator_qt_thread = self.qt_thread();
        let sensors = Arc::clone(&self.as_mut().rust_mut().sensors_map);
        let sensors_qt_thread = self.qt_thread();
        let timeout_sensors = Arc::clone(&self.as_mut().rust_mut().sensors_map);
        let timeout_network_tx = network_tx.clone();

        // Start our threads
        self.rust_mut().join_handles = Some([
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
                    accumulator_sensors,
                    accumulator_sensors_changed,
                    accumulator_qt_thread,
                ))
            }),
            // Create a SensorsWorker
            // Reads network requests from the NetworkServer, collates the commands
            // by mutating the sensors hashmap, and requests signal changes to Qt
            std::thread::spawn(move || {
                block_on(SensorsWorker::run(
                    network_rx,
                    sensors,
                    sensors_changed,
                    sensors_qt_thread,
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
