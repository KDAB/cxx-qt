// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::generator::{cpp::qobject::GeneratedCppQObjectBlocks, naming::qobject::QObjectNames};

pub fn generate(qobject_idents: &QObjectNames) -> GeneratedCppQObjectBlocks {
    let mut result = GeneratedCppQObjectBlocks::default();

    let rust_struct = qobject_idents.rust_struct.cxx_qualified();

    result
        .includes
        .insert("#include <cxx-qt/type.h>".to_owned());

    result
        .base_classes
        .push(format!("::rust::cxxqt1::CxxQtType<{rust_struct}>"));

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::generator::naming::qobject::tests::create_qobjectname;

    #[test]
    fn test_generate_cpp_cxxqt_type() {
        let qobject_idents = create_qobjectname();

        let generated = generate(&qobject_idents);

        // includes
        assert_eq!(generated.includes.len(), 1);
        assert!(generated.includes.contains("#include <cxx-qt/type.h>"));

        // base class
        assert_eq!(generated.base_classes.len(), 1);
        assert_eq!(
            generated.base_classes[0],
            "::rust::cxxqt1::CxxQtType<MyObjectRust>"
        );
    }
}
