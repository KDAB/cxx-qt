// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx::{type_id, ExternType};
use std::mem::MaybeUninit;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct QDate {
    jd: i64,
}

extern "C" {
    #[link_name = "cxxqt1$qdate$init"]
    fn qdate_init(this: &mut MaybeUninit<QDate>, y: i32, m: i32, d: i32);
    #[link_name = "cxxqt1$qdate$year"]
    fn qdate_year(this: &QDate) -> i32;
    #[link_name = "cxxqt1$qdate$month"]
    fn qdate_month(this: &QDate) -> i32;
    #[link_name = "cxxqt1$qdate$day"]
    fn qdate_day(this: &QDate) -> i32;
    #[link_name = "cxxqt1$qdate$set$date"]
    fn qdate_set_date(this: &mut QDate, y: i32, m: i32, d: i32) -> bool;
}

impl Default for QDate {
    fn default() -> Self {
        Self::new(0, 0, 0)
    }
}

impl QDate {
    pub fn new(y: i32, m: i32, d: i32) -> Self {
        let mut s = MaybeUninit::<QDate>::uninit();

        // Safety:
        //
        // Static checks on the C++ side ensure that QDate has the
        // same binary footprint in C++ and Rust.
        unsafe {
            qdate_init(&mut s, y, m, d);
            s.assume_init()
        }
    }

    pub fn year(&self) -> i32 {
        unsafe { qdate_year(self) }
    }

    pub fn month(&self) -> i32 {
        unsafe { qdate_month(self) }
    }

    pub fn day(&self) -> i32 {
        unsafe { qdate_day(self) }
    }

    pub fn set_date(&mut self, y: i32, m: i32, d: i32) -> bool {
        unsafe { qdate_set_date(self, y, m, d) }
    }
}

// Safety:
//
// Static checks on the C++ side ensure that QDate is trivial.
unsafe impl ExternType for QDate {
    type Id = type_id!("QDate");
    type Kind = cxx::kind::Trivial;
}

impl From<&QDate> for QDate {
    fn from(qdate: &QDate) -> Self {
        *qdate
    }
}
