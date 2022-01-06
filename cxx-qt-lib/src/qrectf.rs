// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx::{type_id, ExternType};
use std::mem::MaybeUninit;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct QRectF {
    xp: f64,
    yp: f64,
    w: f64,
    h: f64,
}

extern "C" {
    #[link_name = "cxxqt1$qrectf$init"]
    fn qrectf_init(this: &mut MaybeUninit<QRectF>, xp: f64, yp: f64, w: f64, h: f64);
}

impl Default for QRectF {
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0)
    }
}

impl QRectF {
    pub fn new(xp: f64, yp: f64, w: f64, h: f64) -> Self {
        let mut s = MaybeUninit::<QRectF>::uninit();

        // Safety:
        //
        // Static checks on the C++ side ensure that QRectF has the
        // same binary footprint in C++ and Rust.
        unsafe {
            qrectf_init(&mut s, xp, yp, w, h);
            s.assume_init()
        }
    }

    pub fn height(&self) -> f64 {
        self.h
    }

    pub fn set_height(&mut self, h: f64) {
        self.h = h;
    }

    pub fn set_width(&mut self, w: f64) {
        self.w = w;
    }

    pub fn set_x(&mut self, x: f64) {
        self.xp = x;
    }

    pub fn set_y(&mut self, y: f64) {
        self.yp = y;
    }

    pub fn width(&self) -> f64 {
        self.w
    }

    pub fn x(&self) -> f64 {
        self.xp
    }

    pub fn y(&self) -> f64 {
        self.yp
    }
}

// Safety:
//
// Static checks on the C++ side ensure that QRectF is trivial.
unsafe impl ExternType for QRectF {
    type Id = type_id!("QRectF");
    type Kind = cxx::kind::Trivial;
}

impl From<&QRectF> for QRectF {
    fn from(qrectf: &QRectF) -> Self {
        *qrectf
    }
}
