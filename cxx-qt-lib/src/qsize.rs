// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx::{type_id, ExternType};
use std::mem::MaybeUninit;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct QSize {
    w: i32,
    h: i32,
}

extern "C" {
    #[link_name = "cxxqt1$qsize$init"]
    fn qsize_init(this: &mut MaybeUninit<QSize>, w: i32, h: i32);
}

impl Default for QSize {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

impl QSize {
    pub fn new(w: i32, h: i32) -> Self {
        let mut s = MaybeUninit::<QSize>::uninit();

        // Safety:
        //
        // Static checks on the C++ side ensure that QSize has the
        // same binary footprint in C++ and Rust.
        unsafe {
            qsize_init(&mut s, w, h);
            s.assume_init()
        }
    }

    pub fn height(&self) -> i32 {
        self.h
    }

    pub fn set_height(&mut self, h: i32) {
        self.h = h;
    }

    pub fn set_width(&mut self, w: i32) {
        self.w = w;
    }

    pub fn width(&self) -> i32 {
        self.w
    }
}

// Safety:
//
// Static checks on the C++ side ensure that QSize is trivial.
unsafe impl ExternType for QSize {
    type Id = type_id!("QSize");
    type Kind = cxx::kind::Trivial;
}

impl From<&QSize> for QSize {
    fn from(qsize: &QSize) -> Self {
        *qsize
    }
}
