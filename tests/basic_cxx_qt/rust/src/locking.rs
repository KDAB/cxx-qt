// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

/// Two QObject that allow for testing that locking works
#[cxx_qt::bridge(cxx_file_stem = "locking")]
pub mod ffi {
    unsafe extern "RustQt" {
        /// A QObject which has cxx_qt::Locking
        #[cxx_qt::qobject]
        type RustLockingEnabled = super::RustLockingEnabledRust;

        #[qinvokable]
        fn get_counter(self: &qobject::RustLockingEnabled) -> u32;

        #[qinvokable]
        fn increment(self: Pin<&mut qobject::RustLockingEnabled>);
    }

    unsafe extern "RustQt" {
        /// A QObject which has !cxx_qt::Locking
        #[cxx_qt::qobject]
        type RustLockingDisabled = super::RustLockingDisabledRust;

        #[qinvokable]
        fn get_counter(self: &qobject::RustLockingDisabled) -> u32;

        #[qinvokable]
        fn increment(self: Pin<&mut qobject::RustLockingDisabled>);
    }

    unsafe impl !cxx_qt::Locking for qobject::RustLockingDisabled {}
}

use core::pin::Pin;
use cxx_qt::CxxQtType;
use std::{
    sync::atomic::{AtomicU32, Ordering},
    thread,
    time::Duration,
};

#[derive(Default)]
pub struct RustLockingEnabledRust {
    pub(crate) counter: AtomicU32,
}

// TODO: this will change to qobject::RustLockingEnabled once
// https://github.com/KDAB/cxx-qt/issues/559 is done
impl ffi::RustLockingEnabled {
    fn get_counter(&self) -> u32 {
        self.rust().counter.load(Ordering::Acquire)
    }

    fn increment(self: Pin<&mut Self>) {
        let counter = self.as_ref().get_counter();
        thread::sleep(Duration::from_millis(100));
        self.rust().counter.store(counter + 1, Ordering::Release);
    }
}

#[derive(Default)]
pub struct RustLockingDisabledRust {
    pub(crate) counter: AtomicU32,
}

// TODO: this will change to qobject::RustLockingDisabled once
// https://github.com/KDAB/cxx-qt/issues/559 is done
impl ffi::RustLockingDisabled {
    fn get_counter(&self) -> u32 {
        self.rust().counter.load(Ordering::Acquire)
    }

    fn increment(self: Pin<&mut Self>) {
        let counter = self.as_ref().get_counter();
        thread::sleep(Duration::from_millis(100));
        self.rust().counter.store(counter + 1, Ordering::Release);
    }
}
