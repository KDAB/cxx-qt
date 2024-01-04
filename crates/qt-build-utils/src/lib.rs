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

#![allow(clippy::too_many_arguments)]

mod parse_cflags;

use std::{
    env,
    fs::File,
    io::Write,
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
    #[error("qmake version ({qmake_version}) does not match version specified by QT_VERSION_MAJOR ({qt_version_major})")]
    QtVersionMajorDoesNotMatch {
        qmake_version: u32,
        qt_version_major: u32,
    },
}

fn command_help_output(command: &str) -> std::io::Result<std::process::Output> {
    Command::new(command).args(["--help"]).output()
}

/// Linking executables (including tests) with Cargo that link to Qt fails to link with GNU ld.bfd,
/// which is the default on most Linux distributions, so use GNU ld.gold, lld, or mold instead.
/// If you are using a C++ build system such as CMake to do the final link of the executable, you do
/// not need to call this function.
///
/// With Apple devices we set -fapple-link-rtlib as we build with -nodefaultlibs
/// otherwise we cannot user helpers from the compiler runtime in Qt
///
/// This does nothing on non-Unix platforms.
pub fn setup_linker() {
    if env::var("CARGO_CFG_UNIX").is_err() {
        return;
    }

    if let Ok(vendor) = env::var("CARGO_CFG_TARGET_VENDOR") {
        if vendor == "apple" {
            // Tell clang link to clang_rt as we build with -nodefaultlibs
            // otherwise we cannot use helpers from the compiler runtime in Qt
            println!("cargo:rustc-link-arg=-fapple-link-rtlib");
        }
    }

    let flags = env::var("CARGO_ENCODED_RUSTFLAGS").unwrap();
    // Don't override custom flags
    if !flags.contains("-fuse-ld") {
        // ld is the system default linker. On Linux, this is usually GNU ld.bfd, but it may be symlinked to another
        // linker. On macOS, Xcode ships lld with the executable named ld.
        let ld_help = String::from_utf8(
            command_help_output("ld")
                .expect("Could not run ld command")
                .stdout,
        )
        .unwrap();
        // bfd supports some exotic targets that other linkers do not.
        let ld_is_bfd = ld_help.contains("symbolsrec")
            || ld_help.contains("verilog")
            || ld_help.contains("tekhex");

        // Whatever linker is being used that's not bfd will likely work.
        if !ld_is_bfd {
            return;
        }

        // mold is fastest, but specifing mold with -fuse-ld requires GCC >= 12 or Clang.
        // Unfortunately cargo does not provide a means to set the linker driver via build scripts,
        // so linking would fail trying to use -fuse-ld=mold with GCC < 12 even if clang is installed.
        // So, prefer lld and gold to mold for robustness on the widest range of systems.
        // mold can still be used by manually specifying it in ~/.cargo/config.toml or the RUSTFLAGS environment variable.
        if command_help_output("lld").is_ok() {
            println!("cargo:rustc-link-arg=-fuse-ld=lld");
        } else if command_help_output("ld.gold").is_ok() {
            println!("cargo:rustc-link-arg=-fuse-ld=gold");
        } else if command_help_output("mold").is_ok() {
            println!("cargo:rustc-link-arg=-fuse-ld=mold");
        } else {
            println!("cargo:warning=Neither mold, lld, nor gold linkers were found. Linking with GNU ld.bfd will likely fail.");
        }
    }
}

/// Paths to files generated by [QtBuild::moc]
pub struct MocProducts {
    /// Generated C++ file
    pub cpp: PathBuf,
    /// Generated JSON file
    pub metatypes_json: PathBuf,
}

/// Paths to C++ files generated by [QtBuild::register_qml_module]
pub struct QmlModuleRegistrationFiles {
    /// File generated by [rcc](https://doc.qt.io/qt-6/rcc.html) for the QML plugin. The compiled static library
    /// must be linked with [+whole-archive](https://doc.rust-lang.org/rustc/command-line-arguments.html#linking-modifiers-whole-archive)
    /// or the linker will discard the generated static variables because they are not referenced from `main`.
    pub rcc: PathBuf,
    /// Files generated by [qmlcachegen](https://doc.qt.io/qt-6/qtqml-qtquick-compiler-tech.html). Must be linked with `+whole-archive`.
    pub qmlcachegen: Vec<PathBuf>,
    /// File generated by [qmltyperegistrar](https://www.qt.io/blog/qml-type-registration-in-qt-5.15) CLI tool.
    pub qmltyperegistrar: PathBuf,
    /// File with generated [QQmlEngineExtensionPlugin](https://doc.qt.io/qt-6/qqmlengineextensionplugin.html) that calls the function generated by qmltyperegistrar.
    pub plugin: PathBuf,
    /// File that automatically registers the QQmlExtensionPlugin at startup. Must be linked with `+whole-archive`.
    pub plugin_init: PathBuf,
}

/// Helper for build.rs scripts using Qt
/// ```
/// let qt_modules = vec!["Core", "Gui"]
///     .iter()
///     .map(|m| String::from(*m))
///     .collect();
/// let qtbuild = qt_build_utils::QtBuild::new(qt_modules).expect("Could not find Qt installation");
/// ```
pub struct QtBuild {
    version: SemVer,
    qmake_executable: String,
    moc_executable: Option<String>,
    qmltyperegistrar_executable: Option<String>,
    qmlcachegen_executable: Option<String>,
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
    /// detected. With CMake, you can get this from the `Qt::qmake` target's `IMPORTED_LOCATION`
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
    pub fn new(mut qt_modules: Vec<String>) -> Result<Self, QtBuildError> {
        if qt_modules.is_empty() {
            qt_modules.push("Core".to_string());
        }
        println!("cargo:rerun-if-env-changed=QMAKE");
        println!("cargo:rerun-if-env-changed=QT_VERSION_MAJOR");
        fn verify_candidate(candidate: &str) -> Result<(&str, versions::SemVer), QtBuildError> {
            match Command::new(candidate)
                .args(["-query", "QT_VERSION"])
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
                        qmltyperegistrar_executable: None,
                        qmlcachegen_executable: None,
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
                        qmltyperegistrar_executable: None,
                        qmlcachegen_executable: None,
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
                    eprintln!("Candidate qmake executable `{executable_name}` is for Qt{qmake_version} but QT_VERSION_MAJOR environment variable specified as {qt_version_major}. Trying next candidate executable name `{}`...", candidate_executable_names[index + 1]);
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
                .args(["-query", var_name])
                .output()
                .unwrap()
                .stdout,
        )
        .unwrap()
        .trim()
        .to_string()
    }

    fn cargo_link_qt_library(
        &self,
        name: &str,
        prefix_path: &str,
        lib_path: &str,
        link_lib: &str,
        prl_path: &str,
        builder: &mut cc::Build,
    ) {
        println!("cargo:rustc-link-lib={link_lib}");

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
                    "cargo:warning=Could not open {} file to read libraries to link: {}",
                    &prl_path, e
                );
            }
        }
    }

    /// Some prl files include their architecture in their naming scheme.
    /// Just try all known architectures and fallback to non when they all failed.
    fn find_qt_module_prl(
        &self,
        lib_path: &str,
        prefix: &str,
        version_major: u32,
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
                        "cargo:warning=failed checking for existence of {}: {}",
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

    /// Tell Cargo to link each Qt module.
    pub fn cargo_link_libraries(&self, builder: &mut cc::Build) {
        let prefix_path = self.qmake_query("QT_INSTALL_PREFIX");
        let lib_path = self.qmake_query("QT_INSTALL_LIBS");
        println!("cargo:rustc-link-search={lib_path}");

        let target = env::var("TARGET");
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

        for qt_module in &self.qt_modules {
            let framework = match &target {
                Ok(target) => {
                    if target.contains("apple") {
                        Path::new(&format!("{lib_path}/Qt{qt_module}.framework")).exists()
                    } else {
                        false
                    }
                }
                Err(_) => false,
            };

            let (link_lib, prl_path) = if framework {
                (
                    format!("framework=Qt{qt_module}"),
                    format!("{lib_path}/Qt{qt_module}.framework/Resources/Qt{qt_module}.prl"),
                )
            } else {
                (
                    format!("Qt{}{qt_module}", self.version.major),
                    self.find_qt_module_prl(&lib_path, prefix, self.version.major, qt_module),
                )
            };

            self.cargo_link_qt_library(
                &format!("Qt{}{qt_module}", self.version.major),
                &prefix_path,
                &lib_path,
                &link_lib,
                &prl_path,
                builder,
            );
        }

        let emscripten_targeted = match env::var("CARGO_CFG_TARGET_OS") {
            Ok(val) => val == "emscripten",
            Err(_) => false,
        };
        if emscripten_targeted {
            let platforms_path = format!("{}/platforms", self.qmake_query("QT_INSTALL_PLUGINS"));
            println!("cargo:rustc-link-search={platforms_path}");
            self.cargo_link_qt_library(
                "qwasm",
                &prefix_path,
                &lib_path,
                "qwasm",
                &format!("{platforms_path}/libqwasm.prl"),
                builder,
            );
        }
    }

    /// Get the include paths for Qt, including Qt module subdirectories. This is intended
    /// to be passed to whichever tool you are using to invoke the C++ compiler.
    pub fn include_paths(&self) -> Vec<PathBuf> {
        let root_path = self.qmake_query("QT_INSTALL_HEADERS");
        let mut paths = Vec::new();
        for qt_module in &self.qt_modules {
            paths.push(format!("{root_path}/Qt{qt_module}"));
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
        //
        for qmake_query_var in [
            "QT_HOST_LIBEXECS/get",
            "QT_HOST_LIBEXECS",
            "QT_HOST_BINS/get",
            "QT_HOST_BINS",
            "QT_INSTALL_LIBEXECS/get",
            "QT_INSTALL_LIBEXECS",
            "QT_INSTALL_BINS/get",
            "QT_INSTALL_BINS",
        ] {
            let executable_path = format!("{}/{tool_name}", self.qmake_query(qmake_query_var));
            match Command::new(&executable_path).args(["-help"]).output() {
                Ok(_) => return Ok(executable_path),
                Err(_) => continue,
            }
        }
        Err(())
    }

    /// Run moc on a C++ header file and save the output into [cargo's OUT_DIR](https://doc.rust-lang.org/cargo/reference/environment-variables.html).
    /// The return value contains the path to the generated C++ file, which can then be passed to [cc::Build::files](https://docs.rs/cc/latest/cc/struct.Build.html#method.file),
    /// as well as the path to the generated metatypes.json file, which can be passed to [register_qml_module](Self::register_qml_module).
    ///
    /// * uri - Should be passed if the input_file is part of a QML module
    pub fn moc(&mut self, input_file: impl AsRef<Path>, uri: Option<&str>) -> MocProducts {
        if self.moc_executable.is_none() {
            self.moc_executable = Some(self.get_qt_tool("moc").expect("Could not find moc"));
        }

        let input_path = input_file.as_ref();
        let output_path = PathBuf::from(&format!(
            "{}/moc_{}.cpp",
            env::var("OUT_DIR").unwrap(),
            input_path.file_name().unwrap().to_str().unwrap()
        ));

        let metatypes_json_path = PathBuf::from(&format!("{}.json", output_path.display()));

        let mut include_args = String::new();
        for include_path in self.include_paths() {
            include_args += &format!("-I {} ", include_path.display());
        }

        let mut cmd = Command::new(self.moc_executable.as_ref().unwrap());

        if let Some(uri) = uri {
            cmd.arg(&format!("-Muri={}", uri));
        }

        cmd.args(include_args.trim_end().split(' '));
        cmd.arg(input_path.to_str().unwrap())
            .arg("-o")
            .arg(output_path.to_str().unwrap())
            .arg("--output-json");
        let cmd = cmd
            .output()
            .unwrap_or_else(|_| panic!("moc failed for {}", input_path.display()));

        if !cmd.status.success() {
            panic!(
                "moc failed for {}:\n{}",
                input_path.display(),
                String::from_utf8_lossy(&cmd.stderr)
            );
        }

        MocProducts {
            cpp: output_path,
            metatypes_json: metatypes_json_path,
        }
    }

    /// Generate C++ files to automatically register a QML module at build time using the JSON output from [moc](Self::moc).
    ///
    /// This generates a [qmldir file](https://doc.qt.io/qt-6/qtqml-modules-qmldir.html) for the QML module.
    /// The `qml_files` and `qrc_files` are registered with the [Qt Resource System](https://doc.qt.io/qt-6/resources.html) in
    /// the [default QML import path](https://doc.qt.io/qt-6/qtqml-syntax-imports.html#qml-import-path) `qrc:/qt/qml/uri/of/module/`.
    ///
    /// When using Qt 6, this will [run qmlcachegen](https://doc.qt.io/qt-6/qtqml-qtquick-compiler-tech.html) to compile the specified .qml files ahead-of-time.
    pub fn register_qml_module(
        &mut self,
        metatypes_json: &[impl AsRef<Path>],
        uri: &str,
        version_major: usize,
        version_minor: usize,
        plugin_name: &str,
        qml_files: &[impl AsRef<Path>],
        qrc_files: &[impl AsRef<Path>],
    ) -> QmlModuleRegistrationFiles {
        if self.qmltyperegistrar_executable.is_none() {
            self.qmltyperegistrar_executable = Some(
                self.get_qt_tool("qmltyperegistrar")
                    .expect("Could not find qmltyperegistrar"),
            );
        }
        // qmlcachegen has a different CLI in Qt 5, so only support Qt >= 6
        if self.qmlcachegen_executable.is_none() && self.version.major >= 6 {
            if let Ok(qmlcachegen_executable) = self.get_qt_tool("qmlcachegen") {
                self.qmlcachegen_executable = Some(qmlcachegen_executable);
            }
        }

        let qml_uri_dirs = uri.replace('.', "/");

        let out_dir = env::var("OUT_DIR").unwrap();
        let qml_module_dir = format!("{out_dir}/qml_modules/{qml_uri_dirs}");
        std::fs::create_dir_all(&qml_module_dir).expect("Could not create QML module directory");

        let qml_uri_underscores = uri.replace('.', "_");
        let qmltypes_path = format!("{qml_module_dir}/plugin.qmltypes");
        let plugin_class_name = format!("{qml_uri_underscores}_plugin");

        // Generate qmldir file
        let qmldir_file_path = format!("{qml_module_dir}/qmldir");
        {
            let mut qmldir = File::create(&qmldir_file_path).expect("Could not create qmldir file");
            write!(
                qmldir,
                "module {uri}
optional plugin {plugin_name}
classname {plugin_class_name}
typeinfo plugin.qmltypes
prefer :/qt/qml/{qml_uri_dirs}/
"
            )
            .expect("Could not write qmldir file");
        }

        // Generate .qrc file and run rcc on it
        let qrc_path = format!("{qml_module_dir}/qml_module_resources.qrc");
        {
            fn qrc_file_line(file_path: &impl AsRef<Path>) -> String {
                let path_display = file_path.as_ref().display();
                format!(
                    "    <file alias=\"{}\">{}</file>\n",
                    path_display,
                    std::fs::canonicalize(file_path)
                        .unwrap_or_else(|_| panic!("Could not canonicalize path {}", path_display))
                        .display()
                )
            }

            let mut qml_files_qrc = String::new();
            for file_path in qml_files {
                qml_files_qrc.push_str(&qrc_file_line(file_path));
            }
            for file_path in qrc_files {
                qml_files_qrc.push_str(&qrc_file_line(file_path));
            }

            let mut qrc = File::create(&qrc_path).expect("Could not create qrc file");
            write!(
                qrc,
                r#"<RCC>
<qresource prefix="/">
    <file alias="/qt/qml/{qml_uri_dirs}">{qml_module_dir}</file>
</qresource>
<qresource prefix="/qt/qml/{qml_uri_dirs}">
{qml_files_qrc}
    <file alias="qmldir">{qml_module_dir}/qmldir</file>
</qresource>
</RCC>
"#
            )
            .expect("Could note write qrc file");
        }

        // Run qmlcachegen
        // qmlcachegen needs to be run once for each .qml file with --resource-path,
        // then once for the module with --resource-name.
        let mut qmlcachegen_file_paths = Vec::new();
        if let Some(qmlcachegen_executable) = &self.qmlcachegen_executable {
            let qmlcachegen_dir = format!("{out_dir}/qmlcachegen/{qml_uri_dirs}");
            std::fs::create_dir_all(&qmlcachegen_dir)
                .expect("Could not create qmlcachegen directory for QML module");

            let common_args = vec![
                "-i".to_string(),
                qmldir_file_path.to_string(),
                "--resource".to_string(),
                qrc_path.clone(),
            ];

            let mut qml_file_qrc_paths = Vec::new();
            for file in qml_files {
                let qrc_resource_path =
                    format!("/qt/qml/{qml_uri_dirs}/{}", file.as_ref().display());

                let qml_compiled_file = format!(
                    "{qmlcachegen_dir}/{}.cpp",
                    file.as_ref().file_name().unwrap().to_string_lossy()
                );
                qmlcachegen_file_paths.push(PathBuf::from(&qml_compiled_file));

                let specific_args = vec![
                    "--resource-path".to_string(),
                    qrc_resource_path.clone(),
                    "-o".to_string(),
                    qml_compiled_file,
                    std::fs::canonicalize(file)
                        .unwrap()
                        .to_string_lossy()
                        .to_string(),
                ];

                let cmd = Command::new(qmlcachegen_executable)
                    .args(common_args.iter().chain(&specific_args))
                    .output()
                    .unwrap_or_else(|_| {
                        panic!(
                            "qmlcachegen failed for {} in QML module {uri}",
                            file.as_ref().display()
                        )
                    });
                if !cmd.status.success() {
                    panic!(
                        "qmlcachegen failed for {} in QML module {uri}:\n{}",
                        file.as_ref().display(),
                        String::from_utf8_lossy(&cmd.stderr)
                    );
                }
                qml_file_qrc_paths.push(qrc_resource_path);
            }

            let qmlcachegen_loader = format!("{qmlcachegen_dir}/qmlcache_loader.cpp");
            let specific_args = vec![
                "--resource-name".to_string(),
                format!("qmlcache_{qml_uri_underscores}"),
                "-o".to_string(),
                qmlcachegen_loader.clone(),
            ];

            let cmd = Command::new(qmlcachegen_executable)
                .args(
                    common_args
                        .iter()
                        .chain(&specific_args)
                        .chain(&qml_file_qrc_paths),
                )
                .output()
                .unwrap_or_else(|_| panic!("qmlcachegen failed for QML module {uri}"));
            if !cmd.status.success() {
                panic!(
                    "qmlcachegen failed for QML module {uri}:\n{}",
                    String::from_utf8_lossy(&cmd.stderr)
                );
            }
            qmlcachegen_file_paths.push(PathBuf::from(&qmlcachegen_loader));
        }

        // Run qmltyperegistrar
        let qmltyperegistrar_output_path = PathBuf::from(&format!(
            "{out_dir}/{qml_uri_underscores}_qmltyperegistration.cpp"
        ));
        {
            let mut args = vec![
                "--generate-qmltypes".to_string(),
                qmltypes_path,
                "--major-version".to_string(),
                version_major.to_string(),
                "--minor-version".to_string(),
                version_minor.to_string(),
                "--import-name".to_string(),
                uri.to_string(),
                "-o".to_string(),
                qmltyperegistrar_output_path.to_string_lossy().to_string(),
            ];
            args.extend(
                metatypes_json
                    .iter()
                    .map(|f| f.as_ref().to_string_lossy().to_string()),
            );
            let cmd = Command::new(self.qmltyperegistrar_executable.as_ref().unwrap())
                .args(args)
                .output()
                .unwrap_or_else(|_| panic!("qmltyperegistrar failed for {uri}"));
            if !cmd.status.success() {
                panic!(
                    "qmltyperegistrar failed for {uri}:\n{}",
                    String::from_utf8_lossy(&cmd.stderr)
                );
            }
        }

        // Generate QQmlEngineExtensionPlugin
        let qml_plugin_cpp_path = PathBuf::from(format!("{out_dir}/{plugin_class_name}.cpp"));
        let qml_plugin_init_path = PathBuf::from(format!("{out_dir}/{plugin_class_name}_init.cpp"));
        {
            // This function is generated by qmltyperegistrar
            let register_types_function = format!("qml_register_types_{qml_uri_underscores}");

            let mut qml_plugin_cpp = File::create(&qml_plugin_cpp_path).unwrap();
            write!(
                qml_plugin_cpp,
                r#"
#include <QtQml/qqmlextensionplugin.h>

extern void {register_types_function}();

class {plugin_class_name} : public QQmlEngineExtensionPlugin
{{
    Q_OBJECT
    Q_PLUGIN_METADATA(IID "org.qt-project.Qt.QQmlEngineExtensionInterface")

public:
    {plugin_class_name}(QObject *parent = nullptr) : QQmlEngineExtensionPlugin(parent)
    {{
        volatile auto registration = &{register_types_function};
        Q_UNUSED(registration);
    }}
}};

// The moc-generated cpp file doesn't compile on its own; it needs to be #included here.
#include "moc_{plugin_class_name}.cpp.cpp"
"#
            )
            .unwrap();
            self.moc(&qml_plugin_cpp_path, Some(uri));

            // Generate file to load static QQmlExtensionPlugin
            let mut qml_plugin_init = File::create(&qml_plugin_init_path).unwrap();
            write!(
                qml_plugin_init,
                r#"
#include <QtPlugin>
Q_IMPORT_PLUGIN({plugin_class_name});
"#
            )
            .unwrap();
        }

        QmlModuleRegistrationFiles {
            rcc: self.qrc(&qrc_path),
            qmlcachegen: qmlcachegen_file_paths,
            qmltyperegistrar: qmltyperegistrar_output_path,
            plugin: qml_plugin_cpp_path,
            plugin_init: qml_plugin_init_path,
        }
    }

    /// Run [rcc](https://doc.qt.io/qt-6/resources.html) on a .qrc file and save the output into [cargo's OUT_DIR](https://doc.rust-lang.org/cargo/reference/environment-variables.html).
    /// The path to the generated C++ file is returned, which can then be passed to [cc::Build::files](https://docs.rs/cc/latest/cc/struct.Build.html#method.file).
    /// The compiled static library must be linked with [+whole-archive](https://doc.rust-lang.org/rustc/command-line-arguments.html#linking-modifiers-whole-archive)
    /// or the linker will discard the generated static variables because they are not referenced from `main`.
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

        let cmd = Command::new(self.rcc_executable.as_ref().unwrap())
            .args([
                input_path.to_str().unwrap(),
                "-o",
                output_path.to_str().unwrap(),
                "--name",
                input_path.file_name().unwrap().to_str().unwrap(),
            ])
            .output()
            .unwrap_or_else(|_| panic!("rcc failed for {}", input_path.display()));

        if !cmd.status.success() {
            panic!(
                "rcc failed for {}:\n{}",
                input_path.display(),
                String::from_utf8_lossy(&cmd.stderr)
            );
        }

        output_path
    }

    /// Run [rcc](https://doc.qt.io/qt-6/resources.html) on a .qrc file and return the paths of the sources
    pub fn qrc_list(&mut self, input_file: &impl AsRef<Path>) -> Vec<PathBuf> {
        if self.rcc_executable.is_none() {
            self.rcc_executable = Some(self.get_qt_tool("rcc").expect("Could not find rcc"));
        }

        // Add the qrc file contents to the cargo rerun list
        let input_path = input_file.as_ref();
        let cmd_list = Command::new(self.rcc_executable.as_ref().unwrap())
            .args(["--list", input_path.to_str().unwrap()])
            .output()
            .unwrap_or_else(|_| panic!("rcc --list failed for {}", input_path.display()));

        if !cmd_list.status.success() {
            panic!(
                "rcc --list failed for {}:\n{}",
                input_path.display(),
                String::from_utf8_lossy(&cmd_list.stderr)
            );
        }

        String::from_utf8_lossy(&cmd_list.stdout)
            .split('\n')
            .map(PathBuf::from)
            .collect()
    }
}
