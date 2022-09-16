// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

pub mod emitter;
pub mod getter;
pub mod setter;

use crate::{
    generator::{
        naming::{property::QPropertyName, qobject::QObjectName},
        rust::qobject::GeneratedRustQObjectBlocks,
    },
    parser::property::ParsedQProperty,
};
use syn::Result;

pub fn generate_rust_properties(
    properties: &Vec<ParsedQProperty>,
    qobject_idents: &QObjectName,
) -> Result<GeneratedRustQObjectBlocks> {
    let mut generated = GeneratedRustQObjectBlocks::default();

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
        let emitter = emitter::generate(&idents, qobject_idents);
        generated
            .cxx_mod_contents
            .append(&mut emitter.cxx_bridge_as_items()?);
        generated
            .cxx_qt_mod_contents
            .append(&mut emitter.implementation_as_items()?);
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
            },
            ParsedQProperty {
                ident: format_ident!("opaque_property"),
                ty: tokens_to_syn::<syn::Type>(quote! { UniquePtr<QColor> }),
                vis: tokens_to_syn::<syn::Visibility>(quote! { pub }),
                cxx_type: Some("QColor".to_owned()),
            },
        ];
        let qobject_idents = create_qobjectname();

        let generated = generate_rust_properties(&properties, &qobject_idents).unwrap();

        // Check that we have the expected number of blocks
        assert_eq!(generated.cxx_mod_contents.len(), 6);
        assert_eq!(generated.cxx_qt_mod_contents.len(), 8);

        // Trivial Property

        // Getter
        assert_eq!(
            generated.cxx_mod_contents[0],
            tokens_to_syn::<syn::Item>(quote! {
                extern "Rust" {
                    #[cxx_name = "getTrivialProperty"]
                    unsafe fn get_trivial_property<'a>(self: &'a MyObject, cpp: &'a MyObjectQt) -> &'a i32;
                }
            })
        );
        assert_eq!(
            generated.cxx_qt_mod_contents[0],
            tokens_to_syn::<syn::Item>(quote! {
                impl MyObject {
                    pub fn get_trivial_property<'a>(&'a self, cpp: &'a MyObjectQt) -> &'a i32 {
                        cpp.get_trivial_property()
                    }
                }
            })
        );
        assert_eq!(
            generated.cxx_qt_mod_contents[1],
            tokens_to_syn::<syn::Item>(quote! {
                impl MyObjectQt {
                    pub fn get_trivial_property(&self) -> &i32 {
                        &self.rust().trivial_property
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
            generated.cxx_qt_mod_contents[2],
            tokens_to_syn::<syn::Item>(quote! {
                impl MyObject {
                    pub fn set_trivial_property(&mut self, cpp: Pin<&mut MyObjectQt>, value: i32) {
                        cpp.set_trivial_property(value);
                    }
                }
            })
        );
        assert_eq!(
            generated.cxx_qt_mod_contents[3],
            tokens_to_syn::<syn::Item>(quote! {
                impl MyObjectQt {
                    pub fn set_trivial_property(mut self: Pin<&mut Self>, value: i32) {
                        unsafe {
                            self.as_mut().rust_mut().trivial_property = value;
                        }
                        self.as_mut().emit_trivial_property_changed();
                    }
                }
            })
        );

        // Emitter
        assert_eq!(
            generated.cxx_mod_contents[2],
            tokens_to_syn::<syn::Item>(quote! {
                unsafe extern "C++" {
                    #[rust_name = "emit_trivial_property_changed"]
                    fn emitTrivialPropertyChanged(self: Pin<&mut MyObjectQt>);
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
                    unsafe fn get_opaque_property<'a>(self: &'a MyObject, cpp: &'a MyObjectQt) -> &'a UniquePtr<QColor>;
                }
            })
        );
        assert_eq!(
            generated.cxx_qt_mod_contents[4],
            tokens_to_syn::<syn::Item>(quote! {
                impl MyObject {
                    pub fn get_opaque_property<'a>(&'a self, cpp: &'a MyObjectQt) -> &'a UniquePtr<QColor> {
                        cpp.get_opaque_property()
                    }
                }
            })
        );
        assert_eq!(
            generated.cxx_qt_mod_contents[5],
            tokens_to_syn::<syn::Item>(quote! {
                impl MyObjectQt {
                    pub fn get_opaque_property(&self) -> &UniquePtr<QColor> {
                        &self.rust().opaque_property
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
            generated.cxx_qt_mod_contents[6],
            tokens_to_syn::<syn::Item>(quote! {
                impl MyObject {
                    pub fn set_opaque_property(&mut self, cpp: Pin<&mut MyObjectQt>, value: UniquePtr<QColor>) {
                        cpp.set_opaque_property(value);
                    }
                }
            })
        );
        assert_eq!(
            generated.cxx_qt_mod_contents[7],
            tokens_to_syn::<syn::Item>(quote! {
                impl MyObjectQt {
                    pub fn set_opaque_property(mut self: Pin<&mut Self>, value: UniquePtr<QColor>) {
                        unsafe {
                            self.as_mut().rust_mut().opaque_property = value;
                        }
                        self.as_mut().emit_opaque_property_changed();
                    }
                }
            })
        );

        // Emitter
        assert_eq!(
            generated.cxx_mod_contents[5],
            tokens_to_syn::<syn::Item>(quote! {
                unsafe extern "C++" {
                    #[rust_name = "emit_opaque_property_changed"]
                    fn emitOpaquePropertyChanged(self: Pin<&mut MyObjectQt>);
                }
            })
        );
    }
}
