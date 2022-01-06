// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

/// For a given type generate what will be the namespace
///
/// We can have `crate::module::Object` (or more parent modules) as an input for idents
/// from this `module` is chosen as the namespace suffix.
pub fn type_to_namespace(prefix: &[&str], idents: &[syn::Ident]) -> Result<Vec<String>, String> {
    // Check we have at least crate::module::Object
    if idents.len() < 3 {
        return Err("Type needs at least three parts".to_owned());
    }

    // Check that we start with crate
    //
    // We can assume the first index exists, as we have checked length above to be >= 3
    if idents[0].to_string().as_str() != "crate" {
        return Err("First ident of a type must be crate".to_owned());
    }

    // Build the namespace
    let mut namespace = prefix
        .to_vec()
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    namespace.push(idents[idents.len() - 2].to_string());

    Ok(namespace)
}

#[cfg(test)]
mod tests {
    use super::*;

    use quote::format_ident;

    #[test]
    fn type_to_namespace_err() {
        let pointer = vec![format_ident!("crate")];
        let prefix = vec!["cxx_qt"];

        // 1 ident should fail
        let namespace = type_to_namespace(&prefix, &pointer);
        assert!(namespace.is_err());
        // 0 idents should fail
        let namespace = type_to_namespace(&prefix, &[]);
        assert!(namespace.is_err());
    }

    #[test]
    fn type_to_namespace_err_two() {
        let pointer = vec![format_ident!("crate"), format_ident!("Object")];
        let prefix = vec!["cxx_qt"];

        // 2 idents should fail
        let namespace = type_to_namespace(&prefix, &pointer);
        assert!(namespace.is_err());
    }

    #[test]
    fn type_to_namespace_nested_pointer() {
        // More than three idents should pick second last
        let pointer = vec![
            format_ident!("crate"),
            format_ident!("moduleA"),
            format_ident!("moduleB"),
            format_ident!("Object"),
        ];
        let prefix = vec!["cxx_qt"];
        let namespace = type_to_namespace(&prefix, &pointer);
        assert!(namespace.is_ok());

        let namespace = namespace.unwrap();
        assert_eq!(namespace.len(), 2);
        assert_eq!(namespace[0].as_str(), "cxx_qt");
        assert_eq!(namespace[1].as_str(), "moduleB");
    }

    #[test]
    fn type_to_namespace_pointer() {
        // Three idents should pick second last
        let pointer = vec![
            format_ident!("crate"),
            format_ident!("module"),
            format_ident!("Object"),
        ];
        let prefix = vec!["cxx_qt"];
        let namespace = type_to_namespace(&prefix, &pointer);
        assert!(namespace.is_ok());

        let namespace = namespace.unwrap();
        assert_eq!(namespace.len(), 2);
        assert_eq!(namespace[0].as_str(), "cxx_qt");
        assert_eq!(namespace[1].as_str(), "module");
    }
}
