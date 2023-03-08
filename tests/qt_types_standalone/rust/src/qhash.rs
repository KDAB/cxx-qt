// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx_qt_lib::{QHash, QHashPair_QString_QVariant, QString, QVariant};

#[cxx::bridge]
mod qhash_cxx {
    // ANCHOR: book_qhash
    unsafe extern "C++" {
        include!("cxx-qt-lib/qhash.h");
        type QHash_QString_QVariant = cxx_qt_lib::QHash<cxx_qt_lib::QHashPair_QString_QVariant>;
    }
    // ANCHOR_END: book_qhash

    extern "Rust" {
        fn construct_qhash_qstring_qvariant() -> QHash_QString_QVariant;
        fn read_qhash_qstring_qvariant(s: &QHash_QString_QVariant) -> bool;
        fn clone_qhash_qstring_qvariant(s: &QHash_QString_QVariant) -> QHash_QString_QVariant;
    }
}

fn construct_qhash_qstring_qvariant() -> QHash<QHashPair_QString_QVariant> {
    let mut h = QHash::<QHashPair_QString_QVariant>::default();
    h.insert(QString::from("kdab"), QVariant::from(&10));
    h.insert(QString::from("Qt"), QVariant::from(&QString::from("Rust")));
    h
}

fn read_qhash_qstring_qvariant(h: &QHash<QHashPair_QString_QVariant>) -> bool {
    // Check that the iterator works
    if h.iter().count() != 2 || h.iter().count() != 2 {
        return false;
    }

    // Check that value method works
    let value_kdab = match h.get_or_default(&QString::from("kdab")).value::<i32>() {
        Some(value) => value == 10,
        None => false,
    };
    let value_qt = match h.get_or_default(&QString::from("Qt")).value::<QString>() {
        Some(value) => value.to_string() == "Rust",
        _ => false,
    };

    // Check that contains method works
    h.contains(&QString::from("kdab"))
        && value_kdab
        && h.contains(&QString::from("Qt"))
        && value_qt
        && !h.contains(&QString::from("github"))
        && h.len() == 2
}

fn clone_qhash_qstring_qvariant(
    h: &QHash<QHashPair_QString_QVariant>,
) -> QHash<QHashPair_QString_QVariant> {
    h.clone()
}
