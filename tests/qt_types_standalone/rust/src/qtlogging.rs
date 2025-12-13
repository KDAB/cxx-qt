// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Joshua Booth <joshua.n.booth@gmail.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx_qt_lib::{q_info, QString};

#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    extern "Rust" {
        fn log_info(message: &QString) -> QString;
    }
}

fn log_info(message: &QString) -> QString {
    q_info!("Message: {message}");
    QString::from(&format!("{}:{} - Message: {message}", file!(), line!() - 1))
}
