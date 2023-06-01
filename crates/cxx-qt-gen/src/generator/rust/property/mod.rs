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
        rust::qobject::GeneratedRustQObjectBlocks,
    },
    parser::property::ParsedQProperty,
};
use syn::Result;

use super::signals::generate_rust_signals;

pub fn generate_rust_properties(
    properties: &Vec<ParsedQProperty>,
    qobject_idents: &QObjectName,
) -> Result<GeneratedRustQObjectBlocks> {
    let mut generated = GeneratedRustQObjectBlocks::default();
    let mut signals = vec![];

    for property in properties {
        let idents = QPropertyName::from(property);

        // Getters
        let getter = getter::generate(&idents, qobject_idents, &property.ty);
        generated
            .cxx_mod_contents
            .append(&mut getter.cxx_bridge_as_items()?);
        generated
            .cxx_qt_mod_contents
            .append(&mut getter.implementation_as_items()?);

        // Setters
        let setter = setter::generate(&idents, qobject_idents, &property.ty);
        generated
            .cxx_mod_contents
            .append(&mut setter.cxx_bridge_as_items()?);
        generated
            .cxx_qt_mod_contents
            .append(&mut setter.implementation_as_items()?);

        // Signals
        signals.push(signal::generate(&idents, qobject_idents));
    }

    generated.append(&mut generate_rust_signals(&signals, qobject_idents)?);

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
                vis: syn::Visibility::Inherited,
            },
            ParsedQProperty {
                ident: format_ident!("opaque_property"),
                ty: parse_quote! { UniquePtr<QColor> },
                vis: parse_quote! { pub },
            },
            ParsedQProperty {
                ident: format_ident!("unsafe_property"),
                ty: parse_quote! { *mut T },
                vis: syn::Visibility::Inherited,
            },
        ];
        let qobject_idents = create_qobjectname();

        let generated = generate_rust_properties(&properties, &qobject_idents).unwrap();

        // Check that we have the expected number of blocks
        assert_eq!(generated.cxx_mod_contents.len(), 12);
        assert_eq!(generated.cxx_qt_mod_contents.len(), 18);

        // Trivial Property

        // Getter
        assert_tokens_eq(
            &generated.cxx_mod_contents[0],
            parse_quote! {
                extern "Rust" {
                    #[cxx_name = "getTrivialProperty"]
                    unsafe fn trivial_property<'a>(self: &'a MyObject, cpp: &'a MyObjectQt) -> &'a i32;
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[0],
            parse_quote! {
                impl MyObject {
                    #[doc(hidden)]
                    pub fn trivial_property<'a>(&'a self, cpp: &'a MyObjectQt) -> &'a i32 {
                        cpp.trivial_property()
                    }
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[1],
            parse_quote! {
                impl MyObjectQt {
                    #[doc = "Getter for the Q_PROPERTY "]
                    #[doc = "trivial_property"]
                    pub fn trivial_property(&self) -> &i32 {
                        &self.rust().trivial_property
                    }
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[2],
            parse_quote! {
                impl MyObjectQt {
                    #[doc = "unsafe getter for the Q_PROPERTY "]
                    #[doc = "trivial_property"]
                    #[doc = "\n"]
                    #[doc = "This allows for modifying the Q_PROPERTY without calling the property changed Q_SIGNAL"]
                    #[doc = "\n"]
                    #[doc = "After modifying the property, make sure to call the corresponding changed signal: "]
                    #[doc = "trivial_property_changed"]
                    pub unsafe fn trivial_property_mut<'a>(self: Pin<&'a mut Self>) -> &'a mut i32 {
                        &mut self.rust_mut().get_unchecked_mut().trivial_property
                    }
                }
            },
        );

        // Setters
        assert_tokens_eq(
            &generated.cxx_mod_contents[1],
            parse_quote! {
                extern "Rust" {
                    #[cxx_name = "setTrivialProperty"]
                    fn set_trivial_property(self: &mut MyObject, cpp: Pin<&mut MyObjectQt>, value: i32);
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[3],
            parse_quote! {
                impl MyObject {
                    #[doc(hidden)]
                    pub fn set_trivial_property(&mut self, cpp: Pin<&mut MyObjectQt>, value: i32) {
                        cpp.set_trivial_property(value);
                    }
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[4],
            parse_quote! {
                impl MyObjectQt {
                    #[doc = "Setter for the Q_PROPERTY "]
                    #[doc = "trivial_property"]
                    pub fn set_trivial_property(mut self: Pin<&mut Self>, value: i32) {
                        if self.rust().trivial_property == value {
                            return;
                        }
                        unsafe {
                            self.as_mut().rust_mut().trivial_property = value;
                        }
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
                    #[cxx_name = "getOpaqueProperty"]
                    unsafe fn opaque_property<'a>(self: &'a MyObject, cpp: &'a MyObjectQt) -> &'a UniquePtr<QColor>;
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[5],
            parse_quote! {
                impl MyObject {
                    #[doc(hidden)]
                    pub fn opaque_property<'a>(&'a self, cpp: &'a MyObjectQt) -> &'a UniquePtr<QColor> {
                        cpp.opaque_property()
                    }
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[6],
            parse_quote! {
                impl MyObjectQt {
                    #[doc = "Getter for the Q_PROPERTY "]
                    #[doc = "opaque_property"]
                    pub fn opaque_property(&self) -> &UniquePtr<QColor> {
                        &self.rust().opaque_property
                    }
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[7],
            parse_quote! {
                impl MyObjectQt {
                    #[doc = "unsafe getter for the Q_PROPERTY "]
                    #[doc = "opaque_property"]
                    #[doc = "\n"]
                    #[doc = "This allows for modifying the Q_PROPERTY without calling the property changed Q_SIGNAL"]
                    #[doc = "\n"]
                    #[doc = "After modifying the property, make sure to call the corresponding changed signal: "]
                    #[doc = "opaque_property_changed"]
                    pub unsafe fn opaque_property_mut<'a>(self: Pin<&'a mut Self>) -> &'a mut UniquePtr<QColor> {
                        &mut self.rust_mut().get_unchecked_mut().opaque_property
                    }
                }
            },
        );

        // Setters
        assert_tokens_eq(
            &generated.cxx_mod_contents[3],
            parse_quote! {
                extern "Rust" {
                    #[cxx_name = "setOpaqueProperty"]
                    fn set_opaque_property(self: &mut MyObject, cpp: Pin<&mut MyObjectQt>, value: UniquePtr<QColor>);
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[8],
            parse_quote! {
                impl MyObject {
                    #[doc(hidden)]
                    pub fn set_opaque_property(&mut self, cpp: Pin<&mut MyObjectQt>, value: UniquePtr<QColor>) {
                        cpp.set_opaque_property(value);
                    }
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[9],
            parse_quote! {
                impl MyObjectQt {
                    #[doc = "Setter for the Q_PROPERTY "]
                    #[doc = "opaque_property"]
                    pub fn set_opaque_property(mut self: Pin<&mut Self>, value: UniquePtr<QColor>) {
                        if self.rust().opaque_property == value {
                            return;
                        }
                        unsafe {
                            self.as_mut().rust_mut().opaque_property = value;
                        }
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
                    #[cxx_name = "getUnsafeProperty"]
                    unsafe fn unsafe_property<'a>(self: &'a MyObject, cpp: &'a MyObjectQt) -> &'a *mut T;
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[10],
            parse_quote! {
                impl MyObject {
                    #[doc(hidden)]
                    pub fn unsafe_property<'a>(&'a self, cpp: &'a MyObjectQt) -> &'a *mut T {
                        cpp.unsafe_property()
                    }
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[11],
            parse_quote! {
                impl MyObjectQt {
                    #[doc = "Getter for the Q_PROPERTY "]
                    #[doc = "unsafe_property"]
                    pub fn unsafe_property(&self) -> &*mut T {
                        &self.rust().unsafe_property
                    }
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[12],
            parse_quote! {
                impl MyObjectQt {
                    #[doc = "unsafe getter for the Q_PROPERTY "]
                    #[doc = "unsafe_property"]
                    #[doc = "\n"]
                    #[doc = "This allows for modifying the Q_PROPERTY without calling the property changed Q_SIGNAL"]
                    #[doc = "\n"]
                    #[doc = "After modifying the property, make sure to call the corresponding changed signal: "]
                    #[doc = "unsafe_property_changed"]
                    pub unsafe fn unsafe_property_mut<'a>(self: Pin<&'a mut Self>) -> &'a mut *mut T {
                        &mut self.rust_mut().get_unchecked_mut().unsafe_property
                    }
                }
            },
        );

        // Setters
        assert_tokens_eq(
            &generated.cxx_mod_contents[5],
            parse_quote! {
                extern "Rust" {
                    #[cxx_name = "setUnsafeProperty"]
                    unsafe fn set_unsafe_property(self: &mut MyObject, cpp: Pin<&mut MyObjectQt>, value: *mut T);
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[13],
            parse_quote! {
                impl MyObject {
                    #[doc(hidden)]
                    pub fn set_unsafe_property(&mut self, cpp: Pin<&mut MyObjectQt>, value: *mut T) {
                        cpp.set_unsafe_property(value);
                    }
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[14],
            parse_quote! {
                impl MyObjectQt {
                    #[doc = "Setter for the Q_PROPERTY "]
                    #[doc = "unsafe_property"]
                    pub fn set_unsafe_property(mut self: Pin<&mut Self>, value: *mut T) {
                        if self.rust().unsafe_property == value {
                            return;
                        }
                        unsafe {
                            self.as_mut().rust_mut().unsafe_property = value;
                        }
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
                    fn emitTrivialPropertyChanged(self: Pin<&mut MyObjectQt>, );
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
                    fn trivialPropertyChangedConnect(self: Pin <&mut MyObjectQt>, func: fn(Pin<&mut MyObjectQt>, ), conn_type : CxxQtConnectionType) -> CxxQtQMetaObjectConnection;
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[15],
            parse_quote! {
                impl MyObjectQt {
                    #[doc = "Connect the given function pointer to the signal "]
                    #[doc = "trivialPropertyChanged"]
                    #[doc = ", so that when the signal is emitted the function pointer is executed."]
                    #[doc = "\n"]
                    #[doc = "Note that this method uses a AutoConnection connection type."]
                    #[must_use]
                    fn on_trivial_property_changed(self: Pin<&mut MyObjectQt>, func: fn(Pin<&mut MyObjectQt>, )) -> CxxQtQMetaObjectConnection
                    {
                        self.connect_trivial_property_changed(func, CxxQtConnectionType::AutoConnection)
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
                    fn emitOpaquePropertyChanged(self: Pin<&mut MyObjectQt>, );
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
                    fn opaquePropertyChangedConnect(self: Pin <&mut MyObjectQt>, func: fn(Pin<&mut MyObjectQt>, ), conn_type : CxxQtConnectionType) -> CxxQtQMetaObjectConnection;
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[16],
            parse_quote! {
                impl MyObjectQt {
                    #[doc = "Connect the given function pointer to the signal "]
                    #[doc = "opaquePropertyChanged"]
                    #[doc = ", so that when the signal is emitted the function pointer is executed."]
                    #[doc = "\n"]
                    #[doc = "Note that this method uses a AutoConnection connection type."]
                    #[must_use]
                    fn on_opaque_property_changed(self: Pin<&mut MyObjectQt>, func: fn(Pin<&mut MyObjectQt>, )) -> CxxQtQMetaObjectConnection
                    {
                        self.connect_opaque_property_changed(func, CxxQtConnectionType::AutoConnection)
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
                    fn emitUnsafePropertyChanged(self: Pin<&mut MyObjectQt>, );
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
                    fn unsafePropertyChangedConnect(self: Pin <&mut MyObjectQt>, func: fn(Pin<&mut MyObjectQt>, ), conn_type : CxxQtConnectionType) -> CxxQtQMetaObjectConnection;
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[17],
            parse_quote! {
                impl MyObjectQt {
                    #[doc = "Connect the given function pointer to the signal "]
                    #[doc = "unsafePropertyChanged"]
                    #[doc = ", so that when the signal is emitted the function pointer is executed."]
                    #[doc = "\n"]
                    #[doc = "Note that this method uses a AutoConnection connection type."]
                    #[must_use]
                    fn on_unsafe_property_changed(self: Pin<&mut MyObjectQt>, func: fn(Pin<&mut MyObjectQt>, )) -> CxxQtQMetaObjectConnection
                    {
                        self.connect_unsafe_property_changed(func, CxxQtConnectionType::AutoConnection)
                    }
                }
            },
        );
    }
}
