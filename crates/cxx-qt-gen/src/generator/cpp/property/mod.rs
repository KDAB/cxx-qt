// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::generator::{
    cpp::{qobject::GeneratedCppQObjectBlocks, types::CppType},
    naming::{property::QPropertyName, qobject::QObjectName},
};
use crate::parser::property::ParsedQProperty;
use syn::Result;

mod emitter;
mod getter;
mod meta;
mod setter;
mod signal;

pub fn generate_cpp_properties(
    generated: &mut GeneratedCppQObjectBlocks,
    properties: &Vec<ParsedQProperty>,
    qobject_idents: &QObjectName,
) -> Result<()> {
    let qobject_ident = qobject_idents.cpp_class.cpp.to_string();
    for property in properties {
        // Cache the idents as they are used in multiple places
        let idents = QPropertyName::from(property);
        let cxx_ty = CppType::from(&property.ty, &property.cxx_type)?;

        generated.metaobjects.push(meta::generate(&idents, &cxx_ty));
        generated
            .methods
            .push(getter::generate(&idents, &qobject_ident, &cxx_ty));
        generated
            .slots
            .push(setter::generate(&idents, &qobject_ident, &cxx_ty));
        generated.signals.push(signal::generate(&idents));
        generated
            .methods
            .push(emitter::generate(&idents, &qobject_ident));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::generator::naming::qobject::tests::create_qobjectname;
    use crate::tests::tokens_to_syn;
    use indoc::indoc;
    use pretty_assertions::assert_str_eq;
    use quote::{format_ident, quote};

    #[test]
    fn test_generate_cpp_properties() {
        let mut generated = GeneratedCppQObjectBlocks::default();
        let properties = vec![
            ParsedQProperty {
                ident: format_ident!("trivial_property"),
                ty: tokens_to_syn(quote! { i32 }),
                vis: syn::Visibility::Inherited,
                cxx_type: None,
            },
            ParsedQProperty {
                ident: format_ident!("opaque_property"),
                ty: tokens_to_syn(quote! { UniquePtr<QColor> }),
                vis: syn::Visibility::Inherited,
                cxx_type: Some("QColor".to_owned()),
            },
        ];
        let qobject_idents = create_qobjectname();

        assert!(generate_cpp_properties(&mut generated, &properties, &qobject_idents).is_ok());

        // metaobjects
        assert_eq!(generated.metaobjects.len(), 2);
        assert_str_eq!(generated.metaobjects[0], "Q_PROPERTY(qint32 trivialProperty READ getTrivialProperty WRITE setTrivialProperty NOTIFY trivialPropertyChanged)");
        assert_str_eq!(generated.metaobjects[1], "Q_PROPERTY(QColor opaqueProperty READ getOpaqueProperty WRITE setOpaqueProperty NOTIFY opaquePropertyChanged)");

        // methods
        assert_eq!(generated.methods.len(), 4);
        assert_str_eq!(
            generated.methods[0].header,
            "const qint32& getTrivialProperty() const;"
        );
        assert_str_eq!(
            generated.methods[0].source,
            indoc! {r#"
            const qint32&
            MyObject::getTrivialProperty() const
            {
                const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
                return rust::cxxqtlib1::cxx_qt_convert<const qint32&, const qint32&>{}(m_rustObj->getTrivialProperty(*this));
            }
            "#}
        );
        assert_str_eq!(
            generated.methods[1].header,
            "void emitTrivialPropertyChanged();"
        );
        assert_str_eq!(
            generated.methods[1].source,
            indoc! {r#"
            void
            MyObject::emitTrivialPropertyChanged()
            {
                const auto signalSuccess = QMetaObject::invokeMethod(this, "trivialPropertyChanged", Qt::QueuedConnection);
                Q_ASSERT(signalSuccess);
            }
            "#}
        );

        assert_str_eq!(
            generated.methods[2].header,
            "const QColor& getOpaqueProperty() const;"
        );
        assert_str_eq!(
            generated.methods[2].source,
            indoc! {r#"
            const QColor&
            MyObject::getOpaqueProperty() const
            {
                const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
                return rust::cxxqtlib1::cxx_qt_convert<const QColor&, const ::std::unique_ptr<QColor>&>{}(m_rustObj->getOpaqueProperty(*this));
            }
            "#}
        );
        assert_str_eq!(
            generated.methods[3].header,
            "void emitOpaquePropertyChanged();"
        );
        assert_str_eq!(
            generated.methods[3].source,
            indoc! {r#"
            void
            MyObject::emitOpaquePropertyChanged()
            {
                const auto signalSuccess = QMetaObject::invokeMethod(this, "opaquePropertyChanged", Qt::QueuedConnection);
                Q_ASSERT(signalSuccess);
            }
            "#}
        );

        // slots
        assert_eq!(generated.slots.len(), 2);
        assert_str_eq!(
            generated.slots[0].header,
            "void setTrivialProperty(const qint32& value);"
        );
        assert_str_eq!(
            generated.slots[0].source,
            indoc! {r#"
            void
            MyObject::setTrivialProperty(const qint32& value)
            {
                const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
                m_rustObj->setTrivialProperty(*this, rust::cxxqtlib1::cxx_qt_convert<qint32, const qint32&>{}(value));
            }
            "#}
        );
        assert_str_eq!(
            generated.slots[1].header,
            "void setOpaqueProperty(const QColor& value);"
        );
        assert_str_eq!(
            generated.slots[1].source,
            indoc! {r#"
            void
            MyObject::setOpaqueProperty(const QColor& value)
            {
                const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
                m_rustObj->setOpaqueProperty(*this, rust::cxxqtlib1::cxx_qt_convert<::std::unique_ptr<QColor>, const QColor&>{}(value));
            }
            "#}
        );

        // signals
        assert_eq!(generated.signals.len(), 2);
        assert_str_eq!(generated.signals[0], "void trivialPropertyChanged();");
        assert_str_eq!(generated.signals[1], "void opaquePropertyChanged();");
    }
}
