// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{
    generator::{
        cfg::try_eval_attributes,
        cpp::{
            fragment::{CppFragment, CppNamedType},
            get_cpp_params,
            qobject::GeneratedCppQObjectBlocks,
            GeneratedOpt,
        },
    },
    naming::cpp::{syn_return_type_to_cpp_except, syn_type_to_cpp_return_type},
    naming::TypeNames,
    parser::method::{ParsedMethod, ParsedQInvokableSpecifiers},
};
use syn::Result;

pub fn generate_cpp_methods(
    invokables: &Vec<&ParsedMethod>,
    type_names: &TypeNames,
    opt: &GeneratedOpt,
) -> Result<GeneratedCppQObjectBlocks> {
    let mut generated = GeneratedCppQObjectBlocks::default();
    for &invokable in invokables {
        // Skip if the cfg attributes are not resolved to true
        if !try_eval_attributes(opt.cfg_evaluator.as_ref(), &invokable.cfgs)? {
            continue;
        }

        let return_cxx_ty = syn_type_to_cpp_return_type(&invokable.method.sig.output, type_names)?;

        let parameters: Vec<CppNamedType> = get_cpp_params(&invokable.method, type_names)?;

        let parameter_types = parameters
            .iter()
            .map(|parameter| format!("{ty} {ident}", ident = parameter.ident, ty = parameter.ty))
            .collect::<Vec<String>>()
            .join(", ");
        let is_const = if !invokable.mutable { " const" } else { "" };

        let mut is_final = "";
        let mut is_override = "";
        let mut is_virtual = "";
        let mut is_pure = "";

        // Set specifiers into string values
        invokable
            .specifiers
            .iter()
            .for_each(|specifier| match specifier {
                ParsedQInvokableSpecifiers::Final => is_final = " final",
                ParsedQInvokableSpecifiers::Override => is_override = " override",
                ParsedQInvokableSpecifiers::Virtual => is_virtual = "virtual ",
                ParsedQInvokableSpecifiers::Pure => is_pure = " = 0",
            });

        let is_qinvokable = invokable
            .is_qinvokable
            .then_some("Q_INVOKABLE ")
            .unwrap_or_default();

        // Matching return type or void
        let return_cxx_ty = if let Some(return_cxx_ty) = &return_cxx_ty {
            return_cxx_ty
        } else {
            "void"
        };

        // Note that we are generating a header to match the extern "Rust" method
        // in Rust for our invokable.
        //
        // CXX generates the source and we just need the matching header.
        let has_noexcept = syn_return_type_to_cpp_except(&invokable.method.sig.output);
        generated.methods.push(CppFragment::Header(format!(
            "{is_qinvokable}{is_virtual}{return_cxx_ty} {ident}({parameter_types}){is_const} {has_noexcept}{is_final}{is_override}{is_pure};",
            ident = invokable.name.cxx_unqualified(),
        )));
    }

    Ok(generated)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::generator::cpp::property::tests::require_header;
    use crate::tests::CfgEvaluatorTest;
    use pretty_assertions::assert_str_eq;
    use std::collections::HashSet;
    use syn::{parse_quote, ForeignItemFn};

    #[test]
    fn test_generate_cpp_invokables_cfg() {
        let method: ForeignItemFn = parse_quote! {
            #[cfg(test_cfg_disabled)]
            #[cxx_name = "voidInvokable"]
            fn void_invokable(self: &MyObject);
        };
        let invokables = vec![ParsedMethod::mock_qinvokable(&method)];
        let type_names = TypeNames::mock();
        let opt = GeneratedOpt {
            cfg_evaluator: Box::new(CfgEvaluatorTest::default()),
        };
        let generated =
            generate_cpp_methods(&invokables.iter().collect(), &type_names, &opt).unwrap();

        assert!(generated.methods.is_empty());
    }

    #[test]
    fn test_generate_cpp_invokables() {
        let method1: ForeignItemFn = parse_quote! {
            #[cxx_name = "voidInvokable"]
            fn void_invokable(self: &MyObject);
        };
        let method2: ForeignItemFn = parse_quote! {
            #[cxx_name = "trivialInvokable"]
            fn trivial_invokable(self: &MyObject, param: i32) -> i32;
        };
        let method3: ForeignItemFn = parse_quote! {
            #[cxx_name = "opaqueInvokable"]
            fn opaque_invokable(self: Pin<&mut MyObject>, param: &QColor) -> UniquePtr<QColor>;
        };
        let method4: ForeignItemFn = parse_quote! {
            #[cxx_name = "specifiersInvokable"]
            fn specifiers_invokable(self: &MyObject, param: i32) -> i32;
        };
        let method5: ForeignItemFn = parse_quote! {
            #[cxx_name = "cppMethod"]
            fn cpp_method(self: &MyObject);
        };
        let invokables = vec![
            ParsedMethod::mock_qinvokable(&method1),
            ParsedMethod::mock_qinvokable(&method2),
            ParsedMethod::mock_qinvokable(&method3).make_mutable(),
            ParsedMethod::mock_qinvokable(&method4).with_specifiers({
                let mut specifiers = HashSet::new();
                specifiers.insert(ParsedQInvokableSpecifiers::Final);
                specifiers.insert(ParsedQInvokableSpecifiers::Override);
                specifiers.insert(ParsedQInvokableSpecifiers::Virtual);
                specifiers
            }),
            ParsedMethod {
                is_qinvokable: false,
                ..ParsedMethod::mock_qinvokable(&method5)
            },
        ];
        let mut type_names = TypeNames::mock();
        type_names.mock_insert("QColor", None, None, None);

        let generated = generate_cpp_methods(
            &invokables.iter().collect(),
            &type_names,
            &GeneratedOpt::default(),
        )
        .unwrap();

        // methods
        assert_eq!(generated.methods.len(), 5);

        let header = require_header(&generated.methods[0]).unwrap();
        assert_str_eq!(header, "Q_INVOKABLE void voidInvokable() const noexcept;");

        let header = require_header(&generated.methods[1]).unwrap();
        assert_str_eq!(
            header,
            "Q_INVOKABLE ::std::int32_t trivialInvokable(::std::int32_t param) const noexcept;"
        );

        let header = require_header(&generated.methods[2]).unwrap();
        assert_str_eq!(
            header,
            "Q_INVOKABLE ::std::unique_ptr<QColor> opaqueInvokable(QColor const& param) noexcept;"
        );

        let header = require_header(&generated.methods[3]).unwrap();
        assert_str_eq!(
            header,
            "Q_INVOKABLE virtual ::std::int32_t specifiersInvokable(::std::int32_t param) const noexcept final override;"
        );

        let header = require_header(&generated.methods[4]).unwrap();
        assert_str_eq!(header, "void cppMethod() const noexcept;");

        assert_eq!(generated.private_methods.len(), 0);
    }

    #[test]
    fn test_generate_cpp_invokables_mapped_cxx_name() {
        let method_declaration: ForeignItemFn = parse_quote! {
            #[cxx_name = "trivialInvokable"]
            fn trivial_invokable(self: &MyObject, param: A) -> B;
        };

        let method = ParsedMethod::mock_qinvokable(&method_declaration);
        let invokables = vec![&method];

        let mut type_names = TypeNames::default();
        type_names.mock_insert("A", None, Some("A1"), None);
        type_names.mock_insert("B", None, Some("B2"), None);

        let generated =
            generate_cpp_methods(&invokables, &type_names, &GeneratedOpt::default()).unwrap();

        // methods
        assert_eq!(generated.methods.len(), 1);

        let header = require_header(&generated.methods[0]).unwrap();
        assert_str_eq!(
            header,
            "Q_INVOKABLE B2 trivialInvokable(A1 param) const noexcept;"
        );

        // private methods
        assert_eq!(generated.private_methods.len(), 0);
    }
}
