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

#[derive(Clone)]
pub struct QmlCacheArguments {
    pub uri: String,
    pub qmldir_path: PathBuf,
    pub qmldir_qrc_path: PathBuf,
}

pub struct QmlCacheProducts {
    pub qml_cache_path: PathBuf,
    pub qml_resource_path: String,
}

/// A wrapper around the [qmlcachegen](https://www.qt.io/blog/qml-type-registration-in-qt-5.15) tool
pub struct QtToolQmlCacheGen {
    executable: PathBuf,
}

impl QtToolQmlCacheGen {
    /// Construct a [QtToolQmlCacheGen] from a given [QtInstallation]
    pub fn new(qt_installation: &dyn QtInstallation) -> Self {
        let executable = qt_installation
            .try_find_tool(QtTool::QmlCacheGen)
            .expect("Could not find qmlcachegen");

        Self { executable }
    }

    pub fn compile(
        &self,
        common_args: QmlCacheArguments,
        file: impl AsRef<Path>,
    ) -> QmlCacheProducts {
        let qmlcachegen_dir = QtTool::QmlCacheGen
            .writable_path()
            .join(&common_args.qml_uri_dirs);
        std::fs::create_dir_all(&qmlcachegen_dir)
            .expect("Could not create qmlcachegen directory for QML module");

        let uri = common_args.uri;
        let qml_uri_dirs = uri.replace('.', "/");

        let common_args = [
            "-i".to_string(),
            common_args.qmldir_path.to_string_lossy().to_string(),
            "--resource".to_string(),
            common_args.qmldir_qrc_path.to_string_lossy().to_string(),
        ];

        let qml_cache_file = qmlcachegen_dir.join(format!(
            "{}.cpp",
            file.as_ref().file_name().unwrap().to_string_lossy()
        ));

        let qrc_resource_path = format!("/qt/qml/{qml_uri_dirs}/{}", file.as_ref().display());

        let specific_args = vec![
            "--resource-path".to_string(),
            qrc_resource_path.to_string(),
            "-o".to_string(),
            qml_cache_file.to_string_lossy().to_string(),
            std::fs::canonicalize(file)
                .unwrap()
                .to_string_lossy()
                .to_string(),
        ];

        let cmd = Command::new(&self.executable)
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

        QmlCacheProducts {
            qml_cache_path,
            qml_resource_path,
        }
    }

    pub fn compile_loader(
        &self,
        common_args: QmlCacheArguments,
        qml_file_qrc_paths: &[PathBuf],
    ) -> PathBuf {
        let qmlcachegen_dir = QtTool::QmlCacheGen
            .writable_path()
            .join(&common_args.qml_uri_dirs);
        std::fs::create_dir_all(&qmlcachegen_dir)
            .expect("Could not create qmlcachegen directory for QML module");

        let common_args = [
            "-i".to_string(),
            common_args.qmldir_path.to_string_lossy().to_string(),
            "--resource".to_string(),
            common_args.qmldir_qrc_path.to_string_lossy().to_string(),
        ];

        let qmlcachegen_loader = qmlcachegen_dir.join("qmlcache_loader.cpp");
        let uri = common_args.uri;
        let qml_uri_dirs = uri.replace('.', "/");
        let qml_uri_underscores = common_args.qml_uri_dirs.replace('/', "_");
        let specific_args = vec![
            "--resource-name".to_string(),
            format!("qmlcache_{qml_uri_underscores}"),
            "-o".to_string(),
            qmlcachegen_loader.to_string_lossy().to_string(),
        ];

        let cmd = Command::new(&self.executable)
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

        qmlcachegen_loader
    }
}
