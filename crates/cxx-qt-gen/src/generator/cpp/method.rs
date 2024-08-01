// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::generator::cpp::get_cpp_params;
use crate::{
    generator::{
        cpp::{
            fragment::{CppFragment, CppNamedType},
            qobject::GeneratedCppQObjectBlocks,
        },
        naming::{method::QMethodName, qobject::QObjectNames},
    },
    naming::cpp::{syn_return_type_to_cpp_except, syn_type_to_cpp_return_type},
    naming::TypeNames,
    parser::method::{ParsedMethod, ParsedQInvokableSpecifiers},
};
use indoc::formatdoc;
use syn::Result;

pub fn generate_cpp_methods(
    invokables: &Vec<&ParsedMethod>,
    qobject_idents: &QObjectNames,
    type_names: &TypeNames,
) -> Result<GeneratedCppQObjectBlocks> {
    let mut generated = GeneratedCppQObjectBlocks::default();
    let qobject_ident = qobject_idents.name.cxx_unqualified();
    for &invokable in invokables {
        let idents = QMethodName::try_from(invokable)?;
        let return_cxx_ty = syn_type_to_cpp_return_type(&invokable.method.sig.output, type_names)?;

        let parameters: Vec<CppNamedType> = get_cpp_params(&invokable.method, type_names)?;

        let body = format!(
            "{ident}({parameter_names})",
            ident = idents.wrapper.cxx_unqualified(),
            parameter_names = parameters
                .iter()
                .map(|parameter| parameter.ident.as_str())
                .collect::<Vec<&str>>()
                .join(", "),
        );
        let parameter_types = parameters
            .iter()
            .map(|parameter| format!("{ty} {ident}", ident = parameter.ident, ty = parameter.ty))
            .collect::<Vec<String>>()
            .join(", ");
        let is_const = if !invokable.mutable { " const" } else { "" };

        let mut is_final = "";
        let mut is_override = "";
        let mut is_virtual = "";

        // Set specifiers into string values
        invokable
            .specifiers
            .iter()
            .for_each(|specifier| match specifier {
                ParsedQInvokableSpecifiers::Final => is_final = " final",
                ParsedQInvokableSpecifiers::Override => is_override = " override",
                ParsedQInvokableSpecifiers::Virtual => is_virtual = "virtual ",
            });

        // Matching return type or void
        let return_type = if let Some(return_cxx_ty) = &return_cxx_ty {
            return_cxx_ty
        } else {
            "void"
        };

        generated.methods.push(CppFragment::Pair {
            header: format!(
                "{is_qinvokable}{is_virtual}{return_cxx_ty} {ident}({parameter_types}){is_const}{is_final}{is_override};",
                return_cxx_ty = return_type,
                ident = idents.name.cxx_unqualified(),
                parameter_types = parameter_types,
                is_qinvokable = if invokable.is_qinvokable {
                    "Q_INVOKABLE "
                } else {
                    ""
                },
                is_final = is_final,
                is_override = is_override,
                is_virtual = is_virtual,
            ),
            source: formatdoc! {
                r#"
                    {return_cxx_ty}
                    {qobject_ident}::{ident}({parameter_types}){is_const}
                    {{
                        const ::rust::cxxqt1::MaybeLockGuard<{qobject_ident}> guard(*this);
                        {body};
                    }}
                    "#,
                return_cxx_ty = if let Some(return_cxx_ty) = &return_cxx_ty {
                    return_cxx_ty
                } else {
                    "void"
                },
                ident = idents.name.cxx_unqualified(),
                body = if return_cxx_ty.is_some() {
                    format!("return {body}", body = body)
                } else {
                    body
                },
            },
        });

        // Note that we are generating a header to match the extern "Rust" method
        // in Rust for our invokable.
        //
        // CXX generates the source and we just need the matching header.
        let has_noexcept = syn_return_type_to_cpp_except(&invokable.method.sig.output);
        generated.private_methods.push(CppFragment::Header(format!(
            "{return_cxx_ty} {ident}({parameter_types}){is_const} {has_noexcept};",
            return_cxx_ty = if let Some(return_cxx_ty) = &return_cxx_ty {
                return_cxx_ty
            } else {
                "void"
            },
            ident = idents.wrapper.cxx_unqualified(),
        )));
    }

    Ok(generated)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::generator::naming::qobject::tests::create_qobjectname;
    use crate::naming::Name;
    use crate::parser::parameter::ParsedFunctionParameter;
    use indoc::indoc;
    use pretty_assertions::assert_str_eq;
    use quote::format_ident;
    use std::collections::HashSet;
    use syn::{parse_quote, ForeignItemFn};

    #[test]
    fn test_generate_cpp_invokables() {
        let method1: ForeignItemFn = parse_quote! { fn void_invokable(self: &MyObject); };
        let method2: ForeignItemFn =
            parse_quote! { fn trivial_invokable(self: &MyObject, param: i32) -> i32; };
        let method3: ForeignItemFn = parse_quote! { fn opaque_invokable(self: Pin<&mut MyObject>, param: &QColor) -> UniquePtr<QColor>; };
        let method4: ForeignItemFn =
            parse_quote! { fn specifiers_invokable(self: &MyObject, param: i32) -> i32; };
        let method5: ForeignItemFn = parse_quote! { fn cpp_method(self: &MyObject); };
        let invokables = vec![
            ParsedMethod::from_method_and_params(&method1, vec![]),
            ParsedMethod::from_method_and_params(
                &method2,
                vec![ParsedFunctionParameter {
                    ident: format_ident!("param"),
                    ty: parse_quote! { i32 },
                }],
            ),
            ParsedMethod::mut_from_method_and_params(
                &method3,
                vec![ParsedFunctionParameter {
                    ident: format_ident!("param"),
                    ty: parse_quote! { &QColor },
                }],
            ),
            ParsedMethod {
                specifiers: {
                    let mut specifiers = HashSet::new();
                    specifiers.insert(ParsedQInvokableSpecifiers::Final);
                    specifiers.insert(ParsedQInvokableSpecifiers::Override);
                    specifiers.insert(ParsedQInvokableSpecifiers::Virtual);
                    specifiers
                },
                ..ParsedMethod::from_method_and_params(
                    &method4,
                    vec![ParsedFunctionParameter {
                        ident: format_ident!("param"),
                        ty: parse_quote! { i32 },
                    }],
                )
            },
            ParsedMethod {
                is_qinvokable: false,
                ..ParsedMethod::from_method_and_params(&method5, vec![])
            },
        ];
        let qobject_idents = create_qobjectname();

        let mut type_names = TypeNames::mock();
        type_names.mock_insert("QColor", None, None, None);

        let generated =
            generate_cpp_methods(&invokables.iter().collect(), &qobject_idents, &type_names)
                .unwrap();

        // methods
        assert_eq!(generated.methods.len(), 5);

        let (header, source) = if let CppFragment::Pair { header, source } = &generated.methods[0] {
            (header, source)
        } else {
            panic!("Expected pair")
        };
        assert_str_eq!(header, "Q_INVOKABLE void voidInvokable() const;");
        assert_str_eq!(
            source,
            indoc! {r#"
            void
            MyObject::voidInvokable() const
            {
                const ::rust::cxxqt1::MaybeLockGuard<MyObject> guard(*this);
                voidInvokableWrapper();
            }
            "#}
        );

        let (header, source) = if let CppFragment::Pair { header, source } = &generated.methods[1] {
            (header, source)
        } else {
            panic!("Expected pair")
        };
        assert_str_eq!(
            header,
            "Q_INVOKABLE ::std::int32_t trivialInvokable(::std::int32_t param) const;"
        );
        assert_str_eq!(
            source,
            indoc! {r#"
            ::std::int32_t
            MyObject::trivialInvokable(::std::int32_t param) const
            {
                const ::rust::cxxqt1::MaybeLockGuard<MyObject> guard(*this);
                return trivialInvokableWrapper(param);
            }
            "#}
        );

        let (header, source) = if let CppFragment::Pair { header, source } = &generated.methods[2] {
            (header, source)
        } else {
            panic!("Expected pair")
        };
        assert_str_eq!(
            header,
            "Q_INVOKABLE ::std::unique_ptr<QColor> opaqueInvokable(QColor const& param);"
        );
        assert_str_eq!(
            source,
            indoc! {r#"
            ::std::unique_ptr<QColor>
            MyObject::opaqueInvokable(QColor const& param)
            {
                const ::rust::cxxqt1::MaybeLockGuard<MyObject> guard(*this);
                return opaqueInvokableWrapper(param);
            }
            "#}
        );

        let (header, source) = if let CppFragment::Pair { header, source } = &generated.methods[3] {
            (header, source)
        } else {
            panic!("Expected pair")
        };
        assert_str_eq!(
            header,
            "Q_INVOKABLE virtual ::std::int32_t specifiersInvokable(::std::int32_t param) const final override;"
        );
        assert_str_eq!(
            source,
            indoc! {r#"
            ::std::int32_t
            MyObject::specifiersInvokable(::std::int32_t param) const
            {
                const ::rust::cxxqt1::MaybeLockGuard<MyObject> guard(*this);
                return specifiersInvokableWrapper(param);
            }
            "#}
        );

        let (header, source) = if let CppFragment::Pair { header, source } = &generated.methods[4] {
            (header, source)
        } else {
            panic!("Expected pair")
        };
        assert_str_eq!(header, "void cppMethod() const;");
        assert_str_eq!(
            source,
            indoc! {r#"
            void
            MyObject::cppMethod() const
            {
                const ::rust::cxxqt1::MaybeLockGuard<MyObject> guard(*this);
                cppMethodWrapper();
            }
            "#}
        );

        // private methods
        assert_eq!(generated.private_methods.len(), 5);

        let header = if let CppFragment::Header(header) = &generated.private_methods[0] {
            header
        } else {
            panic!("Expected header")
        };
        assert_str_eq!(header, "void voidInvokableWrapper() const noexcept;");

        let header = if let CppFragment::Header(header) = &generated.private_methods[1] {
            header
        } else {
            panic!("Expected header")
        };
        assert_str_eq!(
            header,
            "::std::int32_t trivialInvokableWrapper(::std::int32_t param) const noexcept;"
        );

        let header = if let CppFragment::Header(header) = &generated.private_methods[2] {
            header
        } else {
            panic!("Expected header")
        };
        assert_str_eq!(
            header,
            "::std::unique_ptr<QColor> opaqueInvokableWrapper(QColor const& param) noexcept;"
        );

        let header = if let CppFragment::Header(header) = &generated.private_methods[3] {
            header
        } else {
            panic!("Expected header")
        };
        assert_str_eq!(
            header,
            "::std::int32_t specifiersInvokableWrapper(::std::int32_t param) const noexcept;"
        );

        let header = if let CppFragment::Header(header) = &generated.private_methods[4] {
            header
        } else {
            panic!("Expected header")
        };
        assert_str_eq!(header, "void cppMethodWrapper() const noexcept;");
    }

    #[test]
    fn test_generate_cpp_invokables_mapped_cxx_name() {
        let method_declaration: ForeignItemFn =
            parse_quote! { fn trivial_invokable(self: &MyObject, param: A) -> B; };

        let method = ParsedMethod {
            method: method_declaration.clone(),
            qobject_ident: format_ident!("MyObject"),
            mutable: false,
            safe: true,
            parameters: vec![ParsedFunctionParameter {
                ident: format_ident!("param"),
                ty: parse_quote! { i32 },
            }],
            specifiers: HashSet::new(),
            is_qinvokable: true,
            name: Name::from_rust_ident_and_attrs(
                &method_declaration.sig.ident,
                &method_declaration.attrs,
                None,
                None,
            )
            .unwrap(),
        };
        let invokables = vec![&method];
        let qobject_idents = create_qobjectname();

        let mut type_names = TypeNames::default();
        type_names.mock_insert("A", None, Some("A1"), None);
        type_names.mock_insert("B", None, Some("B2"), None);

        let generated = generate_cpp_methods(&invokables, &qobject_idents, &type_names).unwrap();

        // methods
        assert_eq!(generated.methods.len(), 1);

        let (header, source) = if let CppFragment::Pair { header, source } = &generated.methods[0] {
            (header, source)
        } else {
            panic!("Expected pair")
        };
        assert_str_eq!(header, "Q_INVOKABLE B2 trivialInvokable(A1 param) const;");
        assert_str_eq!(
            source,
            indoc! {r#"
            B2
            MyObject::trivialInvokable(A1 param) const
            {
                const ::rust::cxxqt1::MaybeLockGuard<MyObject> guard(*this);
                return trivialInvokableWrapper(param);
            }
            "#}
        );

        // private methods
        assert_eq!(generated.private_methods.len(), 1);

        let header = if let CppFragment::Header(header) = &generated.private_methods[0] {
            header
        } else {
            panic!("Expected header")
        };
        assert_str_eq!(
            header,
            "B2 trivialInvokableWrapper(A1 param) const noexcept;"
        );
    }
}
