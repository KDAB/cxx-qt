// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx::{type_id, ExternType};
use std::mem::MaybeUninit;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct QPointF {
    pub x: f64,
    pub y: f64,
}

extern "C" {
    #[link_name = "cxxqt1$qpointf$init"]
    fn qpointf_init(this: &mut MaybeUninit<QPointF>, x: f64, y: f64);
}

impl QPointF {
    pub fn new(x: f64, y: f64) -> Self {
        let mut s = MaybeUninit::<QPointF>::uninit();

        // Safety:
        //
        // Static checks on the C++ side ensure that QPointF has the
        // same binary footprint in C++ and Rust.
        unsafe {
            qpointf_init(&mut s, x, y);
            s.assume_init()
        }
    }
}

// Safety:
//
// Static checks on the C++ side ensure that QPointF is trivial.
unsafe impl ExternType for QPointF {
    type Id = type_id!("QPointF");
    type Kind = cxx::kind::Trivial;
}
