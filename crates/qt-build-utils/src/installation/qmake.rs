// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use semver::Version;
use std::{
    env,
    io::ErrorKind,
    path::{Path, PathBuf},
    process::Command,
};

use crate::{parse_cflags, utils, QtBuildError, QtInstallation, QtTool};

/// A implementation of [QtInstallation] using qmake
pub struct QtInstallationQMake {
    qmake_path: PathBuf,
    qmake_version: Version,
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
    fn include_paths(&self, qt_modules: &[String]) -> Vec<PathBuf> {
        let root_path = self.qmake_query("QT_INSTALL_HEADERS");
        let lib_path = self.qmake_query("QT_INSTALL_LIBS");
        let mut paths = Vec::new();
        for qt_module in qt_modules {
            // Add the usual location for the Qt module
            paths.push(format!("{root_path}/Qt{qt_module}"));

            // Ensure that we add any framework's headers path
            let header_path = format!("{lib_path}/Qt{qt_module}.framework/Headers");
            if utils::is_apple_target() && Path::new(&header_path).exists() {
                paths.push(header_path);
            }
        }

        // Add the QT_INSTALL_HEADERS itself
        paths.push(root_path);

        paths
            .iter()
            .map(PathBuf::from)
            // Only add paths if they exist
            .filter(|path| path.exists())
            .collect()
    }

    fn link_modules(&self, builder: &mut cc::Build, qt_modules: &[String]) {
        let prefix_path = self.qmake_query("QT_INSTALL_PREFIX");
        let lib_path = self.qmake_query("QT_INSTALL_LIBS");
        println!("cargo::rustc-link-search={lib_path}");

        let target = env::var("TARGET");

        // Add the QT_INSTALL_LIBS as a framework link search path as well
        //
        // Note that leaving the kind empty should default to all,
        // but this doesn't appear to find frameworks in all situations
        // https://github.com/KDAB/cxx-qt/issues/885
        //
        // Note this doesn't have an adverse affect running all the time
        // as it appears that all rustc-link-search are added
        //
        // Note that this adds the framework path which allows for
        // includes such as <QtCore/QObject> to be resolved correctly
        if utils::is_apple_target() {
            println!("cargo::rustc-link-search=framework={lib_path}");

            // Ensure that any framework paths are set to -F
            for framework_path in self.qmake_framework_paths() {
                builder.flag_if_supported(format!("-F{}", framework_path.display()));
                // Also set the -rpath otherwise frameworks can not be found at runtime
                println!(
                    "cargo::rustc-link-arg=-Wl,-rpath,{}",
                    framework_path.display()
                );
            }
        }

        let prefix = match &target {
            Ok(target) => {
                if target.contains("windows") {
                    ""
                } else {
                    "lib"
                }
            }
            Err(_) => "lib",
        };

        for qt_module in qt_modules {
            let framework = if utils::is_apple_target() {
                Path::new(&format!("{lib_path}/Qt{qt_module}.framework")).exists()
            } else {
                false
            };

            let (link_lib, prl_path) = if framework {
                (
                    format!("framework=Qt{qt_module}"),
                    format!("{lib_path}/Qt{qt_module}.framework/Resources/Qt{qt_module}.prl"),
                )
            } else {
                (
                    format!("Qt{}{qt_module}", self.qmake_version.major),
                    self.find_qt_module_prl(&lib_path, prefix, self.qmake_version.major, qt_module),
                )
            };

            self.link_qt_library(
                &format!("Qt{}{qt_module}", self.qmake_version.major),
                &prefix_path,
                &lib_path,
                &link_lib,
                &prl_path,
                builder,
            );
        }

        if utils::is_emscripten_target() {
            let platforms_path = format!("{}/platforms", self.qmake_query("QT_INSTALL_PLUGINS"));
            println!("cargo::rustc-link-search={platforms_path}");
            self.link_qt_library(
                "qwasm",
                &prefix_path,
                &lib_path,
                "qwasm",
                &format!("{platforms_path}/libqwasm.prl"),
                builder,
            );
        }
    }

    fn try_find_tool(&self, tool: QtTool) -> Option<PathBuf> {
        self.try_qmake_find_tool(tool.binary_name())
    }

    fn version(&self) -> semver::Version {
        self.qmake_version.clone()
    }
}

impl QtInstallationQMake {
    /// Some prl files include their architecture in their naming scheme.
    /// Just try all known architectures and fallback to non when they all failed.
    fn find_qt_module_prl(
        &self,
        lib_path: &str,
        prefix: &str,
        version_major: u64,
        qt_module: &str,
    ) -> String {
        for arch in ["", "_arm64-v8a", "_armeabi-v7a", "_x86", "_x86_64"] {
            let prl_path = format!(
                "{}/{}Qt{}{}{}.prl",
                lib_path, prefix, version_major, qt_module, arch
            );
            match Path::new(&prl_path).try_exists() {
                Ok(exists) => {
                    if exists {
                        return prl_path;
                    }
                }
                Err(e) => {
                    println!(
                        "cargo::warning=failed checking for existence of {}: {}",
                        prl_path, e
                    );
                }
            }
        }

        format!(
            "{}/{}Qt{}{}.prl",
            lib_path, prefix, version_major, qt_module
        )
    }

    fn link_qt_library(
        &self,
        name: &str,
        prefix_path: &str,
        lib_path: &str,
        link_lib: &str,
        prl_path: &str,
        builder: &mut cc::Build,
    ) {
        println!("cargo::rustc-link-lib={link_lib}");

        match std::fs::read_to_string(prl_path) {
            Ok(prl) => {
                for line in prl.lines() {
                    if let Some(line) = line.strip_prefix("QMAKE_PRL_LIBS = ") {
                        parse_cflags::parse_libs_cflags(
                            name,
                            line.replace(r"$$[QT_INSTALL_LIBS]", lib_path)
                                .replace(r"$$[QT_INSTALL_PREFIX]", prefix_path)
                                .as_bytes(),
                            builder,
                        );
                    }
                }
            }
            Err(e) => {
                println!(
                    "cargo::warning=Could not open {} file to read libraries to link: {}",
                    &prl_path, e
                );
            }
        }
    }

    /// Get the framework paths for Qt. This is intended
    /// to be passed to whichever tool you are using to invoke the C++ compiler.
    fn qmake_framework_paths(&self) -> Vec<PathBuf> {
        let mut framework_paths = vec![];

        if utils::is_apple_target() {
            // Note that this adds the framework path which allows for
            // includes such as <QtCore/QObject> to be resolved correctly
            let framework_path = self.qmake_query("QT_INSTALL_LIBS");
            framework_paths.push(framework_path);
        }

        framework_paths
            .iter()
            .map(PathBuf::from)
            // Only add paths if they exist
            .filter(|path| path.exists())
            .collect()
    }

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
