// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Nicolas Fella <nicolas.fella@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx_qt_lib_extras::QLocale;

#[cxx::bridge]
mod qlocale_cxx {
    unsafe extern "C++" {
        include!("cxx-qt-lib-extras/qlocale.h");
        type QLocale = cxx_qt_lib_extras::QLocale;
    }

    extern "Rust" {
        fn construct_qlocale() -> QLocale;
        fn clone_qlocale(l: &QLocale) -> QLocale;
    }
}

fn construct_qlocale() -> QLocale {
    QLocale::default()
}

fn clone_qlocale(l: &QLocale) -> QLocale {
    l.clone()
}
