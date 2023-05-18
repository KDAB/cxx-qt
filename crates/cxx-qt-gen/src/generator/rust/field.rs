// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{
    generator::{
        naming::{property::QPropertyName, qobject::QObjectName},
        rust::{fragment::RustFragmentPair, qobject::GeneratedRustQObjectBlocks},
    },
    parser::property::ParsedRustField,
};
use quote::quote;
use syn::Result;

pub fn generate_rust_fields(
    fields: &Vec<ParsedRustField>,
    qobject_idents: &QObjectName,
) -> Result<GeneratedRustQObjectBlocks> {
    let mut generated = GeneratedRustQObjectBlocks::default();
    let cpp_class_name_rust = &qobject_idents.cpp_class.rust;

    for field in fields {
        let idents = QPropertyName::from(&field.ident);
        let getter_rust = &idents.getter.rust;
        let getter_mutable_rust = &idents.getter_mutable.rust;
        let setter_rust = &idents.setter.rust;
        let ident = &idents.name.rust;
        let ty = &field.ty;
        let vis = &field.vis;

        let fragment = RustFragmentPair {
            cxx_bridge: vec![],
            implementation: vec![
                quote! {
                    impl #cpp_class_name_rust {
                        #vis fn #getter_rust(&self) -> &#ty {
                            &self.rust().#ident
                        }
                    }
                },
                quote! {
                    impl #cpp_class_name_rust {
                        #vis fn #getter_mutable_rust<'a>(self: Pin<&'a mut Self>) -> &'a mut #ty {
                            unsafe { &mut self.rust_mut().get_unchecked_mut().#ident }
                        }
                    }
                },
                quote! {
                    impl #cpp_class_name_rust {
                        #vis fn #setter_rust(self: Pin<&mut Self>, value: #ty) {
                            unsafe {
                                self.rust_mut().#ident = value;
                            }
                        }
                    }
                },
            ],
        };

        generated
            .cxx_qt_mod_contents
            .append(&mut fragment.implementation_as_items()?);
    }

    Ok(generated)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{
        generator::naming::qobject::tests::create_qobjectname,
        tests::{assert_tokens_eq, tokens_to_syn},
    };
    use quote::{format_ident, quote};

    #[test]
    fn test_generate_rust_properties() {
        let properties = vec![
            ParsedRustField {
                ident: format_ident!("private_field"),
                ty: tokens_to_syn::<syn::Type>(quote! { i32 }),
                vis: syn::Visibility::Inherited,
            },
            ParsedRustField {
                ident: format_ident!("public_field"),
                ty: tokens_to_syn::<syn::Type>(quote! { f64 }),
                vis: tokens_to_syn::<syn::Visibility>(quote! { pub }),
            },
        ];
        let qobject_idents = create_qobjectname();

        let generated = generate_rust_fields(&properties, &qobject_idents).unwrap();

        // Check that we have the expected number of blocks
        assert_eq!(generated.cxx_mod_contents.len(), 0);
        assert_eq!(generated.cxx_qt_mod_contents.len(), 6);

        // Private Field
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[0],
            quote! {
                impl MyObjectQt {
                    fn private_field(&self) -> &i32 {
                        &self.rust().private_field
                    }
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[1],
            quote! {
                impl MyObjectQt {
                    fn private_field_mut<'a>(self: Pin<&'a mut Self>) -> &'a mut i32 {
                        unsafe { &mut self.rust_mut().get_unchecked_mut().private_field }
                    }
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[2],
            quote! {
                impl MyObjectQt {
                    fn set_private_field(self: Pin<&mut Self>, value: i32) {
                        unsafe {
                            self.rust_mut().private_field = value;
                        }
                    }
                }
            },
        );

        // Public Field
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[3],
            quote! {
                impl MyObjectQt {
                    pub fn public_field(&self) -> &f64 {
                        &self.rust().public_field
                    }
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[4],
            quote! {
                impl MyObjectQt {
                    pub fn public_field_mut<'a>(self: Pin<&'a mut Self>) -> &'a mut f64 {
                        unsafe { &mut self.rust_mut().get_unchecked_mut().public_field }
                    }
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[5],
            quote! {
                impl MyObjectQt {
                    pub fn set_public_field(self: Pin<&mut Self>, value: f64) {
                        unsafe {
                            self.rust_mut().public_field = value;
                        }
                    }
                }
            },
        );
    }
}
