// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::{env, path::PathBuf};

mod moc;
pub use moc::{MocArguments, MocProducts, QtToolMoc};

mod qmltyperegistrar;
pub use qmltyperegistrar::QtToolQmlTypeRegistrar;

mod rcc;
pub use rcc::QtToolRcc;

/// An enum representing known Qt tools
#[non_exhaustive]
#[derive(Eq, Hash, PartialEq)]
pub enum QtTool {
    /// Moc
    Moc,
    /// Rcc (Qt resources)
    Rcc,
    /// Qml cachegen
    QmlCacheGen,
    /// Qml Type Registrar
    QmlTypeRegistrar,
    // TODO: could add a Custom(&str) thing here
}

impl QtTool {
    pub(crate) fn binary_name(&self) -> &str {
        match self {
            Self::Moc => "moc",
            Self::Rcc => "rcc",
            Self::QmlCacheGen => "qmlcachegen",
            Self::QmlTypeRegistrar => "qmltyperegistrar",
        }
    }

    /// Return a directory where files can be written by this tool
    ///
    /// Note the location might not exist yet
    pub(crate) fn writable_path(&self) -> PathBuf {
        PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR was not set")).join(self.binary_name())
    }
}
