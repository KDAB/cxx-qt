// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

/// A trait to allow indenting multi-line string
/// This is specifically useful when using formatdoc! with a multi-line string argument.
/// As the formatdoc! formatting doesn't support indenting multi-line arguments, we can indent
/// those ourselves.
pub(crate) trait Indent {
    fn indented(&self, indent: usize) -> String;
}

impl Indent for str {
    fn indented(&self, indent: usize) -> String {
        self.lines()
            .map(|line| " ".repeat(indent) + line)
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::{formatdoc, indoc};
    use pretty_assertions::assert_str_eq;

    #[test]
    fn indent_string() {
        let multiline_string = indoc! { r"
            A,
            B,
        "};

        assert_str_eq!(
            formatdoc! { r"
            enum Test {{
            {multiline_string}
            }}
        ", multiline_string = multiline_string.indented(2) },
            indoc! { r"
            enum Test {
              A,
              B,
            }
        "}
        );
    }
}
