// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx_qt_lib::{QMap, QMapPair_QString_QVariant, QString, QVariant};

#[cxx::bridge]
mod qmap_cxx {
    // ANCHOR: book_qmap
    unsafe extern "C++" {
        include!("cxx-qt-lib/qmap.h");
        type QMap_QString_QVariant = cxx_qt_lib::QMap<cxx_qt_lib::QMapPair_QString_QVariant>;
    }
    // ANCHOR_END: book_qset

    extern "Rust" {
        fn construct_qmap_qstring_qvariant() -> QMap_QString_QVariant;
        fn read_qmap_qstring_qvariant(s: &QMap_QString_QVariant) -> bool;
        fn clone_qmap_qstring_qvariant(s: &QMap_QString_QVariant) -> QMap_QString_QVariant;
    }
}

fn construct_qmap_qstring_qvariant() -> QMap<QMapPair_QString_QVariant> {
    let mut h = QMap::<QMapPair_QString_QVariant>::default();
    h.insert(QString::from("kdab"), QVariant::from(&10));
    h.insert(QString::from("Qt"), QVariant::from(&QString::from("Rust")));
    h
}

fn read_qmap_qstring_qvariant(h: &QMap<QMapPair_QString_QVariant>) -> bool {
    // Check that the iterator works
    if h.iter().count() != 2 || h.iter().count() != 2 {
        return false;
    }

    // Check that that contains and value method works
    let value_kdab = h
        .get_or_default(&QString::from("kdab"))
        .value::<i32>()
        .map_or_else(|| false, |value| value == 10);
    let value_qt = h
        .get_or_default(&QString::from("Qt"))
        .value::<QString>()
        .map_or_else(|| false, |value| value.to_string() == "Rust");

    h.contains(&QString::from("kdab"))
        && value_kdab
        && h.contains(&QString::from("Qt"))
        && value_qt
        && !h.contains(&QString::from("github"))
        && h.len() == 2
}

fn clone_qmap_qstring_qvariant(
    h: &QMap<QMapPair_QString_QVariant>,
) -> QMap<QMapPair_QString_QVariant> {
    h.clone()
}
