// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::{
    io,
    path::{Path, PathBuf},
};

/// An individial `<file>` line within a [QResource]
#[derive(Debug, Clone)]
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

    /// Get the path of this file
    pub fn get_path(&self) -> &Path {
        &self.path
    }

    fn write(self, writer: &mut impl io::Write) -> io::Result<()> {
        let alias = self
            .alias
            .unwrap_or_else(|| self.path.to_string_lossy().to_string());
        let alias = alias.escape_default();
        #[cfg(test)]
        let path = &self.path;
        #[cfg(not(test))]
        let path = std::fs::canonicalize(self.path)?;
        writeln!(
            writer,
            "    <file alias=\"{alias}\">{path}</file>",
            path = path.display()
        )
    }
}

/// A `<qresource>` block within a [QResources]
#[derive(Debug, Clone)]
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

    /// Get the files inside of this resource
    pub fn get_files(&self) -> impl Iterator<Item = &QResourceFile> {
        self.files.iter()
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

    /// Get the prefix, if set
    pub fn get_prefix(&self) -> Option<&str> {
        self.prefix.as_deref()
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

        writeln!(writer, "  <qresource{language}{prefix}>")?;
        for file in self.files.into_iter() {
            file.write(writer)?;
        }
        writeln!(writer, "  </qresource>")
    }
}

/// A helper for building Qt resource collection files
#[derive(Debug, Clone)]
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

    /// Get the resources as mutable references
    pub fn get_resources_mut(&mut self) -> impl Iterator<Item = &mut QResource> {
        self.resources.iter_mut()
    }

    /// Get the resources
    pub fn get_resources(&self) -> impl Iterator<Item = &QResource> {
        self.resources.iter()
    }

    /// Convert to a string representation
    pub fn write(self, writer: &mut impl io::Write) -> io::Result<()> {
        writeln!(writer, "<RCC>")?;
        for resource in self.resources.into_iter() {
            resource.write(writer)?;
        }
        writeln!(writer, "</RCC>")
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
            "    <file alias=\"alias\">path</file>\n"
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
            "  <qresource language=\"language\" prefix=\"prefix\">\n  </qresource>\n"
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
            "<RCC>
  <qresource>
    <file alias=\"a\">a</file>
  </qresource>
  <qresource>
    <file alias=\"b\">b</file>
  </qresource>
  <qresource prefix=\"prefix\">
    <file alias=\"c\">c</file>
    <file alias=\"d\">d</file>
    <file alias=\"alias\">e</file>
  </qresource>
</RCC>
"
        );
    }

    #[test]
    fn resources_from_files() {
        let mut result = Vec::new();
        QResources::from(["a", "b"]).write(&mut result).unwrap();
        assert_eq!(
            String::from_utf8(result).unwrap(),
            format!(
                "<RCC>
  <qresource>
    <file alias=\"a\">a</file>
    <file alias=\"b\">b</file>
  </qresource>
</RCC>
",
            )
        );
    }
}
