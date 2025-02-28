// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use crate::generator::naming::qobject::QObjectNames;
use crate::naming::Name;
use crate::{
    generator::rust::{fragment::GeneratedRustFragment, signals::generate_rust_signal},
    naming::TypeNames,
    parser::externcxxqt::ParsedExternCxxQt,
    syntax::path::path_compare_str,
};
use quote::{format_ident, quote};
use syn::{parse_quote, Attribute, Result};

impl GeneratedRustFragment {
    pub fn from_extern_cxx_qt(
        extern_cxxqt_block: &ParsedExternCxxQt,
        type_names: &TypeNames,
    ) -> Result<Self> {
        let extern_block_namespace = if let Some(namespace) = &extern_cxxqt_block.namespace {
            quote! { #[namespace = #namespace ] }
        } else {
            quote! {}
        };

        // Add the pass through blocks
        let unsafety = &extern_cxxqt_block.unsafety;
        let items = &extern_cxxqt_block.passthrough_items;
        let mut generated = extern_cxxqt_block
            .qobjects
            .iter()
            .map(|ty| -> Result<GeneratedRustFragment> {
                let mut generated = vec![];

                let qobject_names = QObjectNames::from_extern_qobject(ty, type_names)?;

                let base = ty
                    .base_class
                    .as_ref()
                    .map(|name| type_names.lookup(name))
                    .transpose()?
                    .cloned()
                    .unwrap_or(Name::new(format_ident!("QObject")).with_module(parse_quote! {::cxx_qt}));

                let base_unqualified = base.rust_unqualified();
                let base_qualified = base.rust_qualified();

                let struct_name = ty.name.rust_qualified();
                let struct_name_unqualified = ty.name.rust_unqualified();
                let (upcast_fn, upcast_fn_attrs, upcast_fn_qualified) = qobject_names
                    .cxx_qt_ffi_method("upcastPtr")
                    .into_cxx_parts();
                let (downcast_fn, downcast_fn_attrs, downcast_fn_qualified) = qobject_names
                    .cxx_qt_ffi_method("downcastPtr")
                    .into_cxx_parts();

                generated.push(GeneratedRustFragment {
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
                        impl ::cxx_qt::Upcast<#base_qualified> for #struct_name {
                            unsafe fn upcast_ptr(this: *const Self) -> *const #base_qualified {
                                #upcast_fn_qualified(this)
                            }

                            unsafe fn from_base_ptr(base: *const #base_qualified) -> *const Self {
                                #downcast_fn_qualified(base)
                            }
                        }
                    }],
                });

                let namespace = if let Some(namespace) = &ty.name.namespace() {
                    quote! { #[namespace = #namespace ] }
                } else {
                    quote! {}
                };
                let cpp_name = &ty.name.cxx_unqualified();
                let rust_name = &ty.name.rust_unqualified();
                let vis = &ty.declaration.vis;
                let ident = &ty.name.rust_unqualified();
                let cxx_name = if &rust_name.to_string() == cpp_name {
                    quote! {}
                } else {
                    let cxx_name = cpp_name.to_string();
                    quote! {
                        #[cxx_name = #cxx_name]
                    }
                };
                let cfgs: Vec<&Attribute> = ty
                    .declaration
                    .attrs
                    .iter()
                    .filter(|attr| path_compare_str(attr.meta.path(), &["cfg"]))
                    .collect();
                let docs: Vec<&Attribute> = ty
                    .declaration
                    .attrs
                    .iter()
                    .filter(|attr| path_compare_str(attr.meta.path(), &["doc"]))
                    .collect();
                generated.push(GeneratedRustFragment::from_cxx_item(parse_quote! {
                    #extern_block_namespace
                    #unsafety extern "C++" {
                        #namespace
                        #cxx_name
                        #(#cfgs)*
                        #(#docs)*
                        #vis type #ident;
                    }
                }));
                Ok(GeneratedRustFragment::flatten(generated))
            })
            .collect::<Result<Vec<_>>>()?;

        if !items.is_empty() {
            generated.push(GeneratedRustFragment::from_cxx_item(parse_quote! {
                #extern_block_namespace
                #unsafety extern "C++" {
                    #(#items)*
                }
            }));
        }

        // Build the signals
        for signal in &extern_cxxqt_block.signals {
            let qobject_name = type_names.lookup(&signal.qobject_ident)?;
            generated.push(generate_rust_signal(
                signal,
                qobject_name,
                type_names,
                // Copy the same safety as the extern C++Qt block into the generated extern C++
                extern_cxxqt_block.unsafety.map(|_| quote! { unsafe }),
            )?);
        }

        Ok(GeneratedRustFragment::flatten(generated))
    }
}
