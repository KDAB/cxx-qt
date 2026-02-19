// SPDX-FileCopyrightText: 2026 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Ben Ford <ben.ford@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
/// A Qt manifest.json file, which specifies a set of artifacts needed for installation
pub(crate) struct ParsedQtManifest {
    pub(crate) schema_version: u8,
    pub(crate) artifacts: Vec<ParsedQtArtifact>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
/// Descriptor for a Qt artifact, included download information
pub(crate) struct ParsedQtArtifact {
    pub(crate) version: semver::Version,
    pub(crate) arch: String,
    pub(crate) os: String,
    pub(crate) url: String,
    sha256: String,
    pub(crate) content: Vec<String>,
}

impl ParsedQtArtifact {
    /// Assert that the hashes are the same, from bytes
    pub fn verify(&self, hash: &[u8]) {
        let mut hash_string = String::new();

        for byte in hash {
            let formatted = format!("{:02x}", byte);
            hash_string.push_str(&formatted);
        }

        assert!(self.sha256 == hash_string);
    }
}
