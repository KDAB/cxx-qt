// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx_qt_lib::{QString, QUrl};

#[cxx::bridge]
mod qurl_cxx {
    unsafe extern "C++" {
        include!("cxx-qt-lib/include/qt_types.h");

        type QString = cxx_qt_lib::QString;
        type QUrl = cxx_qt_lib::QUrl;
    }

    extern "Rust" {
        fn construct_qurl(test: &QString) -> QUrl;
        fn read_qurl(u: &QUrl, test: &QString) -> bool;
        fn clone_qurl(u: &QUrl) -> QUrl;
    }
}

fn construct_qurl(test: &QString) -> QUrl {
    QUrl::from(test)
}

fn read_qurl(u: &QUrl, test: &QString) -> bool {
    u.to_string() == test.to_string()
}

fn clone_qurl(u: &QUrl) -> QUrl {
    u.clone()
}
