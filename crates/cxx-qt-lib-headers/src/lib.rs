// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Be Wilson <be.wilson@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! This crate is a hack so build.rs for cxx-qt-lib and cxx-qt-build both have access to cxx-qt-lib's C++ headers.
//! This must be a separate crate from cxx-qt-lib because cxx-qt-lib cannot be a build dependency of cxx-qt-build.
//! Otherwise Cargo links the executable compiled from a build.rs that uses cxx-qt-build to Qt, so running
//! build.rs fails when Qt is linked dynamically if the Qt libraries are not in PATH (Windows)/LD_LIBRARY_PATH (Unix).

use std::{fs::File, io::Write, path::Path};

static HEADERS: [(&str, &str); 21] = [
    (include_str!("../include/common.h"), "common.h"),
    (include_str!("../include/convert.h"), "convert.h"),
    (include_str!("../include/cxxqt_thread.h"), "cxxqt_thread.h"),
    (include_str!("../include/qcolor.h"), "qcolor.h"),
    (include_str!("../include/qdate.h"), "qdate.h"),
    (include_str!("../include/qdatetime.h"), "qdatetime.h"),
    (include_str!("../include/qhash.h"), "qhash.h"),
    (include_str!("../include/qmodelindex.h"), "qmodelindex.h"),
    (include_str!("../include/qpoint.h"), "qpoint.h"),
    (include_str!("../include/qpointf.h"), "qpointf.h"),
    (include_str!("../include/qrect.h"), "qrect.h"),
    (include_str!("../include/qrectf.h"), "qrectf.h"),
    (include_str!("../include/qset.h"), "qset.h"),
    (include_str!("../include/qsize.h"), "qsize.h"),
    (include_str!("../include/qsizef.h"), "qsizef.h"),
    (include_str!("../include/qstring.h"), "qstring.h"),
    (include_str!("../include/qtime.h"), "qtime.h"),
    (include_str!("../include/qurl.h"), "qurl.h"),
    (include_str!("../include/qvariant.h"), "qvariant.h"),
    (include_str!("../include/qvector.h"), "qvector.h"),
    (include_str!("../include/std_types.h"), "std_types.h"),
];

/// Write the cxx-qt-lib headers to the specified directory.
pub fn write_headers(directory: impl AsRef<Path>) {
    let directory = directory.as_ref();
    std::fs::create_dir_all(directory).expect("Could not create cxx-qt-lib header directory");
    for (file_contents, file_name) in HEADERS {
        let h_path = format!("{}/{}", directory.display(), file_name);
        let mut header = File::create(h_path).expect("Could not create cxx-qt-lib header");
        write!(header, "{}", file_contents).expect("Could not write cxx-qt-lib header");
    }
}
