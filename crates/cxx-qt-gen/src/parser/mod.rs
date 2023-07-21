// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

pub mod constructor;
pub mod cxxqtdata;
pub mod inherit;
pub mod invokable;
pub mod parameter;
pub mod property;
pub mod qobject;
pub mod signals;

use crate::syntax::attribute::{attribute_find_path, attribute_tokens_to_map, AttributeDefault};
use cxxqtdata::ParsedCxxQtData;
use syn::{
    spanned::Spanned, token::Brace, Error, Ident, ItemMod, LitStr, Path, PathSegment, Result,
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
        let mut cxx_qt_data = ParsedCxxQtData::default();
        let mut others = vec![];
        let mut cxx_file_stem = module.ident.to_string();

        // Remove the cxx_qt::bridge attribute
        if let Some(index) = attribute_find_path(&module.attrs, &["cxx_qt", "bridge"]) {
            let attr_map = attribute_tokens_to_map::<Ident, LitStr>(
                &module.attrs[index],
                AttributeDefault::None,
            )?;
            // Parse any namespace in the cxx_qt::bridge macro
            cxx_qt_data.namespace =
                if let Some(lit_str) = attr_map.get(&quote::format_ident!("namespace")) {
                    lit_str.value()
                } else {
                    "".to_owned()
                };

            // Parse any custom file stem
            if let Some(stem) = attr_map.get(&quote::format_ident!("cxx_file_stem")) {
                cxx_file_stem = stem.value();
            }

            module.attrs.remove(index);
        } else {
            return Err(Error::new(
                module.span(),
                "Tried to parse a module which doesn't have a cxx_qt::bridge attribute",
            ));
        }

        // Check that there are items in the module
        if let Some(mut items) = module.content {
            let bridge_namespace = cxx_qt_data.namespace.clone();

            // Find any QObject structs
            cxx_qt_data.find_qobject_types(&items.1)?;

            // Loop through any qobjects that were found
            if !cxx_qt_data.qobjects.is_empty() {
                for item in items.1.drain(..) {
                    // Try to find any CXX-Qt items, if found add them to the relevant
                    // qobject. Otherwise return them to be added to other
                    if let Some(other) = cxx_qt_data.parse_cxx_qt_item(item)? {
                        // Load any CXX name mappings
                        cxx_qt_data.populate_mappings_from_item(
                            &other,
                            &bridge_namespace,
                            &module.ident,
                        )?;

                        // Unknown item so add to the other list
                        others.push(other);
                    }
                }
            } else {
                // Load any CXX name mappings
                for item in &items.1 {
                    cxx_qt_data.populate_mappings_from_item(
                        item,
                        &bridge_namespace,
                        &module.ident,
                    )?;
                }

                // No qobjects found so pass everything through
                others.extend(items.1);
            }

            // Add all the QObject types to the qualified mappings
            for ident in cxx_qt_data.qobjects.keys() {
                let mut path = Path::from(module.ident.clone());
                path.segments.push(PathSegment::from(ident.clone()));
                cxx_qt_data.qualified_mappings.insert(ident.clone(), path);
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
                    #[cxx_qt::qobject]
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
        assert_eq!(parser.cxx_qt_data.qualified_mappings.len(), 1);
        assert_eq!(
            parser
                .cxx_qt_data
                .qualified_mappings
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
                    #[cxx_qt::qobject]
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
                    #[cxx_qt::qobject]
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
}
