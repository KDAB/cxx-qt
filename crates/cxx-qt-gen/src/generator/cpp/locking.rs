// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::generator::cpp::qobject::GeneratedCppQObjectBlocks;
use syn::Result;

pub fn generate(_base_class: Option<&str>) -> Result<GeneratedCppQObjectBlocks> {
    let mut result = GeneratedCppQObjectBlocks::default();

    result
        .includes
        .insert("#include <cxx-qt/locking.h>".to_owned());

    const LOCKING: &str = "::rust::cxxqt1::CxxQtLocking";

    result.base_classes.push(format!("virtual {LOCKING}"));
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_cpp_locking() {
        let generated = generate(None).unwrap();

        // includes
        assert_eq!(generated.includes.len(), 1);
        assert!(generated.includes.contains("#include <cxx-qt/locking.h>"));

        // base class
        assert_eq!(generated.base_classes.len(), 1);
        assert_eq!(
            generated.base_classes[0],
            "virtual ::rust::cxxqt1::CxxQtLocking"
        );
    }
}
