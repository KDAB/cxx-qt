// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::generator::cpp::qobject::GeneratedCppQObjectBlocks;
use syn::Result;

pub fn generate() -> Result<GeneratedCppQObjectBlocks> {
    let mut result = GeneratedCppQObjectBlocks::default();

    result
        .includes
        .insert("#include <cxx-qt-common/cxxqt_locking.h>".to_owned());

    result
        .base_classes
        .push("::rust::cxxqtlib1::CxxQtLocking".to_owned());

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_cpp_locking() {
        let generated = generate().unwrap();

        // includes
        assert_eq!(generated.includes.len(), 1);
        assert!(generated
            .includes
            .contains("#include <cxx-qt-common/cxxqt_locking.h>"));

        // base class
        assert_eq!(generated.base_classes.len(), 1);
        assert_eq!(generated.base_classes[0], "::rust::cxxqtlib1::CxxQtLocking");
    }
}
