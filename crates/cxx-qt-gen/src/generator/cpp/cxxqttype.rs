// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::generator::{cpp::qobject::GeneratedCppQObjectBlocks, naming::qobject::QObjectName};
use syn::Result;

pub fn generate(qobject_idents: &QObjectName) -> Result<GeneratedCppQObjectBlocks> {
    let mut result = GeneratedCppQObjectBlocks::default();

    let rust_ident = qobject_idents.rust_struct.cpp.to_string();

    result
        .includes
        .insert("#include <cxx-qt-common/cxxqt_type.h>".to_owned());

    result
        .base_classes
        .push(format!("::rust::cxxqtlib1::CxxQtType<{rust_ident}>"));

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::generator::naming::qobject::tests::create_qobjectname;

    #[test]
    fn test_generate_cpp_cxxqt_type() {
        let qobject_idents = create_qobjectname();

        let generated = generate(&qobject_idents).unwrap();

        // includes
        assert_eq!(generated.includes.len(), 1);
        assert!(generated
            .includes
            .contains("#include <cxx-qt-common/cxxqt_type.h>"));

        // base class
        assert_eq!(generated.base_classes.len(), 1);
        assert_eq!(
            generated.base_classes[0],
            "::rust::cxxqtlib1::CxxQtType<MyObjectRust>"
        );
    }
}
