// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::path::PathBuf;

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
