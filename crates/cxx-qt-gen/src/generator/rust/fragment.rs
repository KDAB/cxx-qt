// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::generator::naming::qobject::QObjectNames;
use crate::naming::{Name, TypeNames};
use proc_macro2::Ident;
use quote::format_ident;
use syn::{parse_quote, Item, Result};

#[derive(Default, Eq, PartialEq, Debug)]
pub struct GeneratedRustFragment {
    /// Module for the CXX bridge
    pub cxx_mod_contents: Vec<Item>,
    /// Items for the CXX-Qt module
    pub cxx_qt_mod_contents: Vec<Item>,
}

impl GeneratedRustFragment {
    pub fn append(&mut self, other: Self) {
        self.cxx_mod_contents.extend(other.cxx_mod_contents);
        self.cxx_qt_mod_contents.extend(other.cxx_qt_mod_contents);
    }

    pub fn qobject_import() -> Self {
        Self {
            cxx_mod_contents: vec![parse_quote! {
                extern "C++" {
                    #[doc(hidden)]
                    #[namespace=""]
                    type QObject = cxx_qt::QObject;
                }
            }],
            cxx_qt_mod_contents: vec![],
        }
    }

    /// Generate the required trait function implementations for casting QObjects
    pub fn generate_casting_impl(
        qobject_names: &QObjectNames,
        type_names: &TypeNames,
        type_name: &Name,
        base_class: &Option<Ident>,
    ) -> Result<Self> {
        // Create name from base ident
        let base = base_class
            .as_ref()
            .map(|name| type_names.lookup(name))
            .transpose()?
            .cloned()
            .unwrap_or(Name::new(format_ident!("QObject")).with_module(parse_quote! {::cxx_qt}));

        let base_unqualified = base.rust_unqualified();
        let base_qualified = base.rust_qualified();

        let struct_name = type_name.rust_qualified();
        let struct_name_unqualified = type_name.rust_unqualified();

        // Create ffi function names
        let (upcast_fn, upcast_fn_attrs, upcast_fn_qualified) = qobject_names
            .cxx_qt_ffi_method("upcastPtr")
            .into_cxx_parts();
        let (downcast_fn, downcast_fn_attrs, downcast_fn_qualified) = qobject_names
            .cxx_qt_ffi_method("downcastPtr")
            .into_cxx_parts();

        Ok(Self {
            cxx_mod_contents: vec![parse_quote! {
                extern "C++" {
                    #[doc(hidden)]
                    #(#upcast_fn_attrs)*
                    unsafe fn #upcast_fn(thiz: *const #struct_name_unqualified) -> *const #base_unqualified;

                    #[doc(hidden)]
                    #(#downcast_fn_attrs)*
                    unsafe fn #downcast_fn(base: *const #base_unqualified) -> *const #struct_name_unqualified;
                }
            }],
            cxx_qt_mod_contents: vec![parse_quote! {
                unsafe impl ::cxx_qt::casting::Upcast<#base_qualified> for #struct_name {
                    unsafe fn upcast_ptr(this: *const Self) -> *const #base_qualified {
                        #upcast_fn_qualified(this)
                    }

                        unsafe fn from_base_ptr(base: *const #base_qualified) -> *const Self {
                            #downcast_fn_qualified(base)
                        }
                    }
                },
                // Add back once we figure out the bug with QObject, for automatic transitive casts
                // parse_quote! {
                //     unsafe impl ::cxx_qt::MainCast for #struct_name {
                //        type Base = #base_qualified;
                //     }
                // }
            ],
        })
    }

    // Create a singular GeneratedRustFragment from a Vector of multiple
    pub fn flatten(others: Vec<Self>) -> Self {
        let mut this = Self::default();
        for other in others {
            this.append(other);
        }
        this
    }

    pub fn from_cxx_item(contents: Item) -> Self {
        Self {
            cxx_mod_contents: vec![contents],
            ..Default::default()
        }
    }
}
