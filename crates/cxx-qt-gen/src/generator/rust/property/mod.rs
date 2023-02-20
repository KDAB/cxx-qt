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
    parser::property::{ParsedQProperty, MaybeCustomFn},
};
use syn::Result;

pub fn generate_rust_properties(
    properties: &Vec<ParsedQProperty>,
    qobject_idents: &QObjectName,
) -> Result<GeneratedRustQObjectBlocks> {
    let mut generated = GeneratedRustQObjectBlocks::default();

    for property in properties {
        let idents = QPropertyName::from(property);

        let getter_setter_not_explicit = property.get.is_none() && property.set.is_none();

        // Getters
        let default_getter = match property.get {
            Some(MaybeCustomFn::Default) => true,
            _ => false,
        };
        if getter_setter_not_explicit || default_getter {
            let getter = getter::generate(&idents, qobject_idents, &property.ty);
            generated
                .cxx_mod_contents
                .append(&mut getter.cxx_bridge_as_items()?);
            generated
                .cxx_qt_mod_contents
                .append(&mut getter.implementation_as_items()?);
        } else if let Some(getter_attr) = &property.get {
            if let MaybeCustomFn::Custom(getter_fn) = getter_attr {
                let getter = getter::generate_custom(&idents, qobject_idents, getter_fn, &property.ty);
                generated
                    .cxx_mod_contents
                    .append(&mut getter.cxx_bridge_as_items().unwrap());
                generated
                    .cxx_qt_mod_contents
                    .append(&mut getter.implementation_as_items().unwrap());
            }
        }

        // Setters
        let default_setter = match property.set {
            Some(MaybeCustomFn::Default) => true,
            _ => false,
        };
        if getter_setter_not_explicit || default_setter {
            let setter = setter::generate(&idents, qobject_idents, &property.ty);
            generated
                .cxx_mod_contents
                .append(&mut setter.cxx_bridge_as_items()?);
            generated
                .cxx_qt_mod_contents
                .append(&mut setter.implementation_as_items()?);
        }

        // Signals
        let notify = signal::generate(&idents, qobject_idents);
        generated
            .cxx_mod_contents
            .append(&mut notify.cxx_bridge_as_items()?);
        generated
            .cxx_qt_mod_contents
            .append(&mut notify.implementation_as_items()?);
    }

    Ok(generated)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{generator::naming::qobject::tests::create_qobjectname, tests::tokens_to_syn};
    use quote::{format_ident, quote};

    #[test]
    fn test_generate_rust_properties() {
        let properties = vec![
            ParsedQProperty {
                ident: format_ident!("trivial_property"),
                ty: tokens_to_syn::<syn::Type>(quote! { i32 }),
                vis: syn::Visibility::Inherited,
                cxx_type: None,
                get: None,
                set: None,
            },
            ParsedQProperty {
                ident: format_ident!("opaque_property"),
                ty: tokens_to_syn::<syn::Type>(quote! { UniquePtr<QColor> }),
                vis: tokens_to_syn::<syn::Visibility>(quote! { pub }),
                cxx_type: Some("QColor".to_owned()),
                get: None,
                set: None,
            },
            ParsedQProperty {
                ident: format_ident!("unsafe_property"),
                ty: tokens_to_syn::<syn::Type>(quote! { *mut T }),
                vis: syn::Visibility::Inherited,
                cxx_type: None,
                get: None,
                set: None,
            },
            ParsedQProperty {
                ident: format_ident!("custom_getter"),
                ty: tokens_to_syn::<syn::Type>(quote! { i32 }),
                vis: syn::Visibility::Inherited,
                cxx_type: None,
                get: Some(MaybeCustomFn::Custom(Box::new(tokens_to_syn::<syn::Expr>(quote! { Self::custom_getter_fn })))),
                set: None,
            },
        ];
        let qobject_idents = create_qobjectname();

        let generated = generate_rust_properties(&properties, &qobject_idents).unwrap();

        // Check that we have the expected number of blocks
        assert_eq!(generated.cxx_mod_contents.len(), 11);
        assert_eq!(generated.cxx_qt_mod_contents.len(), 18);

        // Trivial Property

        // Getter
        assert_eq!(
            generated.cxx_mod_contents[0],
            tokens_to_syn::<syn::Item>(quote! {
                extern "Rust" {
                    #[cxx_name = "getTrivialProperty"]
                    unsafe fn trivial_property<'a>(self: &'a MyObject, cpp: &'a MyObjectQt) -> &'a i32;
                }
            })
        );
        assert_eq!(
            generated.cxx_qt_mod_contents[0],
            tokens_to_syn::<syn::Item>(quote! {
                impl MyObject {
                    pub fn trivial_property<'a>(&'a self, cpp: &'a MyObjectQt) -> &'a i32 {
                        cpp.trivial_property()
                    }
                }
            })
        );
        assert_eq!(
            generated.cxx_qt_mod_contents[1],
            tokens_to_syn::<syn::Item>(quote! {
                impl MyObjectQt {
                    pub fn trivial_property(&self) -> &i32 {
                        &self.rust().trivial_property
                    }
                }
            })
        );
        assert_eq!(
            generated.cxx_qt_mod_contents[2],
            tokens_to_syn::<syn::Item>(quote! {
                impl MyObjectQt {
                    pub unsafe fn trivial_property_mut<'a>(mut self: Pin<&'a mut Self>) -> &'a mut i32 {
                        &mut self.rust_mut().get_unchecked_mut().trivial_property
                    }
                }
            })
        );

        // Setters
        assert_eq!(
            generated.cxx_mod_contents[1],
            tokens_to_syn::<syn::Item>(quote! {
                extern "Rust" {
                    #[cxx_name = "setTrivialProperty"]
                    fn set_trivial_property(self: &mut MyObject, cpp: Pin<&mut MyObjectQt>, value: i32);
                }
            })
        );
        assert_eq!(
            generated.cxx_qt_mod_contents[3],
            tokens_to_syn::<syn::Item>(quote! {
                impl MyObject {
                    pub fn set_trivial_property(&mut self, cpp: Pin<&mut MyObjectQt>, value: i32) {
                        cpp.set_trivial_property(value);
                    }
                }
            })
        );
        assert_eq!(
            generated.cxx_qt_mod_contents[4],
            tokens_to_syn::<syn::Item>(quote! {
                impl MyObjectQt {
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
            })
        );

        // Notify
        assert_eq!(
            generated.cxx_mod_contents[2],
            tokens_to_syn::<syn::Item>(quote! {
                unsafe extern "C++" {
                    #[rust_name = "trivial_property_changed"]
                    fn trivialPropertyChanged(self: Pin<&mut MyObjectQt>);
                }
            })
        );

        // Opaque Property

        // Getter
        assert_eq!(
            generated.cxx_mod_contents[3],
            tokens_to_syn::<syn::Item>(quote! {
                extern "Rust" {
                    #[cxx_name = "getOpaqueProperty"]
                    unsafe fn opaque_property<'a>(self: &'a MyObject, cpp: &'a MyObjectQt) -> &'a UniquePtr<QColor>;
                }
            })
        );
        assert_eq!(
            generated.cxx_qt_mod_contents[5],
            tokens_to_syn::<syn::Item>(quote! {
                impl MyObject {
                    pub fn opaque_property<'a>(&'a self, cpp: &'a MyObjectQt) -> &'a UniquePtr<QColor> {
                        cpp.opaque_property()
                    }
                }
            })
        );
        assert_eq!(
            generated.cxx_qt_mod_contents[6],
            tokens_to_syn::<syn::Item>(quote! {
                impl MyObjectQt {
                    pub fn opaque_property(&self) -> &UniquePtr<QColor> {
                        &self.rust().opaque_property
                    }
                }
            })
        );
        assert_eq!(
            generated.cxx_qt_mod_contents[7],
            tokens_to_syn::<syn::Item>(quote! {
                impl MyObjectQt {
                    pub unsafe fn opaque_property_mut<'a>(mut self: Pin<&'a mut Self>) -> &'a mut UniquePtr<QColor> {
                        &mut self.rust_mut().get_unchecked_mut().opaque_property
                    }
                }
            })
        );

        // Setters
        assert_eq!(
            generated.cxx_mod_contents[4],
            tokens_to_syn::<syn::Item>(quote! {
                extern "Rust" {
                    #[cxx_name = "setOpaqueProperty"]
                    fn set_opaque_property(self: &mut MyObject, cpp: Pin<&mut MyObjectQt>, value: UniquePtr<QColor>);
                }
            })
        );
        assert_eq!(
            generated.cxx_qt_mod_contents[8],
            tokens_to_syn::<syn::Item>(quote! {
                impl MyObject {
                    pub fn set_opaque_property(&mut self, cpp: Pin<&mut MyObjectQt>, value: UniquePtr<QColor>) {
                        cpp.set_opaque_property(value);
                    }
                }
            })
        );
        assert_eq!(
            generated.cxx_qt_mod_contents[9],
            tokens_to_syn::<syn::Item>(quote! {
                impl MyObjectQt {
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
            })
        );

        // Notify
        assert_eq!(
            generated.cxx_mod_contents[5],
            tokens_to_syn::<syn::Item>(quote! {
                unsafe extern "C++" {
                    #[rust_name = "opaque_property_changed"]
                    fn opaquePropertyChanged(self: Pin<&mut MyObjectQt>);
                }
            })
        );

        // Unsafe Property

        // Getter
        assert_eq!(
            generated.cxx_mod_contents[6],
            tokens_to_syn::<syn::Item>(quote! {
                extern "Rust" {
                    #[cxx_name = "getUnsafeProperty"]
                    unsafe fn unsafe_property<'a>(self: &'a MyObject, cpp: &'a MyObjectQt) -> &'a *mut T;
                }
            })
        );
        assert_eq!(
            generated.cxx_qt_mod_contents[10],
            tokens_to_syn::<syn::Item>(quote! {
                impl MyObject {
                    pub fn unsafe_property<'a>(&'a self, cpp: &'a MyObjectQt) -> &'a *mut T {
                        cpp.unsafe_property()
                    }
                }
            })
        );
        assert_eq!(
            generated.cxx_qt_mod_contents[11],
            tokens_to_syn::<syn::Item>(quote! {
                impl MyObjectQt {
                    pub fn unsafe_property(&self) -> &*mut T {
                        &self.rust().unsafe_property
                    }
                }
            })
        );
        assert_eq!(
            generated.cxx_qt_mod_contents[12],
            tokens_to_syn::<syn::Item>(quote! {
                impl MyObjectQt {
                    pub unsafe fn unsafe_property_mut<'a>(mut self: Pin<&'a mut Self>) -> &'a mut *mut T {
                        &mut self.rust_mut().get_unchecked_mut().unsafe_property
                    }
                }
            })
        );

        // Setters
        assert_eq!(
            generated.cxx_mod_contents[7],
            tokens_to_syn::<syn::Item>(quote! {
                extern "Rust" {
                    #[cxx_name = "setUnsafeProperty"]
                    unsafe fn set_unsafe_property(self: &mut MyObject, cpp: Pin<&mut MyObjectQt>, value: *mut T);
                }
            })
        );
        assert_eq!(
            generated.cxx_qt_mod_contents[13],
            tokens_to_syn::<syn::Item>(quote! {
                impl MyObject {
                    pub fn set_unsafe_property(&mut self, cpp: Pin<&mut MyObjectQt>, value: *mut T) {
                        cpp.set_unsafe_property(value);
                    }
                }
            })
        );
        assert_eq!(
            generated.cxx_qt_mod_contents[14],
            tokens_to_syn::<syn::Item>(quote! {
                impl MyObjectQt {
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
            })
        );

        // Notify
        assert_eq!(
            generated.cxx_mod_contents[8],
            tokens_to_syn::<syn::Item>(quote! {
                unsafe extern "C++" {
                    #[rust_name = "unsafe_property_changed"]
                    fn unsafePropertyChanged(self: Pin<&mut MyObjectQt>);
                }
            })
        );

        // Custom getter

        // Getter
        assert_eq!(
            generated.cxx_mod_contents[9],
            tokens_to_syn::<syn::Item>(quote! {
                extern "Rust" {
                    #[cxx_name = "getCustomGetter"]
                    unsafe fn custom_getter<'a>(self: &'a MyObject, cpp: &'a MyObjectQt) -> &'a i32;
                }
            })
        );
        assert_eq!(
            generated.cxx_qt_mod_contents[15],
            tokens_to_syn::<syn::Item>(quote! {
                impl MyObject {
                    pub fn custom_getter<'a>(&'a self, cpp: &'a MyObjectQt) -> &'a i32 {
                        cpp.custom_getter()
                    }
                }
            })
        );
        assert_eq!(
            generated.cxx_qt_mod_contents[16],
            tokens_to_syn::<syn::Item>(quote! {
                impl MyObjectQt {
                    pub fn custom_getter(&self) -> &i32 {
                        (Self::custom_getter_fn)(&self.rust())
                    }
                }
            })
        );
        assert_eq!(
            generated.cxx_qt_mod_contents[17],
            tokens_to_syn::<syn::Item>(quote! {
                impl MyObjectQt {
                    pub unsafe fn custom_getter_mut<'a>(mut self: Pin<&'a mut Self>) -> &'a mut i32 {
                        (Self::custom_getter_fn)(&mut self.rust_mut().get_unchecked_mut())
                    }
                }
            })
        );

        // Notify
        assert_eq!(
            generated.cxx_mod_contents[10],
            tokens_to_syn::<syn::Item>(quote! {
                unsafe extern "C++" {
                    #[rust_name = "custom_getter_changed"]
                    fn customGetterChanged(self: Pin<&mut MyObjectQt>);
                }
            })
        );
    }
}
