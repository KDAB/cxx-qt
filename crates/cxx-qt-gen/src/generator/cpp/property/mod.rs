// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::generator::{
    cpp::{qobject::GeneratedCppQObjectBlocks, signal::generate_cpp_signals},
    naming::{property::QPropertyNames, qobject::QObjectNames},
};
use crate::{
    naming::cpp::syn_type_to_cpp_type,
    naming::TypeNames,
    parser::property::{ParsedQProperty, QPropertyFlag},
};
use syn::Result;

mod getter;
mod meta;
mod setter;
mod signal;

pub fn generate_cpp_properties(
    properties: &Vec<ParsedQProperty>,
    qobject_idents: &QObjectNames,
    type_names: &TypeNames,
) -> Result<GeneratedCppQObjectBlocks> {
    let mut generated = GeneratedCppQObjectBlocks::default();
    let mut signals = vec![];
    let qobject_ident = qobject_idents.name.cxx_unqualified();

    for property in properties {
        // Cache the idents and flags as they are used in multiple places
        let idents = QPropertyNames::from(property);
        let cxx_ty = syn_type_to_cpp_type(&property.ty, type_names)?;

        generated.metaobjects.push(meta::generate(&idents, &cxx_ty));

        let mut includes_read = false; // If the HashSet includes entries read must be specified otherwise it is an error

        for flag in &property.flags {
            match flag {
                QPropertyFlag::Write(_) => {
                    // Gen setters
                    generated
                        .methods
                        .push(setter::generate(&idents, &qobject_ident, &cxx_ty));
                    generated
                        .private_methods
                        .push(setter::generate_wrapper(&idents, &cxx_ty));
                }
                QPropertyFlag::Read(_) => {
                    includes_read = true;
                    // Gen Getters
                    generated
                        .methods
                        .push(getter::generate(&idents, &qobject_ident, &cxx_ty));
                    generated
                        .private_methods
                        .push(getter::generate_wrapper(&idents, &cxx_ty));
                }
                QPropertyFlag::Notify(_) => {
                    // Gen signal
                    signals.push(signal::generate(&idents, qobject_idents));
                }
            }
        }

        if !includes_read {
            // TODO: Change to throwing an error, but no syn types present in this function
            panic!("If flags are specified, read cannot be inferred and so must be specified")
        }
    }

    generated.append(&mut generate_cpp_signals(
        &signals,
        qobject_idents,
        type_names,
    )?);

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
    use syn::{parse_quote, ItemStruct};

    #[test]
    fn test_optional_write() {
        let mut input: ItemStruct = parse_quote! {
            #[qproperty(i32, num, read, write, notify)]
            struct MyStruct;
        };
        let property = ParsedQProperty::parse(input.attrs.remove(0)).unwrap();

        let properties = vec![property];

        let qobject_idents = create_qobjectname();

        let mut type_names = TypeNames::mock();
        type_names.mock_insert("i32", None, None, None);
        let generated = generate_cpp_properties(&properties, &qobject_idents, &type_names).unwrap();

        println!("generated code: \n{:?}", generated.metaobjects);
        println!("generated methods: \n{:?}", generated.methods);
    }

    #[test]
    fn test_generate_cpp_properties() {
        let properties = vec![
            ParsedQProperty {
                ident: format_ident!("trivial_property"),
                ty: parse_quote! { i32 },
                flags: Default::default(),
            },
            ParsedQProperty {
                ident: format_ident!("opaque_property"),
                ty: parse_quote! { UniquePtr<QColor> },
                flags: Default::default(),
            },
        ];

        let qobject_idents = create_qobjectname();

        let mut type_names = TypeNames::mock();
        type_names.mock_insert("QColor", None, None, None);
        let generated = generate_cpp_properties(&properties, &qobject_idents, &type_names).unwrap();

        // metaobjects
        assert_eq!(generated.metaobjects.len(), 2);
        assert_str_eq!(generated.metaobjects[0], "Q_PROPERTY(::std::int32_t trivialProperty READ getTrivialProperty WRITE setTrivialProperty NOTIFY trivialPropertyChanged)");
        assert_str_eq!(generated.metaobjects[1], "Q_PROPERTY(::std::unique_ptr<QColor> opaqueProperty READ getOpaqueProperty WRITE setOpaqueProperty NOTIFY opaquePropertyChanged)");

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
                const ::rust::cxxqt1::MaybeLockGuard<MyObject> guard(*this);
                return getTrivialPropertyWrapper();
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
                    const ::rust::cxxqt1::MaybeLockGuard<MyObject> guard(*this);
                    setTrivialPropertyWrapper(value);
                }
                "#}
        );

        let (header, source) = if let CppFragment::Pair { header, source } = &generated.methods[2] {
            (header, source)
        } else {
            panic!("Expected pair!")
        };
        assert_str_eq!(
            header,
            "::std::unique_ptr<QColor> const& getOpaqueProperty() const;"
        );
        assert_str_eq!(
            source,
            indoc! {r#"
            ::std::unique_ptr<QColor> const&
            MyObject::getOpaqueProperty() const
            {
                const ::rust::cxxqt1::MaybeLockGuard<MyObject> guard(*this);
                return getOpaquePropertyWrapper();
            }
            "#}
        );

        let (header, source) = if let CppFragment::Pair { header, source } = &generated.methods[3] {
            (header, source)
        } else {
            panic!("Expected pair!")
        };
        assert_str_eq!(
            header,
            "Q_SLOT void setOpaqueProperty(::std::unique_ptr<QColor> const& value);"
        );
        assert_str_eq!(
            source,
            indoc! {r#"
            void
            MyObject::setOpaqueProperty(::std::unique_ptr<QColor> const& value)
            {
                const ::rust::cxxqt1::MaybeLockGuard<MyObject> guard(*this);
                setOpaquePropertyWrapper(value);
            }
            "#}
        );

        let header = if let CppFragment::Header(header) = &generated.methods[4] {
            header
        } else {
            panic!("Expected header!")
        };
        assert_str_eq!(header, "Q_SIGNAL void trivialPropertyChanged();");

        let header = if let CppFragment::Header(header) = &generated.methods[5] {
            header
        } else {
            panic!("Expected header!")
        };
        assert_str_eq!(header, "Q_SIGNAL void opaquePropertyChanged();");

        assert_eq!(generated.fragments.len(), 2);
        let (header, source) = if let CppFragment::Pair { header, source } = &generated.fragments[0]
        {
            (header, source)
        } else {
            panic!("Expected Pair")
        };
        assert_str_eq!(
            header,
            indoc! {r#"
            namespace rust::cxxqtgen1 {
            ::QMetaObject::Connection
            MyObject_trivialPropertyChangedConnect(MyObject& self, ::rust::cxxqtgen1::MyObjectCxxQtSignalHandlertrivialPropertyChanged closure, ::Qt::ConnectionType type);
            } // namespace rust::cxxqtgen1
            "#}
        );
        assert_str_eq!(
            source,
            indoc! {r#"
            // Define namespace otherwise we hit a GCC bug
            // https://gcc.gnu.org/bugzilla/show_bug.cgi?id=56480
            namespace rust::cxxqt1 {
            template <>
            SignalHandler<::rust::cxxqtgen1::MyObjectCxxQtSignalParamstrivialPropertyChanged *>::~SignalHandler() noexcept
            {
                if (data[0] == nullptr && data[1] == nullptr)
                {
                    return;
                }

                drop_MyObject_signal_handler_trivialPropertyChanged(::std::move(*this));
            }

            template <>
            template <>
            void SignalHandler<::rust::cxxqtgen1::MyObjectCxxQtSignalParamstrivialPropertyChanged *>::operator()<MyObject&>(MyObject& self)
            {
                call_MyObject_signal_handler_trivialPropertyChanged(*this, self);
            }

            static_assert(alignof(SignalHandler<::rust::cxxqtgen1::MyObjectCxxQtSignalParamstrivialPropertyChanged *>) <= alignof(::std::size_t), "unexpected aligment");
            static_assert(sizeof(SignalHandler<::rust::cxxqtgen1::MyObjectCxxQtSignalParamstrivialPropertyChanged *>) == sizeof(::std::size_t[2]), "unexpected size");
            } // namespace rust::cxxqt1

            namespace rust::cxxqtgen1 {
            ::QMetaObject::Connection
            MyObject_trivialPropertyChangedConnect(MyObject& self, ::rust::cxxqtgen1::MyObjectCxxQtSignalHandlertrivialPropertyChanged closure, ::Qt::ConnectionType type)
            {
                return ::QObject::connect(
                    &self,
                    &MyObject::trivialPropertyChanged,
                    &self,
                    [&, closure = ::std::move(closure)]() mutable {
                        const ::rust::cxxqt1::MaybeLockGuard<MyObject> guard(self);
                        closure.template operator()<MyObject&>(self);
                    },
                    type);
            }
            } // namespace rust::cxxqtgen1
            "#}
        );

        let (header, source) = if let CppFragment::Pair { header, source } = &generated.fragments[1]
        {
            (header, source)
        } else {
            panic!("Expected Pair")
        };
        assert_str_eq!(
            header,
            indoc! {r#"
            namespace rust::cxxqtgen1 {
            ::QMetaObject::Connection
            MyObject_opaquePropertyChangedConnect(MyObject& self, ::rust::cxxqtgen1::MyObjectCxxQtSignalHandleropaquePropertyChanged closure, ::Qt::ConnectionType type);
            } // namespace rust::cxxqtgen1
            "#}
        );
        assert_str_eq!(
            source,
            indoc! {r#"
            // Define namespace otherwise we hit a GCC bug
            // https://gcc.gnu.org/bugzilla/show_bug.cgi?id=56480
            namespace rust::cxxqt1 {
            template <>
            SignalHandler<::rust::cxxqtgen1::MyObjectCxxQtSignalParamsopaquePropertyChanged *>::~SignalHandler() noexcept
            {
                if (data[0] == nullptr && data[1] == nullptr)
                {
                    return;
                }

                drop_MyObject_signal_handler_opaquePropertyChanged(::std::move(*this));
            }

            template <>
            template <>
            void SignalHandler<::rust::cxxqtgen1::MyObjectCxxQtSignalParamsopaquePropertyChanged *>::operator()<MyObject&>(MyObject& self)
            {
                call_MyObject_signal_handler_opaquePropertyChanged(*this, self);
            }

            static_assert(alignof(SignalHandler<::rust::cxxqtgen1::MyObjectCxxQtSignalParamsopaquePropertyChanged *>) <= alignof(::std::size_t), "unexpected aligment");
            static_assert(sizeof(SignalHandler<::rust::cxxqtgen1::MyObjectCxxQtSignalParamsopaquePropertyChanged *>) == sizeof(::std::size_t[2]), "unexpected size");
            } // namespace rust::cxxqt1

            namespace rust::cxxqtgen1 {
            ::QMetaObject::Connection
            MyObject_opaquePropertyChangedConnect(MyObject& self, ::rust::cxxqtgen1::MyObjectCxxQtSignalHandleropaquePropertyChanged closure, ::Qt::ConnectionType type)
            {
                return ::QObject::connect(
                    &self,
                    &MyObject::opaquePropertyChanged,
                    &self,
                    [&, closure = ::std::move(closure)]() mutable {
                        const ::rust::cxxqt1::MaybeLockGuard<MyObject> guard(self);
                        closure.template operator()<MyObject&>(self);
                    },
                    type);
            }
            } // namespace rust::cxxqtgen1
            "#}
        );

        // private methods
        assert_eq!(generated.private_methods.len(), 4);
        let header = if let CppFragment::Header(header) = &generated.private_methods[0] {
            header
        } else {
            panic!("Expected header")
        };
        assert_str_eq!(
            header,
            "::std::int32_t const& getTrivialPropertyWrapper() const noexcept;"
        );

        let header = if let CppFragment::Header(header) = &generated.private_methods[1] {
            header
        } else {
            panic!("Expected header")
        };
        assert_str_eq!(
            header,
            "void setTrivialPropertyWrapper(::std::int32_t value) noexcept;"
        );

        let header = if let CppFragment::Header(header) = &generated.private_methods[2] {
            header
        } else {
            panic!("Expected header")
        };
        assert_str_eq!(
            header,
            "::std::unique_ptr<QColor> const& getOpaquePropertyWrapper() const noexcept;"
        );

        let header = if let CppFragment::Header(header) = &generated.private_methods[3] {
            header
        } else {
            panic!("Expected header")
        };
        assert_str_eq!(
            header,
            "void setOpaquePropertyWrapper(::std::unique_ptr<QColor> value) noexcept;"
        );
    }

    #[test]
    fn test_generate_cpp_properties_mapped_cxx_name() {
        let properties = vec![ParsedQProperty {
            ident: format_ident!("mapped_property"),
            ty: parse_quote! { A },
            flags: Default::default(),
        }];
        let qobject_idents = create_qobjectname();

        let mut type_names = TypeNames::mock();
        type_names.mock_insert("A", None, Some("A1"), None);

        let generated = generate_cpp_properties(&properties, &qobject_idents, &type_names).unwrap();

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
                const ::rust::cxxqt1::MaybeLockGuard<MyObject> guard(*this);
                return getMappedPropertyWrapper();
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
                    const ::rust::cxxqt1::MaybeLockGuard<MyObject> guard(*this);
                    setMappedPropertyWrapper(value);
                }
                "#}
        );
        let header = if let CppFragment::Header(header) = &generated.methods[2] {
            header
        } else {
            panic!("Expected header!")
        };
        assert_str_eq!(header, "Q_SIGNAL void mappedPropertyChanged();");

        assert_eq!(generated.fragments.len(), 1);
        let (header, source) = if let CppFragment::Pair { header, source } = &generated.fragments[0]
        {
            (header, source)
        } else {
            panic!("Expected Pair")
        };
        assert_str_eq!(
            header,
            indoc! {r#"
            namespace rust::cxxqtgen1 {
            ::QMetaObject::Connection
            MyObject_mappedPropertyChangedConnect(MyObject& self, ::rust::cxxqtgen1::MyObjectCxxQtSignalHandlermappedPropertyChanged closure, ::Qt::ConnectionType type);
            } // namespace rust::cxxqtgen1
            "#}
        );
        assert_str_eq!(
            source,
            indoc! {r#"
            // Define namespace otherwise we hit a GCC bug
            // https://gcc.gnu.org/bugzilla/show_bug.cgi?id=56480
            namespace rust::cxxqt1 {
            template <>
            SignalHandler<::rust::cxxqtgen1::MyObjectCxxQtSignalParamsmappedPropertyChanged *>::~SignalHandler() noexcept
            {
                if (data[0] == nullptr && data[1] == nullptr)
                {
                    return;
                }

                drop_MyObject_signal_handler_mappedPropertyChanged(::std::move(*this));
            }

            template <>
            template <>
            void SignalHandler<::rust::cxxqtgen1::MyObjectCxxQtSignalParamsmappedPropertyChanged *>::operator()<MyObject&>(MyObject& self)
            {
                call_MyObject_signal_handler_mappedPropertyChanged(*this, self);
            }

            static_assert(alignof(SignalHandler<::rust::cxxqtgen1::MyObjectCxxQtSignalParamsmappedPropertyChanged *>) <= alignof(::std::size_t), "unexpected aligment");
            static_assert(sizeof(SignalHandler<::rust::cxxqtgen1::MyObjectCxxQtSignalParamsmappedPropertyChanged *>) == sizeof(::std::size_t[2]), "unexpected size");
            } // namespace rust::cxxqt1

            namespace rust::cxxqtgen1 {
            ::QMetaObject::Connection
            MyObject_mappedPropertyChangedConnect(MyObject& self, ::rust::cxxqtgen1::MyObjectCxxQtSignalHandlermappedPropertyChanged closure, ::Qt::ConnectionType type)
            {
                return ::QObject::connect(
                    &self,
                    &MyObject::mappedPropertyChanged,
                    &self,
                    [&, closure = ::std::move(closure)]() mutable {
                        const ::rust::cxxqt1::MaybeLockGuard<MyObject> guard(self);
                        closure.template operator()<MyObject&>(self);
                    },
                    type);
            }
            } // namespace rust::cxxqtgen1
            "#}
        );

        // private methods
        assert_eq!(generated.private_methods.len(), 2);
        let header = if let CppFragment::Header(header) = &generated.private_methods[0] {
            header
        } else {
            panic!("Expected header")
        };
        assert_str_eq!(
            header,
            "A1 const& getMappedPropertyWrapper() const noexcept;"
        );

        let header = if let CppFragment::Header(header) = &generated.private_methods[1] {
            header
        } else {
            panic!("Expected header")
        };
        assert_str_eq!(header, "void setMappedPropertyWrapper(A1 value) noexcept;");
    }
}
