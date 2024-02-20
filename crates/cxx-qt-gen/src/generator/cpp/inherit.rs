// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

use indoc::formatdoc;

use crate::{
    generator::cpp::{fragment::CppFragment, qobject::GeneratedCppQObjectBlocks},
    naming::cpp::syn_type_to_cpp_return_type,
    naming::TypeNames,
    parser::inherit::ParsedInheritedMethod,
};

use syn::Result;

pub fn generate(
    inherited_methods: &[ParsedInheritedMethod],
    base_class: &Option<String>,
    type_names: &TypeNames,
) -> Result<GeneratedCppQObjectBlocks> {
    let mut result = GeneratedCppQObjectBlocks::default();

    for method in inherited_methods {
        let return_type = syn_type_to_cpp_return_type(&method.method.sig.output, type_names)?;
        let base_class = base_class.as_deref().unwrap_or("QObject");

        result.methods.push(CppFragment::Header(formatdoc! {
        r#"
              template <class... Args>
              {return_type} {wrapper_ident}(Args ...args){mutability}
              {{
                  return {base_class}::{func_ident}(args...);
              }}"#,
        mutability = if method.mutable { "" } else { " const" },
        func_ident = method.ident.cpp,
        wrapper_ident = method.wrapper_ident(),
        return_type = return_type.unwrap_or_else(|| "void".to_string()),
        base_class = base_class
        }));
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_str_eq;
    use syn::{parse_quote, ForeignItemFn};

    use crate::{parser::inherit::ParsedInheritedMethod, syntax::safety::Safety};

    use super::*;

    fn generate_from_foreign(
        method: ForeignItemFn,
        base_class: Option<&str>,
    ) -> Result<GeneratedCppQObjectBlocks> {
        let inherited_methods = vec![ParsedInheritedMethod::parse(method, Safety::Safe).unwrap()];
        let base_class = base_class.map(|s| s.to_owned());
        generate(&inherited_methods, &base_class, &TypeNames::default())
    }

    fn assert_generated_eq(expected: &str, generated: &GeneratedCppQObjectBlocks) {
        assert_eq!(generated.methods.len(), 1);
        if let CppFragment::Header(header) = &generated.methods[0] {
            assert_str_eq!(header, expected);
        } else {
            panic!("Expected header fragment");
        }
    }

    #[test]
    fn test_immutable() {
        let generated = generate_from_foreign(
            parse_quote! {
                fn test(self: &T, a: B, b: C);
            },
            Some("TestBaseClass"),
        )
        .unwrap();

        assert_generated_eq(
            indoc::indoc! {"
                template <class... Args>
                void testCxxQtInherit(Args ...args) const
                {
                    return TestBaseClass::test(args...);
                }"
            },
            &generated,
        );
    }

    #[test]
    fn test_mutable() {
        let generated = generate_from_foreign(
            parse_quote! {
                fn test(self: Pin<&mut T>);
            },
            Some("TestBaseClass"),
        )
        .unwrap();

        assert_generated_eq(
            indoc::indoc! {"
                template <class... Args>
                void testCxxQtInherit(Args ...args)
                {
                    return TestBaseClass::test(args...);
                }"
            },
            &generated,
        );
    }

    #[test]
    fn test_default_base_class() {
        let generated = generate_from_foreign(
            parse_quote! {
                fn test(self: &T);
            },
            None,
        )
        .unwrap();

        assert_generated_eq(
            indoc::indoc! {"
                template <class... Args>
                void testCxxQtInherit(Args ...args) const
                {
                    return QObject::test(args...);
                }"
            },
            &generated,
        );
    }
}
