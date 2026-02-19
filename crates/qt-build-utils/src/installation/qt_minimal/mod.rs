// SPDX-FileCopyrightText: 2026 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

mod artifact;
mod checksum;
mod download;
mod extract;

use std::path::PathBuf;

use crate::QtInstallation;

/// A implementation of [QtInstallation] using qtminimal
pub struct QtInstallationQtMinimal {
    path_qt: PathBuf,
    version: semver::Version,
}

impl TryFrom<semver::Version> for QtInstallationQtMinimal {
    type Error = anyhow::Error;

    fn try_from(version: semver::Version) -> Result<Self, Self::Error> {
        // Parse all artifacts
        let manifest: artifact::ParsedQtManifest =
            serde_json::from_str(qt_artifacts::QT_MANIFEST_JSON)?;

        // Find artifacts matching Qt version
        let artifacts: Vec<artifact::ParsedQtArtifact> = manifest
            .artifacts
            .into_iter()
            // TODO: check OS and arch and likely make a function on the manifest struct
            .filter(|artifact| artifact.version == version)
            .collect();

        // Find the first bin / include
        let artifact_bin = artifacts
            .iter()
            .find(|artifact| artifact.content.contains(&"bin".to_string()))
            .expect("At least one artifact to have a bin folder");
        let artifact_include = artifacts
            .iter()
            .find(|artifact| artifact.content.contains(&"include".to_string()))
            .expect("At least one artifact to have an include folder");

        // Download the artifacts
        let extract_target_dir = PathBuf::from(".").canonicalize()?;
        artifact_bin.download_and_extract(&extract_target_dir)?;
        if artifact_bin != artifact_include {
            artifact_include.download_and_extract(&extract_target_dir)?;
        }

        Ok(Self {
            path_qt: extract_target_dir.join("qt"),
            version,
        })
    }
}

impl QtInstallation for QtInstallationQtMinimal {
    fn framework_paths(&self, _qt_modules: &[String]) -> Vec<std::path::PathBuf> {
        // TODO: macos support
        vec![]
    }

    fn include_paths(&self, _qt_modules: &[String]) -> Vec<std::path::PathBuf> {
        let mut paths = vec![];
        let root_path = self.path_qt.join("include");
        paths.push(root_path);

        // TODO: loop over qt modules

        paths
    }

    fn link_modules(&self, _builder: &mut cc::Build, _qt_modules: &[String]) {
        let lib_path = self.path_qt.join("lib");
        println!("cargo::rustc-link-search={}", lib_path.display());

        // TODO: loop over qt modules
    }

    fn try_find_tool(&self, tool: crate::QtTool) -> anyhow::Result<std::path::PathBuf> {
        // Tools could be either in libexec or bin
        let path_bin = self.path_qt.join("bin").join(tool.binary_name());
        let path_libexec = self.path_qt.join("libexec").join(tool.binary_name());

        if path_bin.exists() {
            return Ok(path_bin);
        } else if path_libexec.exists() {
            return Ok(path_libexec);
        }

        Err(anyhow::anyhow!(
            "Failed to find {} in bin/ or libexec/ under {}",
            tool.binary_name(),
            self.path_qt.display()
        ))
    }

    fn version(&self) -> semver::Version {
        self.version.clone()
    }
}
