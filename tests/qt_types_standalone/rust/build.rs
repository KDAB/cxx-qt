// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use cxx_qt_build::CxxQtBuilder;

fn main() {
    CxxQtBuilder::new()
        .file("src/qbytearray.rs")
        .file("src/qcolor.rs")
        .file("src/qdate.rs")
        .file("src/qdatetime.rs")
        .file("src/qhash.rs")
        .file("src/qlist.rs")
        .file("src/qmap.rs")
        .file("src/qmodelindex.rs")
        .file("src/qpersistentmodelindex.rs")
        .file("src/qpoint.rs")
        .file("src/qpointf.rs")
        .file("src/qrect.rs")
        .file("src/qrectf.rs")
        .file("src/qset.rs")
        .file("src/qsize.rs")
        .file("src/qsizef.rs")
        .file("src/qstring.rs")
        .file("src/qstringlist.rs")
        .file("src/qtime.rs")
        .file("src/qurl.rs")
        .file("src/qvariant.rs")
        .file("src/qvector.rs")
        .file("src/qvector2d.rs")
        .file("src/qvector3d.rs")
        .file("src/qvector4d.rs")
        .build();
}
