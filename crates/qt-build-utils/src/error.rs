// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use thiserror::Error;

#[derive(Error, Debug)]
/// Errors that can occur while using [crate::QtBuild]
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
