// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Be Wilson <be.wilson@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! This crate provides information about the Qt installation and can invoke Qt's
//! [moc](https://doc.qt.io/qt-6/moc.html) code generator. This crate does not build
//! any C++ code on its own. It is intended to be used in [build.rs scripts](https://doc.rust-lang.org/cargo/reference/build-scripts.html)
//! together with
//! [cc](https://docs.rs/cc/latest/cc/),
//! [cxx_build](https://docs.rs/cxx-build/latest/cxx_build/), or
//! [cpp_build](https://docs.rs/cpp_build/latest/cpp_build/).

use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
};

pub use versions::SemVer;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum QtBuildError {
    /// `QMAKE` environment variable was set but Qt was not detected
    #[error("QMAKE environment variable specified as {qmake_env_var} but could not detect Qt: {error:?}")]
    QMakeSetQtMissing {
        qmake_env_var: String,
        error: Box<QtBuildError>,
    },
    /// Qt was not found
    #[error("Could not find Qt")]
    QtMissing,
    /// Executing `qmake -query` failed
    #[error("Executing `qmake -query` failed: {0:?}")]
    QmakeFailed(#[from] std::io::Error),
    /// `QT_VERSION_MAJOR` environment variable was specified but could not be parsed as an integer
    #[error("QT_VERSION_MAJOR environment variable specified as {qt_version_major_env_var} but could not parse as integer: {source:?}")]
    QtVersionMajorInvalid {
        qt_version_major_env_var: String,
        source: std::num::ParseIntError,
    },
    /// `QT_VERSION_MAJOR` environment variable was specified but the Qt version specified by `qmake -query QT_VERSION` did not match
    #[error("qmake version ({qmake_version}) does not match version specified by QT_VERISON_MAJOR ({qt_version_major})")]
    QtVersionMajorDoesNotMatch {
        qmake_version: u32,
        qt_version_major: u32,
    },
}

/// Helper for build.rs scripts using Qt
/// ```
/// let qt_modules = vec!["Core", "Gui"]
///     .iter()
///     .map(|m| String::from(*m))
///     .collect();
/// let qtbuild = qt_build::QtBuild::new(qt_modules).expect("Could not find Qt installation");
/// ```
pub struct QtBuild {
    version: SemVer,
    qmake_executable: String,
    moc_executable: Option<String>,
    rcc_executable: Option<String>,
    qt_modules: Vec<String>,
}

impl QtBuild {
    /// Search for where Qt is installed using qmake. Specify the Qt modules you are
    /// linking with the `qt_modules` parameter, ommitting the `Qt` prefix (`"Core"`
    /// rather than `"QtCore"`). After construction, use the [QtBuild::qmake_query]
    /// method to get information about the Qt installation.
    ///
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
    /// detected. With CMake, you can get this from the `Qt${QT_VERSION_MAJOR}::qmake`
    /// target's `IMPORTED_LOCATION` property, for example:
    /// ```cmake
    /// find_package(QT NAMES Qt6 Qt5 COMPONENTS Core REQUIRED)
    /// find_package(Qt${QT_VERSION_MAJOR} COMPONENTS Core REQUIRED)
    /// get_target_property(QMAKE Qt${QT_VERSION_MAJOR}::qmake IMPORTED_LOCATION)
    ///
    /// execute_process(
    ///     COMMAND cmake -E env
    ///         "CARGO_TARGET_DIR=${CMAKE_CURRENT_BINARY_DIR}/cargo"
    ///         "QMAKE=${QMAKE}"
    ///         cargo build
    ///     WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
    /// )
    /// ```
    pub fn new(mut qt_modules: Vec<String>) -> Result<Self, QtBuildError> {
        if qt_modules.is_empty() {
            qt_modules.push("Core".to_string());
        }
        println!("cargo:rerun-if-env-changed=QMAKE");
        println!("cargo:rerun-if-env-changed=QT_VERSION_MAJOR");
        fn verify_candidate(candidate: &str) -> Result<(&str, versions::SemVer), QtBuildError> {
            match Command::new(&candidate)
                .args(&["-query", "QT_VERSION"])
                .output()
            {
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => Err(QtBuildError::QtMissing),
                Err(e) => Err(QtBuildError::QmakeFailed(e)),
                Ok(output) => {
                    if output.status.success() {
                        let version_string = std::str::from_utf8(&output.stdout)
                            .unwrap()
                            .trim()
                            .to_string();
                        let qmake_version = versions::SemVer::new(&version_string).unwrap();
                        if let Ok(env_version) = env::var("QT_VERSION_MAJOR") {
                            let env_version = match env_version.trim().parse::<u32>() {
                                Err(e) if *e.kind() == std::num::IntErrorKind::Empty => {
                                    println!(
                                        "cargo:warning=QT_VERSION_MAJOR environment variable defined but empty"
                                    );
                                    return Ok((candidate, qmake_version));
                                }
                                Err(e) => {
                                    return Err(QtBuildError::QtVersionMajorInvalid {
                                        qt_version_major_env_var: env_version,
                                        source: e,
                                    })
                                }
                                Ok(int) => int,
                            };
                            if env_version == qmake_version.major {
                                return Ok((candidate, qmake_version));
                            } else {
                                return Err(QtBuildError::QtVersionMajorDoesNotMatch {
                                    qmake_version: qmake_version.major,
                                    qt_version_major: env_version,
                                });
                            }
                        }
                        Ok((candidate, qmake_version))
                    } else {
                        Err(QtBuildError::QtMissing)
                    }
                }
            }
        }

        if let Ok(qmake_env_var) = env::var("QMAKE") {
            match verify_candidate(qmake_env_var.trim()) {
                Ok((executable_name, version)) => {
                    return Ok(Self {
                        qmake_executable: executable_name.to_string(),
                        moc_executable: None,
                        rcc_executable: None,
                        version,
                        qt_modules,
                    });
                }
                Err(e) => {
                    return Err(QtBuildError::QMakeSetQtMissing {
                        qmake_env_var,
                        error: Box::new(e),
                    })
                }
            }
        }

        // Fedora 36 renames Qt5's qmake to qmake-qt5
        let candidate_executable_names = ["qmake6", "qmake-qt5", "qmake"];
        for (index, executable_name) in candidate_executable_names.iter().enumerate() {
            match verify_candidate(executable_name) {
                Ok((executable_name, version)) => {
                    return Ok(Self {
                        qmake_executable: executable_name.to_string(),
                        moc_executable: None,
                        rcc_executable: None,
                        version,
                        qt_modules,
                    });
                }
                // If QT_VERSION_MAJOR is specified, it is expected that one of the versioned
                // executable names will not match, so the unversioned `qmake` needs to be
                // attempted last and QtVersionMajorDoesNotMatch should only be returned if
                // none of the candidate executable names match.
                Err(QtBuildError::QtVersionMajorDoesNotMatch {
                    qmake_version,
                    qt_version_major,
                }) => {
                    if index == candidate_executable_names.len() - 1 {
                        return Err(QtBuildError::QtVersionMajorDoesNotMatch {
                            qmake_version,
                            qt_version_major,
                        });
                    }
                    eprintln!("Candidate qmake executable `{executable_name}` is for Qt{qmake_version} but QT_VERISON_MAJOR environment variable specified as {qt_version_major}. Trying next candidate executable name `{}`...", candidate_executable_names[index + 1]);
                    continue;
                }
                Err(QtBuildError::QtMissing) => continue,
                Err(e) => return Err(e),
            }
        }

        Err(QtBuildError::QtMissing)
    }

    /// Get the output of running `qmake -query var_name`
    pub fn qmake_query(&self, var_name: &str) -> String {
        std::str::from_utf8(
            &Command::new(&self.qmake_executable)
                .args(&["-query", var_name])
                .output()
                .unwrap()
                .stdout,
        )
        .unwrap()
        .trim()
        .to_string()
    }

    /// Tell Cargo to link each Qt module.
    pub fn cargo_link_libraries(&self) {
        lazy_static::lazy_static! {
            static ref QMAKE_PRL_LIBS: regex::Regex = regex::RegexBuilder::new(r"^QMAKE_PRL_LIBS = (.*)$").multi_line(true).build().unwrap();
        }
        let lib_path = self.qmake_query("QT_INSTALL_LIBS");
        println!("cargo:rustc-link-search={}", lib_path);

        // The needed information is in qmake's .prl files, so using pkgconfig is not necessary.
        // There is no guarantee that pkgconfig is installed if Qt is installed, particularly on
        // Windows. However, the pkg_config crate provides a useful function for parsing the
        // information from the .prl files into linking instructions for Cargo.
        let pkg_config = pkg_config::Config::new();

        #[cfg(windows)]
        let prefix = "";
        #[cfg(not(windows))]
        let prefix = "lib";

        for qt_module in &self.qt_modules {
            println!("cargo:rustc-link-lib=Qt{}{}", self.version.major, qt_module);
            let prl_path = format!(
                "{}/{}Qt{}{}.prl",
                lib_path, prefix, self.version.major, qt_module
            );
            match std::fs::read_to_string(&prl_path) {
                Ok(prl) => {
                    if let Some(captures) = QMAKE_PRL_LIBS.captures(&prl) {
                        let link_args = captures
                            .get(1)
                            .unwrap()
                            .as_str()
                            .replace(r"$$[QT_INSTALL_LIBS]", &lib_path)
                            .replace(r"$$[QT_INSTALL_PREFIX]", &lib_path);
                        let mut lib = pkg_config::Library::new();
                        lib.parse_libs_cflags(
                            &format!("Qt{}{}", self.version.major, qt_module),
                            link_args.as_bytes(),
                            &pkg_config,
                        );
                    }
                }
                Err(e) => {
                    println!(
                        "cargo:warning=Could not open {} file to read libraries to link: {}",
                        &prl_path, e
                    );
                }
            }
        }
    }

    /// Get the include paths for Qt, including Qt module subdirectories. This is intended
    /// to be passed to whichever tool you are using to invoke the C++ compiler.
    pub fn include_paths(&self) -> Vec<PathBuf> {
        let root_path = self.qmake_query("QT_INSTALL_HEADERS");
        let mut paths = Vec::new();
        for qt_module in &self.qt_modules {
            paths.push(format!("{}/Qt{}", root_path, qt_module));
        }
        paths.push(root_path);
        paths.iter().map(PathBuf::from).collect()
    }

    /// Version of the detected Qt installation
    pub fn version(&self) -> &SemVer {
        &self.version
    }

    /// Lazy load the path of a Qt executable tool
    /// Skip doing this in the constructor because not every user of this crate will use each tool
    fn get_qt_tool(&self, tool_name: &str) -> Result<String, ()> {
        for qmake_query_var in [
            "QT_HOST_LIBEXECS",
            "QT_HOST_BINS",
            "QT_INSTALL_LIBEXECS",
            "QT_INSTALL_BINS",
        ] {
            let executable_path = format!("{}/{}", self.qmake_query(qmake_query_var), tool_name);
            match Command::new(&executable_path).args(&["-help"]).output() {
                Ok(_) => return Ok(executable_path),
                Err(_) => continue,
            }
        }
        Err(())
    }

    /// Run moc on a C++ header file and save the output into [cargo's OUT_DIR](https://doc.rust-lang.org/cargo/reference/environment-variables.html).
    /// The path to the generated C++ file is returned, which can then be passed to [cc::Build::files](https://docs.rs/cc/latest/cc/struct.Build.html#method.file).
    pub fn moc(&mut self, input_file: impl AsRef<Path>) -> PathBuf {
        if self.moc_executable.is_none() {
            self.moc_executable = Some(self.get_qt_tool("moc").expect("Could not find moc"));
        }

        let input_path = input_file.as_ref();
        let output_path = PathBuf::from(&format!(
            "{}/moc_{}.cpp",
            env::var("OUT_DIR").unwrap(),
            input_path.file_name().unwrap().to_str().unwrap()
        ));

        let _ = Command::new(self.moc_executable.as_ref().unwrap())
            .args(&[
                input_path.to_str().unwrap(),
                "-o",
                output_path.to_str().unwrap(),
            ])
            .output()
            .unwrap_or_else(|_| panic!("moc failed for {}", input_path.display()));

        output_path
    }

    /// Run [rcc](https://doc.qt.io/qt-6/resources.html) on a .qrc file and save the output into [cargo's OUT_DIR](https://doc.rust-lang.org/cargo/reference/environment-variables.html).
    /// The path to the generated C++ file is returned, which can then be passed to [cc::Build::files](https://docs.rs/cc/latest/cc/struct.Build.html#method.file).
    pub fn qrc(&mut self, input_file: &impl AsRef<Path>) -> PathBuf {
        if self.rcc_executable.is_none() {
            self.rcc_executable = Some(self.get_qt_tool("rcc").expect("Could not find rcc"));
        }

        let input_path = input_file.as_ref();
        let output_path = PathBuf::from(&format!(
            "{}/{}.cpp",
            env::var("OUT_DIR").unwrap(),
            input_path.file_name().unwrap().to_str().unwrap()
        ));

        let _ = Command::new(self.rcc_executable.as_ref().unwrap())
            .args(&[
                input_path.to_str().unwrap(),
                "-o",
                output_path.to_str().unwrap(),
            ])
            .output()
            .unwrap_or_else(|_| panic!("rcc failed for {}", input_path.display()));

        output_path
    }
}
