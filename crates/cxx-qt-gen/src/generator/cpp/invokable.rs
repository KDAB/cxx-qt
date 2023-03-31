// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{
    generator::{
        cpp::{
            fragment::{CppFragment, CppNamedType},
            qobject::GeneratedCppQObjectBlocks,
            types::CppType,
            CXX_QT_CONVERT, RUST_OBJ_MUTEX_LOCK_GUARD,
        },
        naming::{invokable::QInvokableName, qobject::QObjectName},
    },
    parser::{
        cxxqtdata::ParsedCxxMappings,
        invokable::{ParsedQInvokable, ParsedQInvokableSpecifiers},
    },
};
use indoc::formatdoc;
use syn::{spanned::Spanned, Error, FnArg, Pat, PatIdent, PatType, Result, ReturnType};

pub fn generate_cpp_invokables(
    invokables: &Vec<ParsedQInvokable>,
    qobject_idents: &QObjectName,
    cxx_mappings: &ParsedCxxMappings,
) -> Result<GeneratedCppQObjectBlocks> {
    let mut generated = GeneratedCppQObjectBlocks::default();
    let qobject_ident = qobject_idents.cpp_class.cpp.to_string();
    for invokable in invokables {
        let idents = QInvokableName::from(invokable);
        let cxx_ty = if let ReturnType::Type(_, ty) = &invokable.method.sig.output {
            Some(CppType::from(ty, &invokable.return_cxx_type, cxx_mappings)?)
        } else {
            None
        };

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
                            ty: CppType::from(ty, &None, cxx_mappings)?,
                        }))
                    }
                } else {
                    Ok(None)
                }
            })
            .filter_map(|result| result.map_or_else(|e| Some(Err(e)), |v| v.map(Ok)))
            .collect::<Result<Vec<CppNamedType>>>()?;

        let body = format!(
            "m_rustObj->{ident}({parameter_names})",
            ident = idents.wrapper.cpp,
            parameter_names = vec!["*this"]
                .into_iter()
                .chain(parameters.iter().map(|parameter| parameter.ident.as_str()))
                .collect::<Vec<&str>>()
                .join(", "),
        );
        let parameter_types = parameters
            .iter()
            .map(|parameter| {
                format!(
                    "{ty} {ident}",
                    ident = parameter.ident,
                    ty = parameter.ty.as_cxx_ty()
                )
            })
            .collect::<Vec<String>>()
            .join(", ");
        let is_const = if !invokable.mutable { " const" } else { "" };

        generated.methods.push(CppFragment::Pair {
            header: format!(
                "Q_INVOKABLE {is_virtual}{cxx_ty} {ident}({parameter_types}){is_const}{is_final}{is_override};",
                cxx_ty = if let Some(cxx_ty) = &cxx_ty {
                    cxx_ty.as_cxx_ty()
                } else {
                    "void"
                },
                ident = idents.name.cpp,
                parameter_types = parameter_types,
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
                    {cxx_ty}
                    {qobject_ident}::{ident}({parameter_types}){is_const}
                    {{
                        {rust_obj_guard}
                        {body};
                    }}
                    "#,
                cxx_ty = if let Some(cxx_ty) = &cxx_ty {
                    cxx_ty.as_cxx_ty()
                } else {
                    "void"
                },
                ident = idents.name.cpp,
                is_const = is_const,
                parameter_types = parameter_types,
                qobject_ident = qobject_ident,
                rust_obj_guard = RUST_OBJ_MUTEX_LOCK_GUARD,
                body = if let Some(cxx_ty) = &cxx_ty {
                    format!("return {convert}<{cxx_ty}, {rust_ty}>{{}}({body})",
                        convert = CXX_QT_CONVERT,
                        cxx_ty = cxx_ty.as_cxx_ty(),
                        rust_ty = cxx_ty.as_rust_ty(),
                        body = body
                    )
                } else {
                    body
                },
            },
        });
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
            ParsedQInvokable {
                method: parse_quote! { fn void_invokable(&self) {} },
                mutable: false,
                parameters: vec![],
                return_cxx_type: None,
                specifiers: HashSet::new(),
            },
            ParsedQInvokable {
                method: parse_quote! { fn trivial_invokable(&self, param: i32) -> i32 {} },
                mutable: false,
                parameters: vec![ParsedFunctionParameter {
                    ident: format_ident!("param"),
                    ty: parse_quote! { i32 },
                    cxx_type: None,
                }],
                return_cxx_type: None,
                specifiers: HashSet::new(),
            },
            ParsedQInvokable {
                method: parse_quote! { fn opaque_invokable(self: Pin<&mut Self>, param: &QColor) -> UniquePtr<QColor> {} },
                mutable: true,
                parameters: vec![ParsedFunctionParameter {
                    ident: format_ident!("param"),
                    ty: parse_quote! { &QColor },
                    cxx_type: None,
                }],
                return_cxx_type: Some("QColor".to_owned()),
                specifiers: HashSet::new(),
            },
            ParsedQInvokable {
                method: parse_quote! { fn specifiers_invokable(&self, param: i32) -> i32 {} },
                mutable: false,
                parameters: vec![ParsedFunctionParameter {
                    ident: format_ident!("param"),
                    ty: parse_quote! { i32 },
                    cxx_type: None,
                }],
                return_cxx_type: None,
                specifiers: {
                    let mut specifiers = HashSet::new();
                    specifiers.insert(ParsedQInvokableSpecifiers::Final);
                    specifiers.insert(ParsedQInvokableSpecifiers::Override);
                    specifiers.insert(ParsedQInvokableSpecifiers::Virtual);
                    specifiers
                },
            },
        ];
        let qobject_idents = create_qobjectname();

        let generated =
            generate_cpp_invokables(&invokables, &qobject_idents, &ParsedCxxMappings::default())
                .unwrap();

        // methods
        assert_eq!(generated.methods.len(), 4);

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
                const ::std::lock_guard<::std::recursive_mutex> guard(*m_rustObjMutex);
                m_rustObj->voidInvokableWrapper(*this);
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
                const ::std::lock_guard<::std::recursive_mutex> guard(*m_rustObjMutex);
                return ::rust::cxxqtlib1::cxx_qt_convert<::std::int32_t, ::std::int32_t>{}(m_rustObj->trivialInvokableWrapper(*this, param));
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
            "Q_INVOKABLE QColor opaqueInvokable(QColor const& param);"
        );
        assert_str_eq!(
            source,
            indoc! {r#"
            QColor
            MyObject::opaqueInvokable(QColor const& param)
            {
                const ::std::lock_guard<::std::recursive_mutex> guard(*m_rustObjMutex);
                return ::rust::cxxqtlib1::cxx_qt_convert<QColor, ::std::unique_ptr<QColor>>{}(m_rustObj->opaqueInvokableWrapper(*this, param));
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
                const ::std::lock_guard<::std::recursive_mutex> guard(*m_rustObjMutex);
                return ::rust::cxxqtlib1::cxx_qt_convert<::std::int32_t, ::std::int32_t>{}(m_rustObj->specifiersInvokableWrapper(*this, param));
            }
            "#}
        );
    }

    #[test]
    fn test_generate_cpp_invokables_mapped_cxx_name() {
        let invokables = vec![ParsedQInvokable {
            method: parse_quote! { fn trivial_invokable(&self, param: A) -> B {} },
            mutable: false,
            parameters: vec![ParsedFunctionParameter {
                ident: format_ident!("param"),
                ty: parse_quote! { i32 },
                cxx_type: None,
            }],
            return_cxx_type: None,
            specifiers: HashSet::new(),
        }];
        let qobject_idents = create_qobjectname();

        let mut cxx_mappings = ParsedCxxMappings::default();
        cxx_mappings
            .cxx_names
            .insert("A".to_owned(), "A1".to_owned());
        cxx_mappings
            .cxx_names
            .insert("B".to_owned(), "B2".to_owned());

        let generated =
            generate_cpp_invokables(&invokables, &qobject_idents, &cxx_mappings).unwrap();

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
                const ::std::lock_guard<::std::recursive_mutex> guard(*m_rustObjMutex);
                return ::rust::cxxqtlib1::cxx_qt_convert<B2, B2>{}(m_rustObj->trivialInvokableWrapper(*this, param));
            }
            "#}
        );
    }
}
