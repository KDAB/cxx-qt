// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx::{type_id, ExternType};
use std::mem::MaybeUninit;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct QPoint {
    x: i32,
    y: i32,
}

extern "C" {
    #[link_name = "cxxqt1$qpoint$init"]
    fn qpoint_init(this: &mut MaybeUninit<QPoint>, x: i32, y: i32);
}

impl Default for QPoint {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

impl QPoint {
    pub fn new(x: i32, y: i32) -> Self {
        let mut s = MaybeUninit::<QPoint>::uninit();

        // Safety:
        //
        // Static checks on the C++ side ensure that QPointF has the
        // same binary footprint in C++ and Rust.
        unsafe {
            qpoint_init(&mut s, x, y);
            s.assume_init()
        }
    }

    pub fn set_x(&mut self, x: i32) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: i32) {
        self.y = y;
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }
}

// Safety:
//
// Static checks on the C++ side ensure that QPoint is trivial.
unsafe impl ExternType for QPoint {
    type Id = type_id!("QPoint");
    type Kind = cxx::kind::Trivial;
}

impl From<&QPoint> for QPoint {
    fn from(qpoint: &QPoint) -> Self {
        *qpoint
    }
}
