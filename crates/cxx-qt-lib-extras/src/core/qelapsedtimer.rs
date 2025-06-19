// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Laurent Montel <laurent.montel@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx::{type_id, ExternType};

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib-extras/core/qelapsedtimer.h");
        type QElapsedTimer = crate::QElapsedTimer;

        /// Returns false if the timer has never been started or invalidated by a call to invalidate().
        #[rust_name = "is_valid"]
        fn isValid(self: &QElapsedTimer) -> bool;

        /// Marks this QElapsedTimer object as invalid.
        fn invalidate(self: &mut QElapsedTimer);

        /// Starts this timer. Once started, a timer value can be checked with elapsed() or msecsSinceReference().
        fn start(self: &mut QElapsedTimer);
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        #[doc(hidden)]
        #[rust_name = "qelapsedtimer_restart"]
        fn qelapsedtimerRestart(e: &mut QElapsedTimer) -> i64;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[rust_name = "qelapsedtimer_init_default"]
        fn construct() -> QElapsedTimer;
    }
}

/// The QElapsedTimer struct provides a fast way to calculate elapsed times.
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct QElapsedTimer {
    t1: i64,
    t2: i64,
}

impl Default for QElapsedTimer {
    /// Constructs an invalid QElapsedTimer. A timer becomes valid once it has been started.
    fn default() -> Self {
        ffi::qelapsedtimer_init_default()
    }
}

impl QElapsedTimer {
    /// Restarts the timer and returns the number of milliseconds elapsed since the previous start.
    /// This function is equivalent to obtaining the elapsed time with elapsed() and then starting the timer again with start(),
    /// but it does so in one single operation, avoiding the need to obtain the clock value twice.
    pub fn restart(mut self) -> i64 {
        ffi::qelapsedtimer_restart(&mut self)
    }
}

// Safety:
//
// Static checks on the C++ side ensure that QElapsedTimer is trivial.
unsafe impl ExternType for QElapsedTimer {
    type Id = type_id!("QElapsedTimer");
    type Kind = cxx::kind::Trivial;
}
