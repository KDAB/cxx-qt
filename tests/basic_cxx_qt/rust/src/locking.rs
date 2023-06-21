// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

/// Two QObject that allow for testing that locking works
#[cxx_qt::bridge(cxx_file_stem = "locking")]
pub mod qobject {
    unsafe extern "RustQt" {
        /// A QObject which has cxx_qt::Locking
        #[cxx_qt::qobject]
        type RustLockingEnabled = super::RustLockingEnabledRust;

        #[qinvokable]
        fn get_counter(self: &RustLockingEnabled) -> u32;

        #[qinvokable]
        fn increment(self: Pin<&mut RustLockingEnabled>);
    }

    unsafe extern "RustQt" {
        /// A QObject which has !cxx_qt::Locking
        #[cxx_qt::qobject]
        type RustLockingDisabled = super::RustLockingDisabledRust;

        #[qinvokable]
        fn get_counter(self: &RustLockingDisabled) -> u32;

        #[qinvokable]
        fn increment(self: Pin<&mut RustLockingDisabled>);
    }

    unsafe impl !cxx_qt::Locking for RustLockingDisabled {}
}

use core::pin::Pin;
use std::{
    sync::atomic::{AtomicU32, Ordering},
    thread,
    time::Duration,
};

#[derive(Default)]
pub struct RustLockingEnabledRust {
    pub(crate) counter: AtomicU32,
}

impl qobject::RustLockingEnabled {
    fn get_counter(&self) -> u32 {
        self.counter.load(Ordering::Acquire)
    }

    fn increment(self: Pin<&mut Self>) {
        let counter = self.get_counter();
        thread::sleep(Duration::from_millis(100));
        self.counter.store(counter + 1, Ordering::Release);
    }
}

#[derive(Default)]
pub struct RustLockingDisabledRust {
    pub(crate) counter: AtomicU32,
}

impl qobject::RustLockingDisabled {
    fn get_counter(&self) -> u32 {
        self.counter.load(Ordering::Acquire)
    }

    fn increment(self: Pin<&mut Self>) {
        let counter = self.get_counter();
        thread::sleep(Duration::from_millis(100));
        self.counter.store(counter + 1, Ordering::Release);
    }
}
