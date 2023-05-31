// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

/// Two QObject that allow for testing that locking works
#[cxx_qt::bridge(cxx_file_stem = "locking")]
pub mod ffi {
    use std::{
        sync::atomic::{AtomicU32, Ordering},
        thread,
        time::Duration,
    };

    /// A QObject which has cxx_qt::Locking
    #[cxx_qt::qobject]
    #[derive(Default)]
    pub struct RustLockingEnabled {
        counter: AtomicU32,
    }

    impl qobject::RustLockingEnabled {
        #[qinvokable]
        pub fn get_counter(&self) -> u32 {
            self.counter().load(Ordering::Acquire)
        }

        #[qinvokable]
        pub fn increment(self: Pin<&mut Self>) {
            let counter = self.as_ref().get_counter();
            thread::sleep(Duration::from_millis(100));
            self.counter().store(counter + 1, Ordering::Release);
        }
    }

    /// A QObject which has !cxx_qt::Locking
    #[cxx_qt::qobject]
    #[derive(Default)]
    pub struct RustLockingDisabled {
        counter: AtomicU32,
    }

    unsafe impl !cxx_qt::Locking for qobject::RustLockingDisabled {}

    impl qobject::RustLockingDisabled {
        #[qinvokable]
        pub fn get_counter(&self) -> u32 {
            self.counter().load(Ordering::Acquire)
        }

        #[qinvokable]
        pub fn increment(self: Pin<&mut Self>) {
            let counter = self.as_ref().get_counter();
            thread::sleep(Duration::from_millis(100));
            self.counter().store(counter + 1, Ordering::Release);
        }
    }
}
