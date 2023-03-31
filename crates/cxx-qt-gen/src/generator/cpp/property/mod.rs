// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::generator::{
    cpp::{qobject::GeneratedCppQObjectBlocks, types::CppType},
    naming::{property::QPropertyName, qobject::QObjectName},
};
use crate::parser::{cxxqtdata::ParsedCxxMappings, property::ParsedQProperty};
use syn::Result;

mod getter;
mod meta;
mod setter;
mod signal;

pub fn generate_cpp_properties(
    properties: &Vec<ParsedQProperty>,
    qobject_idents: &QObjectName,
    cxx_mappings: &ParsedCxxMappings,
) -> Result<GeneratedCppQObjectBlocks> {
    let mut generated = GeneratedCppQObjectBlocks::default();
    let qobject_ident = qobject_idents.cpp_class.cpp.to_string();
    for property in properties {
        // Cache the idents as they are used in multiple places
        let idents = QPropertyName::from(property);
        let cxx_ty = CppType::from(&property.ty, &property.cxx_type, cxx_mappings)?;

        generated.metaobjects.push(meta::generate(&idents, &cxx_ty));
        generated
            .methods
            .push(getter::generate(&idents, &qobject_ident, &cxx_ty));
        generated
            .methods
            .push(setter::generate(&idents, &qobject_ident, &cxx_ty));
        generated.methods.push(signal::generate(&idents));
    }

    Ok(generated)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::generator::naming::qobject::tests::create_qobjectname;
    use crate::CppFragment;
    use indoc::indoc;
    use pretty_assertions::assert_str_eq;
    use quote::format_ident;
    use syn::parse_quote;

    #[test]
    fn test_generate_cpp_properties() {
        let properties = vec![
            ParsedQProperty {
                ident: format_ident!("trivial_property"),
                ty: parse_quote! { i32 },
                vis: syn::Visibility::Inherited,
                cxx_type: None,
            },
            ParsedQProperty {
                ident: format_ident!("opaque_property"),
                ty: parse_quote! { UniquePtr<QColor> },
                vis: syn::Visibility::Inherited,
                cxx_type: Some("QColor".to_owned()),
            },
        ];
        let qobject_idents = create_qobjectname();

        let generated =
            generate_cpp_properties(&properties, &qobject_idents, &ParsedCxxMappings::default())
                .unwrap();

        // metaobjects
        assert_eq!(generated.metaobjects.len(), 2);
        assert_str_eq!(generated.metaobjects[0], "Q_PROPERTY(::std::int32_t trivialProperty READ getTrivialProperty WRITE setTrivialProperty NOTIFY trivialPropertyChanged)");
        assert_str_eq!(generated.metaobjects[1], "Q_PROPERTY(QColor opaqueProperty READ getOpaqueProperty WRITE setOpaqueProperty NOTIFY opaquePropertyChanged)");

        // methods
        assert_eq!(generated.methods.len(), 6);
        let (header, source) = if let CppFragment::Pair { header, source } = &generated.methods[0] {
            (header, source)
        } else {
            panic!("Expected pair!")
        };
        assert_str_eq!(header, "::std::int32_t const& getTrivialProperty() const;");
        assert_str_eq!(
            source,
            indoc! {r#"
            ::std::int32_t const&
            MyObject::getTrivialProperty() const
            {
                const ::std::lock_guard<::std::recursive_mutex> guard(*m_rustObjMutex);
                return ::rust::cxxqtlib1::cxx_qt_convert<::std::int32_t const&, ::std::int32_t const&>{}(m_rustObj->getTrivialProperty(*this));
            }
            "#}
        );

        let (header, source) = if let CppFragment::Pair { header, source } = &generated.methods[1] {
            (header, source)
        } else {
            panic!("Expected pair!")
        };
        assert_str_eq!(
            header,
            "Q_SLOT void setTrivialProperty(::std::int32_t const& value);"
        );
        assert_str_eq!(
            source,
            indoc! {r#"
                void
                MyObject::setTrivialProperty(::std::int32_t const& value)
                {
                    const ::std::lock_guard<::std::recursive_mutex> guard(*m_rustObjMutex);
                    m_rustObj->setTrivialProperty(*this, ::rust::cxxqtlib1::cxx_qt_convert<::std::int32_t, ::std::int32_t const&>{}(value));
                }
                "#}
        );
        let header = if let CppFragment::Header(header) = &generated.methods[2] {
            header
        } else {
            panic!("Expected header!")
        };
        assert_str_eq!(header, "Q_SIGNAL void trivialPropertyChanged();");
        let (header, source) = if let CppFragment::Pair { header, source } = &generated.methods[3] {
            (header, source)
        } else {
            panic!("Expected pair!")
        };
        assert_str_eq!(header, "QColor const& getOpaqueProperty() const;");
        assert_str_eq!(
            source,
            indoc! {r#"
            QColor const&
            MyObject::getOpaqueProperty() const
            {
                const ::std::lock_guard<::std::recursive_mutex> guard(*m_rustObjMutex);
                return ::rust::cxxqtlib1::cxx_qt_convert<QColor const&, ::std::unique_ptr<QColor> const&>{}(m_rustObj->getOpaqueProperty(*this));
            }
            "#}
        );

        let (header, source) = if let CppFragment::Pair { header, source } = &generated.methods[4] {
            (header, source)
        } else {
            panic!("Expected pair!")
        };
        assert_str_eq!(
            header,
            "Q_SLOT void setOpaqueProperty(QColor const& value);"
        );
        assert_str_eq!(
            source,
            indoc! {r#"
            void
            MyObject::setOpaqueProperty(QColor const& value)
            {
                const ::std::lock_guard<::std::recursive_mutex> guard(*m_rustObjMutex);
                m_rustObj->setOpaqueProperty(*this, ::rust::cxxqtlib1::cxx_qt_convert<::std::unique_ptr<QColor>, QColor const&>{}(value));
            }
            "#}
        );

        let header = if let CppFragment::Header(header) = &generated.methods[5] {
            header
        } else {
            panic!("Expected header!")
        };
        assert_str_eq!(header, "Q_SIGNAL void opaquePropertyChanged();");
    }

    #[test]
    fn test_generate_cpp_properties_mapped_cxx_name() {
        let properties = vec![ParsedQProperty {
            ident: format_ident!("mapped_property"),
            ty: parse_quote! { A1 },
            vis: syn::Visibility::Inherited,
            cxx_type: None,
        }];
        let qobject_idents = create_qobjectname();

        let mut cxx_mapping = ParsedCxxMappings::default();
        cxx_mapping
            .cxx_names
            .insert("A".to_owned(), "A1".to_owned());

        let generated =
            generate_cpp_properties(&properties, &qobject_idents, &cxx_mapping).unwrap();

        // metaobjects
        assert_eq!(generated.metaobjects.len(), 1);
        assert_str_eq!(generated.metaobjects[0], "Q_PROPERTY(A1 mappedProperty READ getMappedProperty WRITE setMappedProperty NOTIFY mappedPropertyChanged)");

        // methods
        assert_eq!(generated.methods.len(), 3);
        let (header, source) = if let CppFragment::Pair { header, source } = &generated.methods[0] {
            (header, source)
        } else {
            panic!("Expected pair!")
        };
        assert_str_eq!(header, "A1 const& getMappedProperty() const;");
        assert_str_eq!(
            source,
            indoc! {r#"
            A1 const&
            MyObject::getMappedProperty() const
            {
                const ::std::lock_guard<::std::recursive_mutex> guard(*m_rustObjMutex);
                return ::rust::cxxqtlib1::cxx_qt_convert<A1 const&, A1 const&>{}(m_rustObj->getMappedProperty(*this));
            }
            "#}
        );

        let (header, source) = if let CppFragment::Pair { header, source } = &generated.methods[1] {
            (header, source)
        } else {
            panic!("Expected pair!")
        };
        assert_str_eq!(header, "Q_SLOT void setMappedProperty(A1 const& value);");
        assert_str_eq!(
            source,
            indoc! {r#"
                void
                MyObject::setMappedProperty(A1 const& value)
                {
                    const ::std::lock_guard<::std::recursive_mutex> guard(*m_rustObjMutex);
                    m_rustObj->setMappedProperty(*this, ::rust::cxxqtlib1::cxx_qt_convert<A1, A1 const&>{}(value));
                }
                "#}
        );
        let header = if let CppFragment::Header(header) = &generated.methods[2] {
            header
        } else {
            panic!("Expected header!")
        };
        assert_str_eq!(header, "Q_SIGNAL void mappedPropertyChanged();");
    }
}
