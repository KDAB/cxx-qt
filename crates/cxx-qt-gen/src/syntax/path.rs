// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use syn::{Ident, Path, PathSegment};

/// Returns whether the [syn::Path] matches a given string slice
pub fn path_compare_str(path: &Path, string: &[&str]) -> bool {
    // Check if the following is true
    // - the lengths are the same
    // - each segment is the same (filtered out)
    path.segments.len() == string.len()
        && path
            .segments
            .iter()
            .zip(string)
            .filter(|&(segment, string)| segment.ident.to_string().as_str() != *string)
            .count()
            == 0
}

/// For a given ident and base ident build a qualified path
///
/// Eg `module` and `T` build `module::T`
pub fn path_from_idents(base: &Ident, ident: &Ident) -> Path {
    let mut qualified_path = Path::from(base.clone());
    qualified_path
        .segments
        .push(PathSegment::from(ident.clone()));
    qualified_path
}

#[cfg(test)]
mod tests {
    use quote::format_ident;
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

    #[test]
    fn test_path_from_idents() {
        let path = path_from_idents(&format_ident!("ffi"), &format_ident!("T"));
        let expected_path: Path = parse_quote! { ffi::T };
        assert_eq!(path, expected_path);
    }
}
