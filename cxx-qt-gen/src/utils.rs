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
}
