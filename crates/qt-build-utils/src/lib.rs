// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Be Wilson <be.wilson@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#![deny(missing_docs)]

//! This crate provides information about the Qt installation and can invoke Qt's
//! [moc](https://doc.qt.io/qt-6/moc.html) code generator. This crate does not build
//! any C++ code on its own. It is intended to be used in [build.rs scripts](https://doc.rust-lang.org/cargo/reference/build-scripts.html)
//! together with
//! [cc](https://docs.rs/cc/latest/cc/),
//! [cxx_build](https://docs.rs/cxx-build/latest/cxx_build/), or
//! [cpp_build](https://docs.rs/cpp_build/latest/cpp_build/).

#![allow(clippy::too_many_arguments)]

mod parse_cflags;

mod tool;
pub use tool::QtTool;

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
/// Errors that can occur while using [QtBuild]
pub enum QtBuildError {
    /// `QMAKE` environment variable was set but Qt was not detected
    #[error("QMAKE environment variable specified as {qmake_env_var} but could not detect Qt: {error:?}")]
    QMakeSetQtMissing {
        /// The value of the qmake environment variable when the error occurred
        qmake_env_var: String,
        /// The inner error that occurred
        error: Box<anyhow::Error>,
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
        /// The Qt major version from `QT_VERSION_MAJOR`
        qt_version_major_env_var: String,
        /// The [std::num::ParseIntError] when parsing the `QT_VERSION_MAJOR`
        source: std::num::ParseIntError,
    },
    /// `QT_VERSION_MAJOR` environment variable was specified but the Qt version specified by `qmake -query QT_VERSION` did not match
    #[error("qmake version ({qmake_version}) does not match version specified by QT_VERSION_MAJOR ({qt_version_major})")]
    QtVersionMajorDoesNotMatch {
        /// The qmake version
        qmake_version: u64,
        /// The Qt major version from `QT_VERSION_MAJOR`
        qt_version_major: u64,
    },
}

fn command_help_output(command: &str) -> std::io::Result<std::process::Output> {
    Command::new(command).args(["--help"]).output()
}

/// Whether apple is the current target
fn is_apple_target() -> bool {
    env::var("TARGET")
        .map(|target| target.contains("apple"))
        .unwrap_or_else(|_| false)
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
            println!("cargo::rustc-link-arg=-fapple-link-rtlib");
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
            println!("cargo::rustc-link-arg=-fuse-ld=lld");
        } else if command_help_output("ld.gold").is_ok() {
            println!("cargo::rustc-link-arg=-fuse-ld=gold");
        } else if command_help_output("mold").is_ok() {
            println!("cargo::rustc-link-arg=-fuse-ld=mold");
        } else {
            println!("cargo::warning=Neither mold, lld, nor gold linkers were found. Linking with GNU ld.bfd will likely fail.");
        }
    }
}

#[doc(hidden)]
#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Initializer {
    pub file: Option<PathBuf>,
    pub init_call: Option<String>,
    pub init_declaration: Option<String>,
}

impl Initializer {
    #[doc(hidden)]
    pub fn default_signature(name: &str) -> Self {
        Self {
            file: None,
            init_call: Some(format!("{name}();")),
            init_declaration: Some(format!("extern \"C\" bool {name}();")),
        }
    }

    #[doc(hidden)]
    // Strip the init files from the public initializers
    // For downstream dependencies, it's often enough to just declare the init function and
    // call it.
    pub fn strip_file(mut self) -> Self {
        self.file = None;
        self
    }
}

/// Paths to files generated by [QtBuild::moc]
pub struct MocProducts {
    /// Generated C++ file
    pub cpp: PathBuf,
    /// Generated JSON file
    pub metatypes_json: PathBuf,
}

/// Arguments for a Qt moc invocation.
/// See: [QtBuild::moc]
#[derive(Default, Clone)]
pub struct MocArguments {
    uri: Option<String>,
    include_paths: Vec<PathBuf>,
}

impl MocArguments {
    /// Should be passed if the input_file is part of a QML module
    pub fn uri(mut self, uri: String) -> Self {
        self.uri = Some(uri);
        self
    }

    /// Additional include path to pass to moc
    pub fn include_path(mut self, include_path: PathBuf) -> Self {
        self.include_paths.push(include_path);
        self
    }

    /// Additional include paths to pass to moc.
    pub fn include_paths(mut self, mut include_paths: Vec<PathBuf>) -> Self {
        self.include_paths.append(&mut include_paths);
        self
    }
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
    /// Initializer that automatically registers the QQmlExtensionPlugin at startup.
    pub plugin_init: Initializer,
    /// An optional include path that should be included
    pub include_path: Option<PathBuf>,
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
    pub fn new(mut qt_modules: Vec<String>) -> Result<Self, QtBuildError> {
        if qt_modules.is_empty() {
            qt_modules.push("Core".to_string());
        }
        println!("cargo::rerun-if-env-changed=QMAKE");
        println!("cargo::rerun-if-env-changed=QT_VERSION_MAJOR");
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
                        let qmake_version = versions::SemVer::new(version_string).unwrap();
                        if let Ok(env_version) = env::var("QT_VERSION_MAJOR") {
                            let env_version = match env_version.trim().parse::<u64>() {
                                Err(e) if *e.kind() == std::num::IntErrorKind::Empty => {
                                    println!(
                                        "cargo::warning=QT_VERSION_MAJOR environment variable defined but empty"
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
                            if env_version == qmake_version.major as u64 {
                                return Ok((candidate, qmake_version));
                            } else {
                                return Err(QtBuildError::QtVersionMajorDoesNotMatch {
                                    qmake_version: qmake_version.major as u64,
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
                        error: Box::new(e.into()),
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

    /// Tell Cargo to link each Qt module.
    pub fn cargo_link_libraries(&self, builder: &mut cc::Build) {
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
        if is_apple_target() {
            println!("cargo::rustc-link-search=framework={lib_path}");

            // Ensure that any framework paths are set to -F
            for framework_path in self.framework_paths() {
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

        for qt_module in &self.qt_modules {
            let framework = if is_apple_target() {
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
            println!("cargo::rustc-link-search={platforms_path}");
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

    /// Get the framework paths for Qt. This is intended
    /// to be passed to whichever tool you are using to invoke the C++ compiler.
    pub fn framework_paths(&self) -> Vec<PathBuf> {
        let mut framework_paths = vec![];

        if is_apple_target() {
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

    /// Get the include paths for Qt, including Qt module subdirectories. This is intended
    /// to be passed to whichever tool you are using to invoke the C++ compiler.
    pub fn include_paths(&self) -> Vec<PathBuf> {
        let root_path = self.qmake_query("QT_INSTALL_HEADERS");
        let lib_path = self.qmake_query("QT_INSTALL_LIBS");
        let mut paths = Vec::new();
        for qt_module in &self.qt_modules {
            // Add the usual location for the Qt module
            paths.push(format!("{root_path}/Qt{qt_module}"));

            // Ensure that we add any framework's headers path
            let header_path = format!("{lib_path}/Qt{qt_module}.framework/Headers");
            if is_apple_target() && Path::new(&header_path).exists() {
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
    pub fn moc(&mut self, input_file: impl AsRef<Path>, arguments: MocArguments) -> MocProducts {
        if self.moc_executable.is_none() {
            self.moc_executable = Some(self.get_qt_tool("moc").expect("Could not find moc"));
        }

        let input_path = input_file.as_ref();

        // Put all the moc files into one place, this can then be added to the include path
        let moc_dir = PathBuf::from(format!(
            "{}/qt-build-utils/moc",
            env::var("OUT_DIR").unwrap()
        ));
        std::fs::create_dir_all(&moc_dir).expect("Could not create moc dir");
        let output_path = moc_dir.join(format!(
            "moc_{}.cpp",
            input_path.file_name().unwrap().to_str().unwrap()
        ));

        let metatypes_json_path = PathBuf::from(&format!("{}.json", output_path.display()));

        let mut include_args = vec![];
        // Qt includes
        for include_path in self
            .include_paths()
            .iter()
            .chain(arguments.include_paths.iter())
        {
            include_args.push(format!("-I{}", include_path.display()));
        }

        let mut cmd = Command::new(self.moc_executable.as_ref().unwrap());

        if let Some(uri) = arguments.uri {
            cmd.arg(format!("-Muri={uri}"));
        }

        cmd.args(include_args);
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
        let qt_build_utils_dir = PathBuf::from(format!("{out_dir}/qt-build-utils"));
        std::fs::create_dir_all(&qt_build_utils_dir).expect("Could not create qt_build_utils dir");

        let qml_module_dir = qt_build_utils_dir.join("qml_modules").join(&qml_uri_dirs);
        std::fs::create_dir_all(&qml_module_dir).expect("Could not create QML module directory");

        let qml_uri_underscores = uri.replace('.', "_");
        let qmltypes_path = qml_module_dir.join("plugin.qmltypes");
        let plugin_class_name = format!("{qml_uri_underscores}_plugin");

        // Generate qmldir file
        let qmldir_file_path = qml_module_dir.join("qmldir");
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
        let qrc_path =
            qml_module_dir.join(format!("qml_module_resources_{qml_uri_underscores}.qrc"));
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
            let qml_module_dir_str = qml_module_dir.to_str().unwrap();
            write!(
                qrc,
                r#"<RCC>
<qresource prefix="/">
    <file alias="/qt/qml/{qml_uri_dirs}">{qml_module_dir_str}</file>
</qresource>
<qresource prefix="/qt/qml/{qml_uri_dirs}">
{qml_files_qrc}
    <file alias="qmldir">{qml_module_dir_str}/qmldir</file>
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
            let qmlcachegen_dir = qt_build_utils_dir.join("qmlcachegen").join(&qml_uri_dirs);
            std::fs::create_dir_all(&qmlcachegen_dir)
                .expect("Could not create qmlcachegen directory for QML module");

            let common_args = [
                "-i".to_string(),
                qmldir_file_path.to_string_lossy().to_string(),
                "--resource".to_string(),
                qrc_path.to_string_lossy().to_string(),
            ];

            let mut qml_file_qrc_paths = Vec::new();
            for file in qml_files {
                let qrc_resource_path =
                    format!("/qt/qml/{qml_uri_dirs}/{}", file.as_ref().display());

                let qml_compiled_file = qmlcachegen_dir.join(format!(
                    "{}.cpp",
                    file.as_ref().file_name().unwrap().to_string_lossy()
                ));
                qmlcachegen_file_paths.push(PathBuf::from(&qml_compiled_file));

                let specific_args = vec![
                    "--resource-path".to_string(),
                    qrc_resource_path.clone(),
                    "-o".to_string(),
                    qml_compiled_file.to_string_lossy().to_string(),
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

            let qmlcachegen_loader = qmlcachegen_dir.join("qmlcache_loader.cpp");
            let specific_args = vec![
                "--resource-name".to_string(),
                format!("qmlcache_{qml_uri_underscores}"),
                "-o".to_string(),
                qmlcachegen_loader.to_string_lossy().to_string(),
            ];

            // If there are no QML files there is nothing for qmlcachegen to run with
            if !qml_files.is_empty() {
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
        }

        let qml_plugin_dir = PathBuf::from(format!("{out_dir}/qt-build-utils/qml_plugin"));
        std::fs::create_dir_all(&qml_plugin_dir).expect("Could not create qml_plugin dir");

        // Run qmltyperegistrar
        let qmltyperegistrar_output_path =
            qml_plugin_dir.join(format!("{qml_uri_underscores}_qmltyperegistration.cpp"));

        // Filter out empty jsons
        let metatypes_json: Vec<_> = metatypes_json
            .iter()
            .filter(|f| {
                std::fs::metadata(f)
                    .unwrap_or_else(|_| {
                        panic!("couldn't open json file {}", f.as_ref().to_string_lossy())
                    })
                    .len()
                    > 0
            })
            .map(|f| f.as_ref().to_string_lossy().to_string())
            .collect();

        // Only run qmltyperegistrar if we have valid json files left out
        if !metatypes_json.is_empty() {
            let mut args = vec![
                "--generate-qmltypes".to_string(),
                qmltypes_path.to_string_lossy().to_string(),
                "--major-version".to_string(),
                version_major.to_string(),
                "--minor-version".to_string(),
                version_minor.to_string(),
                "--import-name".to_string(),
                uri.to_string(),
                "-o".to_string(),
                qmltyperegistrar_output_path.to_string_lossy().to_string(),
            ];
            args.extend(metatypes_json);
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
        let qml_plugin_cpp_path = qml_plugin_dir.join(format!("{plugin_class_name}.cpp"));
        let include_path;
        {
            let mut declarations = Vec::default();
            let mut usages = Vec::default();

            let mut generate_usage = |return_type: &str, function_name: &str| {
                declarations.push(format!("extern {return_type} {function_name}();"));
                usages.push(format!("volatile auto {function_name}_usage = &{function_name};\nQ_UNUSED({function_name}_usage);"));
            };

            // This function is generated by qmltyperegistrar
            generate_usage("void", &format!("qml_register_types_{qml_uri_underscores}"));
            generate_usage(
                "int",
                &format!("qInitResources_qml_module_resources_{qml_uri_underscores}_qrc"),
            );

            if !qml_files.is_empty() && self.qmlcachegen_executable.is_some() {
                generate_usage(
                    "int",
                    &format!("qInitResources_qmlcache_{qml_uri_underscores}"),
                );
            }
            let declarations = declarations.join("\n");
            let usages = usages.join("\n");

            std::fs::write(
                &qml_plugin_cpp_path,
                format!(
                    r#"
#include <QtQml/qqmlextensionplugin.h>

// TODO: Add missing handling for GHS (Green Hills Software compiler) that is in
// https://code.qt.io/cgit/qt/qtbase.git/plain/src/corelib/global/qtsymbolmacros.h
{declarations}

class {plugin_class_name} : public QQmlEngineExtensionPlugin
{{
    Q_OBJECT
    Q_PLUGIN_METADATA(IID "org.qt-project.Qt.QQmlEngineExtensionInterface")

public:
    {plugin_class_name}(QObject *parent = nullptr) : QQmlEngineExtensionPlugin(parent)
    {{
        {usages}
    }}
}};

// The moc-generated cpp file doesn't compile on its own; it needs to be #included here.
#include "moc_{plugin_class_name}.cpp.cpp"
"#,
                ),
            )
            .expect("Failed to write plugin definition");

            let moc_product = self.moc(
                &qml_plugin_cpp_path,
                MocArguments::default().uri(uri.to_owned()),
            );
            // Pass the include directory of the moc file to the caller
            include_path = moc_product.cpp.parent().map(|path| path.to_path_buf());

            // Generate Initializer for static QQmlExtensionPlugin
            let plugin_init = Initializer {
                file: None,
                init_call: None,
                init_declaration: Some(format!(
                    r#"
#include <QtPlugin>
Q_IMPORT_PLUGIN({plugin_class_name});
"#
                )),
            };

            let rcc = self.qrc(&qrc_path);
            QmlModuleRegistrationFiles {
                // The rcc file is automatically initialized when importing the plugin.
                // so we don't need to treat it like an initializer here.
                rcc: rcc.file.unwrap(),
                qmlcachegen: qmlcachegen_file_paths,
                qmltyperegistrar: qmltyperegistrar_output_path,
                plugin: qml_plugin_cpp_path,
                plugin_init,
                include_path,
            }
        }
    }

    /// Run [rcc](https://doc.qt.io/qt-6/resources.html) on a .qrc file and save the output into [cargo's OUT_DIR](https://doc.rust-lang.org/cargo/reference/environment-variables.html).
    /// The path to the generated C++ file is returned, which can then be passed to [cc::Build::files](https://docs.rs/cc/latest/cc/struct.Build.html#method.file).
    /// This function also returns a String that contains the name of the resource initializer
    /// function.
    /// The build system must ensure that if the .cpp file is built into a static library, either
    /// the `+whole-archive` flag is used, or the initializer function is called by the
    /// application.
    pub fn qrc(&mut self, input_file: &impl AsRef<Path>) -> Initializer {
        if self.rcc_executable.is_none() {
            self.rcc_executable = Some(self.get_qt_tool("rcc").expect("Could not find rcc"));
        }

        let input_path = input_file.as_ref();
        let output_folder = PathBuf::from(&format!(
            "{}/qt-build-utils/qrc",
            env::var("OUT_DIR").unwrap()
        ));
        std::fs::create_dir_all(&output_folder).expect("Could not create qrc dir");
        let output_path = output_folder.join(format!(
            "{}.cpp",
            input_path.file_name().unwrap().to_string_lossy(),
        ));
        let name = input_path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .replace('.', "_");

        let cmd = Command::new(self.rcc_executable.as_ref().unwrap())
            .args([
                input_path.to_str().unwrap(),
                "-o",
                output_path.to_str().unwrap(),
                "--name",
                &name,
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

        let qt_6_5 = SemVer {
            major: 6,
            minor: 5,
            ..SemVer::default()
        };
        let init_header = if self.version >= qt_6_5 {
            // With Qt6.5 the Q_INIT_RESOURCE macro is in the QtResource header
            "QtCore/QtResource"
        } else {
            "QtCore/QDir"
        };
        Initializer {
            file: Some(output_path),
            init_call: Some(format!("Q_INIT_RESOURCE({name});")),
            init_declaration: Some(format!("#include <{init_header}>")),
        }
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
