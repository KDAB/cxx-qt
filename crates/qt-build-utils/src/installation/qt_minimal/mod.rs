// SPDX-FileCopyrightText: 2026 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[allow(dead_code)]
mod artifact;
#[allow(dead_code)]
mod checksum;
#[allow(dead_code)]
mod download;

use crate::QtInstallation;

/// A implementation of [QtInstallation] using qtminimal
pub struct QtInstallationQtMinimal;

impl QtInstallation for QtInstallationQtMinimal {
    fn framework_paths(&self, _qt_modules: &[String]) -> Vec<std::path::PathBuf> {
        todo!()
    }

    fn include_paths(&self, _qt_modules: &[String]) -> Vec<std::path::PathBuf> {
        todo!()
    }

    fn link_modules(&self, _builder: &mut cc::Build, _qt_modules: &[String]) {
        todo!()
    }

    fn try_find_tool(&self, _tool: crate::QtTool) -> anyhow::Result<std::path::PathBuf> {
        todo!()
    }

    fn version(&self) -> semver::Version {
        todo!()
    }
}
