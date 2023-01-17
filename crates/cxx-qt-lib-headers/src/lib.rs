// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Be Wilson <be.wilson@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! This crate is a hack so build.rs for cxx-qt-lib and cxx-qt-build both have access to cxx-qt-lib's C++ headers.
//! This must be a separate crate from cxx-qt-lib because cxx-qt-lib cannot be a build dependency of cxx-qt-build.
//! Otherwise Cargo links the executable compiled from a build.rs that uses cxx-qt-build to Qt, so running
//! build.rs fails when Qt is linked dynamically if the Qt libraries are not in PATH (Windows)/LD_LIBRARY_PATH (Unix).

use std::{fs::File, io::Write, path::Path};

/// Write the cxx-qt-lib headers to the specified directory.
pub fn write_headers(directory: impl AsRef<Path>) {
    let directory = directory.as_ref();
    std::fs::create_dir_all(directory).expect("Could not create cxx-qt-lib header directory");
    for (file_contents, file_name) in [
        (include_str!("../include/core/qbytearray.h"), "qbytearray.h"),
        (
            include_str!("../include/core/qcoreapplication.h"),
            "qcoreapplication.h",
        ),
        (include_str!("../include/core/qdate.h"), "qdate.h"),
        (include_str!("../include/core/qdatetime.h"), "qdatetime.h"),
        (include_str!("../include/core/qhash.h"), "qhash.h"),
        (include_str!("../include/core/qlist.h"), "qlist.h"),
        (
            include_str!("../include/core/qlist_qvector.h"),
            "qlist_qvector.h",
        ),
        (include_str!("../include/core/qmap.h"), "qmap.h"),
        (include_str!("../include/core/qmargins.h"), "qmargins.h"),
        (include_str!("../include/core/qmarginsf.h"), "qmarginsf.h"),
        (
            include_str!("../include/core/qmodelindex.h"),
            "qmodelindex.h",
        ),
        (
            include_str!("../include/core/qpersistentmodelindex.h"),
            "qpersistentmodelindex.h",
        ),
        (include_str!("../include/core/qpoint.h"), "qpoint.h"),
        (include_str!("../include/core/qpointf.h"), "qpointf.h"),
        (include_str!("../include/core/qrect.h"), "qrect.h"),
        (include_str!("../include/core/qrectf.h"), "qrectf.h"),
        (include_str!("../include/core/qset.h"), "qset.h"),
        (include_str!("../include/core/qsize.h"), "qsize.h"),
        (include_str!("../include/core/qsizef.h"), "qsizef.h"),
        (include_str!("../include/core/qstring.h"), "qstring.h"),
        (
            include_str!("../include/core/qstringlist.h"),
            "qstringlist.h",
        ),
        (include_str!("../include/core/qt.h"), "qt.h"),
        (include_str!("../include/core/qtime.h"), "qtime.h"),
        (include_str!("../include/core/qurl.h"), "qurl.h"),
        (include_str!("../include/core/qvariant.h"), "qvariant.h"),
        (include_str!("../include/core/qvector.h"), "qvector.h"),
        #[cfg(feature = "qt_gui")]
        (include_str!("../include/gui/qcolor.h"), "qcolor.h"),
        #[cfg(feature = "qt_gui")]
        (
            include_str!("../include/gui/qguiapplication.h"),
            "qguiapplication.h",
        ),
        #[cfg(feature = "qt_gui")]
        (include_str!("../include/gui/qvector2d.h"), "qvector2d.h"),
        #[cfg(feature = "qt_gui")]
        (include_str!("../include/gui/qvector3d.h"), "qvector3d.h"),
        #[cfg(feature = "qt_gui")]
        (include_str!("../include/gui/qvector4d.h"), "qvector4d.h"),
        #[cfg(feature = "qt_qml")]
        (
            include_str!("../include/qml/qqmlapplicationengine.h"),
            "qqmlapplicationengine.h",
        ),
        #[cfg(feature = "qt_qml")]
        (include_str!("../include/qml/qqmlengine.h"), "qqmlengine.h"),
        (include_str!("../include/common.h"), "common.h"),
        (include_str!("../include/convert.h"), "convert.h"),
        (include_str!("../include/cxxqt_thread.h"), "cxxqt_thread.h"),
        (include_str!("../include/std_types.h"), "std_types.h"),
    ] {
        let h_path = format!("{}/{file_name}", directory.display());
        let mut header = File::create(h_path).expect("Could not create cxx-qt-lib header");
        write!(header, "{file_contents}").expect("Could not write cxx-qt-lib header");
    }
}
