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
    pub w: f64,
    pub h: f64,
}

extern "C" {
    #[link_name = "cxxqt1$qsizef$init"]
    fn qsizef_init(this: &mut MaybeUninit<QSizeF>, w: f64, h: f64);
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
}

// Safety:
//
// Static checks on the C++ side ensure that QSizeF is trivial.
unsafe impl ExternType for QSizeF {
    type Id = type_id!("QSizeF");
    type Kind = cxx::kind::Trivial;
}
