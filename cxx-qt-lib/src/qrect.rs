// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx::{type_id, ExternType};
use std::mem::MaybeUninit;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct QRect {
    // Note that Qt stores QRect as two points rather than a point and size (which QRectF is)
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

extern "C" {
    #[link_name = "cxxqt1$qrect$init"]
    fn qrect_init(this: &mut MaybeUninit<QRect>, xp: i32, yp: i32, w: i32, h: i32);
}

impl Default for QRect {
    fn default() -> Self {
        Self::new(0, 0, 0, 0)
    }
}

impl QRect {
    pub fn new(xp: i32, yp: i32, w: i32, h: i32) -> Self {
        let mut s = MaybeUninit::<QRect>::uninit();

        // Safety:
        //
        // Static checks on the C++ side ensure that QRect has the
        // same binary footprint in C++ and Rust.
        unsafe {
            qrect_init(&mut s, xp, yp, w, h);
            s.assume_init()
        }
    }

    pub fn height(&self) -> i32 {
        self.y2 - self.y1 + 1
    }

    pub fn set_height(&mut self, h: i32) {
        self.y2 = self.y1 + h - 1;
    }

    pub fn set_width(&mut self, w: i32) {
        self.x2 = self.x1 + w - 1;
    }

    pub fn set_x(&mut self, x: i32) {
        self.x1 = x;
    }

    pub fn set_y(&mut self, y: i32) {
        self.y1 = y;
    }

    pub fn width(&self) -> i32 {
        self.x2 - self.x1 + 1
    }

    pub fn x(&self) -> i32 {
        self.x1
    }

    pub fn y(&self) -> i32 {
        self.y1
    }
}

// Safety:
//
// Static checks on the C++ side ensure that QRect is trivial.
unsafe impl ExternType for QRect {
    type Id = type_id!("QRect");
    type Kind = cxx::kind::Trivial;
}

impl From<&QRect> for QRect {
    fn from(qrect: &QRect) -> Self {
        *qrect
    }
}
