// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

use indoc::formatdoc;

use crate::{
    generator::{
        cfg::try_eval_attributes,
        cpp::{fragment::CppFragment, qobject::GeneratedCppQObjectBlocks},
        GeneratedOpt,
    },
    naming::cpp::syn_type_to_cpp_return_type,
    naming::TypeNames,
    parser::inherit::ParsedInheritedMethod,
};

use syn::Result;

pub fn generate(
    inherited_methods: &[&ParsedInheritedMethod],
    base_class: Option<&str>,
    type_names: &TypeNames,
    opt: &GeneratedOpt,
) -> Result<GeneratedCppQObjectBlocks> {
    let mut result = GeneratedCppQObjectBlocks::default();

    for &method in inherited_methods {
        // Skip if the cfg attributes are not resolved to true
        if !try_eval_attributes(opt.cfg_evaluator.as_ref(), &method.cfgs)? {
            continue;
        }

        let return_type = syn_type_to_cpp_return_type(&method.method.sig.output, type_names)?;
        // Note that no qobject macro with no base class is an error
        //
        // So a default of QObject is fine here
        let base_class = base_class.unwrap_or("QObject");

        result.methods.push(CppFragment::Header(formatdoc! {
        r#"
              template <class... Args>
              {return_type} {wrapper_ident}(Args ...args){mutability}
              {{
                  return {base_class}::{func_ident}(args...);
              }}"#,
        mutability = if method.mutable { "" } else { " const" },
        func_ident = method.name.cxx_unqualified(),
        wrapper_ident = method.wrapper_ident(),
        return_type = return_type.unwrap_or_else(|| "void".to_owned()),
        base_class = base_class
        }));
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::tests::CfgEvaluatorTest;
    use pretty_assertions::assert_str_eq;
    use syn::{parse_quote, ForeignItemFn};

    use super::*;
    use crate::generator::cpp::property::tests::require_header;
    use crate::parser::inherit::ParsedInheritedMethod;
    use crate::parser::CaseConversion;

    fn generate_from_foreign(
        method: ForeignItemFn,
        base_class: Option<&str>,
    ) -> Result<GeneratedCppQObjectBlocks> {
        let method = ParsedInheritedMethod::parse(method, CaseConversion::none())?;
        let inherited_methods = vec![&method];
        generate(
            &inherited_methods,
            base_class,
            &TypeNames::default(),
            &GeneratedOpt::default(),
        )
    }

    fn assert_generated_eq(expected: &str, generated: &GeneratedCppQObjectBlocks) {
        assert_eq!(generated.methods.len(), 1);
        let header = require_header(&generated.methods[0]).unwrap();
        assert_str_eq!(header, expected);
    }

    #[test]
    fn test_cfg() {
        let method = parse_quote! {
            #[cfg(test_cfg_disabled)]
            fn test(self: &T, a: B, b: C);
        };
        let method = ParsedInheritedMethod::parse(method, CaseConversion::none()).unwrap();
        let inherited_methods = vec![&method];
        let base_class = Some("TestBaseClass");
        let opt = GeneratedOpt {
            cfg_evaluator: Box::new(CfgEvaluatorTest::default()),
        };
        let generated =
            generate(&inherited_methods, base_class, &TypeNames::default(), &opt).unwrap();

        assert!(generated.methods.is_empty());
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
