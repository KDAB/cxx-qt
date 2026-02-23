// SPDX-FileCopyrightText: 2026 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

mod artifact;
mod checksum;
mod download;
mod extract;

use std::fs::DirEntry;
use std::path::{Path, PathBuf};

use crate::installation::qt_minimal::artifact::ParsedQtArtifact;
use crate::{QtBuildError, QtInstallation};

/// A implementation of [QtInstallation] using qtminimal
pub struct QtInstallationQtMinimal {
    path_qt: PathBuf,
    version: semver::Version,
}

impl TryFrom<PathBuf> for QtInstallationQtMinimal {
    type Error = anyhow::Error;

    fn try_from(path_qt: PathBuf) -> Result<Self, Self::Error> {
        // Verify that the expected folders exist
        for folder in ["bin", "include", "lib", "libexec"] {
            if !path_qt.join(folder).exists() {
                return Err(anyhow::anyhow!(
                    "Failed to find {folder} in Qt path: {}",
                    path_qt.display()
                ));
            }
        }

        // Find qtpaths binary
        let Some(qtpaths) = ["bin", "libexec"]
            .into_iter()
            .map(|folder| {
                path_qt
                    .join(folder)
                    .join(crate::QtTool::QtPaths.binary_name())
            })
            .find(|path| path.exists())
        else {
            return Err(anyhow::anyhow!(
                "Failed to find qtpaths in Qt path: {}",
                path_qt.display()
            ));
        };

        // Determine the Qt version from qtpaths
        let version = semver::Version::parse(
            &crate::QtToolQtPaths::from_path_buf(qtpaths)
                .query("QT_VERSION")
                .expect("Could not query qtpaths for QT_VERSION"),
        )
        .expect("Could not parse Qt version");

        Ok(Self { path_qt, version })
    }
}

impl TryFrom<semver::Version> for QtInstallationQtMinimal {
    type Error = anyhow::Error;

    fn try_from(version: semver::Version) -> Result<Self, Self::Error> {
        // Parse all artifacts
        let manifest: artifact::ParsedQtManifest =
            serde_json::from_str(qt_artifacts::QT_MANIFEST_JSON)?;

        // Find artifacts for the Qt version
        let artifacts = Self::match_artifact_requirements(manifest.artifacts, &[version.clone()]);

        // Find the first bin / include
        let artifact_bin = artifacts
            .iter()
            .find(|artifact| artifact.content.contains(&"bin".to_string()))
            .ok_or_else(|| QtBuildError::QtMissing)?;
        let artifact_include = artifacts
            .iter()
            .find(|artifact| artifact.content.contains(&"include".to_string()))
            .ok_or_else(|| QtBuildError::QtMissing)?;

        // Download the artifacts
        let extract_target_dir = Self::qt_minimal_root()
            .join(format!(
                "{}.{}.{}",
                version.major, version.minor, version.patch
            ))
            .join(&artifact_bin.os)
            .join(&artifact_bin.arch);
        artifact_bin.download_and_extract(&extract_target_dir);
        if artifact_bin != artifact_include {
            artifact_include.download_and_extract(&extract_target_dir);
        }

        Self::try_from(extract_target_dir.join("qt"))
    }
}

impl QtInstallation for QtInstallationQtMinimal {
    fn framework_paths(&self, qt_modules: &[String]) -> Vec<std::path::PathBuf> {
        let path_lib = self.path_qt.join("lib");
        super::shared::framework_paths_for_qt_modules(qt_modules, path_lib)
    }

    fn include_paths(&self, qt_modules: &[String]) -> Vec<std::path::PathBuf> {
        let path_include = self.path_qt.join("include");
        let path_lib = self.path_qt.join("lib");
        super::shared::include_paths_for_qt_modules(qt_modules, path_include, path_lib)
    }

    fn link_modules(&self, builder: &mut cc::Build, qt_modules: &[String]) {
        let path_frameworks = self.framework_paths(qt_modules);
        let path_lib = self.path_qt.join("lib");
        let path_prefix = self.path_qt.clone();
        let path_plugins = self.path_qt.join("plugins");
        let qt_version = self.version.clone();
        super::shared::link_for_qt_modules(
            builder,
            qt_modules,
            path_frameworks,
            path_lib,
            path_prefix,
            path_plugins,
            qt_version,
        );
    }

    fn try_find_tool(&self, tool: crate::QtTool) -> anyhow::Result<std::path::PathBuf> {
        // Tools could be either in libexec or bin
        for folder in ["bin", "libexec"] {
            let path = self.path_qt.join(folder).join(tool.binary_name());
            if path.exists() {
                return Ok(path);
            }
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

impl QtInstallationQtMinimal {
    fn qt_minimal_root() -> PathBuf {
        // Check if a custom root has been set
        let path = if let Ok(root) = std::env::var("QT_MINIMAL_DOWNLOAD_ROOT") {
            PathBuf::from(root)
        } else {
            // Otherwise fallback to user data dir
            dirs::data_local_dir()
                .expect("User local data directory to be found")
                .join("qt_minimal_download")
        };

        std::fs::create_dir_all(&path).expect("Could not create Qt minimal root path");

        path
    }

    /// Get a collection of the locally installed Qt artifacts
    pub(crate) fn local_artifacts() -> anyhow::Result<Vec<ParsedQtArtifact>> {
        let base_dir = Self::qt_minimal_root();

        // Expects folder structure like:
        // version/os/arch/qt/{bin, include}
        // e.g. will find an artifact at 6.10.0/linux/x86_64/qt/bin
        let mut artifacts = vec![];

        // Iterate versions
        for version in list_dirs(&base_dir) {
            let path = version;
            // TODO: Later skip unknown folders,
            // this will error if a directory exists which isn't a version number
            let semver = semver::Version::parse(path.file_name().to_str().unwrap())
                .expect("Could not parse semver from directory name");

            for os in list_dirs(&path.path()) {
                let path = os;
                let os = path.file_name().to_str().unwrap().to_string();

                for arch in list_dirs(&path.path()) {
                    let path = arch;
                    let dir_entries = list_dirs(&path.path());

                    // Expects one qt dir
                    let qt_dir_path = dir_entries
                        .iter()
                        .filter(|dir| dir.file_name() == "qt")
                        .last()
                        .expect("Expected to find a Qt dir in this folder");

                    let qt_folders = list_dirs(&qt_dir_path.path());
                    for dir in qt_folders {
                        let filename = dir.file_name();
                        // Will be set if bin or include dirs are found
                        let mut artifact_type = None;

                        if filename == "bin" {
                            artifact_type = Some("bin");
                        } else if filename == "include" {
                            artifact_type = Some("include");
                        }

                        if let Some(artifact_type) = artifact_type {
                            artifacts.push(ParsedQtArtifact::new(
                                semver.clone(),
                                path.file_name().to_string_lossy().to_string(),
                                os.clone(),
                                qt_dir_path.path().to_string_lossy().to_string(),
                                artifact_type.to_string(),
                            ))
                        }
                    }
                }
            }
        }

        Ok(artifacts)
    }

    /// Find artifacts matching Qt version
    pub(crate) fn match_artifact_requirements(
        artifacts: Vec<ParsedQtArtifact>,
        versions: &[semver::Version],
    ) -> Vec<ParsedQtArtifact> {
        // Arch could be x86_64
        // OS could be linux
        // https://doc.rust-lang.org/cargo/appendix/glossary.html#target
        //
        // TODO: is there a better way to find the arch and os ?
        // and should this be configurable via env var overrides?
        let target = std::env::var("TARGET").expect("TARGET to be set");
        let target_parts: Vec<_> = target.split("-").collect();
        let arch = target_parts
            .first()
            .expect("TARGET to have a <arch><sub> component");
        let os = target_parts
            .get(2)
            .expect("TARGET to have a <sys> component");

        artifacts
            .into_iter()
            .filter(|artifact| {
                artifact.arch == *arch && artifact.os == *os && versions.contains(&artifact.version)
            })
            .collect()
    }
}

/// Get all valid directories in path bufs, ignoring errors
fn list_dirs(path: &Path) -> Vec<DirEntry> {
    path.read_dir().unwrap().flatten().collect()
}
