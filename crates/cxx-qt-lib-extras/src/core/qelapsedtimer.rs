// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Laurent Montel <laurent.montel@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx::{type_id, ExternType};

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib-extras/qelapsedtimer.h");
        type QElapsedTimer = crate::QElapsedTimer;

        /// Returns `false` if the timer has never been started or invalidated by a call to [`invalidate`](Self::invalidate).
        #[rust_name = "is_valid"]
        fn isValid(self: &QElapsedTimer) -> bool;

        /// Marks this `QElapsedTimer` object as invalid.
        ///
        /// An invalid object can be checked with [`is_valid`](Self::is_valid). Calculations of timer elapsed since invalid data are undefined and will likely produce bizarre results.
        fn invalidate(self: &mut QElapsedTimer);

        /// Starts this timer. Once started, a timer value can be checked with [elapsed](https://doc.qt.io/qt/qelapsedtimer.html#elapsed)() or [msecsSinceReference](https://doc.qt.io/qt/qelapsedtimer.html#msecsSinceReference)().
        ///
        /// Also, starting a timer makes it valid again.
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

/// The `QElapsedTimer` class provides a fast way to calculate elapsed times.
///
/// Qt Documentation: [QElapsedTimer](https://doc.qt.io/qt/qelapsedtimer.html#details)
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct QElapsedTimer {
    t1: i64,
    t2: i64,
}

impl Default for QElapsedTimer {
    /// Constructs an invalid `QElapsedTimer`. A timer becomes valid once it has been started.
    fn default() -> Self {
        ffi::qelapsedtimer_init_default()
    }
}

impl QElapsedTimer {
    /// Restarts the timer and returns the number of milliseconds elapsed since the previous start.
    /// This function is equivalent to obtaining the elapsed time with [elapsed](https://doc.qt.io/qt/qelapsedtimer.html#elapsed)() and then starting the timer again with [`start`](Self::start),
    /// but it does so in one single operation, avoiding the need to obtain the clock value twice.
    ///
    /// Calling this function on a `QElapsedTimer` that is invalid results in undefined behavior.
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
