// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use semver::Version;
use std::{
    cell::RefCell, collections::HashMap, env, io::ErrorKind, path::PathBuf, process::Command,
};

use crate::{QtBuildError, QtInstallation, QtTool};

/// A implementation of [QtInstallation] using qmake
pub struct QtInstallationQMake {
    qmake_path: PathBuf,
    qmake_version: Version,
    // Internal cache of paths for tools
    //
    // Note that this only stores valid resolved paths.
    // If we failed to find the tool, we will not cache the failure and instead retry if called
    // again.
    // This is partially because anyhow::Error is not Clone, and partially because retrying gives
    // the caller the ability to change the environment and try again.
    tool_cache: RefCell<HashMap<QtTool, PathBuf>>,
}

impl QtInstallationQMake {
    /// The directories specified by the `PATH` environment variable are where qmake is
    /// searched for. Alternatively, the `QMAKE` environment variable may be set to specify
    /// an explicit path to qmake.
    ///
    /// If multiple major versions (for example, `5` and `6`) of Qt could be installed, set
    /// the `QT_VERSION_MAJOR` environment variable to force which one to use. When using Cargo
    /// as the build system for the whole build, prefer using `QT_VERSION_MAJOR` over the `QMAKE`
    /// environment variable because it will account for different names for the qmake executable
    /// that some Linux distributions use.
    ///
    /// However, when building a Rust staticlib that gets linked to C++ code by a C++ build
    /// system, it is best to use the `QMAKE` environment variable to ensure that the Rust
    /// staticlib is linked to the same installation of Qt that the C++ build system has
    /// detected.
    /// With CMake, this will automatically be set up for you when using cxxqt_import_crate.
    ///
    /// Alternatively, you can get this from the `Qt::qmake` target's `IMPORTED_LOCATION`
    /// property, for example:
    /// ```cmake
    /// find_package(Qt6 COMPONENTS Core)
    /// if(NOT Qt6_FOUND)
    ///     find_package(Qt5 5.15 COMPONENTS Core REQUIRED)
    /// endif()
    /// get_target_property(QMAKE Qt::qmake IMPORTED_LOCATION)
    ///
    /// execute_process(
    ///     COMMAND cmake -E env
    ///         "CARGO_TARGET_DIR=${CMAKE_CURRENT_BINARY_DIR}/cargo"
    ///         "QMAKE=${QMAKE}"
    ///         cargo build
    ///     WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
    /// )
    /// ```
    pub fn new() -> anyhow::Result<Self> {
        // Try the QMAKE variable first
        if let Some(result) = Self::try_from_qmake_env() {
            return result;
        }

        // Try variable candidates within the patch
        Self::try_from_path()
    }

    pub(crate) fn try_from_path() -> anyhow::Result<Self> {
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

    pub(crate) fn try_from_qmake_env() -> Option<anyhow::Result<Self>> {
        println!("cargo::rerun-if-env-changed=QMAKE");

        if let Ok(qmake_env_var) = env::var("QMAKE") {
            return Some(
                QtInstallationQMake::try_from(PathBuf::from(&qmake_env_var)).map_err(|err| {
                    QtBuildError::QMakeSetQtMissing {
                        qmake_env_var,
                        error: err.into(),
                    }
                    .into()
                }),
            );
        }

        None
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
            tool_cache: HashMap::default().into(),
        })
    }
}

impl QtInstallation for QtInstallationQMake {
    fn framework_paths(&self, qt_modules: &[String]) -> Vec<PathBuf> {
        let path_lib = PathBuf::from(self.qmake_query("QT_INSTALL_LIBS"));
        super::shared::framework_paths_for_qt_modules(qt_modules, path_lib)
    }

    fn include_paths(&self, qt_modules: &[String]) -> Vec<PathBuf> {
        let path_include = PathBuf::from(self.qmake_query("QT_INSTALL_HEADERS"));
        let path_lib = PathBuf::from(self.qmake_query("QT_INSTALL_LIBS"));
        super::shared::include_paths_for_qt_modules(qt_modules, path_include, path_lib)
    }

    fn link_modules(&self, builder: &mut cc::Build, qt_modules: &[String]) {
        let path_frameworks = self.framework_paths(qt_modules);
        let path_lib = PathBuf::from(self.qmake_query("QT_INSTALL_LIBS"));
        let path_prefix = PathBuf::from(self.qmake_query("QT_INSTALL_PREFIX"));
        let path_plugins = PathBuf::from(self.qmake_query("QT_INSTALL_PLUGINS"));
        let qt_version = self.qmake_version.clone();
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

    fn try_find_tool(&self, tool: QtTool) -> anyhow::Result<PathBuf> {
        let find_tool = || self.try_qmake_find_tool(tool.binary_name());
        // Attempt to use the cache
        let Ok(mut tool_cache) = self.tool_cache.try_borrow_mut() else {
            return find_tool();
        };
        // Read the tool from the cache or insert
        if let Some(path) = tool_cache.get(&tool) {
            return Ok(path.clone());
        }
        let path = find_tool()?;
        tool_cache.insert(tool, path.clone());
        Ok(path)
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
        .to_owned()
    }

    fn try_qmake_find_tool(&self, tool_name: &str) -> anyhow::Result<PathBuf> {
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
        let mut failed_paths = vec![];
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
            let test_output = Command::new(&executable_path).args(["-help"]).output();
            match test_output {
                Err(_err) => {
                    failed_paths.push(executable_path);
                    None
                }
                Ok(_) => Some(executable_path),
            }
        })
        .ok_or_else(|| anyhow::anyhow!("Failed to find {tool_name}, tried: {failed_paths:?}"))
    }
}
