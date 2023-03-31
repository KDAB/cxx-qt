// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use syn::{spanned::Spanned, Error, Ident, Path, Result};

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

/// Returns the first [syn::Ident] from the [syn::Path] segments, errors if there are none or many
pub fn path_to_single_ident(path: &Path) -> Result<Ident> {
    if path.segments.len() == 1 {
        Ok(path.segments[0].ident.clone())
    } else {
        Err(Error::new(
            path.span(),
            "Expected only one segment in the Path",
        ))
    }
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

    #[test]
    fn test_path_to_single_ident() {
        let path: Path = parse_quote! { a::b::c };
        assert!(path_to_single_ident(&path).is_err());

        let path: Path = parse_quote! { a };
        let ident = path_to_single_ident(&path).unwrap();
        assert_eq!(ident, "a");
    }
}
