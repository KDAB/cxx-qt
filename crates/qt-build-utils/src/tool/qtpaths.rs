// SPDX-FileCopyrightText: 2026 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::{path::PathBuf, process::Command};

use crate::{QtInstallation, QtTool};

/// Arguments for [QtToolQtPaths::query]
#[derive(Default)]
pub struct QtPathsQueryArguments {
    query: Option<String>,
    qtconf: Option<String>,
    // NOTE: could add query-format here later
}

impl QtPathsQueryArguments {
    /// Which Qt property to query for
    pub fn query(mut self, query: &str) -> Self {
        self.query = Some(query.to_string());
        self
    }

    /// Path to a qt.conf file
    pub fn qtconf(mut self, qtconf: &str) -> Self {
        self.qtconf = Some(qtconf.to_string());
        self
    }
}

impl From<&str> for QtPathsQueryArguments {
    fn from(value: &str) -> Self {
        Self::default().query(value)
    }
}

/// A wrapper around the qtpaths tool
pub struct QtToolQtPaths {
    executable: PathBuf,
}

impl QtToolQtPaths {
    /// Construct a [QtToolQtPaths] from a given [QtInstallation]
    pub fn new(qt_installation: &dyn QtInstallation) -> Self {
        let executable = qt_installation
            .try_find_tool(QtTool::QtPaths)
            .expect("Could not find qtpaths");

        Self { executable }
    }

    // NOTE: if we later have query format json, could have a method that returns a HashMap of all values

    /// Find the path for a given Qt property
    ///
    /// Note: this will fail on Qt 5
    pub fn query(&self, query_args: impl Into<QtPathsQueryArguments>) -> Option<String> {
        let query_args = query_args.into();
        let mut args = vec![];

        // Determine if there is a specific query
        if let Some(query) = &query_args.query {
            args.extend(["--query", query]);
        } else {
            args.push("--query")
        }

        // Determine if there is a custom qtconf
        if let Some(qtconf) = &query_args.qtconf {
            args.extend(["--qtconf", qtconf]);
        }

        // Run the qtpaths command and trim the output
        let output = Command::new(&self.executable)
            .args(args)
            // NOTE: Qt 5 will fail as there is no -query parameter
            .output()
            .ok()?
            .stdout;
        Some(String::from_utf8_lossy(&output).trim().to_owned())
    }
}
