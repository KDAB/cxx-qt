// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

pub mod cpp;
pub mod rust;

use std::{error::Error, path::PathBuf};

fn header_prefix_from_out_dir() -> Result<String, Box<dyn Error>> {
    // This file should be written by cxx-qt-build
    let header_prefix_path = PathBuf::from(std::env::var("OUT_DIR")?)
        // CODECOV_EXCLUDE_START
        .join("cxx-qt-gen")
        .join("include-prefix.txt");
    Ok(std::fs::read_to_string(header_prefix_path)?)
    // CODECOV_EXCLUDE_STOP
}

pub(crate) fn get_header_prefix() -> String {
    header_prefix_from_out_dir().unwrap_or_else(|_err| "cxx-qt-gen".to_owned())
}
