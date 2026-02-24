// SPDX-FileCopyrightText: 2026 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Ben Ford <ben.ford@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use semver::Version;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize)]
/// A Qt manifest.json file, which specifies a set of artifacts needed for installation
pub(crate) struct ParsedQtManifest {
    pub(crate) schema_version: u8,
    pub(crate) artifacts: Vec<ParsedQtArtifact>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
/// Descriptor for a Qt artifact, included download information
pub(crate) struct ParsedQtArtifact {
    pub(crate) version: Version,
    pub(crate) arch: String,
    pub(crate) os: String,
    pub(crate) url: String,
    sha256: String,
    pub(crate) content: Vec<String>,
}

impl ParsedQtArtifact {
    /// Download the artifact and extract to the given target path
    pub fn download_and_extract(&self, target_path: &Path) -> PathBuf {
        // Download to a temporary location
        let http_client = reqwest::blocking::Client::builder()
            .timeout(None)
            .build()
            .expect("Http client failed to build");
        let temp_dir = tempfile::TempDir::new().expect("Could not create temporary directory");
        let archive_path =
            super::download::download_from_url(&self.url, &self.sha256, &temp_dir, &http_client)
                .expect("Could not download url");

        // Verify the checksum
        self.verify(&super::checksum::hash_file(&archive_path).expect("Could not hash file"))
            .expect("Could not verify sha256 hash");

        // Extract into the target folder
        super::extract::extract_archive(&archive_path, target_path)
            .expect("Could not extract archive into target");

        target_path.to_path_buf()
    }

    /// Used to create a found artifact without a checksum, used by local artifact discovery
    pub fn new(
        version: Version,
        arch: String,
        os: String,
        url: String,
        content_type: String,
    ) -> Self {
        Self {
            version,
            arch,
            os,
            url,
            sha256: "".to_string(),
            content: vec![content_type],
        }
    }

    /// Assert that the hashes are the same, from bytes
    pub fn verify(&self, hash: &[u8]) -> anyhow::Result<()> {
        let mut hash_string = String::new();

        for byte in hash {
            let formatted = format!("{:02x}", byte);
            hash_string.push_str(&formatted);
        }

        if self.sha256 != hash_string {
            return Err(anyhow::anyhow!("sha256 does not match for: {}", &self.url));
        }

        Ok(())
    }
}
