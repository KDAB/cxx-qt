// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

/// Two QObject that allow for testing that locking works
#[cxx_qt::bridge(cxx_file_stem = "locking")]
pub mod ffi {
    use std::sync::atomic::AtomicU32;

    /// A QObject which has cxx_qt::Locking
    #[cxx_qt::qobject]
    #[derive(Default)]
    pub struct RustLockingEnabled {
        pub(crate) counter: AtomicU32,
    }

    unsafe extern "RustQt" {
        #[qinvokable]
        fn get_counter(self: &qobject::RustLockingEnabled) -> u32;

        #[qinvokable]
        fn increment(self: Pin<&mut qobject::RustLockingEnabled>);
    }

    /// A QObject which has !cxx_qt::Locking
    #[cxx_qt::qobject]
    #[derive(Default)]
    pub struct RustLockingDisabled {
        pub(crate) counter: AtomicU32,
    }

    unsafe impl !cxx_qt::Locking for qobject::RustLockingDisabled {}

    unsafe extern "RustQt" {
        #[qinvokable]
        fn get_counter(self: &qobject::RustLockingDisabled) -> u32;

        #[qinvokable]
        fn increment(self: Pin<&mut qobject::RustLockingDisabled>);
    }
}

use core::pin::Pin;
use std::{sync::atomic::Ordering, thread, time::Duration};

// TODO: this will change to qobject::RustLockingEnabled once
// https://github.com/KDAB/cxx-qt/issues/559 is done
impl ffi::RustLockingEnabledQt {
    fn get_counter(&self) -> u32 {
        self.counter().load(Ordering::Acquire)
    }

    fn increment(self: Pin<&mut Self>) {
        let counter = self.as_ref().get_counter();
        thread::sleep(Duration::from_millis(100));
        self.counter().store(counter + 1, Ordering::Release);
    }
}

// TODO: this will change to qobject::RustLockingDisabled once
// https://github.com/KDAB/cxx-qt/issues/559 is done
impl ffi::RustLockingDisabledQt {
    fn get_counter(&self) -> u32 {
        self.counter().load(Ordering::Acquire)
    }

    fn increment(self: Pin<&mut Self>) {
        let counter = self.as_ref().get_counter();
        thread::sleep(Duration::from_millis(100));
        self.counter().store(counter + 1, Ordering::Release);
    }
}
