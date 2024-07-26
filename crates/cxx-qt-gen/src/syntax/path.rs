// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use syn::Path;

/// Returns whether the [Path] matches a given string slice
pub fn path_compare_str(path: &Path, string: &[&str]) -> bool {
    // Check if the following is true
    // - the lengths are the same
    // - no segments are different
    path.segments.len() == string.len()
        && !path
            .segments
            .iter()
            .zip(string)
            .any(|(segment, string)| segment.ident.to_string().as_str() != *string)
}

#[cfg(test)]
mod tests {
    use syn::parse_quote;

    use super::*;

    #[test]
    fn test_path_compare_str() {
        let path: Path = parse_quote! { a::b::c };
        assert!(!path_compare_str(&path, &["a", "b"]));
        assert!(path_compare_str(&path, &["a", "b", "c"]));
        assert!(!path_compare_str(&path, &["a", "c", "b"]));
        assert!(!path_compare_str(&path, &["a", "b", "c", "d"]));
    }
}
