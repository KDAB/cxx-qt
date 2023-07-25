// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

pub mod getter;
pub mod setter;
pub mod signal;

use crate::{
    generator::{
        naming::{property::QPropertyName, qobject::QObjectName},
        rust::qobject::GeneratedRustQObject,
    },
    parser::property::ParsedQProperty,
};
use std::collections::BTreeMap;
use syn::{Ident, Path, Result};

use super::signals::generate_rust_signals;

pub fn generate_rust_properties(
    properties: &Vec<ParsedQProperty>,
    qobject_idents: &QObjectName,
    qualified_mappings: &BTreeMap<Ident, Path>,
) -> Result<GeneratedRustQObject> {
    let mut generated = GeneratedRustQObject::default();
    let mut signals = vec![];

    for property in properties {
        let idents = QPropertyName::from(property);

        // Getters
        let getter = getter::generate(&idents, qobject_idents, &property.ty, qualified_mappings);
        generated
            .cxx_mod_contents
            .append(&mut getter.cxx_bridge_as_items()?);
        generated
            .cxx_qt_mod_contents
            .append(&mut getter.implementation_as_items()?);

        // Setters
        let setter = setter::generate(&idents, qobject_idents, &property.ty, qualified_mappings);
        generated
            .cxx_mod_contents
            .append(&mut setter.cxx_bridge_as_items()?);
        generated
            .cxx_qt_mod_contents
            .append(&mut setter.implementation_as_items()?);

        // Signals
        signals.push(signal::generate(&idents, qobject_idents));
    }

    generated.append(&mut generate_rust_signals(
        &signals,
        qobject_idents,
        qualified_mappings,
    )?);

    Ok(generated)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{generator::naming::qobject::tests::create_qobjectname, tests::assert_tokens_eq};
    use quote::format_ident;
    use syn::parse_quote;

    #[test]
    fn test_generate_rust_properties() {
        let properties = vec![
            ParsedQProperty {
                ident: format_ident!("trivial_property"),
                ty: parse_quote! { i32 },
            },
            ParsedQProperty {
                ident: format_ident!("opaque_property"),
                ty: parse_quote! { UniquePtr<QColor> },
            },
            ParsedQProperty {
                ident: format_ident!("unsafe_property"),
                ty: parse_quote! { *mut T },
            },
        ];
        let qobject_idents = create_qobjectname();

        let generated = generate_rust_properties(
            &properties,
            &qobject_idents,
            &BTreeMap::<Ident, Path>::default(),
        )
        .unwrap();

        // Check that we have the expected number of blocks
        assert_eq!(generated.cxx_mod_contents.len(), 12);
        assert_eq!(generated.cxx_qt_mod_contents.len(), 9);

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
                impl MyObject {
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
                impl MyObject {
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
                impl MyObject {
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
                impl MyObject {
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
                impl MyObject {
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
                impl MyObject {
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

        assert_tokens_eq(
            &generated.cxx_mod_contents[6],
            parse_quote! {
                unsafe extern "C++" {
                    #[doc = "Notify for the Q_PROPERTY"]
                    #[rust_name = "trivial_property_changed"]
                    fn trivialPropertyChanged(self: Pin<&mut MyObject>, );
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_mod_contents[7],
            parse_quote! {
                unsafe extern "C++" {
                    #[doc = "Connect the given function pointer to the signal "]
                    #[doc = "trivialPropertyChanged"]
                    #[doc = ", so that when the signal is emitted the function pointer is executed."]
                    #[must_use]
                    #[rust_name = "connect_trivial_property_changed"]
                    fn trivialPropertyChangedConnect(self: Pin <&mut MyObject>, func: fn(Pin<&mut MyObject>, ), conn_type : CxxQtConnectionType) -> CxxQtQMetaObjectConnection;
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[6],
            parse_quote! {
                impl MyObject {
                    #[doc = "Connect the given function pointer to the signal "]
                    #[doc = "trivialPropertyChanged"]
                    #[doc = ", so that when the signal is emitted the function pointer is executed."]
                    #[doc = "\n"]
                    #[doc = "Note that this method uses a AutoConnection connection type."]
                    #[must_use]
                    pub fn on_trivial_property_changed(self: core::pin::Pin<&mut MyObject>, func: fn(core::pin::Pin<&mut MyObject>, )) -> cxx_qt_lib::QMetaObjectConnection
                    {
                        self.connect_trivial_property_changed(func, cxx_qt_lib::ConnectionType::AutoConnection)
                    }
                }
            },
        );

        assert_tokens_eq(
            &generated.cxx_mod_contents[8],
            parse_quote! {
                unsafe extern "C++" {
                    #[doc = "Notify for the Q_PROPERTY"]
                    #[rust_name = "opaque_property_changed"]
                    fn opaquePropertyChanged(self: Pin<&mut MyObject>, );
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_mod_contents[9],
            parse_quote! {
                unsafe extern "C++" {
                    #[doc = "Connect the given function pointer to the signal "]
                    #[doc = "opaquePropertyChanged"]
                    #[doc = ", so that when the signal is emitted the function pointer is executed."]
                    #[must_use]
                    #[rust_name = "connect_opaque_property_changed"]
                    fn opaquePropertyChangedConnect(self: Pin <&mut MyObject>, func: fn(Pin<&mut MyObject>, ), conn_type : CxxQtConnectionType) -> CxxQtQMetaObjectConnection;
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[7],
            parse_quote! {
                impl MyObject {
                    #[doc = "Connect the given function pointer to the signal "]
                    #[doc = "opaquePropertyChanged"]
                    #[doc = ", so that when the signal is emitted the function pointer is executed."]
                    #[doc = "\n"]
                    #[doc = "Note that this method uses a AutoConnection connection type."]
                    #[must_use]
                    pub fn on_opaque_property_changed(self: core::pin::Pin<&mut MyObject>, func: fn(core::pin::Pin<&mut MyObject>, )) -> cxx_qt_lib::QMetaObjectConnection
                    {
                        self.connect_opaque_property_changed(func, cxx_qt_lib::ConnectionType::AutoConnection)
                    }
                }
            },
        );

        assert_tokens_eq(
            &generated.cxx_mod_contents[10],
            parse_quote! {
                unsafe extern "C++" {
                    #[doc = "Notify for the Q_PROPERTY"]
                    #[rust_name = "unsafe_property_changed"]
                    fn unsafePropertyChanged(self: Pin<&mut MyObject>, );
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_mod_contents[11],
            parse_quote! {
                unsafe extern "C++" {
                    #[doc = "Connect the given function pointer to the signal "]
                    #[doc = "unsafePropertyChanged"]
                    #[doc = ", so that when the signal is emitted the function pointer is executed."]
                    #[must_use]
                    #[rust_name = "connect_unsafe_property_changed"]
                    fn unsafePropertyChangedConnect(self: Pin <&mut MyObject>, func: fn(Pin<&mut MyObject>, ), conn_type : CxxQtConnectionType) -> CxxQtQMetaObjectConnection;
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[8],
            parse_quote! {
                impl MyObject {
                    #[doc = "Connect the given function pointer to the signal "]
                    #[doc = "unsafePropertyChanged"]
                    #[doc = ", so that when the signal is emitted the function pointer is executed."]
                    #[doc = "\n"]
                    #[doc = "Note that this method uses a AutoConnection connection type."]
                    #[must_use]
                    pub fn on_unsafe_property_changed(self: core::pin::Pin<&mut MyObject>, func: fn(core::pin::Pin<&mut MyObject>, )) -> cxx_qt_lib::QMetaObjectConnection
                    {
                        self.connect_unsafe_property_changed(func, cxx_qt_lib::ConnectionType::AutoConnection)
                    }
                }
            },
        );
    }
}
