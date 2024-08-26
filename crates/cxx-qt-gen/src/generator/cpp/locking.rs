// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::generator::cpp::qobject::GeneratedCppQObjectBlocks;
use indoc::formatdoc;
use syn::Result;

pub fn generate(base_class: Option<&str>) -> Result<(String, GeneratedCppQObjectBlocks)> {
    let mut result = GeneratedCppQObjectBlocks::default();

    result
        .includes
        .insert("#include <cxx-qt/locking.h>".to_owned());

    const LOCKING: &str = "::rust::cxxqt1::CxxQtLocking";
    const NULL: &str = "::rust::cxxqt1::CxxQtNull";

    // The base class may already inherit from CxxQtLocking, so make sure to only inherit once.
    if let Some(base_class) = base_class {
        // TODO: Do we need to look up the base_class in the Type names?

        result.includes.insert("#include <type_traits>".to_owned());

        let locking_class = formatdoc!(
            "::std::conditional<
                ::std::is_base_of<{LOCKING}, {base_class}>::value,
                {NULL},
                {LOCKING}>::type"
        );

        let class_initializer = format!("{locking_class}()");
        result.base_classes.push(locking_class);

        Ok((class_initializer, result))
    } else {
        result.base_classes.push(LOCKING.to_owned());
        let class_initializer = format!("{LOCKING}()");
        Ok((class_initializer, result))
    }
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
