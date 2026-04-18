// SPDX-FileCopyrightText: 2026 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

mod artifact;
mod checksum;
mod download;
mod extract;

use std::collections::HashMap;
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
        println!("cargo::rerun-if-changed={}", path_qt.display());

        // Verify that the expected folders exist
        //
        // NOTE: libexec does not exist on Windows, so is not mandatory
        for folder in ["bin", "include", "lib"] {
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
            .find(|path| {
                if path.exists() {
                    return true;
                }

                // NOTE: try with .exe for Windows
                let path_exe = path.with_extension("exe");
                if path_exe.exists() {
                    return true;
                }

                false
            })
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

        let link_location = Self::symlink_install_location()?;
        let linked_path = link_location.join(path_qt.strip_prefix(Self::qt_minimal_root())?);
        Ok(Self {
            path_qt: linked_path.to_path_buf(),
            version,
        })
    }
}

impl TryFrom<semver::Version> for QtInstallationQtMinimal {
    type Error = anyhow::Error;

    fn try_from(version: semver::Version) -> Result<Self, Self::Error> {
        // Read local artifacts and filter to requested version
        let local_artifacts = Self::local_artifacts()?;
        let local_matches_grouped = Self::group_artifacts(Self::match_artifact_requirements(
            local_artifacts,
            core::slice::from_ref(&version),
        ));

        // If there is a local artifact containing both bin and include, use that path and skip download
        if let Some(artifact) = local_matches_grouped.first() {
            if let Ok(qt_installation) = Self::try_from(Path::new(&artifact.url).to_path_buf()) {
                return Ok(qt_installation);
            }
        }

        // Parse all remote artifacts
        let manifest: artifact::ParsedQtManifest =
            serde_json::from_str(qt_artifacts::QT_MANIFEST_JSON)?;

        // Find remote artifacts for the requested Qt version
        let artifacts =
            Self::match_artifact_requirements(manifest.artifacts, core::slice::from_ref(&version));

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
    fn framework_paths(&self, qt_modules: &[String]) -> Vec<PathBuf> {
        let path_lib = self.path_qt.join("lib");
        super::shared::framework_paths_for_qt_modules(qt_modules, path_lib)
    }

    fn include_paths(&self, qt_modules: &[String]) -> Vec<PathBuf> {
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

    fn try_find_tool(&self, tool: crate::QtTool) -> anyhow::Result<PathBuf> {
        // Tools could be either in libexec or bin
        for folder in ["bin", "libexec"] {
            let path = self.path_qt.join(folder).join(tool.binary_name());
            if path.exists() {
                return Ok(path);
            }

            // NOTE: try with .exe for Windows
            let path_exe = path.with_extension("exe");
            if path_exe.exists() {
                return Ok(path_exe);
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
        println!("cargo::rerun-if-env-changed=QT_MINIMAL_DOWNLOAD_ROOT");
        let path = if let Ok(root) = std::env::var("QT_MINIMAL_DOWNLOAD_ROOT") {
            PathBuf::from(root)
        } else {
            // Otherwise fallback to user data dir
            dirs::data_local_dir()
                .expect("User local data directory to be found")
                .join("qt_minimal_download")
        };

        // Only create when it doesn't exist to avoid file modifications
        if !path.exists() {
            std::fs::create_dir_all(&path).expect("Could not create Qt minimal root path");
        }

        path
    }

    /// Make sure a local install folder is linked to the build directory
    pub fn symlink_install_location() -> anyhow::Result<PathBuf> {
        let out_dir = std::env::var("OUT_DIR")?;
        let qt_location = QtInstallationQtMinimal::qt_minimal_root();

        let new_location = Path::new(&out_dir).join("qt_minimal_root");

        println!("cargo::rerun-if-changed={}", new_location.display());
        if let Ok(existing_link) = new_location.read_link() {
            if existing_link == qt_location {
                return Ok(new_location);
            }
            #[cfg(unix)]
            {
                std::fs::remove_file(&new_location)?;
            }
            // On windows symlink will be a directory as opposed to a file
            #[cfg(windows)]
            {
                std::fs::remove_dir(&new_location)?;
            }
        }
        // Different symlinking calls per OS
        #[cfg(unix)]
        {
            std::os::unix::fs::symlink(qt_location, &new_location)?;
        }
        #[cfg(windows)]
        {
            std::os::windows::fs::symlink_dir(qt_location, &new_location)?;
        }
        #[cfg(not(any(unix, windows)))]
        panic!(
            "Unknown platform, cannot create symlink to OUT_DIR from {}",
            qt_location.display()
        );
        Ok(new_location)
    }

    /// Get a collection of the locally installed Qt artifacts
    pub(crate) fn local_artifacts() -> anyhow::Result<Vec<ParsedQtArtifact>> {
        let base_dir = Self::qt_minimal_root();
        println!("cargo::rerun-if-changed={}", base_dir.display());

        // Expects folder structure like:
        // version/os/arch/qt/{bin, include}
        // e.g. will find an artifact at 6.10.0/linux/x86_64/qt/bin
        let mut artifacts = vec![];

        // Iterate versions
        for version in list_dirs(&base_dir) {
            println!("cargo::rerun-if-changed={}", version.path().display());

            let path = version;
            // TODO: Later skip unknown folders,
            // this will error if a directory exists which isn't a version number
            let semver = semver::Version::parse(path.file_name().to_str().unwrap())
                .expect("Could not parse semver from directory name");

            for os in list_dirs(&path.path()) {
                println!("cargo::rerun-if-changed={}", os.path().display());

                let path = os;
                let os = path.file_name().to_str().unwrap().to_string();

                for arch in list_dirs(&path.path()) {
                    println!("cargo::rerun-if-changed={}", arch.path().display());

                    let path = arch;
                    let dir_entries = list_dirs(&path.path());

                    // Expects one qt dir
                    let qt_dir_path = dir_entries
                        .iter()
                        .rfind(|dir| dir.file_name() == "qt")
                        .expect("Expected to find a Qt directory");
                    println!("cargo::rerun-if-changed={}", qt_dir_path.path().display());

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
        // Map from the TARGET to an arch and OS pair for the artifacts for supported Qt platforms
        // https://doc.rust-lang.org/cargo/appendix/glossary.html#target
        // https://doc.rust-lang.org/stable/rustc/platform-support.html
        // https://doc.qt.io/qt-6/supported-platforms.html
        println!("cargo::rerun-if-env-changed=TARGET");
        let (arch, os) = match std::env::var("TARGET").expect("TARGET to be set").as_str() {
            // Linux
            "aarch64-unknown-linux-gnu" => ("arm64", "linux"),
            "x86_64-unknown-linux-gnu" => ("x86_64", "linux"),
            // macOS
            "aarch64-apple-darwin" => ("arm64", "macos"),
            "x86_64-apple-darwin" => ("x86_64", "macos"),
            // Windows
            //
            // NOTE: only MSVC currently, how do we map MinGW later?
            "aarch64-pc-windows-msvc" => ("arm64", "windows"),
            "x86_64-pc-windows-msvc" => ("x86_64", "windows"),
            _others => panic!("Unknown TARGET to map to Qt artifact"),
        };

        artifacts
            .into_iter()
            .filter(|artifact| {
                artifact.arch == arch && artifact.os == os && versions.contains(&artifact.version)
            })
            .collect()
    }

    /// Merge together artifacts with the same version
    /// so that we do not have bin/ and include/ split
    //
    // NOTE: later we may support bin and include folders being in different places
    pub(crate) fn group_artifacts(artifacts: Vec<ParsedQtArtifact>) -> Vec<ParsedQtArtifact> {
        artifacts.into_iter().fold(
                            HashMap::<semver::Version, ParsedQtArtifact>::default(),
                            |mut acc, mut artifact| {
                                acc.entry(artifact.version.clone())
                                    .and_modify(|value| {
                                        if value.url == artifact.url {
                                            value.content.append(&mut artifact.content)
                                        } else {
                                            println!("cargo::warning=Found multiple minimal installations of the same version but different urls: {} and {}", value.url, artifact.url);
                                        }
                                    })
                                    .or_insert(artifact);
                                acc
                            },
                        )
            .into_values()
            // Ensure that artifacts contain bin/ and include/
            .filter(|artifact| {
                artifact.content.contains(&"bin".to_string())
                    && artifact.content.contains(&"include".to_string())
            })
            .collect()
    }
}

/// Get all valid directories in path bufs, ignoring errors
fn list_dirs(path: &Path) -> Vec<DirEntry> {
    path.read_dir().unwrap().flatten().collect()
}
