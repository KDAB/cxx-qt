// SPDX-FileCopyrightText: 2026 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Ben Ford <ben.ford@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use reqwest::blocking::Client;
use std::{
    io::{BufWriter, Write},
    path::PathBuf,
};
use tempfile::TempDir;

/// Download a file into a temporary directory over HTTP.
/// Currently, has no error handling, and will crash on any errors.
/// On my machine, runs about twice as long as wget (34 seconds compared to 17)
pub(crate) fn download_from_url(
    url: &str,
    filename: &str,
    temp_dir: &TempDir,
    client: &Client,
) -> anyhow::Result<PathBuf> {
    let response = client.get(url).send()?;
    let download_path = temp_dir.path().join(filename);
    let file = std::fs::File::create(&download_path)?;
    let mut writer = BufWriter::new(file);
    let content = response.bytes()?;

    writer.write_all(&content)?;

    Ok(download_path)
}
