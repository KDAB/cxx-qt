// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx::{type_id, ExternType};
use std::mem::MaybeUninit;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct QSizeF {
    w: f64,
    h: f64,
}

extern "C" {
    #[link_name = "cxxqt1$qsizef$init"]
    fn qsizef_init(this: &mut MaybeUninit<QSizeF>, w: f64, h: f64);
}

impl Default for QSizeF {
    fn default() -> Self {
        Self::new(0.0, 0.0)
    }
}

impl QSizeF {
    pub fn new(w: f64, h: f64) -> Self {
        let mut s = MaybeUninit::<QSizeF>::uninit();

        // Safety:
        //
        // Static checks on the C++ side ensure that QSizeF has the
        // same binary footprint in C++ and Rust.
        unsafe {
            qsizef_init(&mut s, w, h);
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

    pub fn width(&self) -> f64 {
        self.w
    }
}

// Safety:
//
// Static checks on the C++ side ensure that QSizeF is trivial.
unsafe impl ExternType for QSizeF {
    type Id = type_id!("QSizeF");
    type Kind = cxx::kind::Trivial;
}

impl From<&QSizeF> for QSizeF {
    fn from(sizef: &QSizeF) -> Self {
        *sizef
    }
}
