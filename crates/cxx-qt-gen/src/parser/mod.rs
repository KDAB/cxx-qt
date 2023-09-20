// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

pub mod constructor;
pub mod cxxqtdata;
pub mod externcxxqt;
pub mod inherit;
pub mod mappings;
pub mod method;
pub mod parameter;
pub mod property;
pub mod qenum;
pub mod qnamespace;
pub mod qobject;
pub mod signals;

use crate::syntax::{attribute::attribute_take_path, expr::expr_to_string};
use cxxqtdata::ParsedCxxQtData;
use syn::{
    punctuated::Punctuated, spanned::Spanned, token::Brace, Error, ItemMod, Meta, Result, Token,
};

/// A struct representing a module block with CXX-Qt relevant [syn::Item]'s
/// parsed into ParsedCxxQtData, to be used later to generate Rust & C++ code.
///
/// [syn::Item]'s that are not handled specially by CXX-Qt are passed through for CXX to process.
pub struct Parser {
    /// The module which unknown (eg CXX) blocks are stored into
    pub passthrough_module: ItemMod,
    /// Any CXX-Qt data that needs generation later
    pub cxx_qt_data: ParsedCxxQtData,
    /// The stem of the file that the CXX headers for this module will be generated into
    pub cxx_file_stem: String,
}

impl Parser {
    /// Constructs a Parser object from a given [syn::ItemMod] block
    pub fn from(mut module: ItemMod) -> Result<Self> {
        let mut namespace = "".to_owned();
        let mut others = vec![];
        let mut cxx_file_stem = module.ident.to_string();

        // Remove the cxx_qt::bridge attribute
        if let Some(attr) = attribute_take_path(&mut module.attrs, &["cxx_qt", "bridge"]) {
            // If we are no #[cxx_qt::bridge] but #[cxx_qt::bridge(A = B)] then process
            if !matches!(attr.meta, Meta::Path(_)) {
                let nested =
                    attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;
                for meta in nested {
                    match meta {
                        Meta::NameValue(name_value) => {
                            // Parse any namespace in the cxx_qt::bridge macro
                            if name_value.path.is_ident("namespace") {
                                namespace = expr_to_string(&name_value.value)?;
                            // Parse any custom file stem
                            } else if name_value.path.is_ident("cxx_file_stem") {
                                cxx_file_stem = expr_to_string(&name_value.value)?;
                            }
                        }
                        _others => {}
                    }
                }
            }
        } else {
            return Err(Error::new(
                module.span(),
                "Tried to parse a module which doesn't have a cxx_qt::bridge attribute",
            ));
        }

        let mut cxx_qt_data = ParsedCxxQtData::new(module.ident.clone(), namespace);

        // Check that there are items in the module
        if let Some(mut items) = module.content {
            // Find any QObject structs
            cxx_qt_data.find_qobject_types(&items.1)?;

            // Loop through items and load into qobject or others and populate mappings
            for item in items.1.drain(..) {
                // Try to find any CXX-Qt items, if found add them to the relevant
                // qobject or extern C++Qt block. Otherwise return them to be added to other
                if let Some(other) = cxx_qt_data.parse_cxx_qt_item(item)? {
                    // Load any CXX name mappings
                    cxx_qt_data.populate_mappings_from_item(&other)?;

                    // Unknown item so add to the other list
                    others.push(other);
                }
            }
        }

        // Create a new module using only items that are not CXX-Qt items
        module.content = Some((Brace::default(), others));

        // Return the successful Parser object
        Ok(Self {
            passthrough_module: module,
            cxx_qt_data,
            cxx_file_stem,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use quote::format_ident;
    use syn::{parse_quote, ItemMod, Type};

    /// Helper which returns a f64 as a [syn::Type]
    pub fn f64_type() -> Type {
        parse_quote! { f64 }
    }

    #[test]
    fn test_parser_from_empty_module() {
        let module: ItemMod = parse_quote! {
            #[cxx_qt::bridge]
            mod ffi {}
        };
        let parser = Parser::from(module).unwrap();
        let expected_module: ItemMod = parse_quote! {
            mod ffi {}
        };
        assert_eq!(parser.passthrough_module, expected_module);
        assert_eq!(parser.cxx_qt_data.namespace, "");
        assert_eq!(parser.cxx_qt_data.qobjects.len(), 0);
    }

    #[test]
    fn test_parser_from_cxx_items() {
        let module: ItemMod = parse_quote! {
            #[cxx_qt::bridge]
            mod ffi {
                extern "Rust" {
                    fn test();
                }
            }
        };
        let parser = Parser::from(module).unwrap();
        let expected_module: ItemMod = parse_quote! {
            mod ffi {
                extern "Rust" {
                    fn test();
                }
            }
        };
        assert_eq!(parser.passthrough_module, expected_module);
        assert_eq!(parser.cxx_qt_data.namespace, "");
        assert_eq!(parser.cxx_qt_data.qobjects.len(), 0);
    }

    #[test]
    fn test_parser_from_cxx_qt_items() {
        let module: ItemMod = parse_quote! {
            #[cxx_qt::bridge(namespace = "cxx_qt")]
            mod ffi {
                extern "RustQt" {
                    #[qobject]
                    type MyObject = super::MyObjectRust;
                }

                unsafe extern "RustQt" {
                    #[qsignal]
                    fn ready(self: Pin<&mut MyObject>);
                }
            }
        };
        let parser = Parser::from(module.clone()).unwrap();

        assert_ne!(parser.passthrough_module, module);

        assert_eq!(parser.passthrough_module.attrs.len(), 0);
        assert_eq!(parser.passthrough_module.ident, "ffi");
        assert_eq!(parser.passthrough_module.content.unwrap().1.len(), 0);
        assert_eq!(parser.cxx_qt_data.namespace, "cxx_qt");
        assert_eq!(parser.cxx_qt_data.qobjects.len(), 1);
        assert_eq!(parser.cxx_qt_data.cxx_mappings.qualified.len(), 1);
        assert_eq!(
            parser
                .cxx_qt_data
                .cxx_mappings
                .qualified
                .get(&format_ident!("MyObject"))
                .unwrap(),
            &parse_quote! { ffi::MyObject }
        );
    }

    #[test]
    fn test_parser_from_cxx_and_cxx_qt_items() {
        let module: ItemMod = parse_quote! {
            #[cxx_qt::bridge]
            mod ffi {
                extern "RustQt" {
                    #[qobject]
                    type MyObject = super::MyObjectRust;
                }

                unsafe extern "RustQt" {
                    #[qsignal]
                    fn ready(self: Pin<&mut MyObject>);
                }

                extern "Rust" {
                    fn test();
                }
            }
        };
        let parser = Parser::from(module.clone()).unwrap();

        assert_ne!(parser.passthrough_module, module);

        assert_eq!(parser.passthrough_module.attrs.len(), 0);
        assert_eq!(parser.passthrough_module.ident, "ffi");
        assert_eq!(parser.passthrough_module.content.unwrap().1.len(), 1);
        assert_eq!(parser.cxx_qt_data.namespace, "");
        assert_eq!(parser.cxx_qt_data.qobjects.len(), 1);
    }

    #[test]
    fn test_parser_from_error() {
        let module: ItemMod = parse_quote! {
            #[cxx_qt::bridge]
            mod ffi {
                extern "RustQt" {
                    #[qobject]
                    type MyObject = super::MyObjectRust;
                }

                unsafe extern "RustQt" {
                    #[qsignal]
                    fn ready(self: Pin<&mut UnknownObject>);
                }
            }
        };
        let parser = Parser::from(module);
        assert!(parser.is_err());
    }

    #[test]
    fn test_parser_from_error_no_attribute() {
        let module: ItemMod = parse_quote! {
            mod ffi {
                extern "Rust" {
                    fn test();
                }
            }
        };
        let parser = Parser::from(module);
        assert!(parser.is_err());
    }

    #[test]
    fn test_cxx_qobject_namespace() {
        let module: ItemMod = parse_quote! {
            #[cxx_qt::bridge(namespace = "bridge_namespace")]
            mod ffi {
                extern "RustQt" {
                    #[qobject]
                    type MyObjectA = super::MyObjectARust;

                    #[qobject]
                    #[namespace = "type_namespace"]
                    type MyObjectB = super::MyObjectBRust;
                }
            }
        };
        let parser = Parser::from(module).unwrap();
        assert_eq!(parser.cxx_qt_data.cxx_mappings.namespaces.len(), 2);
        assert_eq!(
            parser
                .cxx_qt_data
                .cxx_mappings
                .namespaces
                .get("MyObjectA")
                .unwrap(),
            "bridge_namespace"
        );
        assert_eq!(
            parser
                .cxx_qt_data
                .cxx_mappings
                .namespaces
                .get("MyObjectB")
                .unwrap(),
            "type_namespace"
        );
    }
}
