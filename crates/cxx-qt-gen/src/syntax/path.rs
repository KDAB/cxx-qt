// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use syn::{
    spanned::Spanned, AngleBracketedGenericArguments, Error, GenericArgument, Ident, Path,
    PathArguments, Result, Type, TypePath,
};

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

/// Internal helper to extract angled brackets from a [syn::Path]
fn path_angled_to_arguments(path: &'_ Path) -> Option<&'_ AngleBracketedGenericArguments> {
    if let Some(last) = path.segments.last() {
        if let PathArguments::AngleBracketed(args) = &last.arguments {
            return Some(args);
        }
    }

    None
}

/// In std::collections::HashSet<T> extract the T as a [syn::Path]
///
/// Error if there isn't a single type path but none or many
pub fn path_angled_args_to_type_path(path: &Path) -> Result<Path> {
    let paths = path_angled_args_to_type_path_list(path);
    if paths.len() == 1 {
        Ok(paths[0].clone())
    } else {
        Err(Error::new(
            path.span(),
            "Expected only one Path in the Path's angled bracketed generic arguments",
        ))
    }
}

/// In std::collections::HashMap<K, V> extract the K, V as a Vec of [syn::Path]'s
pub fn path_angled_args_to_type_path_list(path: &Path) -> Vec<Path> {
    let mut items = vec![];
    if let Some(inner) = path_angled_to_arguments(path) {
        for arg in &inner.args {
            if let GenericArgument::Type(Type::Path(TypePath { path, .. })) = arg {
                items.push(path.clone());
            }
        }
    }

    items
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::tests::tokens_to_syn;
    use quote::quote;

    #[test]
    fn test_path_compare_str() {
        let path: Path = tokens_to_syn(quote! { a::b::c });
        assert!(!path_compare_str(&path, &["a", "b"]));
        assert!(path_compare_str(&path, &["a", "b", "c"]));
        assert!(!path_compare_str(&path, &["a", "c", "b"]));
        assert!(!path_compare_str(&path, &["a", "b", "c", "d"]));
    }

    #[test]
    fn test_path_to_single_ident() {
        let path: Path = tokens_to_syn(quote! { a::b::c });
        assert!(path_to_single_ident(&path).is_err());

        let path: Path = tokens_to_syn(quote! { a });
        let ident = path_to_single_ident(&path).unwrap();
        assert_eq!(ident, "a");
    }

    #[test]
    fn test_path_angled_args_to_type_path() {
        let path: Path = tokens_to_syn(quote! { std::collections::HashSet<a::b::c> });
        let path = path_angled_args_to_type_path(&path).unwrap();
        assert!(path_compare_str(&path, &["a", "b", "c"]));

        let path: Path = tokens_to_syn(quote! { std::collections::HashMap<a::b::c, a::b::c> });
        assert!(path_angled_args_to_type_path(&path).is_err());

        let path: Path = tokens_to_syn(quote! { std::collections::HashMap<> });
        assert!(path_angled_args_to_type_path(&path).is_err());

        let path: Path = tokens_to_syn(quote! { a::b::c });
        assert!(path_angled_args_to_type_path(&path).is_err());
    }

    #[test]
    fn test_path_angled_args_to_type_path_list() {
        let path: Path = tokens_to_syn(quote! { std::collections::HashSet<a::b::c> });
        assert_eq!(path_angled_args_to_type_path_list(&path).len(), 1);
        let path = &path_angled_args_to_type_path_list(&path)[0];
        assert!(path_compare_str(path, &["a", "b", "c"]));

        let path: Path = tokens_to_syn(quote! { std::collections::HashMap<a::b::c, a::b::c> });
        assert_eq!(path_angled_args_to_type_path_list(&path).len(), 2);

        let path: Path = tokens_to_syn(quote! { std::collections::HashMap<> });
        assert_eq!(path_angled_args_to_type_path_list(&path).len(), 0);

        let path: Path = tokens_to_syn(quote! { a::b::c });
        assert_eq!(path_angled_args_to_type_path_list(&path).len(), 0);
    }
}
