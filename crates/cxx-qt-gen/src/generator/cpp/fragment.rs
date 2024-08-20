// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[derive(PartialEq, Eq, Debug, Clone)]
/// A fragment of C++ code
pub enum CppFragment {
    /// A fragment which only both a header and a source
    Pair {
        /// The header of the fragment
        header: String,
        /// The source of the fragment
        source: String,
    },
    /// A fragment which only has a header
    Header(String),
    /// A fragment which only has a source
    Source(String),
}

#[cfg(test)]
impl Into<String> for CppFragment {
    fn into(self) -> String {
        match self {
            CppFragment::Pair { header, source } => {
                format!("Header:\n  {header}\nSource:\n  {source}")
            }
            CppFragment::Header(header) => header,
            CppFragment::Source(source) => source,
        }
    }
}

impl Default for CppFragment {
    fn default() -> Self {
        CppFragment::Pair {
            header: String::new(),
            source: String::new(),
        }
    }
}

pub struct CppNamedType {
    pub ident: String,
    pub ty: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_creation() {
        assert_eq!(
            CppFragment::default(),
            CppFragment::Pair {
                header: String::new(),
                source: String::new()
            }
        )
    }

    #[test]
    fn test_into_string() {
        let source = CppFragment::Source(String::from("SOURCE"));
        let str_repr: String = source.into();
        assert_eq!(str_repr.as_str(), "SOURCE")
    }
}
