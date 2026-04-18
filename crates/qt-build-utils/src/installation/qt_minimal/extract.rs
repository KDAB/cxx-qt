// SPDX-FileCopyrightText: 2026 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Ben Ford <ben.ford@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use flate2::read::GzDecoder;
use std::{fs::File, path::Path};
use tar::Archive;
use zip::ZipArchive;

pub(crate) enum ArchiveFormat {
    TarGz,
    Zip,
}

/// Extract archive to same directory as this workspace, not same as path
pub(crate) fn extract_archive(
    archive_path: &Path,
    target_path: &Path,
    archive_format: ArchiveFormat,
) -> anyhow::Result<()> {
    let file = File::open(archive_path)?;

    match archive_format {
        ArchiveFormat::TarGz => {
            let gz_decoder = GzDecoder::new(file);
            let mut archive = Archive::new(gz_decoder);

            // Modify destination in unpack here
            archive.unpack(target_path)?;
        }
        ArchiveFormat::Zip => {
            let mut archive = ZipArchive::new(file)?;

            // Modify destination in unpack here
            archive.extract(target_path)?;
        }
    }

    Ok(())
}
