// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

pub mod getter;
pub mod setter;
pub mod signal;

use super::signals::generate_rust_signals;
use crate::generator::structuring::StructuredQObject;
use crate::{
    generator::{
        naming::{property::QPropertyNames, qobject::QObjectNames},
        rust::fragment::GeneratedRustFragment,
    },
    naming::TypeNames,
    parser::property::ParsedQProperty,
};
use syn::{Ident, Result};

pub fn generate_rust_properties(
    properties: &Vec<ParsedQProperty>,
    qobject_idents: &QObjectNames,
    type_names: &TypeNames,
    module_ident: &Ident,
    structured_qobject: &StructuredQObject,
) -> Result<GeneratedRustFragment> {
    let mut generated = GeneratedRustFragment::default();
    let mut signals = vec![];

    for property in properties {
        let idents = QPropertyNames::try_from_property(property, structured_qobject)?;

        if let Some(getter) = getter::generate(&idents, qobject_idents, &property.ty, type_names)? {
            generated
                .cxx_mod_contents
                .append(&mut getter.cxx_bridge_as_items()?);
            generated
                .cxx_qt_mod_contents
                .append(&mut getter.implementation_as_items()?);
        };

        if let Some(setter) = setter::generate(&idents, qobject_idents, &property.ty, type_names)? {
            generated
                .cxx_mod_contents
                .append(&mut setter.cxx_bridge_as_items()?);
            generated
                .cxx_qt_mod_contents
                .append(&mut setter.implementation_as_items()?);
        }

        if let Some(notify) = signal::generate(&idents, qobject_idents) {
            signals.push(notify)
        }
    }

    generated.append(&mut generate_rust_signals(
        &signals.iter().collect(),
        qobject_idents,
        type_names,
        module_ident,
    )?);

    Ok(generated)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::naming::Name;
    use crate::parser::property::QPropertyFlags;
    use crate::parser::qobject::ParsedQObject;
    use crate::syntax::foreignmod::ForeignTypeIdentAlias;
    use crate::{generator::naming::qobject::tests::create_qobjectname, tests::assert_tokens_eq};
    use quote::format_ident;
    use syn::parse_quote;

    #[test]
    fn test_generate_rust_properties() {
        let properties = vec![
            ParsedQProperty {
                ident: format_ident!("trivial_property"),
                ty: parse_quote! { i32 },
                flags: QPropertyFlags::default(),
            },
            ParsedQProperty {
                ident: format_ident!("opaque_property"),
                ty: parse_quote! { UniquePtr<QColor> },
                flags: QPropertyFlags::default(),
            },
            ParsedQProperty {
                ident: format_ident!("unsafe_property"),
                ty: parse_quote! { *mut T },
                flags: QPropertyFlags::default(),
            },
        ];
        let qobject_idents = create_qobjectname();

        // PROTO mocking only
        let obj = ParsedQObject {
            base_class: None,
            name: Name::new(format_ident!("my_property")),
            rust_type: format_ident!("i32"),
            inherited_methods: vec![],
            constructors: vec![],
            properties: vec![],
            qml_metadata: None,
            locking: false,
            threading: false,
            has_qobject_macro: false,
            declaration: ForeignTypeIdentAlias {
                attrs: vec![],
                ident_left: format_ident!("MyObject"),
                ident_right: format_ident!("MyObjectRust"),
            },
        };

        let structured_qobject = StructuredQObject::from_qobject(&obj);

        let mut type_names = TypeNames::mock();
        type_names.mock_insert("T", None, None, None);
        type_names.mock_insert("QColor", None, None, None);
        let generated = generate_rust_properties(
            &properties,
            &qobject_idents,
            &type_names,
            &format_ident!("ffi"),
            &structured_qobject,
        )
        .unwrap();

        // Check that we have the expected number of blocks
        assert_eq!(generated.cxx_mod_contents.len(), 15);
        assert_eq!(generated.cxx_qt_mod_contents.len(), 30);

        // Trivial Property

        // Getter
        assert_tokens_eq(
            &generated.cxx_mod_contents[0],
            parse_quote! {
                extern "Rust" {
                    #[cxx_name = "getTrivialPropertyWrapper"]
                    unsafe fn trivial_property<'a>(self: &'a MyObject) -> &'a i32;
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[0],
            parse_quote! {
                impl qobject::MyObject {
                    #[doc = "Getter for the Q_PROPERTY "]
                    #[doc = "trivial_property"]
                    pub fn trivial_property(&self) -> &i32 {
                        &self.trivial_property
                    }
                }
            },
        );

        // Setters
        assert_tokens_eq(
            &generated.cxx_mod_contents[1],
            parse_quote! {
                extern "Rust" {
                    #[cxx_name = "setTrivialPropertyWrapper"]
                    fn set_trivial_property(self: Pin<&mut MyObject>, value: i32);
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[1],
            parse_quote! {
                impl qobject::MyObject {
                    #[doc = "Setter for the Q_PROPERTY "]
                    #[doc = "trivial_property"]
                    pub fn set_trivial_property(mut self: core::pin::Pin<&mut Self>, value: i32) {
                        use cxx_qt::CxxQtType;
                        if self.trivial_property == value {
                            return;
                        }
                        self.as_mut().rust_mut().trivial_property = value;
                        self.as_mut().trivial_property_changed();
                    }
                }
            },
        );

        // Opaque Property

        // Getter
        assert_tokens_eq(
            &generated.cxx_mod_contents[2],
            parse_quote! {
                extern "Rust" {
                    #[cxx_name = "getOpaquePropertyWrapper"]
                    unsafe fn opaque_property<'a>(self: &'a MyObject) -> &'a UniquePtr<QColor>;
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[2],
            parse_quote! {
                impl qobject::MyObject {
                    #[doc = "Getter for the Q_PROPERTY "]
                    #[doc = "opaque_property"]
                    pub fn opaque_property(&self) -> &cxx::UniquePtr<QColor> {
                        &self.opaque_property
                    }
                }
            },
        );

        // Setters
        assert_tokens_eq(
            &generated.cxx_mod_contents[3],
            parse_quote! {
                extern "Rust" {
                    #[cxx_name = "setOpaquePropertyWrapper"]
                    fn set_opaque_property(self: Pin<&mut MyObject>, value: UniquePtr<QColor>);
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[3],
            parse_quote! {
                impl qobject::MyObject {
                    #[doc = "Setter for the Q_PROPERTY "]
                    #[doc = "opaque_property"]
                    pub fn set_opaque_property(mut self: core::pin::Pin<&mut Self>, value: cxx::UniquePtr<QColor>) {
                        use cxx_qt::CxxQtType;
                        if self.opaque_property == value {
                            return;
                        }
                        self.as_mut().rust_mut().opaque_property = value;
                        self.as_mut().opaque_property_changed();
                    }
                }
            },
        );

        // Unsafe Property

        // Getter
        assert_tokens_eq(
            &generated.cxx_mod_contents[4],
            parse_quote! {
                extern "Rust" {
                    #[cxx_name = "getUnsafePropertyWrapper"]
                    unsafe fn unsafe_property<'a>(self: &'a MyObject) -> &'a *mut T;
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[4],
            parse_quote! {
                impl qobject::MyObject {
                    #[doc = "Getter for the Q_PROPERTY "]
                    #[doc = "unsafe_property"]
                    pub fn unsafe_property(&self) -> &*mut T {
                        &self.unsafe_property
                    }
                }
            },
        );

        // Setters
        assert_tokens_eq(
            &generated.cxx_mod_contents[5],
            parse_quote! {
                extern "Rust" {
                    #[cxx_name = "setUnsafePropertyWrapper"]
                    unsafe fn set_unsafe_property(self: Pin<&mut MyObject>, value: *mut T);
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[5],
            parse_quote! {
                impl qobject::MyObject {
                    #[doc = "Setter for the Q_PROPERTY "]
                    #[doc = "unsafe_property"]
                    pub fn set_unsafe_property(mut self: core::pin::Pin<&mut Self>, value: *mut T) {
                        use cxx_qt::CxxQtType;
                        if self.unsafe_property == value {
                            return;
                        }
                        self.as_mut().rust_mut().unsafe_property = value;
                        self.as_mut().unsafe_property_changed();
                    }
                }
            },
        );

        // Signals

        // trivial_property

        assert_tokens_eq(
            &generated.cxx_mod_contents[6],
            parse_quote! {
                unsafe extern "C++" {
                    #[cxx_name = "trivialPropertyChanged"]
                    #[doc = "Notify for the Q_PROPERTY"]
                    fn trivial_property_changed(self: Pin<&mut MyObject>);
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_mod_contents[7],
            parse_quote! {
                unsafe extern "C++" {
                    #[doc(hidden)]
                    #[namespace = "rust::cxxqtgen1"]
                    type MyObjectCxxQtSignalHandlertrivialPropertyChanged = cxx_qt::signalhandler::CxxQtSignalHandler<super::MyObjectCxxQtSignalClosuretrivialPropertyChanged>;

                    #[doc(hidden)]
                    #[namespace = "rust::cxxqtgen1"]
                    #[cxx_name = "MyObject_trivialPropertyChangedConnect"]
                    fn MyObject_connect_trivial_property_changed(self_value: Pin<&mut MyObject>, signal_handler: MyObjectCxxQtSignalHandlertrivialPropertyChanged, conn_type: CxxQtConnectionType) -> CxxQtQMetaObjectConnection;
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_mod_contents[8],
            parse_quote! {
                #[namespace = "rust::cxxqtgen1"]
                extern "Rust" {
                    #[doc(hidden)]
                    fn drop_MyObject_signal_handler_trivialPropertyChanged(handler: MyObjectCxxQtSignalHandlertrivialPropertyChanged);

                    #[doc(hidden)]
                    fn call_MyObject_signal_handler_trivialPropertyChanged(handler: &mut MyObjectCxxQtSignalHandlertrivialPropertyChanged, self_value: Pin<&mut MyObject>, );
                }
            },
        );

        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[6],
            parse_quote! {
                impl qobject::MyObject {
                    #[doc = "Connect the given function pointer to the signal "]
                    #[doc = "trivialPropertyChanged"]
                    #[doc = ", so that when the signal is emitted the function pointer is executed."]
                    pub fn connect_trivial_property_changed<F: FnMut(core::pin::Pin<&mut qobject::MyObject>, ) + 'static>(self: core::pin::Pin<&mut qobject::MyObject>, mut closure: F, conn_type: cxx_qt::ConnectionType) -> cxx_qt::QMetaObjectConnectionGuard
                    {
                        cxx_qt::QMetaObjectConnectionGuard::from(ffi::MyObject_connect_trivial_property_changed(
                            self,
                            cxx_qt::signalhandler::CxxQtSignalHandler::<MyObjectCxxQtSignalClosuretrivialPropertyChanged>::new(Box::new(closure)),
                            conn_type,
                        ))
                    }
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[7],
            parse_quote! {
                impl qobject::MyObject {
                    #[doc = "Connect the given function pointer to the signal "]
                    #[doc = "trivialPropertyChanged"]
                    #[doc = ", so that when the signal is emitted the function pointer is executed."]
                    #[doc = "\n"]
                    #[doc = "Note that this method uses a AutoConnection connection type."]
                    pub fn on_trivial_property_changed<F: FnMut(core::pin::Pin<&mut qobject::MyObject>, ) + 'static>(self: core::pin::Pin<&mut qobject::MyObject>, mut closure: F) -> cxx_qt::QMetaObjectConnectionGuard
                    {
                        cxx_qt::QMetaObjectConnectionGuard::from(ffi::MyObject_connect_trivial_property_changed(
                            self,
                            cxx_qt::signalhandler::CxxQtSignalHandler::<MyObjectCxxQtSignalClosuretrivialPropertyChanged>::new(Box::new(closure)),
                            cxx_qt::ConnectionType::AutoConnection,
                        ))
                    }
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[8],
            parse_quote! {
                #[doc(hidden)]
                pub struct MyObjectCxxQtSignalClosuretrivialPropertyChanged {}
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[9],
            parse_quote! {
                impl cxx_qt::signalhandler::CxxQtSignalHandlerClosure for MyObjectCxxQtSignalClosuretrivialPropertyChanged {
                    type Id = cxx::type_id!("::rust::cxxqtgen1::MyObjectCxxQtSignalHandlertrivialPropertyChanged");
                    type FnType = dyn FnMut(core::pin::Pin<&mut qobject::MyObject>, );
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[10],
            parse_quote! {
                use core::mem::drop as drop_MyObject_signal_handler_trivialPropertyChanged;
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[11],
            parse_quote! {
                fn call_MyObject_signal_handler_trivialPropertyChanged(
                    handler: &mut cxx_qt::signalhandler::CxxQtSignalHandler<MyObjectCxxQtSignalClosuretrivialPropertyChanged>,
                    self_value: core::pin::Pin<&mut qobject::MyObject>,
                ) {
                    handler.closure()(self_value, );
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[12],
            parse_quote! {
                cxx_qt::static_assertions::assert_eq_align!(cxx_qt::signalhandler::CxxQtSignalHandler<MyObjectCxxQtSignalClosuretrivialPropertyChanged>, usize);
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[13],
            parse_quote! {
                cxx_qt::static_assertions::assert_eq_size!(cxx_qt::signalhandler::CxxQtSignalHandler<MyObjectCxxQtSignalClosuretrivialPropertyChanged>, [usize; 2]);
            },
        );

        // opaque_property

        assert_tokens_eq(
            &generated.cxx_mod_contents[9],
            parse_quote! {
                unsafe extern "C++" {
                    #[cxx_name = "opaquePropertyChanged"]
                    #[doc = "Notify for the Q_PROPERTY"]
                    fn opaque_property_changed(self: Pin<&mut MyObject>);
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_mod_contents[10],
            parse_quote! {
                unsafe extern "C++" {
                    #[doc(hidden)]
                    #[namespace = "rust::cxxqtgen1"]
                    type MyObjectCxxQtSignalHandleropaquePropertyChanged = cxx_qt::signalhandler::CxxQtSignalHandler<super::MyObjectCxxQtSignalClosureopaquePropertyChanged>;

                    #[doc(hidden)]
                    #[namespace = "rust::cxxqtgen1"]
                    #[cxx_name = "MyObject_opaquePropertyChangedConnect"]
                    fn MyObject_connect_opaque_property_changed(self_value: Pin<&mut MyObject>, signal_handler: MyObjectCxxQtSignalHandleropaquePropertyChanged, conn_type: CxxQtConnectionType) -> CxxQtQMetaObjectConnection;
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_mod_contents[11],
            parse_quote! {
                #[namespace = "rust::cxxqtgen1"]
                extern "Rust" {
                    #[doc(hidden)]
                    fn drop_MyObject_signal_handler_opaquePropertyChanged(handler: MyObjectCxxQtSignalHandleropaquePropertyChanged);

                    #[doc(hidden)]
                    fn call_MyObject_signal_handler_opaquePropertyChanged(handler: &mut MyObjectCxxQtSignalHandleropaquePropertyChanged, self_value: Pin<&mut MyObject>, );
                }
            },
        );

        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[14],
            parse_quote! {
                impl qobject::MyObject {
                    #[doc = "Connect the given function pointer to the signal "]
                    #[doc = "opaquePropertyChanged"]
                    #[doc = ", so that when the signal is emitted the function pointer is executed."]
                    pub fn connect_opaque_property_changed<F: FnMut(core::pin::Pin<&mut qobject::MyObject>, ) + 'static>(self: core::pin::Pin<&mut qobject::MyObject>, mut closure: F, conn_type: cxx_qt::ConnectionType) -> cxx_qt::QMetaObjectConnectionGuard
                    {
                        cxx_qt::QMetaObjectConnectionGuard::from(ffi::MyObject_connect_opaque_property_changed(
                            self,
                            cxx_qt::signalhandler::CxxQtSignalHandler::<MyObjectCxxQtSignalClosureopaquePropertyChanged>::new(Box::new(closure)),
                            conn_type,
                        ))
                    }
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[15],
            parse_quote! {
                impl qobject::MyObject {
                    #[doc = "Connect the given function pointer to the signal "]
                    #[doc = "opaquePropertyChanged"]
                    #[doc = ", so that when the signal is emitted the function pointer is executed."]
                    #[doc = "\n"]
                    #[doc = "Note that this method uses a AutoConnection connection type."]
                    pub fn on_opaque_property_changed<F: FnMut(core::pin::Pin<&mut qobject::MyObject>, ) + 'static>(self: core::pin::Pin<&mut qobject::MyObject>, mut closure: F) -> cxx_qt::QMetaObjectConnectionGuard
                    {
                        cxx_qt::QMetaObjectConnectionGuard::from(ffi::MyObject_connect_opaque_property_changed(
                            self,
                            cxx_qt::signalhandler::CxxQtSignalHandler::<MyObjectCxxQtSignalClosureopaquePropertyChanged>::new(Box::new(closure)),
                            cxx_qt::ConnectionType::AutoConnection,
                        ))
                    }
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[16],
            parse_quote! {
                #[doc(hidden)]
                pub struct MyObjectCxxQtSignalClosureopaquePropertyChanged {}
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[17],
            parse_quote! {
                impl cxx_qt::signalhandler::CxxQtSignalHandlerClosure for MyObjectCxxQtSignalClosureopaquePropertyChanged {
                    type Id = cxx::type_id!("::rust::cxxqtgen1::MyObjectCxxQtSignalHandleropaquePropertyChanged");
                    type FnType = dyn FnMut(core::pin::Pin<&mut qobject::MyObject>, );
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[18],
            parse_quote! {
                use core::mem::drop as drop_MyObject_signal_handler_opaquePropertyChanged;
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[19],
            parse_quote! {
                fn call_MyObject_signal_handler_opaquePropertyChanged(
                    handler: &mut cxx_qt::signalhandler::CxxQtSignalHandler<MyObjectCxxQtSignalClosureopaquePropertyChanged>,
                    self_value: core::pin::Pin<&mut qobject::MyObject>,
                ) {
                    handler.closure()(self_value, );
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[20],
            parse_quote! {
                cxx_qt::static_assertions::assert_eq_align!(cxx_qt::signalhandler::CxxQtSignalHandler<MyObjectCxxQtSignalClosureopaquePropertyChanged>, usize);
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[21],
            parse_quote! {
                cxx_qt::static_assertions::assert_eq_size!(cxx_qt::signalhandler::CxxQtSignalHandler<MyObjectCxxQtSignalClosureopaquePropertyChanged>, [usize; 2]);
            },
        );

        // unsafe_property

        assert_tokens_eq(
            &generated.cxx_mod_contents[12],
            parse_quote! {
                unsafe extern "C++" {
                    #[cxx_name = "unsafePropertyChanged"]
                    #[doc = "Notify for the Q_PROPERTY"]
                    fn unsafe_property_changed(self: Pin<&mut MyObject>);
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_mod_contents[13],
            parse_quote! {
                unsafe extern "C++" {
                    #[doc(hidden)]
                    #[namespace = "rust::cxxqtgen1"]
                    type MyObjectCxxQtSignalHandlerunsafePropertyChanged = cxx_qt::signalhandler::CxxQtSignalHandler<super::MyObjectCxxQtSignalClosureunsafePropertyChanged>;

                    #[doc(hidden)]
                    #[namespace = "rust::cxxqtgen1"]
                    #[cxx_name = "MyObject_unsafePropertyChangedConnect"]
                    fn MyObject_connect_unsafe_property_changed(self_value: Pin<&mut MyObject>, signal_handler: MyObjectCxxQtSignalHandlerunsafePropertyChanged, conn_type: CxxQtConnectionType) -> CxxQtQMetaObjectConnection;
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_mod_contents[14],
            parse_quote! {
                #[namespace = "rust::cxxqtgen1"]
                extern "Rust" {
                    #[doc(hidden)]
                    fn drop_MyObject_signal_handler_unsafePropertyChanged(handler: MyObjectCxxQtSignalHandlerunsafePropertyChanged);

                    #[doc(hidden)]
                    fn call_MyObject_signal_handler_unsafePropertyChanged(handler: &mut MyObjectCxxQtSignalHandlerunsafePropertyChanged, self_value: Pin<&mut MyObject>, );
                }
            },
        );

        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[22],
            parse_quote! {
                impl qobject::MyObject {
                    #[doc = "Connect the given function pointer to the signal "]
                    #[doc = "unsafePropertyChanged"]
                    #[doc = ", so that when the signal is emitted the function pointer is executed."]
                    pub fn connect_unsafe_property_changed<F: FnMut(core::pin::Pin<&mut qobject::MyObject>, ) + 'static>(self: core::pin::Pin<&mut qobject::MyObject>, mut closure: F, conn_type: cxx_qt::ConnectionType) -> cxx_qt::QMetaObjectConnectionGuard
                    {
                        cxx_qt::QMetaObjectConnectionGuard::from(ffi::MyObject_connect_unsafe_property_changed(
                            self,
                            cxx_qt::signalhandler::CxxQtSignalHandler::<MyObjectCxxQtSignalClosureunsafePropertyChanged>::new(Box::new(closure)),
                            conn_type,
                        ))
                    }
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[23],
            parse_quote! {
                impl qobject::MyObject {
                    #[doc = "Connect the given function pointer to the signal "]
                    #[doc = "unsafePropertyChanged"]
                    #[doc = ", so that when the signal is emitted the function pointer is executed."]
                    #[doc = "\n"]
                    #[doc = "Note that this method uses a AutoConnection connection type."]
                    pub fn on_unsafe_property_changed<F: FnMut(core::pin::Pin<&mut qobject::MyObject>, ) + 'static>(self: core::pin::Pin<&mut qobject::MyObject>, mut closure: F) -> cxx_qt::QMetaObjectConnectionGuard
                    {
                        cxx_qt::QMetaObjectConnectionGuard::from(ffi::MyObject_connect_unsafe_property_changed(
                            self,
                            cxx_qt::signalhandler::CxxQtSignalHandler::<MyObjectCxxQtSignalClosureunsafePropertyChanged>::new(Box::new(closure)),
                            cxx_qt::ConnectionType::AutoConnection,
                        ))
                    }
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[24],
            parse_quote! {
                #[doc(hidden)]
                pub struct MyObjectCxxQtSignalClosureunsafePropertyChanged {}
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[25],
            parse_quote! {
                impl cxx_qt::signalhandler::CxxQtSignalHandlerClosure for MyObjectCxxQtSignalClosureunsafePropertyChanged {
                    type Id = cxx::type_id!("::rust::cxxqtgen1::MyObjectCxxQtSignalHandlerunsafePropertyChanged");
                    type FnType = dyn FnMut(core::pin::Pin<&mut qobject::MyObject>, );
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[26],
            parse_quote! {
                use core::mem::drop as drop_MyObject_signal_handler_unsafePropertyChanged;
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[27],
            parse_quote! {
                fn call_MyObject_signal_handler_unsafePropertyChanged(
                    handler: &mut cxx_qt::signalhandler::CxxQtSignalHandler<MyObjectCxxQtSignalClosureunsafePropertyChanged>,
                    self_value: core::pin::Pin<&mut qobject::MyObject>,
                ) {
                    handler.closure()(self_value, );
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[28],
            parse_quote! {
                cxx_qt::static_assertions::assert_eq_align!(cxx_qt::signalhandler::CxxQtSignalHandler<MyObjectCxxQtSignalClosureunsafePropertyChanged>, usize);
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[29],
            parse_quote! {
                cxx_qt::static_assertions::assert_eq_size!(cxx_qt::signalhandler::CxxQtSignalHandler<MyObjectCxxQtSignalClosureunsafePropertyChanged>, [usize; 2]);
            },
        );
    }
}
