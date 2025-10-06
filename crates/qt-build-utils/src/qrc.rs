// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::{
    io,
    path::{Path, PathBuf},
};

/// An individial `<file>` line within a [QResource]
pub struct QResourceFile {
    alias: Option<String>,
    // TODO: compression
    // TODO: empty
    path: PathBuf,
}

impl<T: AsRef<Path>> From<T> for QResourceFile {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl QResourceFile {
    /// Construct a [QResourceFile]
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            alias: None,
            path: path.as_ref().to_path_buf(),
        }
    }

    /// Specify an alias for the [QResourceFile]
    pub fn alias(mut self, alias: impl Into<String>) -> Self {
        self.alias = Some(alias.into());
        self
    }

    fn write(self, writer: &mut impl io::Write) -> io::Result<()> {
        let alias = self
            .alias
            .map(|alias| format!(" alias=\"{}\"", alias.escape_default()))
            .unwrap_or_default();
        let path = self.path.to_string_lossy();
        write!(writer, "<file{alias}>{path}</file>")
    }
}

/// A `<qresource>` block within a [QResources]
pub struct QResource {
    language: Option<String>,
    prefix: Option<String>,
    files: Vec<QResourceFile>,
}

impl Default for QResource {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Into<QResourceFile>> From<T> for QResource {
    fn from(value: T) -> Self {
        Self::new().file(value)
    }
}

impl QResource {
    /// Construct a [QResource]
    pub fn new() -> Self {
        Self {
            language: None,
            prefix: None,
            files: vec![],
        }
    }

    /// Add a [QResourceFile] to the [QResource]
    pub fn file<T: Into<QResourceFile>>(mut self, file: T) -> Self {
        self.files.push(file.into());
        self
    }

    /// Add multiple [QResourceFile] to the [QResource]
    pub fn files<T: Into<QResourceFile>>(mut self, files: impl IntoIterator<Item = T>) -> Self {
        for file in files.into_iter() {
            self.files.push(file.into());
        }
        self
    }

    /// Specify a language for the `<qresource>`
    pub fn language(mut self, language: impl Into<String>) -> Self {
        self.language = Some(language.into());
        self
    }

    /// Specify a prefix for the `<qresource>`
    pub fn prefix(mut self, prefix: impl Into<String>) -> Self {
        self.prefix = Some(prefix.into());
        self
    }

    fn write(self, writer: &mut impl io::Write) -> io::Result<()> {
        let language = self
            .language
            .map(|language| format!(" language=\"{}\"", language.escape_default()))
            .unwrap_or_default();
        let prefix = self
            .prefix
            .map(|prefix| format!(" prefix=\"{}\"", prefix.escape_default()))
            .unwrap_or_default();

        write!(writer, "<qresource{language}{prefix}>")?;
        for file in self.files.into_iter() {
            file.write(writer)?;
        }
        write!(writer, "</qresource>")
    }
}

/// A helper for building Qt resource collection files
pub struct QResources {
    resources: Vec<QResource>,
}

impl Default for QResources {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: IntoIterator<Item = impl Into<QResourceFile>>> From<T> for QResources {
    fn from(value: T) -> Self {
        Self::new().resource(QResource::new().files(value))
    }
}

impl QResources {
    /// Construct a [QResource]
    pub fn new() -> Self {
        Self { resources: vec![] }
    }

    /// Add a [QResource] to the [QResources]
    pub fn resource<T: Into<QResource>>(mut self, resource: T) -> Self {
        self.resources.push(resource.into());
        self
    }

    /// Add multiple [QResource] to the [QResources]
    pub fn resources<T: Into<QResource>>(mut self, resources: impl IntoIterator<Item = T>) -> Self {
        for resource in resources.into_iter() {
            self.resources.push(resource.into());
        }
        self
    }

    /// Convert to a string representation
    pub fn write(self, writer: &mut impl io::Write) -> io::Result<()> {
        write!(writer, "<RCC>")?;
        for resource in self.resources.into_iter() {
            resource.write(writer)?;
        }
        write!(writer, "</RCC>")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn resource_file() {
        let mut result = Vec::new();
        QResourceFile::new("path")
            .alias("alias")
            .write(&mut result)
            .unwrap();
        assert_eq!(
            String::from_utf8(result).unwrap(),
            "<file alias=\"alias\">path</file>"
        );
    }

    #[test]
    fn resource() {
        let mut result = Vec::new();
        QResource::new()
            .language("language")
            .prefix("prefix")
            .write(&mut result)
            .unwrap();
        assert_eq!(
            String::from_utf8(result).unwrap(),
            "<qresource language=\"language\" prefix=\"prefix\"></qresource>"
        );
    }

    #[test]
    fn resources() {
        let mut result = Vec::new();
        QResources::new()
            .resources(["a", "b"])
            .resource(
                QResource::new()
                    .prefix("prefix")
                    .files(["c", "d"])
                    .file(QResourceFile::new("e").alias("alias")),
            )
            .write(&mut result)
            .unwrap();
        assert_eq!(
            String::from_utf8(result).unwrap(),
            "<RCC><qresource><file>a</file></qresource><qresource><file>b</file></qresource><qresource prefix=\"prefix\"><file>c</file><file>d</file><file alias=\"alias\">e</file></qresource></RCC>"
        );
    }

    #[test]
    fn resources_from_files() {
        let mut result = Vec::new();
        QResources::from(["a", "b"]).write(&mut result).unwrap();
        assert_eq!(
            String::from_utf8(result).unwrap(),
            "<RCC><qresource><file>a</file><file>b</file></qresource></RCC>"
        );
    }
}
