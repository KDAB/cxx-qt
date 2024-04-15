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
        .file("src/qcoreapplication.rs")
        .file("src/qdate.rs")
        .file("src/qdatetime.rs")
        .file("src/qguiapplication.rs")
        .file("src/qhash.rs")
        .file("src/qline.rs")
        .file("src/qlinef.rs")
        .file("src/qlist.rs")
        .file("src/qmap.rs")
        .file("src/qmargins.rs")
        .file("src/qmarginsf.rs")
        .file("src/qmetaobjectconnection.rs")
        .file("src/qmodelindex.rs")
        .file("src/qpersistentmodelindex.rs")
        .file("src/qpoint.rs")
        .file("src/qpointf.rs")
        .file("src/qpolygon.rs")
        .file("src/qpolygonf.rs")
        .file("src/qqmlapplicationengine.rs")
        .file("src/qqmlengine.rs")
        .file("src/qrect.rs")
        .file("src/qrectf.rs")
        .file("src/qregion.rs")
        .file("src/qset.rs")
        .file("src/qsize.rs")
        .file("src/qsizef.rs")
        .file("src/qstring.rs")
        .file("src/qstringlist.rs")
        .file("src/qtime.rs")
        .file("src/qtimezone.rs")
        .file("src/qurl.rs")
        .file("src/qvariant.rs")
        .file("src/qvector.rs")
        .file("src/qvector2d.rs")
        .file("src/qvector3d.rs")
        .file("src/qvector4d.rs")
        .with_opts(cxx_qt_lib_headers::build_opts())
        .build();
}
