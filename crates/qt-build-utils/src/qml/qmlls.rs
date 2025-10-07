// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::{
    io,
    path::{Path, PathBuf},
};

/// A helper for building QML Language Server configuration files
#[derive(Default)]
pub struct QmlLsIniBuilder {
    build_dir: Option<PathBuf>,
    no_cmake_calls: Option<bool>,
}

impl QmlLsIniBuilder {
    /// Construct a [QmlLsIniBuilder]
    pub fn new() -> Self {
        Self::default()
    }

    /// Use the given build_dir
    pub fn build_dir(mut self, build_dir: impl AsRef<Path>) -> Self {
        self.build_dir = Some(build_dir.as_ref().to_path_buf());
        self
    }

    /// Enable or disable cmake calls
    pub fn no_cmake_calls(mut self, no_cmake_calls: bool) -> Self {
        self.no_cmake_calls = Some(no_cmake_calls);
        self
    }

    /// Write the resultant qmlls ini file contents
    pub fn write(self, writer: &mut impl io::Write) -> io::Result<()> {
        if self.build_dir.is_none() && self.no_cmake_calls.is_none() {
            return Ok(());
        }

        writeln!(writer, "[General]")?;

        if let Some(build_dir) = self.build_dir {
            writeln!(
                writer,
                "buildDir=\"{}\"",
                build_dir.to_string_lossy().escape_default()
            )?;
        }

        if let Some(no_cmake_calls) = self.no_cmake_calls {
            writeln!(writer, "no-cmake-calls={no_cmake_calls}",)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn qmlls() {
        let mut result = Vec::new();
        QmlLsIniBuilder::new()
            .build_dir("/a/b/c")
            .no_cmake_calls(true)
            .write(&mut result)
            .unwrap();
        assert_eq!(
            String::from_utf8(result).unwrap(),
            "[General]
buildDir=\"/a/b/c\"
no-cmake-calls=true
"
        );
    }
}
