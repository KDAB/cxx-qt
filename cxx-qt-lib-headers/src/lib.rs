// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Be Wilson <be.wilson@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! This crate is a hack so build.rs for cxx-qt-lib and cxx-qt-build both have access to cxx-qt-lib's C++ headers.
//! This must be a separate crate from cxx-qt-lib because cxx-qt-lib cannot be a build dependency of cxx-qt-build.
//! Otherwise Cargo links the executable compiled from a build.rs that uses cxx-qt-build to Qt, so running
//! build.rs fails when Qt is linked dynamically if the Qt libraries are not in PATH (Windows)/LD_LIBRARY_PATH (Unix).

use std::{fs::File, io::Write, path::Path};

static HEADERS: [(&str, &str); 3] = [
    (include_str!("../include/convert.h"), "convert.h"),
    (include_str!("../include/qt_types.h"), "qt_types.h"),
    (
        include_str!("../include/update_requester.h"),
        "update_requester.h",
    ),
];

/// Write the cxx-qt-lib headers to the specified directory.
pub fn write_headers(directory: &impl AsRef<Path>) {
    let directory = directory.as_ref();
    std::fs::create_dir_all(directory).expect("Could not create cxx-qt-lib header directory");
    for (file_contents, file_name) in HEADERS {
        let h_path = format!("{}/{}", directory.display(), file_name);
        let mut header = File::create(&h_path).expect("Could not create cxx-qt-lib header");
        write!(header, "{}", file_contents).expect("Could not write cxx-qt-lib header");
    }
}
