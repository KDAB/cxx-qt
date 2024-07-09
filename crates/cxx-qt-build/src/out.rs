// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! A module for handling anything output related

#[cfg(windows)]
use std::path::Path;

#[cfg(windows)]
pub(crate) fn write_file(
    path: impl AsRef<Path>,
    contents: impl AsRef<[u8]>,
) -> std::io::Result<()> {
    use std::fs::File;
    use std::io::Write;

    // On Windows we're experiencing some issues with file locking and possibly
    // synchronization, especially when combined with cc::Build
    // So force a synchronization here with sync_all()
    let mut file = File::create(path)?;
    file.write_all(contents.as_ref())?;
    file.sync_all()
}

#[cfg(not(windows))]
pub(crate) use std::fs::write as write_file;
