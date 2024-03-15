// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Laurent Montel <laurent.montel@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx_qt_lib::QFontMetrics;

#[cxx::bridge]
mod qfontmetrics_cxx {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qfontmetrics.h");

        type QFontMetrics = cxx_qt_lib::QFontMetrics;
    }

    extern "Rust" {
        fn clone_qfontmetrics(f: &QFontMetrics) -> QFontMetrics;
    }
}

fn clone_qfontmetrics(p: &QFontMetrics) -> QFontMetrics {
    p.clone()
}
