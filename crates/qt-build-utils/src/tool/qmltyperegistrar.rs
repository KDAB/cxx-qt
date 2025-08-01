// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{QtInstallation, QtTool};
use semver::Version;
use std::{
    path::{Path, PathBuf},
    process::Command,
};

/// A wrapper around the [qmltyperegistrar](https://www.qt.io/blog/qml-type-registration-in-qt-5.15) tool
pub struct QtToolQmlTypeRegistrar {
    executable: PathBuf,
}

impl QtToolQmlTypeRegistrar {
    /// Construct a [QtToolQmlTypeRegistrar] from a given [QtInstallation]
    pub fn new(qt_installation: &dyn QtInstallation) -> Self {
        let executable = qt_installation
            .try_find_tool(QtTool::QmlTypeRegistrar)
            .expect("Could not find qmltyperegistrar");

        Self { executable }
    }

    /// Run [qmltyperegistrar](https://www.qt.io/blog/qml-type-registration-in-qt-5.15)
    pub fn compile(
        &self,
        metatypes_json: &[impl AsRef<Path>],
        qmltypes: impl AsRef<Path>,
        uri: &str,
        version: Version,
    ) -> Option<PathBuf> {
        // Filter out empty jsons
        let metatypes_json: Vec<_> = metatypes_json
            .iter()
            .filter(|f| {
                std::fs::metadata(f)
                    .unwrap_or_else(|_| panic!("couldn't open json file {}", f.as_ref().display()))
                    .len()
                    > 0
            })
            .map(|f| f.as_ref().to_string_lossy().into_owned())
            .collect();

        // Only run qmltyperegistrar if we have valid json files left out
        if metatypes_json.is_empty() {
            return None;
        }

        let qml_uri_underscores = uri.replace('.', "_");
        // TODO: note before this was the plugin folder
        let output_folder = QtTool::QmlTypeRegistrar.writable_path();
        std::fs::create_dir_all(&output_folder).expect("Could not create qmltyperegistrar dir");
        let qmltyperegistrar_output_path =
            output_folder.join(format!("{qml_uri_underscores}_qmltyperegistration.cpp"));

        let mut args = vec![
            "--generate-qmltypes".to_owned(),
            qmltypes.as_ref().to_string_lossy().into_owned(),
            "--major-version".to_owned(),
            version.major.to_string(),
            "--minor-version".to_owned(),
            version.minor.to_string(),
            "--import-name".to_owned(),
            uri.to_string(),
            "-o".to_owned(),
            qmltyperegistrar_output_path.to_string_lossy().into_owned(),
        ];
        args.extend(metatypes_json);
        let cmd = Command::new(&self.executable)
            .args(args)
            .output()
            .unwrap_or_else(|_| panic!("qmltyperegistrar failed for {uri}"));
        if !cmd.status.success() {
            panic!(
                "qmltyperegistrar failed for {uri}:\n{}",
                String::from_utf8_lossy(&cmd.stderr)
            );
        }

        Some(qmltyperegistrar_output_path)
    }
}
