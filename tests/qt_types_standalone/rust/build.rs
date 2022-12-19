// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use cxx_qt_build::CxxQtBuilder;

fn main() {
    let qtbuild = qt_build_utils::QtBuild::new(vec!["Core".to_owned()])
        .expect("Could not find Qt installation");

    // Find the Qt version and tell the Rust compiler
    // this allows us to have conditional Rust code
    println!(
        "cargo:rustc-cfg=qt_version_major=\"{}\"",
        qtbuild.version().major
    );

    let mut builder = CxxQtBuilder::new()
        .file("src/qbytearray.rs")
        .file("src/qcolor.rs")
        .file("src/qdate.rs")
        .file("src/qdatetime.rs")
        .file("src/qhash.rs")
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
        .file("src/qtime.rs")
        .file("src/qurl.rs")
        .file("src/qvariant.rs")
        .file("src/qvector.rs");

    // Qt 5 has a different QList<T>
    if qtbuild.version().major == 5 {
        builder = builder.file("src/qt5list.rs");
    }

    builder.build();
}
