// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::generator::structuring::StructuredQObject;
use crate::generator::{
    cpp::{qobject::GeneratedCppQObjectBlocks, signal::generate_cpp_signals},
    naming::{property::QPropertyNames, qobject::QObjectNames},
};
use crate::{
    naming::cpp::syn_type_to_cpp_type, naming::TypeNames, parser::property::ParsedQProperty,
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
    structured_qobject: &StructuredQObject,
) -> Result<GeneratedCppQObjectBlocks> {
    let mut generated = GeneratedCppQObjectBlocks::default();
    let mut signals = vec![];
    let qobject_ident = qobject_idents.name.cxx_unqualified();

    for property in properties {
        // Cache the idents as they are used in multiple places
        let idents = QPropertyNames::try_from_property(property, structured_qobject)?;
        let cxx_ty = syn_type_to_cpp_type(&property.ty, type_names)?;

        generated
            .metaobjects
            .push(meta::generate(&idents, &property.flags, &cxx_ty));

        if let Some(getter) = getter::generate(&idents, &qobject_ident, &cxx_ty) {
            generated.methods.push(getter);
        }

        if let Some(getter_wrapper) = getter::generate_wrapper(&idents, &cxx_ty) {
            generated.private_methods.push(getter_wrapper);
        }

        if let Some(setter) = setter::generate(&idents, &qobject_ident, &cxx_ty) {
            generated.methods.push(setter)
        }

        if let Some(setter_wrapper) = setter::generate_wrapper(&idents, &cxx_ty) {
            generated.private_methods.push(setter_wrapper)
        }

        if let Some(notify) = signal::generate(&idents, qobject_idents) {
            signals.push(notify)
        }
    }

    generated.append(&mut generate_cpp_signals(
        &signals.iter().collect(),
        qobject_idents,
        type_names,
    )?);

    Ok(generated)
}

#[cfg(test)]
pub mod tests {
    use super::*;

    use crate::generator::naming::qobject::tests::create_qobjectname;
    use crate::generator::structuring::Structures;
    use crate::parser::property::QPropertyFlags;
    use crate::parser::qobject::ParsedQObject;
    use crate::{CppFragment, Parser};
    use indoc::indoc;
    use pretty_assertions::assert_str_eq;
    use quote::format_ident;
    use syn::{parse_quote, ItemMod, ItemStruct};

    pub fn require_pair(fragment: &CppFragment) -> core::result::Result<(String, String), String> {
        match fragment {
            CppFragment::Pair { header, source } => Ok((header.clone(), source.clone())),
            _ => Err(format!("Expected a pair, got {fragment:?} instead")),
        }
    }

    pub fn require_header(fragment: &CppFragment) -> core::result::Result<String, String> {
        match fragment {
            CppFragment::Header(header) => Ok(header.clone()),
            _ => Err(format!("Expected just a header, got {fragment:?} instead")),
        }
    }

    pub fn require_source(fragment: &CppFragment) -> core::result::Result<String, String> {
        match fragment {
            CppFragment::Source(source) => Ok(source.clone()),
            _ => Err(format!("Expected just a source, got {fragment:?} instead")),
        }
    }

    fn setup_generated(input: &mut ItemStruct) -> Result<GeneratedCppQObjectBlocks> {
        let property = ParsedQProperty::parse(input.attrs.remove(0)).unwrap();

        let properties = vec![property];

        let qobject_idents = create_qobjectname();

        let obj = ParsedQObject::mock();

        let structured_qobject = StructuredQObject::mock(&obj);

        let type_names = TypeNames::mock();
        generate_cpp_properties(
            &properties,
            &qobject_idents,
            &type_names,
            &structured_qobject,
        )
    }

    // Might be a cleaner way than using functions but not sure a const / static is possible due to parse_quote!()
    fn mock_module_custom_setter() -> ItemMod {
        parse_quote! {
            #[cxx_qt::bridge]
            mod ffi {
                extern "RustQt" {
                    #[qobject]
                    type MyObject = super::MyObjectRust;
                }

                unsafe extern "RustQt" {
                    fn mySetter(self: Pin<&mut MyObject>, value: i32);
                }
            }
        }
    }

    fn mock_module_custom_setter_and_reset() -> ItemMod {
        parse_quote! {
            #[cxx_qt::bridge]
            mod ffi {
                extern "RustQt" {
                    #[qobject]
                    type MyObject = super::MyObjectRust;
                }

                unsafe extern "RustQt" {
                    fn mySetter(self: Pin<&mut MyObject>, value: i32);

                    fn my_resetter(self: Pin<&mut MyObject>);

                }
            }
        }
    }

    #[test]
    fn test_unexpected_headers() {
        let mut input: ItemStruct = parse_quote! {
            #[qproperty(i32, num, READ, WRITE = mySetter)]
            struct MyStruct;
        };

        let property = ParsedQProperty::parse(input.attrs.remove(0)).unwrap();

        let properties = vec![property];

        let qobject_idents = create_qobjectname();

        let module = mock_module_custom_setter();
        let parser = Parser::from(module).unwrap();
        let structures = Structures::new(&parser.cxx_qt_data).unwrap();

        let structured_qobject = structures.qobjects.first().unwrap();

        let type_names = TypeNames::mock();
        let generated = generate_cpp_properties(
            &properties,
            &qobject_idents,
            &type_names,
            structured_qobject,
        )
        .unwrap();

        // should be a pair
        let result = require_header(&generated.methods[0]);
        assert!(result.is_err());

        let result = require_pair(&generated.private_methods[0]);
        assert!(result.is_err());

        let result = require_source(&generated.methods[0]);
        assert!(result.is_err());
    }

    #[test]
    fn test_custom_setter() {
        let mut input: ItemStruct = parse_quote! {
            #[qproperty(i32, num, READ, WRITE = mySetter)]
            struct MyStruct;
        };

        let property = ParsedQProperty::parse(input.attrs.remove(0)).unwrap();

        let properties = vec![property];

        let qobject_idents = create_qobjectname();

        let module = mock_module_custom_setter();
        let parser = Parser::from(module).unwrap();
        let structures = Structures::new(&parser.cxx_qt_data).unwrap();

        let structured_qobject = structures.qobjects.first().unwrap();

        let type_names = TypeNames::mock();
        let generated = generate_cpp_properties(
            &properties,
            &qobject_idents,
            &type_names,
            structured_qobject,
        )
        .unwrap();

        assert_eq!(generated.metaobjects.len(), 1);
        assert_str_eq!(
            generated.metaobjects[0],
            "Q_PROPERTY(::std::int32_t num READ getNum WRITE mySetter)"
        );

        // Methods
        assert_eq!(generated.methods.len(), 1);
        let (header, source) = require_pair(&generated.methods[0]).unwrap();

        assert_str_eq!(header, "::std::int32_t const& getNum() const;");
        assert_str_eq!(
            source,
            indoc! {r#"
            ::std::int32_t const&
            MyObject::getNum() const
            {
                const ::rust::cxxqt1::MaybeLockGuard<MyObject> guard(*this);
                return getNumWrapper();
            }
            "#}
        );
    }

    #[test]
    fn test_reset() {
        let mut input: ItemStruct = parse_quote! {
            #[qproperty(i32, num, READ, WRITE = mySetter, RESET = my_resetter)]
            struct MyStruct;
        };

        let property = ParsedQProperty::parse(input.attrs.remove(0)).unwrap();

        let properties = vec![property];

        let qobject_idents = create_qobjectname();

        // Prototyping, this test need properly rewriting
        let module = mock_module_custom_setter_and_reset();
        let parser = Parser::from(module).unwrap();
        let structures = Structures::new(&parser.cxx_qt_data).unwrap();

        let structured_qobject = structures.qobjects.first().unwrap();

        let type_names = TypeNames::mock();
        let generated = generate_cpp_properties(
            &properties,
            &qobject_idents,
            &type_names,
            structured_qobject,
        )
        .unwrap();

        assert_str_eq!(
            generated.metaobjects[0],
            "Q_PROPERTY(::std::int32_t num READ getNum WRITE mySetter RESET myResetter)"
        );
    }

    #[test]
    fn test_constant_and_required() {
        let mut input: ItemStruct = parse_quote! {
            #[qproperty(i32, num, READ, CONSTANT, REQUIRED)]
            struct MyStruct;
        };
        let generated = setup_generated(&mut input).unwrap();

        assert_str_eq!(
            generated.metaobjects[0],
            "Q_PROPERTY(::std::int32_t num READ getNum CONSTANT REQUIRED)"
        );
    }

    #[test]
    fn test_generate_cpp_properties() {
        let mut input1: ItemStruct = parse_quote! {
            #[qproperty(i32, trivial_property, READ, WRITE, NOTIFY)]
            struct MyStruct;
        };

        let mut input2: ItemStruct = parse_quote! {
            #[qproperty(UniquePtr<QColor>, opaque_property)]
            struct MyStruct;
        };

        let properties = vec![
            ParsedQProperty::parse(input1.attrs.remove(0)).unwrap(),
            ParsedQProperty::parse(input2.attrs.remove(0)).unwrap(),
        ];

        let qobject_idents = create_qobjectname();

        let obj = ParsedQObject::mock();

        let structured_qobject = StructuredQObject::mock(&obj);

        let mut type_names = TypeNames::mock();
        type_names.mock_insert("QColor", None, None, None);
        let generated = generate_cpp_properties(
            &properties,
            &qobject_idents,
            &type_names,
            &structured_qobject,
        )
        .unwrap();

        // metaobjects
        assert_eq!(generated.metaobjects.len(), 2);
        assert_str_eq!(generated.metaobjects[0], "Q_PROPERTY(::std::int32_t trivialProperty READ getTrivialProperty WRITE setTrivialProperty NOTIFY trivialPropertyChanged)");
        assert_str_eq!(generated.metaobjects[1], "Q_PROPERTY(::std::unique_ptr<QColor> opaqueProperty READ getOpaqueProperty WRITE setOpaqueProperty NOTIFY opaquePropertyChanged)");

        // methods
        assert_eq!(generated.methods.len(), 6);

        let (header, source) = require_pair(&generated.methods[0]).unwrap();
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

        let (header, source) = require_pair(&generated.methods[1]).unwrap();
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

        let (header, source) = require_pair(&generated.methods[2]).unwrap();

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

        let (header, source) = require_pair(&generated.methods[3]).unwrap();
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

        let header = require_header(&generated.methods[4]).unwrap();
        assert_str_eq!(header, "Q_SIGNAL void trivialPropertyChanged();");

        let header = require_header(&generated.methods[5]).unwrap();
        assert_str_eq!(header, "Q_SIGNAL void opaquePropertyChanged();");

        assert_eq!(generated.fragments.len(), 2);
        let (header, source) = require_pair(&generated.fragments[0]).unwrap();

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

        let (header, source) = require_pair(&generated.fragments[1]).unwrap();

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
        let header = require_header(&generated.private_methods[0]).unwrap();
        assert_str_eq!(
            header,
            "::std::int32_t const& getTrivialPropertyWrapper() const noexcept;"
        );

        let header = require_header(&generated.private_methods[1]).unwrap();
        assert_str_eq!(
            header,
            "void setTrivialPropertyWrapper(::std::int32_t value) noexcept;"
        );

        let header = require_header(&generated.private_methods[2]).unwrap();

        assert_str_eq!(
            header,
            "::std::unique_ptr<QColor> const& getOpaquePropertyWrapper() const noexcept;"
        );

        let header = require_header(&generated.private_methods[3]).unwrap();

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
            flags: QPropertyFlags::default(),
        }];
        let qobject_idents = create_qobjectname();

        let obj = ParsedQObject::mock();

        let structured_qobject = StructuredQObject::mock(&obj);

        let mut type_names = TypeNames::mock();
        type_names.mock_insert("A", None, Some("A1"), None);

        let generated = generate_cpp_properties(
            &properties,
            &qobject_idents,
            &type_names,
            &structured_qobject,
        )
        .unwrap();

        // metaobjects
        assert_eq!(generated.metaobjects.len(), 1);
        assert_str_eq!(generated.metaobjects[0], "Q_PROPERTY(A1 mappedProperty READ getMappedProperty WRITE setMappedProperty NOTIFY mappedPropertyChanged)");

        // methods
        assert_eq!(generated.methods.len(), 3);
        let (header, source) = require_pair(&generated.methods[0]).unwrap();

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
        let (header, source) = require_pair(&generated.methods[1]).unwrap();

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
        let header = require_header(&generated.methods[2]).unwrap();

        assert_str_eq!(header, "Q_SIGNAL void mappedPropertyChanged();");

        assert_eq!(generated.fragments.len(), 1);

        let (header, source) = require_pair(&generated.fragments[0]).unwrap();

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
        let header = require_header(&generated.private_methods[0]).unwrap();

        assert_str_eq!(
            header,
            "A1 const& getMappedPropertyWrapper() const noexcept;"
        );

        let header = require_header(&generated.private_methods[1]).unwrap();
        assert_str_eq!(header, "void setMappedPropertyWrapper(A1 value) noexcept;");
    }
}
