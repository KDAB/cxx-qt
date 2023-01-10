// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx_qt_lib::{QList, QString, QStringList};

#[cxx::bridge]
mod qstringlist_cxx {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
        include!("cxx-qt-lib/qstringlist.h");
        type QStringList = cxx_qt_lib::QStringList;
    }

    extern "Rust" {
        fn construct_qstringlist(a: &QString, b: &QString) -> QStringList;
        fn read_qstringlist(l: &QStringList) -> bool;
        fn clone_qstringlist(l: &QStringList) -> QStringList;
    }
}

fn construct_qstringlist(a: &QString, b: &QString) -> QStringList {
    let mut list = QList::<QString>::default();
    list.append_clone(a);
    list.append_clone(b);
    QStringList::from(&list)
}

fn read_qstringlist(l: &QStringList) -> bool {
    let qlist = QList::<QString>::from(l);
    l.contains(&QString::from("https://kdab.com/")) && qlist.len() == 2
}

fn clone_qstringlist(l: &QStringList) -> QStringList {
    l.clone()
}
