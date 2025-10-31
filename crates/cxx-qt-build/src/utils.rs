// SPDX-FileCopyrightText: CXX Authors
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: David Tolnay <dtolnay@gmail.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

// Modified from
// https://github.com/dtolnay/cxx/blob/0608b11f31c40d6ca11abbf51395f16c4c16ad5e/gen/build/src/lib.rs#L426

use std::{ffi::OsStr, fs, path::Path};

pub(crate) fn best_effort_copy_headers(
    src: &Path,
    dst: &Path,
    max_depth: usize,
    emit_rerun_if_changed: bool,
) {
    let mut dst_created = false;
    let Ok(mut entries) = fs::read_dir(src) else {
        return;
    };
    // Ensure that we rebuild if there are files added/removed
    if emit_rerun_if_changed {
        println!("cargo::rerun-if-changed={}", src.display());
    }

    while let Some(Ok(entry)) = entries.next() {
        let file_name = entry.file_name();
        if file_name.as_encoded_bytes().starts_with(b".") {
            continue;
        }
        match entry.file_type() {
            Ok(file_type) if file_type.is_dir() && max_depth > 0 => {
                let src = entry.path();
                if src.join("Cargo.toml").exists() || src.join("CACHEDIR.TAG").exists() {
                    continue;
                }
                let dst = dst.join(file_name);
                best_effort_copy_headers(&src, &dst, max_depth - 1, emit_rerun_if_changed);
            }
            Ok(file_type) if file_type.is_file() => {
                let src = entry.path();
                match src.extension().and_then(OsStr::to_str) {
                    Some("h" | "hh" | "hpp") => {}
                    _ => continue,
                }
                if !dst_created && fs::create_dir_all(dst).is_err() {
                    return;
                }

                // Ensure that we rebuild if there are changes
                if emit_rerun_if_changed {
                    println!("cargo::rerun-if-changed={}", src.display());
                }

                dst_created = true;
                let dst = dst.join(file_name);
                let _ = fs::remove_file(&dst);
                let _ = fs::copy(src, dst);
            }
            _ => {}
        }
    }
}
