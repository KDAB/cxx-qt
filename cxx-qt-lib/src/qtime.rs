// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx::{type_id, ExternType};
use std::mem::MaybeUninit;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct QTime {
    mds: i32,
}

extern "C" {
    #[link_name = "cxxqt1$qtime$init"]
    fn qtime_init(this: &mut MaybeUninit<QTime>, h: i32, m: i32, s: i32, ms: i32);
    #[link_name = "cxxqt1$qtime$hour"]
    fn qtime_hour(this: &QTime) -> i32;
    #[link_name = "cxxqt1$qtime$minute"]
    fn qtime_minute(this: &QTime) -> i32;
    #[link_name = "cxxqt1$qtime$second"]
    fn qtime_second(this: &QTime) -> i32;
    #[link_name = "cxxqt1$qtime$msec"]
    fn qtime_msec(this: &QTime) -> i32;
    #[link_name = "cxxqt1$qtime$set$hms"]
    fn qtime_set_hms(this: &mut QTime, h: i32, m: i32, s: i32, ms: i32) -> bool;
}

impl Default for QTime {
    fn default() -> Self {
        Self::new(0, 0, 0, 0)
    }
}

impl QTime {
    pub fn new(h: i32, m: i32, s: i32, ms: i32) -> Self {
        let mut t = MaybeUninit::<QTime>::uninit();

        // Safety:
        //
        // Static checks on the C++ side ensure that QTime has the
        // same binary footprint in C++ and Rust.
        unsafe {
            qtime_init(&mut t, h, m, s, ms);
            t.assume_init()
        }
    }

    pub fn hour(&self) -> i32 {
        unsafe { qtime_hour(self) }
    }

    pub fn minute(&self) -> i32 {
        unsafe { qtime_minute(self) }
    }

    pub fn second(&self) -> i32 {
        unsafe { qtime_second(self) }
    }

    pub fn msec(&self) -> i32 {
        unsafe { qtime_msec(self) }
    }

    pub fn set_hms(&mut self, h: i32, m: i32, s: i32, ms: i32) -> bool {
        unsafe { qtime_set_hms(self, h, m, s, ms) }
    }
}

// Safety:
//
// Static checks on the C++ side ensure that QTime is trivial.
unsafe impl ExternType for QTime {
    type Id = type_id!("QTime");
    type Kind = cxx::kind::Trivial;
}

impl From<&QTime> for QTime {
    fn from(qtime: &QTime) -> Self {
        *qtime
    }
}
