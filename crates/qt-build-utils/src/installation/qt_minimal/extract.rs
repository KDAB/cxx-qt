// SPDX-FileCopyrightText: 2026 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Ben Ford <ben.ford@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use flate2::read::GzDecoder;
use std::{fs::File, path::Path};
use tar::Archive;

/// Extract archive to same directory as this workspace, not same as path
pub(crate) fn extract_archive(archive_path: &Path, target_path: &Path) -> anyhow::Result<()> {
    let tar_gz = File::open(archive_path)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);

    // Modify destination in unpack here
    archive.unpack(target_path)?;

    Ok(())
}
