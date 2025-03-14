// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use semver::Version;
use std::{env, io::ErrorKind, path::PathBuf, process::Command};

use crate::{QtBuildError, QtInstallation, QtTool};

/// TODO
pub struct QtInstallationQMake {
    qmake_path: PathBuf,
    qmake_version: Version,
}

impl QtInstallationQMake {
    /// TODO
    pub fn new() -> anyhow::Result<Self> {
        // Try the QMAKE variable first
        println!("cargo::rerun-if-env-changed=QMAKE");
        if let Ok(qmake_env_var) = env::var("QMAKE") {
            return QtInstallationQMake::try_from(PathBuf::from(&qmake_env_var)).map_err(|err| {
                QtBuildError::QMakeSetQtMissing {
                    qmake_env_var,
                    error: err.into(),
                }
                .into()
            });
        }

        // Try variable candidates within the patch
        ["qmake6", "qmake-qt5", "qmake"]
            .iter()
            // Use the first non-errored installation
            // If there are no valid installations we display the last error
            .fold(None, |acc, qmake_path| {
                Some(acc.map_or_else(
                    // Value is None so try to create installation
                    || QtInstallationQMake::try_from(PathBuf::from(qmake_path)),
                    // Value is Some so pass through or create if Err
                    |prev: anyhow::Result<Self>| {
                        prev.or_else(|_|
                            // Value is Err so try to create installation
                            QtInstallationQMake::try_from(PathBuf::from(qmake_path)))
                    },
                ))
            })
            .unwrap_or_else(|| Err(QtBuildError::QtMissing.into()))
    }
}

impl TryFrom<PathBuf> for QtInstallationQMake {
    type Error = anyhow::Error;

    fn try_from(qmake_path: PathBuf) -> anyhow::Result<Self> {
        // Attempt to read the QT_VERSION from qmake
        let qmake_version = match Command::new(&qmake_path)
            .args(["-query", "QT_VERSION"])
            .output()
        {
            Err(e) if e.kind() == ErrorKind::NotFound => Err(QtBuildError::QtMissing),
            Err(e) => Err(QtBuildError::QmakeFailed(e)),
            Ok(output) if !output.status.success() => Err(QtBuildError::QtMissing),
            Ok(output) => Ok(Version::parse(
                String::from_utf8_lossy(&output.stdout).trim(),
            )?),
        }?;

        // Check QT_VERSION_MAJOR is the same as the qmake version
        println!("cargo::rerun-if-env-changed=QT_VERSION_MAJOR");
        if let Ok(env_qt_version_major) = env::var("QT_VERSION_MAJOR") {
            // Parse to an integer
            let env_qt_version_major = env_qt_version_major.trim().parse::<u64>().map_err(|e| {
                QtBuildError::QtVersionMajorInvalid {
                    qt_version_major_env_var: env_qt_version_major,
                    source: e,
                }
            })?;

            // Ensure the version major is the same
            if qmake_version.major != env_qt_version_major {
                return Err(QtBuildError::QtVersionMajorDoesNotMatch {
                    qmake_version: qmake_version.major,
                    qt_version_major: env_qt_version_major,
                }
                .into());
            }
        }

        Ok(Self {
            qmake_path,
            qmake_version,
        })
    }
}

impl QtInstallation for QtInstallationQMake {
    fn include_paths(&self, _qt_modules: &[String]) -> Vec<PathBuf> {
        todo!()
    }

    fn link_modules(&self, _builder: &mut cc::Build, _qt_modules: &[String]) {
        todo!()
    }

    fn try_find_tool(&self, tool: QtTool) -> Option<PathBuf> {
        self.try_qmake_find_tool(tool.binary_name())
    }

    fn version(&self) -> semver::Version {
        self.qmake_version.clone()
    }
}

impl QtInstallationQMake {
    fn qmake_query(&self, var_name: &str) -> String {
        String::from_utf8_lossy(
            &Command::new(&self.qmake_path)
                .args(["-query", var_name])
                .output()
                .unwrap()
                .stdout,
        )
        .trim()
        .to_string()
    }

    fn try_qmake_find_tool(&self, tool_name: &str) -> Option<PathBuf> {
        // "qmake -query" exposes a list of paths that describe where Qt executables and libraries
        // are located, as well as where new executables & libraries should be installed to.
        // We can use these variables to find any Qt tool.
        //
        // The order is important here.
        // First, we check the _HOST_ variables.
        // In cross-compilation contexts, these variables should point to the host toolchain used
        // for building. The _INSTALL_ directories describe where to install new binaries to
        // (i.e. the target directories).
        // We still use the _INSTALL_ paths as fallback.
        //
        // The _LIBEXECS variables point to the executable Qt-internal tools (i.e. moc and
        // friends), whilst _BINS point to the developer-facing executables (qdoc, qmake, etc.).
        // As we mostly use the Qt-internal tools in this library, check _LIBEXECS first.
        //
        // Furthermore, in some contexts these variables include a `/get` variant.
        // This is important for contexts where qmake and the Qt build tools do not have a static
        // location, but are moved around during building.
        // This notably happens with yocto builds.
        // For each package, yocto builds a `sysroot` folder for both the host machine, as well
        // as the target. This is done to keep package builds reproducable & separate.
        // As a result the qmake executable is copied into each host sysroot for building.
        //
        // In this case the variables compiled into qmake still point to the paths relative
        // from the host sysroot (e.g. /usr/bin).
        // The /get variant in comparison will "get" the right full path from the current environment.
        // Therefore prefer to use the `/get` variant when available.
        // See: https://github.com/KDAB/cxx-qt/pull/430
        //
        // To check & debug all variables available on your system, simply run:
        //
        //              qmake -query
        [
            "QT_HOST_LIBEXECS/get",
            "QT_HOST_LIBEXECS",
            "QT_HOST_BINS/get",
            "QT_HOST_BINS",
            "QT_INSTALL_LIBEXECS/get",
            "QT_INSTALL_LIBEXECS",
            "QT_INSTALL_BINS/get",
            "QT_INSTALL_BINS",
        ]
        .iter()
        // Find the first valid executable path
        .find_map(|qmake_query_var| {
            let executable_path = PathBuf::from(self.qmake_query(qmake_query_var)).join(tool_name);
            Command::new(&executable_path)
                .args(["-help"])
                .output()
                .map(|_| executable_path)
                .ok()
        })
    }
}
