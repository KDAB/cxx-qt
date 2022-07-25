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

/// Helper for build.rs scripts using Qt
/// ```
/// let qt_modules = vec!["Core", "Gui"]
///     .iter()
///     .map(|m| String::from(*m))
///     .collect();
/// let qtbuild = qt_build::QtBuild::new(qt_modules).expect("Could not find Qt installation");
/// ```
pub struct QtBuild {
    pub version: SemVer,
    qmake_executable: String,
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
    /// ```
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
    #[allow(clippy::result_unit_err)]
    pub fn new(mut qt_modules: Vec<String>) -> Result<Self, ()> {
        if qt_modules.is_empty() {
            qt_modules.push("Core".to_string());
        }
        println!("cargo:rerun-if-env-changed=QMAKE");
        println!("cargo:rerun-if-env-changed=QT_VERSION_MAJOR");
        fn verify_candidate(candidate: &str) -> Result<(&str, versions::SemVer), String> {
            match Command::new(&candidate)
                .args(&["-query", "QT_VERSION"])
                .output()
            {
                Err(_) => Err("qmake -query failed".to_string()),
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
                                    eprintln!("QT_VERSION_MAJOR environment variable defined but empty");
                                    return Ok((candidate, qmake_version))
                                }
                                Err(e) => panic!("QT_VERSION_MAJOR environment variable specified but could not parse as integer: {}", e),
                                Ok(int) => int
                            };
                            if env_version == qmake_version.major {
                                return Ok((candidate, qmake_version));
                            } else {
                                return Err("qmake version does not match version specified by QT_VERISON_MAJOR".to_string());
                            }
                        }
                        Ok((candidate, qmake_version))
                    } else {
                        Err("qmake executable not found".to_string())
                    }
                }
            }
        }

        if let Ok(qmake_env_var) = env::var("QMAKE") {
            match verify_candidate(qmake_env_var.trim()) {
                Ok((executable_name, version)) => {
                    return Ok(Self {
                        qmake_executable: executable_name.to_string(),
                        version,
                        qt_modules,
                    });
                }
                Err(e) => panic!(
                    "QMAKE environment variable specified but could not detect Qt: {}",
                    e
                ),
            }
        }

        // Fedora 36 renames Qt5's qmake to qmake-qt5
        for executable_name in ["qmake", "qmake6", "qmake-qt5"] {
            match verify_candidate(executable_name) {
                Ok((executable_name, version)) => {
                    return Ok(Self {
                        qmake_executable: executable_name.to_string(),
                        version,
                        qt_modules,
                    });
                }
                Err(_) => continue,
            }
        }

        Err(())
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
                            .replace(r"$$[QT_INSTALL_PREFIX]", &lib_path);
                        let mut lib = pkg_config::Library::new();
                        lib.parse_libs_cflags(
                            &format!("Qt{}{}", self.version.major, qt_module),
                            link_args.as_bytes(),
                            &pkg_config,
                        );
                    } else {
                        // When Qt is linked dynamically, the .prl files do not have a QT_PRL_LIBS line.
                        // This expected, not an error.
                        println!("cargo:rustc-link-lib=Qt{}{}", self.version.major, qt_module);
                    }
                }
                Err(e) => {
                    eprintln!(
                        "Could not open {} file to read libraries to link: {}",
                        &prl_path, e
                    );
                    println!("cargo:rustc-link-lib=Qt{}{}", self.version.major, qt_module);
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

    /// Run moc on a C++ header file and save the output into [cargo's OUT_DIR](https://doc.rust-lang.org/cargo/reference/environment-variables.html).
    /// The path to the generated C++ file is returned, which can then be passed to [cc::Build::files](https://docs.rs/cc/latest/cc/struct.Build.html#method.file).
    pub fn moc(&self, input_file: &impl AsRef<Path>) -> PathBuf {
        let qmake_query_var = if self.version.major >= 6 {
            "QT_INSTALL_LIBEXECS"
        } else {
            "QT_INSTALL_BINS"
        };

        let input_path = input_file.as_ref();
        let output_path = PathBuf::from(&format!(
            "{}/moc_{}.cpp",
            env::var("OUT_DIR").unwrap(),
            input_path.file_name().unwrap().to_str().unwrap()
        ));

        let _ = Command::new(&format!("{}/moc", self.qmake_query(qmake_query_var)))
            .args(&[
                input_path.to_str().unwrap(),
                "-o",
                output_path.to_str().unwrap(),
            ])
            .output()
            .unwrap_or_else(|_| panic!("moc failed for {}", input_path.display()));

        output_path
    }
}
