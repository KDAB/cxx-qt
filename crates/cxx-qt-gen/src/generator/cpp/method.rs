// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{
    generator::{
        cpp::{
            fragment::{CppFragment, CppNamedType},
            qobject::GeneratedCppQObjectBlocks,
        },
        naming::{method::QMethodName, qobject::QObjectName},
        utils::cpp::{
            syn_return_type_to_cpp_except, syn_type_to_cpp_return_type, syn_type_to_cpp_type,
        },
    },
    parser::{
        cxxqtdata::ParsedCxxMappings,
        method::{ParsedMethod, ParsedQInvokableSpecifiers},
    },
};
use indoc::formatdoc;
use syn::{spanned::Spanned, Error, FnArg, Pat, PatIdent, PatType, Result};

pub fn generate_cpp_methods(
    invokables: &Vec<ParsedMethod>,
    qobject_idents: &QObjectName,
    cxx_mappings: &ParsedCxxMappings,
    lock_guard: Option<&str>,
) -> Result<GeneratedCppQObjectBlocks> {
    let mut generated = GeneratedCppQObjectBlocks::default();
    let qobject_ident = qobject_idents.cpp_class.cpp.to_string();
    for invokable in invokables {
        let idents = QMethodName::from(invokable);
        let return_cxx_ty =
            syn_type_to_cpp_return_type(&invokable.method.sig.output, cxx_mappings)?;

        let parameters: Vec<CppNamedType> = invokable
            .method
            .sig
            .inputs
            .iter()
            .map(|input| {
                if let FnArg::Typed(PatType { pat, ty, .. }) = input {
                    let ident = if let Pat::Ident(PatIdent { ident, .. }) = &**pat {
                        ident
                    } else {
                        return Err(Error::new(input.span(), "Unknown pattern for type"));
                    };

                    // If the name of the argument is self then ignore,
                    // as this is likely the self: Pin<T>
                    if ident == "self" {
                        Ok(None)
                    } else {
                        Ok(Some(CppNamedType {
                            ident: ident.to_string(),
                            ty: syn_type_to_cpp_type(ty, cxx_mappings)?,
                        }))
                    }
                } else {
                    Ok(None)
                }
            })
            .filter_map(|result| result.map_or_else(|e| Some(Err(e)), |v| v.map(Ok)))
            .collect::<Result<Vec<CppNamedType>>>()?;

        let body = format!(
            "{ident}({parameter_names})",
            ident = idents.wrapper.cpp,
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

        generated.methods.push(CppFragment::Pair {
            header: format!(
                "{is_qinvokable}{is_virtual}{return_cxx_ty} {ident}({parameter_types}){is_const}{is_final}{is_override};",
                return_cxx_ty = if let Some(return_cxx_ty) = &return_cxx_ty {
                    return_cxx_ty
                } else {
                    "void"
                },
                ident = idents.name.cpp,
                parameter_types = parameter_types,
                is_qinvokable = if invokable.is_qinvokable {
                    "Q_INVOKABLE "
                } else {
                    ""
                },
                is_final = if invokable.specifiers.contains(&ParsedQInvokableSpecifiers::Final) {
                    " final"
                } else {
                    ""
                },
                is_override = if invokable.specifiers.contains(&ParsedQInvokableSpecifiers::Override) {
                    " override"
                } else {
                    ""
                },
                is_virtual = if invokable.specifiers.contains(&ParsedQInvokableSpecifiers::Virtual) {
                    "virtual "
                } else {
                    ""
                },
            ),
            source: formatdoc! {
                r#"
                    {return_cxx_ty}
                    {qobject_ident}::{ident}({parameter_types}){is_const}
                    {{
                        {rust_obj_guard}
                        {body};
                    }}
                    "#,
                return_cxx_ty = if let Some(return_cxx_ty) = &return_cxx_ty {
                    return_cxx_ty
                } else {
                    "void"
                },
                ident = idents.name.cpp,
                is_const = is_const,
                parameter_types = parameter_types,
                qobject_ident = qobject_ident,
                rust_obj_guard = lock_guard.unwrap_or_default(),
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
            ident = idents.wrapper.cpp,
            parameter_types = parameter_types,
        )));
    }

    Ok(generated)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::generator::naming::qobject::tests::create_qobjectname;
    use crate::parser::parameter::ParsedFunctionParameter;
    use indoc::indoc;
    use pretty_assertions::assert_str_eq;
    use quote::format_ident;
    use std::collections::HashSet;
    use syn::parse_quote;

    #[test]
    fn test_generate_cpp_invokables() {
        let invokables = vec![
            ParsedMethod {
                method: parse_quote! { fn void_invokable(self: &MyObject); },
                qobject_ident: format_ident!("MyObject"),
                mutable: false,
                safe: true,
                parameters: vec![],
                specifiers: HashSet::new(),
                is_qinvokable: true,
            },
            ParsedMethod {
                method: parse_quote! { fn trivial_invokable(self: &MyObject, param: i32) -> i32; },
                qobject_ident: format_ident!("MyObject"),
                mutable: false,
                safe: true,
                parameters: vec![ParsedFunctionParameter {
                    ident: format_ident!("param"),
                    ty: parse_quote! { i32 },
                }],
                specifiers: HashSet::new(),
                is_qinvokable: true,
            },
            ParsedMethod {
                method: parse_quote! { fn opaque_invokable(self: Pin<&mut MyObject>, param: &QColor) -> UniquePtr<QColor>; },
                qobject_ident: format_ident!("MyObject"),
                mutable: true,
                safe: true,
                parameters: vec![ParsedFunctionParameter {
                    ident: format_ident!("param"),
                    ty: parse_quote! { &QColor },
                }],
                specifiers: HashSet::new(),
                is_qinvokable: true,
            },
            ParsedMethod {
                method: parse_quote! { fn specifiers_invokable(self: &MyObject, param: i32) -> i32; },
                qobject_ident: format_ident!("MyObject"),
                mutable: false,
                safe: true,
                parameters: vec![ParsedFunctionParameter {
                    ident: format_ident!("param"),
                    ty: parse_quote! { i32 },
                }],
                specifiers: {
                    let mut specifiers = HashSet::new();
                    specifiers.insert(ParsedQInvokableSpecifiers::Final);
                    specifiers.insert(ParsedQInvokableSpecifiers::Override);
                    specifiers.insert(ParsedQInvokableSpecifiers::Virtual);
                    specifiers
                },
                is_qinvokable: true,
            },
            ParsedMethod {
                method: parse_quote! { fn cpp_method(self: &MyObject); },
                qobject_ident: format_ident!("MyObject"),
                mutable: false,
                safe: true,
                parameters: vec![],
                specifiers: HashSet::new(),
                is_qinvokable: false,
            },
        ];
        let qobject_idents = create_qobjectname();

        let generated = generate_cpp_methods(
            &invokables,
            &qobject_idents,
            &ParsedCxxMappings::default(),
            Some("// ::std::lock_guard"),
        )
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
                // ::std::lock_guard
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
                // ::std::lock_guard
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
                // ::std::lock_guard
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
                // ::std::lock_guard
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
                // ::std::lock_guard
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
        let invokables = vec![ParsedMethod {
            method: parse_quote! { fn trivial_invokable(self: &MyObject, param: A) -> B; },
            qobject_ident: format_ident!("MyObject"),
            mutable: false,
            safe: true,
            parameters: vec![ParsedFunctionParameter {
                ident: format_ident!("param"),
                ty: parse_quote! { i32 },
            }],
            specifiers: HashSet::new(),
            is_qinvokable: true,
        }];
        let qobject_idents = create_qobjectname();

        let mut cxx_mappings = ParsedCxxMappings::default();
        cxx_mappings
            .cxx_names
            .insert("A".to_owned(), "A1".to_owned());
        cxx_mappings
            .cxx_names
            .insert("B".to_owned(), "B2".to_owned());

        let generated = generate_cpp_methods(
            &invokables,
            &qobject_idents,
            &cxx_mappings,
            Some("// ::std::lock_guard"),
        )
        .unwrap();

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
                // ::std::lock_guard
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
