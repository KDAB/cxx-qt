// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

/// An individial `<file>` line within a [QResourceBuilder]
pub struct QResourceFile {
    alias: Option<String>,
    // TODO: compression
    // TODO: empty
    path: String,
}

impl QResourceFile {
    /// Construct a [QResourceFile]
    fn new(path: String) -> Self {
        Self { alias: None, path }
    }

    /// Specify an alias for the [QResourceFile]
    pub fn alias(mut self, alias: String) -> Self {
        self.alias = Some(alias);
        self
    }

    fn build(self) -> String {
        let alias = self
            .alias
            .map(|alias| format!("alias=\"{}\"", alias.escape_default()))
            .unwrap_or_default();
        let path = self.path;
        format!("<file {alias}>{path}</file>")
    }
}

/// A `<qresource>` block within a [QResourceBuilder]
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

impl QResource {
    /// Construct a [QResource]
    fn new() -> Self {
        Self {
            language: None,
            prefix: None,
            files: vec![],
        }
    }

    /// Add a [QResourceFile] with the given path
    pub fn file(self, path: String) -> Self {
        self.file_with_opts(path, |file| file)
    }

    /// Add a [QResourceFile] with the given path and apply additional options
    pub fn file_with_opts(
        mut self,
        path: String,
        opts: impl FnOnce(QResourceFile) -> QResourceFile,
    ) -> Self {
        self.files.push(opts(QResourceFile::new(path)));
        self
    }

    /// Specify a language for the `<qresource>`
    pub fn language(mut self, language: String) -> Self {
        self.language = Some(language);
        self
    }

    /// Specify a prefix for the `<qresource>`
    pub fn prefix(mut self, prefix: String) -> Self {
        self.prefix = Some(prefix);
        self
    }

    fn build(self) -> String {
        let language = self
            .language
            .map(|language| format!("language=\"{}\"", language.escape_default()))
            .unwrap_or_default();
        let prefix = self
            .prefix
            .map(|prefix| format!("prefix=\"{}\"", prefix.escape_default()))
            .unwrap_or_default();
        let files = self
            .files
            .into_iter()
            .map(QResourceFile::build)
            .collect::<Vec<_>>()
            .join("");
        format!("<qresource {language} {prefix}>{files}</qresource>")
    }
}

/// A helper for building Qt resource collection files
pub struct QResourceBuilder {
    resources: Vec<QResource>,
}

impl Default for QResourceBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl QResourceBuilder {
    /// Construct a [QResourceBuilder]
    pub fn new() -> Self {
        Self { resources: vec![] }
    }

    /// Add a [QResource] to the builder with given options
    pub fn resource(mut self, opts: impl FnOnce(QResource) -> QResource) -> Self {
        self.resources.push(opts(QResource::new()));
        self
    }

    /// Convert to a string representation
    pub fn build(self) -> String {
        let resources = self
            .resources
            .into_iter()
            .map(QResource::build)
            .collect::<Vec<_>>()
            .join("");
        format!("<RCC>{resources}</RCC>")
    }
}
