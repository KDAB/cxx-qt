// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

/// A builder for representing a QML uri
#[derive(Clone)]
pub struct QmlUri {
    uri: Vec<String>,
}

impl From<&str> for QmlUri {
    fn from(value: &str) -> Self {
        Self::new(value.split('.'))
    }
}

impl QmlUri {
    /// Construct a [QmlUriBuilder] from a given string
    ///
    /// If the uri segments are not alphanumeric this will panic
    pub fn new(uri: impl IntoIterator<Item = impl Into<String>>) -> Self {
        let uri: Vec<_> = uri.into_iter().map(Into::into).collect();

        // Only allow alphanumeric uri parts for now
        if uri
            .iter()
            .any(|part| part.chars().any(|c| !c.is_ascii_alphanumeric()))
        {
            panic!("QML uri parts must be alphanumeric");
        }

        Self { uri }
    }

    /// Retrieve the QML uri in directory form
    pub fn as_dirs(&self) -> String {
        self.uri.join("/")
    }

    /// Retrieve the QML uri in dot form
    pub fn as_dots(&self) -> String {
        self.uri.join(".")
    }

    /// Retrieve the QML uri in underscore form
    pub fn as_underscores(&self) -> String {
        self.uri.join("_")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn uri() {
        assert_eq!(QmlUri::from("a.b.c").uri, ["a", "b", "c"]);
        assert_eq!(QmlUri::new(["a", "b", "c"]).uri, ["a", "b", "c"]);
    }

    #[test]
    #[should_panic]
    fn uri_invalid() {
        QmlUri::new(["a,b"]);
    }

    #[test]
    fn as_n() {
        let uri = QmlUri::new(["a", "b", "c"]);
        assert_eq!(uri.as_dirs(), "a/b/c");
        assert_eq!(uri.as_dots(), "a.b.c");
        assert_eq!(uri.as_underscores(), "a_b_c");
    }
}
