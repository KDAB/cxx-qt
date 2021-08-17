// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

/// Return whether the given type ident segments suggest the type is a pointer
pub fn is_type_ident_ptr(ident: &[syn::Ident]) -> bool {
    // TODO: can we improve detection?
    // currently we just assume if the first segment is crate then it must be a pointer
    //
    // A pointer must have at least three parts, as it must have at least crate, module, object.
    if ident.len() < 3 {
        false
    } else {
        // The type has at least three parts, so assume it is a pointer if first is crate
        //
        // We have checked if the ident has at least three parts so we can assume the first exists
        ident[0].to_string().as_str() == "crate"
    }
}

/// For a given type generate what will be the namespace
///
/// We can have `crate::module::Object` (or more parent modules) as an input for idents
/// from this `module` is chosen as the namespace suffix.
pub fn type_to_namespace(prefix: &[String], idents: &[syn::Ident]) -> Result<Vec<String>, String> {
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
    let mut namespace = prefix.to_vec();
    namespace.push(idents[idents.len() - 2].to_string());

    Ok(namespace)
}

#[cfg(test)]
mod tests {
    use super::*;

    use quote::format_ident;

    #[test]
    fn is_type_ident_ptr_pointer() {
        // Test that crate::module::Object is detected as a pointer
        let pointer = vec![
            format_ident!("crate"),
            format_ident!("module"),
            format_ident!("Object"),
        ];
        assert!(is_type_ident_ptr(&pointer));
    }

    #[test]
    fn is_type_ident_ptr_primitive() {
        // Test that i32 is not detected as a pointer
        let primitive = vec![format_ident!("i32")];
        assert!(!is_type_ident_ptr(&primitive));
    }

    #[test]
    fn is_type_ident_ptr_two_parts() {
        // Test that crate::Object is not detected as a pointer as we need three parts
        let two_parts = vec![format_ident!("crate"), format_ident!("Object")];
        assert!(!is_type_ident_ptr(&two_parts));
    }

    #[test]
    fn type_to_namespace_err() {
        let pointer = vec![format_ident!("crate")];
        let prefix = vec!["cxx_qt".to_owned()];

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
        let prefix = vec!["cxx_qt".to_owned()];

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
        let prefix = vec!["cxx_qt".to_owned()];
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
        let prefix = vec!["cxx_qt".to_owned()];
        let namespace = type_to_namespace(&prefix, &pointer);
        assert!(namespace.is_ok());

        let namespace = namespace.unwrap();
        assert_eq!(namespace.len(), 2);
        assert_eq!(namespace[0].as_str(), "cxx_qt");
        assert_eq!(namespace[1].as_str(), "module");
    }
}
