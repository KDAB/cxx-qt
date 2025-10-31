// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use semver::Version;
use std::ops::RangeInclusive;

/// Helper for generating cargo cfg for a version range
pub struct CfgGenerator {
    prefix: Option<String>,
    range_major: RangeInclusive<u64>,
    range_minor: RangeInclusive<u64>,
    version: Version,
}

impl CfgGenerator {
    /// Construct a new [CfgGenerator]
    pub fn new(version: Version) -> Self {
        Self {
            range_major: (0..=99),
            range_minor: (0..=99),
            prefix: None,
            version,
        }
    }

    /// Specify a prefix for the [CfgGenerator]
    pub fn prefix(mut self, prefix: impl Into<String>) -> Self {
        self.prefix = Some(prefix.into());
        self
    }

    /// Specify a major range for the [CfgGenerator]
    pub fn range_major(mut self, range: RangeInclusive<u64>) -> Self {
        self.range_major = range;
        self
    }

    /// Specify a minor range for the [CfgGenerator]
    pub fn range_minor(mut self, range: RangeInclusive<u64>) -> Self {
        self.range_minor = range;
        self
    }

    /// Generate cargo cfg with any given prefix and ranges for the version
    pub fn build(self) {
        // Define the major version
        self.define_cfg_check_variable(
            "qt_version_major".to_string(),
            Some(
                self.range_major
                    .clone()
                    .map(|major| major.to_string())
                    .collect(),
            ),
        );
        self.define_cfg_variable(
            "qt_version_major".to_string(),
            Some(self.version.major.to_string()),
        );

        // Tell cargo about all the possible cfg variables
        for major in self.range_major.clone() {
            self.define_cfg_check_variable(format!("qt_version_at_least_{major}"), None);

            for minor in self.range_minor.clone() {
                self.define_cfg_check_variable(
                    format!("qt_version_at_least_{major}_{minor}"),
                    None,
                );
            }
        }

        // Tell cargo which major versions have been reached
        for major in *self.range_major.start()..=self.version.major {
            self.define_cfg_variable(format!("qt_version_at_least_{major}"), None);
        }

        // Tell cargo which minor versions with the major have been reached
        for minor in *self.range_minor.start()..=self.version.minor {
            let major = self.version.major;
            self.define_cfg_variable(format!("qt_version_at_least_{major}_{minor}"), None);
        }
    }
}

impl CfgGenerator {
    fn define_cfg_check_variable(&self, key: String, values: Option<Vec<String>>) {
        let key = self.key_with_prefix(key);

        if let Some(values) = values {
            let values = values
                .iter()
                // Escape and add quotes
                .map(|value| format!("\"{}\"", value.escape_default()))
                .collect::<Vec<_>>()
                .join(", ");

            println!("cargo::rustc-check-cfg=cfg({key}, values({values}))");
        } else {
            println!("cargo::rustc-check-cfg=cfg({key})");
        }
    }

    fn define_cfg_variable(&self, key: String, value: Option<String>) {
        let key = self.key_with_prefix(key);

        if let Some(value) = &value {
            println!("cargo::rustc-cfg={key}=\"{}\"", value.escape_default());
        } else {
            println!("cargo::rustc-cfg={key}");
        }

        let variable_cargo = format!("CARGO_CFG_{key}");
        std::env::set_var(variable_cargo, value.unwrap_or("true".to_string()));
    }

    fn key_with_prefix(&self, key: String) -> String {
        if let Some(prefix) = &self.prefix {
            format!("{prefix}{key}")
        } else {
            key
        }
    }
}
