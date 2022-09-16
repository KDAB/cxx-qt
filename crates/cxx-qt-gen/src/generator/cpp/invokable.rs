// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{
    generator::{
        cpp::{
            fragment::{CppFragmentPair, CppNamedType},
            qobject::GeneratedCppQObjectBlocks,
            types::CppType,
            CXX_QT_CONVERT, RUST_OBJ_MUTEX_LOCK_GUARD,
        },
        naming::{invokable::QInvokableName, qobject::QObjectName},
    },
    parser::invokable::ParsedQInvokable,
};
use indoc::formatdoc;
use syn::{spanned::Spanned, Error, FnArg, Pat, PatIdent, PatType, Result, ReturnType};

pub fn generate_cpp_invokables(
    invokables: &Vec<ParsedQInvokable>,
    qobject_idents: &QObjectName,
) -> Result<GeneratedCppQObjectBlocks> {
    let mut generated = GeneratedCppQObjectBlocks::default();
    let qobject_ident = qobject_idents.cpp_class.cpp.to_string();
    for invokable in invokables {
        let idents = QInvokableName::from(invokable);
        let cxx_ty = if let ReturnType::Type(_, ty) = &invokable.method.sig.output {
            Some(CppType::from(ty, &invokable.return_cxx_type)?)
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
                            ty: CppType::from(ty, &None)?,
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

        generated.methods.push(CppFragmentPair {
            header: format!(
                "Q_INVOKABLE {cxx_ty} {ident}({parameter_types});",
                cxx_ty = if let Some(cxx_ty) = &cxx_ty {
                    cxx_ty.as_cxx_ty()
                } else {
                    "void"
                },
                ident = idents.name.cpp,
                parameter_types = parameter_types,
            ),
            source: formatdoc! {
                r#"
                    {cxx_ty}
                    {qobject_ident}::{ident}({parameter_types})
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
    use crate::tests::tokens_to_syn;
    use indoc::indoc;
    use pretty_assertions::assert_str_eq;
    use quote::quote;

    #[test]
    fn test_generate_cpp_invokables() {
        let invokables = vec![
            ParsedQInvokable {
                method: tokens_to_syn(quote! { fn void_invokable(&self) {} }),
                return_cxx_type: None,
            },
            ParsedQInvokable {
                method: tokens_to_syn(quote! { fn trivial_invokable(&self, param: i32) -> i32 {} }),
                return_cxx_type: None,
            },
            ParsedQInvokable {
                method: tokens_to_syn(
                    quote! { fn opaque_invokable(self: Pin<&mut Self>, param: &QColor) -> UniquePtr<QColor> {} },
                ),
                return_cxx_type: Some("QColor".to_owned()),
            },
        ];
        let qobject_idents = create_qobjectname();

        let generated = generate_cpp_invokables(&invokables, &qobject_idents).unwrap();

        // methods
        assert_eq!(generated.methods.len(), 3);

        assert_str_eq!(
            generated.methods[0].header,
            "Q_INVOKABLE void voidInvokable();"
        );
        assert_str_eq!(
            generated.methods[0].source,
            indoc! {r#"
            void
            MyObject::voidInvokable()
            {
                const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
                m_rustObj->voidInvokableWrapper(*this);
            }
            "#}
        );

        assert_str_eq!(
            generated.methods[1].header,
            "Q_INVOKABLE qint32 trivialInvokable(qint32 param);"
        );
        assert_str_eq!(
            generated.methods[1].source,
            indoc! {r#"
            qint32
            MyObject::trivialInvokable(qint32 param)
            {
                const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
                return rust::cxxqtlib1::cxx_qt_convert<qint32, qint32>{}(m_rustObj->trivialInvokableWrapper(*this, param));
            }
            "#}
        );

        assert_str_eq!(
            generated.methods[2].header,
            "Q_INVOKABLE QColor opaqueInvokable(const QColor& param);"
        );
        assert_str_eq!(
            generated.methods[2].source,
            indoc! {r#"
            QColor
            MyObject::opaqueInvokable(const QColor& param)
            {
                const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
                return rust::cxxqtlib1::cxx_qt_convert<QColor, ::std::unique_ptr<QColor>>{}(m_rustObj->opaqueInvokableWrapper(*this, param));
            }
            "#}
        );
    }
}
