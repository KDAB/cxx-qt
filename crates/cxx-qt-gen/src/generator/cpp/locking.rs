// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::generator::cpp::qobject::GeneratedCppQObjectBlocks;
use syn::Result;

pub fn generate(_base_class: Option<&str>) -> Result<(String, GeneratedCppQObjectBlocks)> {
    let mut result = GeneratedCppQObjectBlocks::default();

    result
        .includes
        .insert("#include <cxx-qt/locking.h>".to_owned());

    const LOCKING: &str = "::rust::cxxqt1::CxxQtLocking";

    result.base_classes.push(format!("virtual {LOCKING}"));
    let class_initializer = format!("{LOCKING}()");
    Ok((class_initializer, result))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_cpp_locking() {
        let (initializer, generated) = generate(None).unwrap();

        // initializer
        assert_eq!(initializer, "::rust::cxxqt1::CxxQtLocking()");

        // includes
        assert_eq!(generated.includes.len(), 1);
        assert!(generated.includes.contains("#include <cxx-qt/locking.h>"));

        // base class
        assert_eq!(generated.base_classes.len(), 1);
        assert_eq!(generated.base_classes[0], "::rust::cxxqt1::CxxQtLocking");
    }

    #[test]
    fn test_generate_cpp_conditional_locking() {
        todo!();
    }
}
