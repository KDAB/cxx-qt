// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{Initializer, QtInstallation, QtTool};

use semver::Version;
use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
};

/// A wrapper around the [rcc](https://doc.qt.io/qt-6/resources.html) tool
pub struct QtToolRcc {
    executable: PathBuf,
    qt_version: Version,
}

impl QtToolRcc {
    /// Construct a [QtToolRcc] from a given [QtInstallation]
    pub fn new(qt_installation: &dyn QtInstallation) -> Self {
        let executable = qt_installation
            .try_find_tool(QtTool::Rcc)
            .expect("Could not find rcc");
        let qt_version = qt_installation.version();

        Self {
            executable,
            qt_version,
        }
    }

    /// Run [rcc](https://doc.qt.io/qt-6/resources.html) on a .qrc file and save the output into [cargo's OUT_DIR](https://doc.rust-lang.org/cargo/reference/environment-variables.html).
    /// The path to the generated C++ file is returned, which can then be passed to [cc::Build::files](https://docs.rs/cc/latest/cc/struct.Build.html#method.file).
    /// This function also returns a String that contains the name of the resource initializer
    /// function.
    /// The build system must ensure that if the .cpp file is built into a static library, either
    /// the `+whole-archive` flag is used, or the initializer function is called by the
    /// application.
    pub fn compile(&self, input_file: impl AsRef<Path>) -> Initializer {
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

        let cmd = Command::new(&self.executable)
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

        let qt_6_5 = Version::new(6, 5, 0);
        let init_header = if self.qt_version >= qt_6_5 {
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
    pub fn list(&self, input_file: impl AsRef<Path>) -> Vec<PathBuf> {
        // Add the qrc file contents to the cargo rerun list
        let input_path = input_file.as_ref();
        let cmd_list = Command::new(&self.executable)
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
