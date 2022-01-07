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
}

struct ParsedDate {
    year: i32,
    month: i32,
    day: i32,
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

    pub fn is_null(&self) -> bool {
        !self.is_valid()
    }

    pub fn is_leap_year(mut y: i32) -> bool {
        // No year 0 in Gregorian calendar, so -1, -5, -9 etc are leap years
        if y < 1 {
            y += 1;
        }

        (y % 4 == 0 && y % 100 != 0) || y % 400 == 0
    }

    pub fn is_valid(&self) -> bool {
        (self.jd > -784350574879) && (self.jd < 784354017364)
    }

    pub fn is_valid_date(year: i32, month: i32, day: i32) -> bool {
        // There is no year 0 in the Gregorian calendar.
        year != 0
            && day > 0
            && month > 0
            && month <= 12
            && day
                <= (if month == 2 {
                    if QDate::is_leap_year(year) {
                        29
                    } else {
                        28
                    }
                } else {
                    QDate::days_in_usual_month(month)
                })
    }

    pub fn year(&self) -> i32 {
        if self.is_null() {
            0
        } else {
            QDate::get_date_from_julian_day(self.jd).year
        }
    }

    pub fn month(&self) -> i32 {
        if self.is_null() {
            0
        } else {
            QDate::get_date_from_julian_day(self.jd).month
        }
    }

    pub fn day(&self) -> i32 {
        if self.is_null() {
            0
        } else {
            QDate::get_date_from_julian_day(self.jd).day
        }
    }

    pub fn set_date(&mut self, y: i32, m: i32, d: i32) -> bool {
        if QDate::is_valid_date(y, m, d) {
            self.jd = QDate::julian_day_from_date(y, m, d);
        } else {
            self.jd = i64::MIN;
        }

        self.is_valid()
    }

    fn days_in_usual_month(month: i32) -> i32 {
        // (February isn't usual.)
        assert!(month != 2 && month > 0 && month <= 12);

        // Long if odd up to July = 7, or if even from 8 = August onwards:
        30 | ((month & 1) ^ (month >> 3))
    }

    fn floor_div_i64(a: i64, b: i64) -> i64 {
        (a - (if a < 0 { b - 1 } else { 0 })) / b
    }

    fn floor_div_i32(a: i32, b: i32) -> i32 {
        (a - (if a < 0 { b - 1 } else { 0 })) / b
    }

    fn get_date_from_julian_day(julian_day: i64) -> ParsedDate {
        // Math from The Calendar FAQ at http://www.tondering.dk/claus/cal/julperiod.php
        // This formula is correct for all julian days, when using mathematical integer
        // division (round to negative infinity), not c++11 integer division (round to zero)
        let a: i64 = julian_day + 32044;
        let b: i64 = QDate::floor_div_i64(4 * a + 3, 146097);
        let c: i64 = a - QDate::floor_div_i64(146097 * b, 4);

        let d: i64 = QDate::floor_div_i64(4 * c + 3, 1461);
        let e: i64 = c - QDate::floor_div_i64(1461 * d, 4);
        let m: i64 = QDate::floor_div_i64(5 * e + 2, 153);

        let day: i64 = e - QDate::floor_div_i64(153 * m + 2, 5) + 1;
        let month: i64 = m + 3 - 12 * QDate::floor_div_i64(m, 10);
        let mut year: i64 = 100 * b + d - 4800 + QDate::floor_div_i64(m, 10);

        // Adjust for no year 0
        if year <= 0 {
            year -= 1;
        }

        ParsedDate {
            year: year as i32,
            month: month as i32,
            day: day as i32,
        }
    }

    fn julian_day_from_date(mut year: i32, month: i32, day: i32) -> i64 {
        // Adjust for no year 0
        if year < 0 {
            year += 1;
        }

        // Math from The Calendar FAQ at http://www.tondering.dk/claus/cal/julperiod.php
        // This formula is correct for all julian days, when using mathematical integer
        // division (round to negative infinity), not c++11 integer division (round to zero)

        let a: i32 = QDate::floor_div_i32(14 - month, 12);
        let y: i64 = (year as i64) + 4800 - (a as i64);
        let m: i32 = month + 12 * a - 3;
        (day as i64)
            + QDate::floor_div_i32(153 * m + 2, 5) as i64
            + 365 * y
            + QDate::floor_div_i64(y, 4)
            - QDate::floor_div_i64(y, 100)
            + QDate::floor_div_i64(y, 400)
            - 32045
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
