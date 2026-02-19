// SPDX-FileCopyrightText: 2026 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Ben Ford <ben.ford@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use reqwest::blocking::Client;
use std::io::Write;
use tempfile::TempDir;

/// Download a file into a temporary directory over HTTP.
/// Currently, has no error handling, and will crash on any errors
pub(crate) fn download_from_url(url: &str, filename: &str, temp_dir: &TempDir, client: &Client) {
    if let Ok(response) = client.get(url).send() {
        let download_path = temp_dir.path().join(filename);
        println!("temp path: {:?}", download_path);
        let mut file = std::fs::File::create(&download_path).expect("Failed to write file");
        let content = response.bytes().expect("Failed to read text");

        file.write_all(&content).expect("Failed to write file");

        println!("Saved image to {}", download_path.to_str().unwrap());
    } else {
        panic!("Failed to download file");
    }
}
